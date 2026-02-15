use std::collections::{HashMap, HashSet, VecDeque};
use std::env;
use std::fs::File;
use std::io::{self, IsTerminal, Write};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::{Duration, Instant};

mod externs;
pub use externs::*;

// Re-export constants for public API
pub use crate::constants::{STAGE_HEIGHT, STAGE_WIDTH};

/// Function pointer type for JIT-compiled Scratch script entry points.
pub type ScriptEntry = unsafe extern "C" fn(*mut RuntimeState);

// ---------------------------------------------------------------------------
// Fiber-based cooperative concurrency
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FiberSyncState {
    /// Thread created, waiting for the scheduler to signal the first resume.
    WaitingToStart,
    /// Fiber is actively executing.
    Running,
    /// Fiber yielded at a yield-point (loop boundary / wait), scheduler may
    /// inspect RuntimeState.
    Yielded,
    /// Fiber finished (the JIT function returned).
    Done,
}

/// Synchronisation primitive shared between a fiber thread and the scheduler.
struct FiberControl {
    state: Mutex<FiberSyncState>,
    condvar: Condvar,
}

impl std::fmt::Debug for FiberControl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let state = self
            .state
            .lock()
            .map(|g| *g)
            .unwrap_or(FiberSyncState::Done);
        f.debug_struct("FiberControl")
            .field("state", &state)
            .finish()
    }
}

impl FiberControl {
    fn new() -> Self {
        Self {
            state: Mutex::new(FiberSyncState::WaitingToStart),
            condvar: Condvar::new(),
        }
    }

    /// Called by the **fiber thread** – block until the scheduler sets the
    /// state to `Running`.
    fn wait_for_resume(&self) {
        let mut guard = self.state.lock().unwrap();
        while *guard != FiberSyncState::Running {
            guard = self.condvar.wait(guard).unwrap();
        }
    }

    /// Called by the **fiber thread** at a yield-point.  Signals `Yielded`
    /// then blocks until the scheduler sets `Running` again.
    fn yield_to_scheduler(&self) {
        {
            let mut guard = self.state.lock().unwrap();
            *guard = FiberSyncState::Yielded;
            self.condvar.notify_all();
        }
        let mut guard = self.state.lock().unwrap();
        while *guard != FiberSyncState::Running {
            guard = self.condvar.wait(guard).unwrap();
        }
    }

    /// Called by the **fiber thread** when the JIT function returns.
    fn signal_done(&self) {
        let mut guard = self.state.lock().unwrap();
        *guard = FiberSyncState::Done;
        self.condvar.notify_all();
    }

    /// Called by the **scheduler** – set state to `Running` and wake the
    /// fiber thread.
    fn resume(&self) {
        let mut guard = self.state.lock().unwrap();
        *guard = FiberSyncState::Running;
        self.condvar.notify_all();
    }

    /// Called by the **scheduler** – block until the fiber reports `Yielded`
    /// or `Done`.
    fn wait_for_yield_or_done(&self) -> FiberSyncState {
        let mut guard = self.state.lock().unwrap();
        loop {
            match *guard {
                FiberSyncState::Yielded | FiberSyncState::Done => return *guard,
                _ => guard = self.condvar.wait(guard).unwrap(),
            }
        }
    }

    fn is_done(&self) -> bool {
        matches!(*self.state.lock().unwrap(), FiberSyncState::Done)
    }
}

/// A single cooperatively-scheduled "thread" that runs one Scratch script.
pub struct Fiber {
    #[allow(dead_code)]
    script_id: u64,
    actor_id: u64,
    /// Tracks which actor is currently loaded in the RuntimeState fields.
    /// Initially equals `actor_id`, but may differ when the fiber is inside
    /// a `run_script` call for a different actor (e.g. during
    /// broadcast-and-wait handler execution).
    current_actor_id: u64,
    control: Arc<FiberControl>,
    handle: Option<thread::JoinHandle<()>>,
}

impl Fiber {
    /// Spawn a new fiber for `script_id` / `actor_id`.  The fiber thread is
    /// created immediately but blocks until `resume()` is called by the
    /// scheduler.
    fn spawn(
        state_ptr: *mut RuntimeState,
        function: ScriptEntry,
        script_id: u64,
        actor_id: u64,
    ) -> Self {
        let control = Arc::new(FiberControl::new());
        let control_for_thread = Arc::clone(&control);

        // Safety: `state_ptr` is valid for the entire duration of program
        // execution.  Only one fiber accesses RuntimeState at a time
        // (enforced by the cooperative turn-taking protocol).
        let raw = state_ptr as usize; // usize is Send

        let handle = thread::spawn(move || {
            let state_ptr = raw as *mut RuntimeState;
            // Wait for the scheduler to tell us to start.
            control_for_thread.wait_for_resume();
            // Execute the JIT-compiled script function.
            unsafe {
                function(state_ptr);
            }
            // Tell the scheduler we are done.
            control_for_thread.signal_done();
        });

        Fiber {
            script_id,
            actor_id,
            current_actor_id: actor_id,
            control,
            handle: Some(handle),
        }
    }

    fn is_done(&self) -> bool {
        self.control.is_done()
    }

    fn join(&mut self) {
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}
const EMPTY_STRING_ID: usize = 0;
const STRING_TAG_MASK: u64 = 0x7fff_0000_0000_0000;
const STRING_TAG_BITS: u64 = 0x7ff9_0000_0000_0000;
const STRING_PAYLOAD_MASK: u64 = 0x0000_ffff_ffff_ffff;
const DEFAULT_LIVE_CANVAS_SYNC_INTERVAL: Duration = Duration::from_millis(16);
const FRAME_SLEEP_COARSE_MARGIN: Duration = Duration::from_micros(800);
// scratch-vm Sequencer uses WORK_TIME = currentStepTime * 0.75.
const SCRATCH_VM_WORK_TIME_RATIO: f64 = 0.75;

#[derive(Debug, Clone)]
pub struct InputState {
    pub mouse_x: f64,
    pub mouse_y: f64,
    pub mouse_down: bool,
    pub keys_down: HashSet<String>,
}

impl Default for InputState {
    fn default() -> Self {
        Self {
            mouse_x: 0.0,
            mouse_y: 0.0,
            mouse_down: false,
            keys_down: HashSet::new(),
        }
    }
}

#[derive(Debug, Default)]
struct AskPromptInner {
    pending_question: Option<String>,
    pending_answer: Option<String>,
    prompt_serial: u64,
}

#[derive(Debug, Default)]
pub struct AskPromptState {
    inner: Mutex<AskPromptInner>,
    signal: Condvar,
}

impl AskPromptState {
    pub fn prompt_snapshot(&self) -> Option<(u64, String)> {
        self.inner.lock().ok().and_then(|inner| {
            inner
                .pending_question
                .as_ref()
                .map(|question| (inner.prompt_serial, question.clone()))
        })
    }

    pub fn submit_answer(&self, answer: String) -> bool {
        let Ok(mut inner) = self.inner.lock() else {
            return false;
        };
        if inner.pending_question.is_none() {
            return false;
        }
        inner.pending_answer = Some(answer);
        self.signal.notify_all();
        true
    }

    pub fn prompt_and_wait(
        &self,
        question: String,
        stop_requested: Option<&Arc<AtomicBool>>,
    ) -> String {
        let Ok(mut inner) = self.inner.lock() else {
            return String::new();
        };
        inner.prompt_serial = inner.prompt_serial.wrapping_add(1);
        inner.pending_question = Some(question);
        inner.pending_answer = None;
        self.signal.notify_all();

        loop {
            if let Some(answer) = inner.pending_answer.take() {
                inner.pending_question = None;
                self.signal.notify_all();
                return answer;
            }
            if stop_requested.is_some_and(|stop| stop.load(Ordering::Relaxed)) {
                inner.pending_question = None;
                inner.pending_answer = None;
                self.signal.notify_all();
                return String::new();
            }
            let Ok((next_inner, _)) = self.signal.wait_timeout(inner, Duration::from_millis(16))
            else {
                return String::new();
            };
            inner = next_inner;
        }
    }
}

#[derive(Debug, Clone)]
pub struct CostumeBitmap {
    pub width: usize,
    pub height: usize,
    pub pixels_rgba: Vec<u8>,
    pub rotation_center_x: f64,
    pub rotation_center_y: f64,
}

#[derive(Debug, Clone)]
pub struct TargetRenderData {
    pub is_stage: bool,
    pub layer_order: i64,
    pub costumes: Vec<CostumeBitmap>,
}

#[derive(Debug, Clone, Copy)]
pub struct TargetInitialVisualState {
    pub x: f64,
    pub y: f64,
    pub direction_deg: f64,
    pub costume_number: f64,
    pub visible: bool,
    pub size_percent: f64,
}

impl Default for TargetInitialVisualState {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            direction_deg: 90.0,
            costume_number: 1.0,
            visible: true,
            size_percent: 100.0,
        }
    }
}

pub fn encode_string_id(index: usize) -> f64 {
    let payload = (index as u64).saturating_add(1) & STRING_PAYLOAD_MASK;
    f64::from_bits(STRING_TAG_BITS | payload)
}

pub fn decode_string_id(value: f64) -> Option<usize> {
    let bits = value.to_bits();
    if (bits & STRING_TAG_MASK) != STRING_TAG_BITS {
        return None;
    }
    let payload = bits & STRING_PAYLOAD_MASK;
    if payload == 0 {
        None
    } else {
        Some((payload - 1) as usize)
    }
}

#[derive(Debug, Clone, Copy)]
struct ScriptTask {
    script_id: u64,
    actor_id: u64,
}

#[derive(Debug, Clone, Copy)]
struct ActorState {
    target_index: u64,
    is_clone: bool,
    alive: bool,
    sprite_x: f64,
    sprite_y: f64,
    direction_deg: f64,
    costume_number: f64,
    visible: bool,
    size_percent: f64,
    pen_down: bool,
    pen_size: f64,
    pen_color: [u8; 3],
    pen_alpha: f64,
}

#[derive(Debug)]
pub struct RuntimeState {
    pub sprite_x: f64,
    pub sprite_y: f64,
    pub direction_deg: f64,
    pub costume_number: f64,
    pub visible: bool,
    pub size_percent: f64,
    pub pen_down: bool,
    pub pen_size: f64,
    pub pen_color: [u8; 3],
    pub pen_alpha: f64,
    pub tempo_bpm: f64,
    pub variables: Vec<f64>,
    pub lists: Vec<Vec<f64>>,
    pub executed_block_count: u64,
    pub remaining_steps: u64,
    step_budget: u64,
    relax_procedure_loop_budget: bool,
    answer_value: f64,
    timer_start: Instant,
    strings: Vec<String>,
    string_index: HashMap<String, usize>,
    canvas_width: usize,
    canvas_height: usize,
    pen_rgba: Vec<u8>,
    canvas_rgb: Vec<u8>,
    target_render_data: Vec<TargetRenderData>,
    target_initial_visuals: Vec<TargetInitialVisualState>,
    live_canvas: Option<Arc<Mutex<Vec<u8>>>>,
    live_canvas_dirty: bool,
    live_canvas_last_sync: Instant,
    live_canvas_sync_interval: Duration,
    input_state: Option<Arc<Mutex<InputState>>>,
    ask_prompt_state: Option<Arc<AskPromptState>>,
    stop_requested: Option<Arc<AtomicBool>>,
    frame_duration: Option<Duration>,
    next_frame_deadline: Option<Instant>,
    current_tick_started_at: Option<Instant>,
    paced_frame_count: u64,
    paced_frame_started_at: Option<Instant>,
    paced_frame_last_at: Option<Instant>,
    paced_loop_guards_in_resume: u64,
    script_functions: Vec<ScriptEntry>,
    script_names: Vec<String>,
    broadcast_messages: Vec<String>,
    broadcast_targets: Vec<Vec<u64>>,
    key_press_options: Vec<String>,
    key_press_targets: Vec<Vec<u64>>,
    previous_keys_down: HashSet<String>,
    clone_targets: Vec<Vec<u64>>,
    script_target_by_id: Vec<u64>,
    target_names: Vec<String>,
    base_actor_by_target: Vec<u64>,
    actors: Vec<ActorState>,
    active_actor_id: u64,
    script_queue: VecDeque<ScriptTask>,
    processing_queued_script: bool,
    /// When set, the runtime is executing inside a fiber and yield-points
    /// should cooperatively yield to the scheduler instead of sleeping.
    active_fiber_control: Option<Arc<FiberControl>>,
    rng_state: u64,
    trace_broadcasts: bool,
    debug_mode: bool,
    break_on_messages: HashSet<String>,
}

impl RuntimeState {
    fn sleep_until(deadline: Instant) {
        loop {
            let now = Instant::now();
            if now >= deadline {
                break;
            }
            let remaining = deadline.saturating_duration_since(now);
            if remaining > FRAME_SLEEP_COARSE_MARGIN {
                std::thread::sleep(remaining - FRAME_SLEEP_COARSE_MARGIN);
                continue;
            }
            std::hint::spin_loop();
        }
    }

    pub fn new(
        initial_variables: Vec<f64>,
        initial_lists: Vec<Vec<f64>>,
        initial_strings: Vec<String>,
        step_budget: u64,
    ) -> Self {
        let mut strings = if initial_strings.is_empty() {
            vec![String::new()]
        } else {
            initial_strings
        };
        if strings.first().is_none_or(|text| !text.is_empty()) {
            strings.insert(0, String::new());
        }

        let mut string_index = HashMap::new();
        for (index, text) in strings.iter().enumerate() {
            string_index.entry(text.clone()).or_insert(index);
        }
        let trace_broadcasts = env::var_os("SCRATCH_TRACE_BROADCASTS").is_some();
        let debug_mode = env_flag_enabled("SCRATCH_DEBUG");
        let break_on_messages = env_message_list("SCRATCH_BREAK_ON_MESSAGE")
            .into_iter()
            .map(|message| normalize_broadcast_message(&message))
            .filter(|message| !message.is_empty())
            .collect::<HashSet<_>>();

        Self {
            sprite_x: 0.0,
            sprite_y: 0.0,
            direction_deg: 90.0,
            costume_number: 1.0,
            visible: true,
            size_percent: 100.0,
            pen_down: false,
            pen_size: 1.0,
            pen_color: [0, 0, 0],
            pen_alpha: 1.0,
            tempo_bpm: 60.0,
            variables: initial_variables,
            lists: initial_lists,
            executed_block_count: 0,
            remaining_steps: step_budget,
            step_budget,
            relax_procedure_loop_budget: false,
            answer_value: encode_string_id(EMPTY_STRING_ID),
            timer_start: Instant::now(),
            strings,
            string_index,
            canvas_width: STAGE_WIDTH,
            canvas_height: STAGE_HEIGHT,
            pen_rgba: vec![0; STAGE_WIDTH * STAGE_HEIGHT * 4],
            canvas_rgb: vec![255; STAGE_WIDTH * STAGE_HEIGHT * 3],
            target_render_data: Vec::new(),
            target_initial_visuals: Vec::new(),
            live_canvas: None,
            live_canvas_dirty: false,
            live_canvas_last_sync: Instant::now(),
            live_canvas_sync_interval: DEFAULT_LIVE_CANVAS_SYNC_INTERVAL,
            input_state: None,
            ask_prompt_state: None,
            stop_requested: None,
            frame_duration: None,
            next_frame_deadline: None,
            current_tick_started_at: None,
            paced_frame_count: 0,
            paced_frame_started_at: None,
            paced_frame_last_at: None,
            paced_loop_guards_in_resume: 0,
            script_functions: Vec::new(),
            script_names: Vec::new(),
            broadcast_messages: Vec::new(),
            broadcast_targets: Vec::new(),
            key_press_options: Vec::new(),
            key_press_targets: Vec::new(),
            previous_keys_down: HashSet::new(),
            clone_targets: Vec::new(),
            script_target_by_id: Vec::new(),
            target_names: Vec::new(),
            base_actor_by_target: Vec::new(),
            actors: Vec::new(),
            active_actor_id: 0,
            script_queue: VecDeque::new(),
            processing_queued_script: false,
            active_fiber_control: None,
            rng_state: 0x4d595df4d0f33173,
            trace_broadcasts,
            debug_mode,
            break_on_messages,
        }
    }

    pub fn configure_render_targets(
        &mut self,
        render_data: Vec<TargetRenderData>,
        initial_visuals: Vec<TargetInitialVisualState>,
    ) {
        self.target_render_data = render_data;
        self.target_initial_visuals = initial_visuals;
        self.live_canvas_dirty = true;
    }

    pub fn install_scheduler(
        &mut self,
        script_functions: Vec<ScriptEntry>,
        script_names: Vec<String>,
        broadcast_messages: Vec<String>,
        broadcast_targets: Vec<Vec<u64>>,
        key_press_options: Vec<String>,
        key_press_targets: Vec<Vec<u64>>,
        clone_targets: Vec<Vec<u64>>,
        script_target_by_id: Vec<u64>,
        target_names: Vec<String>,
        target_count: usize,
    ) {
        self.script_functions = script_functions;
        self.script_names = script_names;
        while self.script_names.len() < self.script_functions.len() {
            let next = self.script_names.len();
            self.script_names.push(format!("script{next}"));
        }
        self.broadcast_messages = broadcast_messages;
        self.broadcast_targets = broadcast_targets;
        self.key_press_options = key_press_options;
        self.key_press_targets = key_press_targets;
        self.previous_keys_down.clear();
        self.clone_targets = clone_targets;
        self.script_target_by_id = script_target_by_id;
        let target_count = target_count.max(1);
        self.target_names = if target_names.is_empty() {
            (0..target_count)
                .map(|index| format!("target{index}"))
                .collect()
        } else {
            target_names
        };
        while self.target_names.len() < target_count {
            let index = self.target_names.len();
            self.target_names.push(format!("target{index}"));
        }
        self.actors = (0..target_count)
            .map(|target_index| {
                let initial = self
                    .target_initial_visuals
                    .get(target_index)
                    .copied()
                    .unwrap_or_default();
                ActorState {
                    target_index: target_index as u64,
                    is_clone: false,
                    alive: true,
                    sprite_x: initial.x,
                    sprite_y: initial.y,
                    direction_deg: initial.direction_deg,
                    costume_number: initial.costume_number.max(1.0),
                    visible: initial.visible,
                    size_percent: initial.size_percent.max(0.0),
                    pen_down: false,
                    pen_size: 1.0,
                    pen_color: [0, 0, 0],
                    pen_alpha: 1.0,
                }
            })
            .collect();
        self.base_actor_by_target = (0..target_count).map(|index| index as u64).collect();
        self.active_actor_id = 0;
        if let Some(actor) = self.actors.first().copied() {
            self.load_actor_from_snapshot(actor);
        }
        self.script_queue.clear();
        self.live_canvas_dirty = true;
    }

    pub fn enqueue_scripts(&mut self, script_ids: &[u64]) {
        if self.should_trace_events() && !script_ids.is_empty() {
            eprintln!("[debug][event] green flag handlers={}", script_ids.len());
        }
        for script_id in script_ids {
            self.enqueue_script_for_base_actor_with_reason(*script_id, Some("green flag"));
        }
    }

    pub fn dequeue_script(&mut self) -> Option<(u64, u64)> {
        self.script_queue
            .pop_front()
            .map(|task| (task.script_id, task.actor_id))
    }

    pub fn run_script(&mut self, script_id: u64, actor_id: u64) {
        let Some(function) = self.script_functions.get(script_id as usize).copied() else {
            return;
        };
        let Some(actor_snapshot) = self.actor_snapshot(actor_id) else {
            return;
        };
        if !actor_snapshot.alive {
            return;
        }
        if self.should_trace_events() {
            eprintln!(
                "[debug][run] script={} (id={}) actor={}",
                self.script_name_for_id(script_id),
                script_id,
                self.actor_label(actor_id)
            );
        }

        let previous_actor = self.active_actor_id;
        let previous_snapshot = self.capture_runtime_snapshot_for_active_actor();

        self.active_actor_id = actor_id;
        self.load_actor_from_snapshot(actor_snapshot);

        let state_ptr = self as *mut RuntimeState;
        unsafe {
            function(state_ptr);
        }

        self.persist_runtime_into_actor(actor_id);

        if let Some(snapshot) = previous_snapshot {
            self.active_actor_id = previous_actor;
            if previous_actor == actor_id {
                if let Some(updated) = self.actor_snapshot(previous_actor) {
                    self.load_actor_from_snapshot(updated);
                } else {
                    self.load_actor_from_snapshot(snapshot);
                }
            } else {
                self.load_actor_from_snapshot(snapshot);
            }
        }
    }

    fn actor_snapshot(&self, actor_id: u64) -> Option<ActorState> {
        self.actors.get(actor_id as usize).copied()
    }

    fn capture_runtime_snapshot_for_active_actor(&self) -> Option<ActorState> {
        let active = self.actors.get(self.active_actor_id as usize).copied()?;
        Some(ActorState {
            sprite_x: self.sprite_x,
            sprite_y: self.sprite_y,
            direction_deg: self.direction_deg,
            costume_number: self.costume_number,
            visible: self.visible,
            size_percent: self.size_percent,
            pen_down: self.pen_down,
            pen_size: self.pen_size,
            pen_color: self.pen_color,
            pen_alpha: self.pen_alpha,
            ..active
        })
    }

    fn load_actor_from_snapshot(&mut self, actor: ActorState) {
        self.sprite_x = actor.sprite_x;
        self.sprite_y = actor.sprite_y;
        self.direction_deg = actor.direction_deg;
        self.costume_number = actor.costume_number;
        self.visible = actor.visible;
        self.size_percent = actor.size_percent;
        self.pen_down = actor.pen_down;
        self.pen_size = actor.pen_size;
        self.pen_color = actor.pen_color;
        self.pen_alpha = actor.pen_alpha;
    }

    fn persist_runtime_into_actor(&mut self, actor_id: u64) {
        let Some(actor) = self.actors.get_mut(actor_id as usize) else {
            return;
        };
        if !actor.alive {
            return;
        }
        actor.sprite_x = self.sprite_x;
        actor.sprite_y = self.sprite_y;
        actor.direction_deg = self.direction_deg;
        actor.costume_number = self.costume_number;
        actor.visible = self.visible;
        actor.size_percent = self.size_percent;
        actor.pen_down = self.pen_down;
        actor.pen_size = self.pen_size;
        actor.pen_color = self.pen_color;
        actor.pen_alpha = self.pen_alpha;
    }

    fn enqueue_script_for_base_actor_with_reason(&mut self, script_id: u64, reason: Option<&str>) {
        if (script_id as usize) >= self.script_functions.len() {
            return;
        }
        let target_index = self
            .script_target_by_id
            .get(script_id as usize)
            .copied()
            .unwrap_or(0);
        let Some(actor_id) = self
            .base_actor_by_target
            .get(target_index as usize)
            .copied()
        else {
            return;
        };
        self.enqueue_task(script_id, actor_id, reason);
    }

    fn enqueue_task(&mut self, script_id: u64, actor_id: u64, reason: Option<&str>) {
        if (script_id as usize) >= self.script_functions.len() {
            return;
        }
        let Some(actor) = self.actors.get(actor_id as usize) else {
            return;
        };
        if !actor.alive {
            return;
        }
        self.script_queue.push_back(ScriptTask {
            script_id,
            actor_id,
        });
        if self.should_trace_events() {
            if let Some(reason) = reason {
                eprintln!(
                    "[debug][queue] event={} script={} (id={}) actor={}",
                    reason,
                    self.script_name_for_id(script_id),
                    script_id,
                    self.actor_label(actor_id)
                );
            } else {
                eprintln!(
                    "[debug][queue] script={} (id={}) actor={}",
                    self.script_name_for_id(script_id),
                    script_id,
                    self.actor_label(actor_id)
                );
            }
        }
    }

    fn actor_ids_for_target(&self, target_index: u64) -> Vec<u64> {
        self.actors
            .iter()
            .enumerate()
            .filter_map(|(actor_id, actor)| {
                if actor.alive && actor.target_index == target_index {
                    Some(actor_id as u64)
                } else {
                    None
                }
            })
            .collect()
    }

    fn broadcast_script_ids_for_message_value(&self, message_value: f64) -> Vec<u64> {
        let message = normalize_broadcast_message(&self.value_as_string(message_value));
        self.broadcast_messages
            .iter()
            .enumerate()
            .filter_map(|(index, candidate)| {
                if normalize_broadcast_message(candidate) == message {
                    self.broadcast_targets.get(index)
                } else {
                    None
                }
            })
            .flat_map(|scripts| scripts.iter().copied())
            .collect()
    }

    fn create_clone(&mut self, target_selector: i64) {
        let (source_actor_id, target_index) = if target_selector < 0 {
            let Some(active_actor) = self.actors.get(self.active_actor_id as usize) else {
                return;
            };
            (self.active_actor_id, active_actor.target_index)
        } else {
            let target_index = target_selector as u64;
            let Some(base_actor) = self
                .base_actor_by_target
                .get(target_index as usize)
                .copied()
            else {
                return;
            };
            (base_actor, target_index)
        };
        let Some(mut clone_actor) = self.actor_snapshot(source_actor_id) else {
            return;
        };
        if !clone_actor.alive {
            return;
        }
        clone_actor.target_index = target_index;
        clone_actor.is_clone = true;
        clone_actor.alive = true;

        let new_actor_id = self.actors.len() as u64;
        self.actors.push(clone_actor);

        let clone_scripts = self
            .clone_targets
            .get(target_index as usize)
            .cloned()
            .unwrap_or_default();
        if self.should_trace_events() {
            let target_name = self
                .target_names
                .get(target_index as usize)
                .map(String::as_str)
                .unwrap_or("unknown");
            eprintln!(
                "[debug][event] clone start target='{}' handlers={}",
                target_name,
                clone_scripts.len()
            );
        }
        for script_id in clone_scripts {
            self.enqueue_task(script_id, new_actor_id, Some("clone start"));
        }
        self.live_canvas_dirty = true;
    }

    fn delete_active_clone(&mut self) {
        let actor_id = self.active_actor_id;
        let Some(actor) = self.actors.get_mut(actor_id as usize) else {
            return;
        };
        if !actor.is_clone {
            return;
        }
        actor.alive = false;
        self.script_queue.retain(|task| task.actor_id != actor_id);
        self.live_canvas_dirty = true;
    }

    pub fn write_canvas_ppm<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        self.compose_canvas_rgb();
        let mut file = File::create(path)?;
        write!(
            file,
            "P6\n{} {}\n255\n",
            self.canvas_width, self.canvas_height
        )?;
        file.write_all(&self.canvas_rgb)
    }

    pub fn set_canvas_scale(&mut self, scale: usize) {
        let scale = scale.max(1);
        self.canvas_width = STAGE_WIDTH * scale;
        self.canvas_height = STAGE_HEIGHT * scale;
        self.pen_rgba = vec![0; self.canvas_width * self.canvas_height * 4];
        self.canvas_rgb = vec![255; self.canvas_width * self.canvas_height * 3];
        self.live_canvas_dirty = true;
    }

    pub fn canvas_dimensions(&self) -> (usize, usize) {
        (self.canvas_width, self.canvas_height)
    }

    pub fn canvas_rgb_copy(&self) -> Vec<u8> {
        self.canvas_rgb.clone()
    }

    pub fn attach_live_canvas(&mut self, live_canvas: Arc<Mutex<Vec<u8>>>) {
        self.live_canvas = Some(live_canvas);
        self.live_canvas_dirty = true;
        self.sync_live_canvas_if_due(true);
    }

    pub fn attach_stop_flag(&mut self, stop_requested: Arc<AtomicBool>) {
        self.stop_requested = Some(stop_requested);
    }

    pub fn attach_input_state(&mut self, input_state: Arc<Mutex<InputState>>) {
        self.input_state = Some(input_state);
    }

    pub fn attach_ask_prompt_state(&mut self, ask_prompt_state: Arc<AskPromptState>) {
        self.ask_prompt_state = Some(ask_prompt_state);
    }

    pub fn set_debug_mode(&mut self, enabled: bool) {
        self.debug_mode = enabled;
    }

    pub fn set_break_on_messages(&mut self, messages: Vec<String>) {
        self.break_on_messages = messages
            .into_iter()
            .map(|message| normalize_broadcast_message(&message))
            .filter(|message| !message.is_empty())
            .collect::<HashSet<_>>();
    }

    pub fn set_target_fps(&mut self, fps: Option<f64>) {
        self.frame_duration = fps
            .filter(|value| value.is_finite() && *value > 0.0)
            .map(|value| Duration::from_secs_f64(1.0 / value));
        self.next_frame_deadline = None;
        self.current_tick_started_at = None;
        self.paced_frame_count = 0;
        self.paced_frame_started_at = None;
        self.paced_frame_last_at = None;
    }

    pub fn set_live_canvas_sync_fps(&mut self, fps: Option<f64>) {
        self.live_canvas_sync_interval = fps
            .filter(|value| value.is_finite() && *value > 0.0)
            .map(|value| Duration::from_secs_f64(1.0 / value))
            .unwrap_or(DEFAULT_LIVE_CANVAS_SYNC_INTERVAL);
    }

    pub fn measured_fps(&self) -> Option<f64> {
        let start = self.paced_frame_started_at?;
        let end = self.paced_frame_last_at?;
        if self.paced_frame_count == 0 || end <= start {
            return None;
        }
        let elapsed = end.duration_since(start).as_secs_f64();
        if elapsed <= 0.0 {
            return None;
        }
        Some(self.paced_frame_count as f64 / elapsed)
    }

    pub fn flush_live_canvas(&mut self) {
        self.live_canvas_dirty = true;
        self.sync_live_canvas_if_due(true);
    }

    pub fn debug_value(&self, value: f64) -> String {
        if decode_string_id(value).is_some() {
            return format!("\"{}\"", self.value_as_string(value));
        }
        value.to_string()
    }

    fn should_trace_events(&self) -> bool {
        self.debug_mode
    }

    fn should_trace_broadcasts(&self) -> bool {
        self.debug_mode || self.trace_broadcasts
    }

    fn should_break_on_message(&self, message: &str) -> bool {
        self.break_on_messages
            .contains(&normalize_broadcast_message(message))
    }

    fn break_on_broadcast_message(&mut self, message: &str, wait: bool) {
        if !self.should_break_on_message(message) {
            return;
        }

        let event_kind = if wait {
            "broadcast and wait"
        } else {
            "broadcast"
        };
        eprintln!(
            "[debug][break] {} '{}' matched breakpoint",
            event_kind, message
        );
        self.flush_live_canvas();

        if io::stdin().is_terminal() {
            eprintln!("[debug][break] Press Enter to continue...");
            let _ = io::stderr().flush();
            let mut line = String::new();
            let _ = io::stdin().read_line(&mut line);
        } else {
            eprintln!("[debug][break] stdin is not interactive; continuing");
        }
    }

    fn script_name_for_id(&self, script_id: u64) -> &str {
        self.script_names
            .get(script_id as usize)
            .map(String::as_str)
            .unwrap_or("<unknown-script>")
    }

    fn actor_label(&self, actor_id: u64) -> String {
        let Some(actor) = self.actors.get(actor_id as usize) else {
            return format!("actor#{actor_id}");
        };
        let target_name = self
            .target_names
            .get(actor.target_index as usize)
            .map(String::as_str)
            .unwrap_or("unknown");
        if actor.is_clone {
            format!("{target_name}#clone{actor_id}")
        } else {
            format!("{target_name}#{actor_id}")
        }
    }

    fn intern_string(&mut self, text: &str) -> usize {
        if let Some(index) = self.string_index.get(text).copied() {
            return index;
        }
        let index = self.strings.len();
        self.strings.push(text.to_string());
        self.string_index.insert(text.to_string(), index);
        index
    }

    fn value_as_string(&self, value: f64) -> String {
        if let Some(index) = decode_string_id(value) {
            return self.strings.get(index).cloned().unwrap_or_default();
        }
        if !value.is_finite() {
            return String::new();
        }
        value.to_string()
    }

    fn value_to_number(&self, value: f64) -> f64 {
        if let Some(index) = decode_string_id(value) {
            return self
                .strings
                .get(index)
                .map(|text| text.trim().parse::<f64>().unwrap_or(0.0))
                .unwrap_or(0.0);
        }
        value
    }

    fn parse_scratch_number_for_compare(&self, text: &str) -> Option<f64> {
        let parsed = if text.trim().is_empty() {
            Some(0.0)
        } else {
            text.trim().parse::<f64>().ok()
        };

        match parsed {
            Some(0.0) if string_is_not_actually_zero(text) => None,
            Some(number) if number.is_nan() => None,
            value => value,
        }
    }

    fn value_to_number_for_compare(&self, value: f64) -> Option<f64> {
        if let Some(index) = decode_string_id(value) {
            return self
                .strings
                .get(index)
                .and_then(|text| self.parse_scratch_number_for_compare(text));
        }
        if value.is_nan() {
            None
        } else {
            Some(value)
        }
    }

    fn compare_values(&self, left: f64, right: f64) -> i8 {
        let left_numeric = self.value_to_number_for_compare(left);
        let right_numeric = self.value_to_number_for_compare(right);

        if let (Some(left_number), Some(right_number)) = (left_numeric, right_numeric) {
            if (left_number == f64::INFINITY && right_number == f64::INFINITY)
                || (left_number == f64::NEG_INFINITY && right_number == f64::NEG_INFINITY)
            {
                return 0;
            }
            return if left_number < right_number {
                -1
            } else if left_number > right_number {
                1
            } else {
                0
            };
        }

        let left_text = self.value_as_string(left).to_lowercase();
        let right_text = self.value_as_string(right).to_lowercase();

        if left_text < right_text {
            -1
        } else if left_text > right_text {
            1
        } else {
            0
        }
    }

    fn values_equal(&self, left: f64, right: f64) -> bool {
        self.compare_values(left, right) == 0
    }

    fn clear_canvas(&mut self) {
        self.pen_rgba.fill(0);
        self.live_canvas_dirty = true;
    }

    fn move_sprite_to(&mut self, new_x: f64, new_y: f64) {
        let from_x = self.sprite_x;
        let from_y = self.sprite_y;
        if self.pen_down {
            self.draw_line(from_x, from_y, new_x, new_y);
        }
        self.sprite_x = new_x;
        self.sprite_y = new_y;
        self.live_canvas_dirty = true;
    }

    fn sync_live_canvas_if_due(&mut self, force: bool) {
        if !self.live_canvas_dirty {
            return;
        }
        if !force && self.live_canvas_last_sync.elapsed() < self.live_canvas_sync_interval {
            return;
        }
        self.sync_live_canvas();
        self.live_canvas_dirty = false;
        self.live_canvas_last_sync = Instant::now();
    }

    fn sync_live_canvas(&mut self) {
        self.compose_canvas_rgb();
        let Some(live_canvas) = &self.live_canvas else {
            return;
        };
        let Ok(mut guard) = live_canvas.lock() else {
            return;
        };
        if guard.len() != self.canvas_rgb.len() {
            *guard = vec![255; self.canvas_rgb.len()];
        }
        guard.copy_from_slice(&self.canvas_rgb);
    }

    fn wait_for_next_frame(&mut self) {
        // ---- fiber mode ----
        if let Some(ref control) = self.active_fiber_control {
            if self.frame_duration.is_some() {
                let control = Arc::clone(control);
                control.yield_to_scheduler();
            }
            // Without frame pacing (turbo / no-gui): don't yield – let loops
            // consume step budget at full speed so that interactive wait-
            // loops terminate via budget exhaustion.
            return;
        }

        // ---- legacy / non-fiber mode: sleep until next frame ----
        let Some(frame_duration) = self.frame_duration else {
            return;
        };

        let now = Instant::now();
        let tick_started_at = *self.current_tick_started_at.get_or_insert(now);

        let deadline = self
            .next_frame_deadline
            .get_or_insert(tick_started_at + frame_duration);
        if *deadline > now {
            Self::sleep_until(*deadline);
        }

        let now = Instant::now();
        while *deadline <= now {
            *deadline += frame_duration;
        }

        self.current_tick_started_at = Some(now);
        if self.paced_frame_started_at.is_none() {
            self.paced_frame_started_at = Some(now);
        }
        self.paced_frame_count = self.paced_frame_count.saturating_add(1);
        self.paced_frame_last_at = Some(now);
    }

    fn should_yield_for_work_time(&mut self) -> bool {
        let Some(frame_duration) = self.frame_duration else {
            return false;
        };
        let now = Instant::now();
        let tick_started_at = *self.current_tick_started_at.get_or_insert(now);
        let work_time =
            Duration::from_secs_f64(frame_duration.as_secs_f64() * SCRATCH_VM_WORK_TIME_RATIO);
        now.duration_since(tick_started_at) >= work_time
    }

    fn note_paced_loop_guard(&mut self) -> bool {
        let is_first = self.paced_loop_guards_in_resume == 0;
        self.paced_loop_guards_in_resume = self.paced_loop_guards_in_resume.saturating_add(1);
        is_first
    }

    /// Apply frame pacing (sleep until next frame deadline).  Called by the
    /// concurrent scheduler once per tick, after all fibers have been stepped.
    fn pace_frame(&mut self) {
        let Some(frame_duration) = self.frame_duration else {
            return;
        };
        let now = Instant::now();
        let tick_started_at = *self.current_tick_started_at.get_or_insert(now);
        let deadline = self
            .next_frame_deadline
            .get_or_insert(tick_started_at + frame_duration);
        if *deadline > now {
            Self::sleep_until(*deadline);
        }
        let now = Instant::now();
        while *deadline <= now {
            *deadline += frame_duration;
        }
        self.current_tick_started_at = Some(now);
        if self.paced_frame_started_at.is_none() {
            self.paced_frame_started_at = Some(now);
        }
        self.paced_frame_count = self.paced_frame_count.saturating_add(1);
        self.paced_frame_last_at = Some(now);
    }

    // ------------------------------------------------------------------
    // Concurrent (fiber-based) execution scheduler
    // ------------------------------------------------------------------

    /// Execute the program sequentially in the current thread.
    pub fn execute_serial(&mut self) {
        while let Some((script_id, actor_id)) = self.dequeue_script() {
            if self
                .stop_requested
                .as_ref()
                .is_some_and(|stop| stop.load(Ordering::Relaxed))
            {
                break;
            }
            self.processing_queued_script = true;
            self.run_script(script_id, actor_id);
            self.processing_queued_script = false;
        }
        self.flush_live_canvas();
    }

    /// Execute the program concurrently: each script runs as a cooperative
    /// fiber and all active fibers advance one yield-step per tick.
    pub fn execute_concurrent(&mut self) {
        let state_ptr = self as *mut RuntimeState;

        let mut fibers: Vec<Fiber> = Vec::new();

        // Spawn fibers for all initially-queued scripts.
        while let Some((script_id, actor_id)) = self.dequeue_script() {
            let function = match self.script_functions.get(script_id as usize).copied() {
                Some(f) => f,
                None => continue,
            };
            eprintln!(
                "[fiber] spawning fiber for script={} actor={}",
                script_id, actor_id
            );
            fibers.push(Fiber::spawn(state_ptr, function, script_id, actor_id));
        }

        // Main tick loop.
        let mut tick = 0u64;
        loop {
            let mut any_active = false;

            // --- step each active fiber one yield ---
            for (fi, fiber) in fibers.iter_mut().enumerate() {
                if fiber.is_done() {
                    continue;
                }
                any_active = true;

                // Load the actor state that was active when this fiber last
                // yielded (or the initial actor on the first resume).
                // `current_actor_id` tracks which actor the fiber is
                // currently operating on – this may differ from the original
                // `actor_id` when the fiber is inside a `run_script` call
                // for a broadcast-and-wait handler.
                if let Some(actor) = self.actor_snapshot(fiber.current_actor_id) {
                    if !actor.alive {
                        continue;
                    }
                    self.active_actor_id = fiber.current_actor_id;
                    self.load_actor_from_snapshot(actor);
                }

                // When frame pacing is active, give this fiber a fresh step
                // budget allocation so that each fiber gets its own
                // work-timer equivalent.  This also ensures that a "stop
                // this script" (which zeroes remaining_steps) does not
                // bleed into the next fiber.
                //
                // Without frame pacing (turbo / no-gui), the global step
                // budget acts as a termination bound – loops consume budget
                // at full speed and exit when budget is exhausted.
                if self.frame_duration.is_some() {
                    self.remaining_steps = self.step_budget;
                }

                // Install the fiber's control so yield-points use it.
                self.active_fiber_control = Some(Arc::clone(&fiber.control));
                self.paced_loop_guards_in_resume = 0;

                if tick < 3 {
                    eprintln!(
                        "[fiber] tick={} resuming fiber[{}] script={} actor={}",
                        tick, fi, fiber.script_id, fiber.current_actor_id
                    );
                }

                // Resume the fiber thread.
                fiber.control.resume();

                // Wait until it yields or finishes.
                let result = fiber.control.wait_for_yield_or_done();

                if tick < 3 {
                    eprintln!("[fiber] tick={} fiber[{}] result={:?}", tick, fi, result);
                }

                // Save actor state back – persist into whichever actor is
                // currently loaded (may differ from the original if the
                // fiber is mid-broadcast-and-wait handler execution).
                self.persist_runtime_into_actor(self.active_actor_id);
                fiber.current_actor_id = self.active_actor_id;
                self.active_fiber_control = None;
            }

            // --- spawn fibers for newly-queued scripts (broadcasts, clones) ---
            let mut new_tasks: Vec<(u64, u64)> = Vec::new();
            while let Some((script_id, actor_id)) = self.dequeue_script() {
                new_tasks.push((script_id, actor_id));
            }
            for (script_id, actor_id) in new_tasks {
                let function = match self.script_functions.get(script_id as usize).copied() {
                    Some(f) => f,
                    None => continue,
                };
                fibers.push(Fiber::spawn(state_ptr, function, script_id, actor_id));
                any_active = true;
            }

            if !any_active {
                break;
            }

            // --- check stop request ---
            if self
                .stop_requested
                .as_ref()
                .is_some_and(|stop| stop.load(Ordering::Relaxed))
            {
                break;
            }

            // --- per-tick housekeeping ---
            self.sync_live_canvas_if_due(false);
            self.enqueue_key_pressed_scripts();

            // --- frame pacing (applied once per tick) ---
            self.pace_frame();
            tick += 1;
        }

        // Clean up: signal remaining fibers so their threads can exit, then
        // join them.
        for fiber in fibers.iter() {
            if !fiber.is_done() {
                fiber.control.resume();
            }
        }
        for fiber in &mut fibers {
            fiber.join();
        }

        self.flush_live_canvas();
    }

    fn draw_line(&mut self, x0: f64, y0: f64, x1: f64, y1: f64) {
        let dx = x1 - x0;
        let dy = y1 - y0;
        let steps = dx.abs().max(dy.abs()).ceil() as i32;
        if steps <= 0 {
            self.draw_disc(x0, y0);
            return;
        }
        for step in 0..=steps {
            let t = step as f64 / steps as f64;
            let x = x0 + dx * t;
            let y = y0 + dy * t;
            self.draw_disc(x, y);
        }
    }

    fn draw_disc(&mut self, x: f64, y: f64) {
        let cx = scratch_to_pixel_x(x, self.canvas_width);
        let cy = scratch_to_pixel_y(y, self.canvas_height);
        let canvas_scale = (self.canvas_width as f64) / (STAGE_WIDTH as f64);
        // Pen size is defined in stage-space units, so scale thickness with canvas resolution.
        // Keep pen size 1 at a single pixel even after scaling.
        let scaled_pen_size = self.pen_size.max(1.0) * canvas_scale.max(1.0);
        let radius = ((scaled_pen_size - 1.0) / 2.0).max(0.0);
        let extent = radius.ceil() as i32;
        let radius_sq = radius * radius;

        for oy in -extent..=extent {
            for ox in -extent..=extent {
                let distance_sq = (ox as f64) * (ox as f64) + (oy as f64) * (oy as f64);
                if distance_sq > radius_sq {
                    continue;
                }
                self.set_pixel(cx + ox, cy + oy, self.pen_color);
            }
        }
    }

    fn set_pixel(&mut self, x: i32, y: i32, rgb: [u8; 3]) {
        if x < 0 || y < 0 {
            return;
        }
        let x = x as usize;
        let y = y as usize;
        if x >= self.canvas_width || y >= self.canvas_height {
            return;
        }
        let offset = (y * self.canvas_width + x) * 4;
        let src_alpha = self.pen_alpha.clamp(0.0, 1.0);
        if src_alpha <= 0.0 {
            return;
        }
        let dst_alpha = (self.pen_rgba[offset + 3] as f64) / 255.0;
        let out_alpha = src_alpha + dst_alpha * (1.0 - src_alpha);
        if out_alpha <= 0.0 {
            return;
        }

        let dst_r = (self.pen_rgba[offset] as f64) / 255.0;
        let dst_g = (self.pen_rgba[offset + 1] as f64) / 255.0;
        let dst_b = (self.pen_rgba[offset + 2] as f64) / 255.0;
        let src_r = (rgb[0] as f64) / 255.0;
        let src_g = (rgb[1] as f64) / 255.0;
        let src_b = (rgb[2] as f64) / 255.0;

        let out_r = (src_r * src_alpha + dst_r * dst_alpha * (1.0 - src_alpha)) / out_alpha;
        let out_g = (src_g * src_alpha + dst_g * dst_alpha * (1.0 - src_alpha)) / out_alpha;
        let out_b = (src_b * src_alpha + dst_b * dst_alpha * (1.0 - src_alpha)) / out_alpha;

        self.pen_rgba[offset] = (out_r * 255.0).round().clamp(0.0, 255.0) as u8;
        self.pen_rgba[offset + 1] = (out_g * 255.0).round().clamp(0.0, 255.0) as u8;
        self.pen_rgba[offset + 2] = (out_b * 255.0).round().clamp(0.0, 255.0) as u8;
        self.pen_rgba[offset + 3] = (out_alpha * 255.0).round().clamp(0.0, 255.0) as u8;
    }

    fn stamp_active_sprite_to_pen_layer(&mut self) {
        let actor = self
            .actors
            .get(self.active_actor_id as usize)
            .copied()
            .unwrap_or(ActorState {
                target_index: 0,
                is_clone: false,
                alive: true,
                sprite_x: self.sprite_x,
                sprite_y: self.sprite_y,
                direction_deg: self.direction_deg,
                costume_number: self.costume_number,
                visible: self.visible,
                size_percent: self.size_percent,
                pen_down: self.pen_down,
                pen_size: self.pen_size,
                pen_color: self.pen_color,
                pen_alpha: self.pen_alpha,
            });
        if !self.blit_actor_costume_to_pen(&actor) {
            self.draw_disc(actor.sprite_x, actor.sprite_y);
        }
    }

    fn compose_canvas_rgb(&mut self) {
        self.canvas_rgb.fill(255);
        self.compose_backdrop();
        self.blend_pen_layer_into_canvas();
        self.compose_sprites();
    }

    fn compose_backdrop(&mut self) {
        let stage_target_index = self
            .target_render_data
            .iter()
            .position(|target| target.is_stage)
            .unwrap_or(0);
        let stage_actor = self
            .base_actor_by_target
            .get(stage_target_index)
            .and_then(|actor_id| self.actor_snapshot(*actor_id))
            .unwrap_or_else(|| {
                let initial = self
                    .target_initial_visuals
                    .get(stage_target_index)
                    .copied()
                    .unwrap_or_default();
                ActorState {
                    target_index: stage_target_index as u64,
                    is_clone: false,
                    alive: true,
                    sprite_x: initial.x,
                    sprite_y: initial.y,
                    direction_deg: initial.direction_deg,
                    costume_number: initial.costume_number,
                    visible: true,
                    size_percent: 100.0,
                    pen_down: false,
                    pen_size: 1.0,
                    pen_color: [0, 0, 0],
                    pen_alpha: 1.0,
                }
            });
        let Some(costume) = self
            .resolve_costume_for_target(stage_target_index, stage_actor.costume_number)
            .cloned()
        else {
            return;
        };
        self.blit_transformed_costume_to_rgb(&costume, 0.0, 0.0, 90.0, 100.0);
    }

    fn blend_pen_layer_into_canvas(&mut self) {
        let pixel_count = self.canvas_width * self.canvas_height;
        for index in 0..pixel_count {
            let src_offset = index * 4;
            let dst_offset = index * 3;
            let alpha = (self.pen_rgba[src_offset + 3] as f64) / 255.0;
            if alpha <= 0.0 {
                continue;
            }
            let inv_alpha = 1.0 - alpha;
            self.canvas_rgb[dst_offset] = ((self.pen_rgba[src_offset] as f64) * alpha
                + (self.canvas_rgb[dst_offset] as f64) * inv_alpha)
                .round()
                .clamp(0.0, 255.0) as u8;
            self.canvas_rgb[dst_offset + 1] = ((self.pen_rgba[src_offset + 1] as f64) * alpha
                + (self.canvas_rgb[dst_offset + 1] as f64) * inv_alpha)
                .round()
                .clamp(0.0, 255.0) as u8;
            self.canvas_rgb[dst_offset + 2] = ((self.pen_rgba[src_offset + 2] as f64) * alpha
                + (self.canvas_rgb[dst_offset + 2] as f64) * inv_alpha)
                .round()
                .clamp(0.0, 255.0) as u8;
        }
    }

    fn compose_sprites(&mut self) {
        let mut draw_order = self
            .actors
            .iter()
            .enumerate()
            .filter_map(|(actor_id, actor)| {
                if !actor.alive || !actor.visible || actor.size_percent <= 0.0 {
                    return None;
                }
                let target_index = actor.target_index as usize;
                let layer_order = self
                    .target_render_data
                    .get(target_index)
                    .map(|target| target.layer_order)?;
                let is_stage = self
                    .target_render_data
                    .get(target_index)
                    .map(|target| target.is_stage)
                    .unwrap_or(false);
                if is_stage {
                    return None;
                }
                Some((layer_order, actor_id as u64, *actor))
            })
            .collect::<Vec<_>>();

        draw_order.sort_by_key(|(layer_order, actor_id, _)| (*layer_order, *actor_id));
        for (_, _, actor) in draw_order {
            self.blit_actor_costume_to_rgb(&actor);
        }
    }

    fn resolve_costume_for_target(
        &self,
        target_index: usize,
        costume_number: f64,
    ) -> Option<&CostumeBitmap> {
        let target = self.target_render_data.get(target_index)?;
        if target.costumes.is_empty() {
            return None;
        }
        let count = target.costumes.len() as i64;
        let raw = (costume_number.floor() as i64).saturating_sub(1);
        let wrapped = raw.rem_euclid(count) as usize;
        target.costumes.get(wrapped)
    }

    fn blit_actor_costume_to_rgb(&mut self, actor: &ActorState) {
        let Some(costume) = self
            .resolve_costume_for_target(actor.target_index as usize, actor.costume_number)
            .cloned()
        else {
            return;
        };
        self.blit_transformed_costume_to_rgb(
            &costume,
            actor.sprite_x,
            actor.sprite_y,
            actor.direction_deg,
            actor.size_percent,
        );
    }

    fn blit_actor_costume_to_pen(&mut self, actor: &ActorState) -> bool {
        let Some(costume) = self
            .resolve_costume_for_target(actor.target_index as usize, actor.costume_number)
            .cloned()
        else {
            return false;
        };
        self.blit_transformed_costume_to_pen(
            &costume,
            actor.sprite_x,
            actor.sprite_y,
            actor.direction_deg,
            actor.size_percent,
        );
        true
    }

    fn blit_transformed_costume_to_rgb(
        &mut self,
        costume: &CostumeBitmap,
        actor_x: f64,
        actor_y: f64,
        direction_deg: f64,
        size_percent: f64,
    ) {
        let Some(transform) =
            CostumeTransform::new(costume, actor_x, actor_y, direction_deg, size_percent)
        else {
            return;
        };
        let (min_x, max_x, min_y, max_y) =
            costume_pixel_bounds(costume, self.canvas_width, self.canvas_height, &transform);

        for py in min_y..=max_y {
            for px in min_x..=max_x {
                let world_x = pixel_to_scratch_x(px as usize, self.canvas_width);
                let world_y = pixel_to_scratch_y(py as usize, self.canvas_height);
                let Some((src_x, src_y)) =
                    sample_costume_coordinates(costume, world_x, world_y, &transform)
                else {
                    continue;
                };
                let src_offset = (src_y * costume.width + src_x) * 4;
                let alpha = (costume.pixels_rgba[src_offset + 3] as f64) / 255.0;
                if alpha <= 0.0 {
                    continue;
                }
                let dst_offset = ((py as usize) * self.canvas_width + (px as usize)) * 3;
                let inv_alpha = 1.0 - alpha;
                self.canvas_rgb[dst_offset] = ((costume.pixels_rgba[src_offset] as f64) * alpha
                    + (self.canvas_rgb[dst_offset] as f64) * inv_alpha)
                    .round()
                    .clamp(0.0, 255.0) as u8;
                self.canvas_rgb[dst_offset + 1] = ((costume.pixels_rgba[src_offset + 1] as f64)
                    * alpha
                    + (self.canvas_rgb[dst_offset + 1] as f64) * inv_alpha)
                    .round()
                    .clamp(0.0, 255.0) as u8;
                self.canvas_rgb[dst_offset + 2] = ((costume.pixels_rgba[src_offset + 2] as f64)
                    * alpha
                    + (self.canvas_rgb[dst_offset + 2] as f64) * inv_alpha)
                    .round()
                    .clamp(0.0, 255.0) as u8;
            }
        }
    }

    fn blit_transformed_costume_to_pen(
        &mut self,
        costume: &CostumeBitmap,
        actor_x: f64,
        actor_y: f64,
        direction_deg: f64,
        size_percent: f64,
    ) {
        let Some(transform) =
            CostumeTransform::new(costume, actor_x, actor_y, direction_deg, size_percent)
        else {
            return;
        };
        let (min_x, max_x, min_y, max_y) =
            costume_pixel_bounds(costume, self.canvas_width, self.canvas_height, &transform);

        for py in min_y..=max_y {
            for px in min_x..=max_x {
                let world_x = pixel_to_scratch_x(px as usize, self.canvas_width);
                let world_y = pixel_to_scratch_y(py as usize, self.canvas_height);
                let Some((src_x, src_y)) =
                    sample_costume_coordinates(costume, world_x, world_y, &transform)
                else {
                    continue;
                };
                let src_offset = (src_y * costume.width + src_x) * 4;
                let src_alpha = (costume.pixels_rgba[src_offset + 3] as f64) / 255.0;
                if src_alpha <= 0.0 {
                    continue;
                }
                let dst_offset = ((py as usize) * self.canvas_width + (px as usize)) * 4;
                let dst_alpha = (self.pen_rgba[dst_offset + 3] as f64) / 255.0;
                let out_alpha = src_alpha + dst_alpha * (1.0 - src_alpha);
                if out_alpha <= 0.0 {
                    continue;
                }

                let src_r = (costume.pixels_rgba[src_offset] as f64) / 255.0;
                let src_g = (costume.pixels_rgba[src_offset + 1] as f64) / 255.0;
                let src_b = (costume.pixels_rgba[src_offset + 2] as f64) / 255.0;
                let dst_r = (self.pen_rgba[dst_offset] as f64) / 255.0;
                let dst_g = (self.pen_rgba[dst_offset + 1] as f64) / 255.0;
                let dst_b = (self.pen_rgba[dst_offset + 2] as f64) / 255.0;

                let out_r = (src_r * src_alpha + dst_r * dst_alpha * (1.0 - src_alpha)) / out_alpha;
                let out_g = (src_g * src_alpha + dst_g * dst_alpha * (1.0 - src_alpha)) / out_alpha;
                let out_b = (src_b * src_alpha + dst_b * dst_alpha * (1.0 - src_alpha)) / out_alpha;

                self.pen_rgba[dst_offset] = (out_r * 255.0).round().clamp(0.0, 255.0) as u8;
                self.pen_rgba[dst_offset + 1] = (out_g * 255.0).round().clamp(0.0, 255.0) as u8;
                self.pen_rgba[dst_offset + 2] = (out_b * 255.0).round().clamp(0.0, 255.0) as u8;
                self.pen_rgba[dst_offset + 3] = (out_alpha * 255.0).round().clamp(0.0, 255.0) as u8;
            }
        }
    }

    fn list_item(&mut self, list_index: usize, index: f64) -> f64 {
        let Some(list) = self.lists.get(list_index) else {
            return encode_string_id(EMPTY_STRING_ID);
        };
        let len = list.len();
        if len == 0 {
            return encode_string_id(EMPTY_STRING_ID);
        }

        let resolved_index = if let Some(string_index) = decode_string_id(index) {
            let selector = self
                .strings
                .get(string_index)
                .map(|text| text.trim().to_ascii_lowercase())
                .unwrap_or_default();
            match selector.as_str() {
                "last" => Some(len - 1),
                "random" | "any" => {
                    let selected = ((next_random_unit(self) * len as f64).floor() as usize)
                        .min(len.saturating_sub(1));
                    Some(selected)
                }
                _ => None,
            }
        } else {
            None
        }
        .or_else(|| {
            let numeric_index = rt_repeat_count(self.value_to_number(index)) as usize;
            if numeric_index == 0 || numeric_index > len {
                None
            } else {
                Some(numeric_index - 1)
            }
        });

        resolved_index
            .and_then(|item_index| {
                self.lists
                    .get(list_index)
                    .and_then(|list| list.get(item_index))
            })
            .copied()
            .unwrap_or_else(|| encode_string_id(EMPTY_STRING_ID))
    }

    fn list_item_num(&self, list_index: usize, item: f64) -> f64 {
        let Some(list) = self.lists.get(list_index) else {
            return 0.0;
        };
        for (idx, list_item) in list.iter().copied().enumerate() {
            if self.values_equal(list_item, item) {
                return (idx + 1) as f64;
            }
        }
        0.0
    }

    fn list_length(&self, list_index: usize) -> f64 {
        self.lists
            .get(list_index)
            .map(|list| list.len() as f64)
            .unwrap_or(0.0)
    }

    fn list_replace_item(&mut self, list_index: usize, index: f64, item: f64) {
        let len = self
            .lists
            .get(list_index)
            .map(|list| list.len())
            .unwrap_or(0);
        if len == 0 {
            return;
        };

        let resolved_index = if let Some(string_index) = decode_string_id(index) {
            let selector = self
                .strings
                .get(string_index)
                .map(|text| text.trim().to_ascii_lowercase())
                .unwrap_or_default();
            match selector.as_str() {
                "last" => Some(len - 1),
                "random" | "any" => {
                    let selected = ((next_random_unit(self) * len as f64).floor() as usize)
                        .min(len.saturating_sub(1));
                    Some(selected)
                }
                _ => None,
            }
        } else {
            None
        }
        .or_else(|| {
            let numeric_index = rt_repeat_count(self.value_to_number(index)) as usize;
            if numeric_index == 0 || numeric_index > len {
                None
            } else {
                Some(numeric_index - 1)
            }
        });

        let Some(item_index) = resolved_index else {
            return;
        };
        let Some(list) = self.lists.get_mut(list_index) else {
            return;
        };
        let Some(slot) = list.get_mut(item_index) else {
            return;
        };
        *slot = item;
    }

    fn list_add_item(&mut self, list_index: usize, item: f64) {
        let Some(list) = self.lists.get_mut(list_index) else {
            return;
        };
        list.push(item);
    }

    fn list_delete_item(&mut self, list_index: usize, index: f64) {
        let len = self
            .lists
            .get(list_index)
            .map(|list| list.len())
            .unwrap_or(0);
        if len == 0 {
            return;
        }

        if let Some(string_index) = decode_string_id(index) {
            let selector = self
                .strings
                .get(string_index)
                .map(|text| text.trim().to_ascii_lowercase())
                .unwrap_or_default();
            match selector.as_str() {
                "all" => {
                    if let Some(list) = self.lists.get_mut(list_index) {
                        list.clear();
                    }
                    return;
                }
                "last" => {
                    if let Some(list) = self.lists.get_mut(list_index) {
                        list.pop();
                    }
                    return;
                }
                "random" | "any" => {
                    let selected = ((next_random_unit(self) * len as f64).floor() as usize)
                        .min(len.saturating_sub(1));
                    if let Some(list) = self.lists.get_mut(list_index) {
                        list.remove(selected);
                    }
                    return;
                }
                _ => {}
            }
        }

        let item_index = rt_repeat_count(self.value_to_number(index)) as usize;
        if item_index == 0 || item_index > len {
            return;
        }
        if let Some(list) = self.lists.get_mut(list_index) {
            list.remove(item_index - 1);
        }
    }

    fn list_delete_all(&mut self, list_index: usize) {
        let Some(list) = self.lists.get_mut(list_index) else {
            return;
        };
        list.clear();
    }

    fn list_contains_item(&self, list_index: usize, item: f64) -> bool {
        self.lists.get(list_index).is_some_and(|list| {
            list.iter()
                .copied()
                .any(|entry| self.values_equal(entry, item))
        })
    }

    fn is_key_down(&self, key_name: &str) -> bool {
        self.input_state
            .as_ref()
            .and_then(|shared| shared.lock().ok())
            .is_some_and(|input| input.keys_down.contains(key_name))
    }

    fn current_keys_down(&self) -> HashSet<String> {
        self.input_state
            .as_ref()
            .and_then(|shared| shared.lock().ok())
            .map(|input| {
                input
                    .keys_down
                    .iter()
                    .map(|key| normalize_key_name(key))
                    .collect::<HashSet<_>>()
            })
            .unwrap_or_default()
    }

    fn enqueue_key_pressed_scripts(&mut self) {
        if self.key_press_options.is_empty() {
            return;
        }

        let current_keys = self.current_keys_down();
        let any_new_key = current_keys
            .iter()
            .any(|key| !self.previous_keys_down.contains(key));

        let mut scripts_to_enqueue = Vec::new();
        for (index, option) in self.key_press_options.iter().enumerate() {
            let triggered = if option == "any" {
                any_new_key
            } else {
                current_keys.contains(option) && !self.previous_keys_down.contains(option)
            };
            if !triggered {
                continue;
            }
            if let Some(script_ids) = self.key_press_targets.get(index) {
                if self.should_trace_events() {
                    eprintln!(
                        "[debug][event] key pressed '{}' handlers={}",
                        option,
                        script_ids.len()
                    );
                }
                scripts_to_enqueue.extend(
                    script_ids
                        .iter()
                        .copied()
                        .map(|script_id| (script_id, option.clone())),
                );
            }
        }

        self.previous_keys_down = current_keys;
        for (script_id, key_option) in scripts_to_enqueue {
            let event_name = format!("key pressed '{}'", key_option);
            self.enqueue_script_for_base_actor_with_reason(script_id, Some(event_name.as_str()));
        }
    }

    fn mouse_x(&self) -> f64 {
        self.input_state
            .as_ref()
            .and_then(|shared| shared.lock().ok())
            .map(|input| input.mouse_x)
            .unwrap_or(0.0)
    }

    fn mouse_y(&self) -> f64 {
        self.input_state
            .as_ref()
            .and_then(|shared| shared.lock().ok())
            .map(|input| input.mouse_y)
            .unwrap_or(0.0)
    }

    fn mouse_down(&self) -> bool {
        self.input_state
            .as_ref()
            .and_then(|shared| shared.lock().ok())
            .map(|input| input.mouse_down)
            .unwrap_or(false)
    }

    fn stop_all_scripts(&mut self) {
        self.script_queue.clear();
        self.remaining_steps = 0;
    }

    fn stop_other_scripts_in_active_target(&mut self) {
        let target_index = self
            .actors
            .get(self.active_actor_id as usize)
            .map(|actor| actor.target_index);
        let Some(target_index) = target_index else {
            return;
        };
        let actor_targets = self
            .actors
            .iter()
            .map(|actor| actor.target_index)
            .collect::<Vec<_>>();
        self.script_queue.retain(|task| {
            actor_targets
                .get(task.actor_id as usize)
                .is_none_or(|actor_target| *actor_target != target_index)
        });
    }

    fn switch_costume_to(&mut self, costume: f64) {
        let mut numeric = self.value_to_number(costume);
        if let Some(string_index) = decode_string_id(costume) {
            let selector = self
                .strings
                .get(string_index)
                .map(|text| text.trim().to_ascii_lowercase())
                .unwrap_or_default();
            if selector == "next costume" || selector == "next backdrop" {
                numeric = self.costume_number + 1.0;
            }
        }
        if numeric.is_finite() && numeric > 0.0 {
            self.costume_number = numeric.floor().max(1.0);
            self.live_canvas_dirty = true;
        }
    }

    fn costume_number_value(&self) -> f64 {
        self.costume_number.floor().max(1.0)
    }

    fn costume_name_value(&mut self) -> f64 {
        let name = format!("costume{}", self.costume_number_value() as u64);
        let id = self.intern_string(&name);
        encode_string_id(id)
    }

    fn active_target_index(&self) -> u64 {
        self.actors
            .get(self.active_actor_id as usize)
            .map(|actor| actor.target_index)
            .unwrap_or(0)
    }

    fn resolve_target_index_by_name(&self, raw: &str) -> Option<u64> {
        let name = raw.trim();
        if name.is_empty() {
            return Some(self.active_target_index());
        }
        if name.eq_ignore_ascii_case("_stage_") || name.eq_ignore_ascii_case("stage") {
            let stage_index = self
                .target_render_data
                .iter()
                .position(|target| target.is_stage)
                .unwrap_or(0);
            return Some(stage_index as u64);
        }
        self.target_names
            .iter()
            .position(|candidate| candidate == name || candidate.trim().eq_ignore_ascii_case(name))
            .map(|index| index as u64)
    }

    fn sensing_of(&mut self, object: f64, property: f64) -> f64 {
        let target_name = self.value_as_string(object);
        let target_index = self
            .resolve_target_index_by_name(&target_name)
            .unwrap_or_else(|| self.active_target_index());
        let actor = self
            .base_actor_by_target
            .get(target_index as usize)
            .copied()
            .and_then(|actor_id| self.actor_snapshot(actor_id));

        let property_text = self.value_as_string(property).trim().to_ascii_lowercase();
        match property_text.as_str() {
            "x position" => actor.map(|state| state.sprite_x).unwrap_or(0.0),
            "y position" => actor.map(|state| state.sprite_y).unwrap_or(0.0),
            "direction" => actor.map(|state| state.direction_deg).unwrap_or(90.0),
            "size" => actor.map(|state| state.size_percent).unwrap_or(100.0),
            "costume #" | "costume number" | "backdrop #" | "backdrop number" => actor
                .map(|state| state.costume_number.floor().max(1.0))
                .unwrap_or(1.0),
            "costume name" | "backdrop name" => actor
                .map(|state| {
                    let name = format!("costume{}", state.costume_number.floor().max(1.0) as u64);
                    let id = self.intern_string(&name);
                    encode_string_id(id)
                })
                .unwrap_or_else(|| encode_string_id(EMPTY_STRING_ID)),
            _ => 0.0,
        }
    }
}

fn normalize_broadcast_message(raw: &str) -> String {
    let filtered = raw
        .chars()
        .filter(|ch| {
            !matches!(
                ch,
                '\u{200b}' | '\u{200c}' | '\u{200d}' | '\u{2060}' | '\u{feff}'
            )
        })
        .collect::<String>();
    filtered.trim().to_ascii_lowercase()
}

fn env_flag_enabled(name: &str) -> bool {
    env::var(name)
        .ok()
        .map(|raw| {
            matches!(
                raw.trim().to_ascii_lowercase().as_str(),
                "1" | "true" | "yes" | "on"
            )
        })
        .unwrap_or(false)
}

fn env_message_list(name: &str) -> Vec<String> {
    env::var(name)
        .ok()
        .map(|raw| {
            raw.split(',')
                .map(|part| part.trim())
                .filter(|part| !part.is_empty())
                .map(ToOwned::to_owned)
                .collect()
        })
        .unwrap_or_default()
}

fn string_is_not_actually_zero(value: &str) -> bool {
    for code in value.chars().map(|ch| ch as u32) {
        if code == 48 || code == 9 {
            return false;
        }
    }
    true
}

fn normalize_key_name(raw: &str) -> String {
    let lowered = raw.trim().to_ascii_lowercase();
    match lowered.as_str() {
        "left" | "left arrow" => "left arrow".to_string(),
        "right" | "right arrow" => "right arrow".to_string(),
        "up" | "up arrow" => "up arrow".to_string(),
        "down" | "down arrow" => "down arrow".to_string(),
        "space" | "spacebar" => "space".to_string(),
        _ => lowered,
    }
}

#[derive(Debug, Clone, Copy)]
struct CostumeTransform {
    actor_x: f64,
    actor_y: f64,
    scale: f64,
    cos_theta: f64,
    sin_theta: f64,
}

impl CostumeTransform {
    fn new(
        costume: &CostumeBitmap,
        actor_x: f64,
        actor_y: f64,
        direction_deg: f64,
        size_percent: f64,
    ) -> Option<Self> {
        if costume.width == 0 || costume.height == 0 {
            return None;
        }
        let scale = (size_percent / 100.0).max(0.0);
        if !scale.is_finite() || scale <= 0.0 {
            return None;
        }
        let theta = (direction_deg - 90.0).to_radians();
        Some(Self {
            actor_x,
            actor_y,
            scale,
            cos_theta: theta.cos(),
            sin_theta: theta.sin(),
        })
    }
}

fn source_to_world(
    costume: &CostumeBitmap,
    src_x: f64,
    src_y: f64,
    transform: &CostumeTransform,
) -> (f64, f64) {
    let local_x = src_x - costume.rotation_center_x;
    let local_y = costume.rotation_center_y - src_y;
    let scaled_x = local_x * transform.scale;
    let scaled_y = local_y * transform.scale;
    let world_x =
        transform.actor_x + scaled_x * transform.cos_theta - scaled_y * transform.sin_theta;
    let world_y =
        transform.actor_y + scaled_x * transform.sin_theta + scaled_y * transform.cos_theta;
    (world_x, world_y)
}

fn costume_pixel_bounds(
    costume: &CostumeBitmap,
    canvas_width: usize,
    canvas_height: usize,
    transform: &CostumeTransform,
) -> (i32, i32, i32, i32) {
    let corners = [
        (0.0, 0.0),
        (costume.width as f64, 0.0),
        (0.0, costume.height as f64),
        (costume.width as f64, costume.height as f64),
    ];

    let mut min_px = i32::MAX;
    let mut max_px = i32::MIN;
    let mut min_py = i32::MAX;
    let mut max_py = i32::MIN;
    for (src_x, src_y) in corners {
        let (world_x, world_y) = source_to_world(costume, src_x, src_y, transform);
        let px = scratch_to_pixel_x(world_x, canvas_width);
        let py = scratch_to_pixel_y(world_y, canvas_height);
        min_px = min_px.min(px);
        max_px = max_px.max(px);
        min_py = min_py.min(py);
        max_py = max_py.max(py);
    }

    (
        min_px.clamp(0, canvas_width as i32 - 1),
        max_px.clamp(0, canvas_width as i32 - 1),
        min_py.clamp(0, canvas_height as i32 - 1),
        max_py.clamp(0, canvas_height as i32 - 1),
    )
}

fn sample_costume_coordinates(
    costume: &CostumeBitmap,
    world_x: f64,
    world_y: f64,
    transform: &CostumeTransform,
) -> Option<(usize, usize)> {
    let dx = world_x - transform.actor_x;
    let dy = world_y - transform.actor_y;
    let local_x = (dx * transform.cos_theta + dy * transform.sin_theta) / transform.scale;
    let local_y = (-dx * transform.sin_theta + dy * transform.cos_theta) / transform.scale;
    let src_x = local_x + costume.rotation_center_x;
    let src_y = costume.rotation_center_y - local_y;
    if src_x < 0.0 || src_y < 0.0 || src_x >= costume.width as f64 || src_y >= costume.height as f64
    {
        return None;
    }
    let sample_x = src_x.round() as isize;
    let sample_y = src_y.round() as isize;
    if sample_x < 0
        || sample_y < 0
        || sample_x >= costume.width as isize
        || sample_y >= costume.height as isize
    {
        return None;
    }
    Some((sample_x as usize, sample_y as usize))
}

fn scratch_to_pixel_x(x: f64, width: usize) -> i32 {
    let normalized = ((x + 240.0) / 480.0).clamp(0.0, 1.0);
    let px = (normalized * (width.saturating_sub(1)) as f64).round() as i32;
    px.clamp(0, width as i32 - 1)
}

fn scratch_to_pixel_y(y: f64, height: usize) -> i32 {
    let normalized = ((180.0 - y) / 360.0).clamp(0.0, 1.0);
    let py = (normalized * (height.saturating_sub(1)) as f64).round() as i32;
    py.clamp(0, height as i32 - 1)
}

fn pixel_to_scratch_x(x: usize, width: usize) -> f64 {
    let max = width.saturating_sub(1).max(1) as f64;
    (x as f64 / max) * 480.0 - 240.0
}

fn pixel_to_scratch_y(y: usize, height: usize) -> f64 {
    let max = height.saturating_sub(1).max(1) as f64;
    180.0 - (y as f64 / max) * 360.0
}

fn next_random_unit(state: &mut RuntimeState) -> f64 {
    // Numerical Recipes LCG (deterministic and cheap for runtime integration).
    state.rng_state = state
        .rng_state
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1);
    let mantissa = state.rng_state >> 11;
    mantissa as f64 / ((1_u64 << 53) as f64)
}

fn parse_hex_color(raw: &str) -> Option<[u8; 3]> {
    let hex = raw
        .trim()
        .trim_start_matches('#')
        .trim_start_matches("0x")
        .trim_start_matches("0X");
    if hex.len() != 6 {
        return None;
    }
    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
    Some([r, g, b])
}

fn hue_to_rgb(color: f64) -> [u8; 3] {
    let hue = color.rem_euclid(200.0) * 360.0 / 200.0;
    hsv_to_rgb(hue, 1.0, 1.0)
}

/// Convert a decimal colour value to RGB, matching scratch-vm's
/// `Color.decimalToRgb`.  The integer is interpreted as 0xRRGGBB.
fn decimal_to_rgb(decimal: f64) -> [u8; 3] {
    let decimal = decimal as i64;
    let r = ((decimal >> 16) & 0xFF) as u8;
    let g = ((decimal >> 8) & 0xFF) as u8;
    let b = (decimal & 0xFF) as u8;
    [r, g, b]
}

fn rgb_to_hsv(rgb: [u8; 3]) -> (f64, f64, f64) {
    let r = (rgb[0] as f64) / 255.0;
    let g = (rgb[1] as f64) / 255.0;
    let b = (rgb[2] as f64) / 255.0;
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;

    let hue = if delta == 0.0 {
        0.0
    } else if max == r {
        60.0 * ((g - b) / delta).rem_euclid(6.0)
    } else if max == g {
        60.0 * (((b - r) / delta) + 2.0)
    } else {
        60.0 * (((r - g) / delta) + 4.0)
    };
    let saturation = if max == 0.0 { 0.0 } else { delta / max };
    let value = max;
    (hue, saturation, value)
}

fn hsv_to_rgb(hue: f64, saturation: f64, value: f64) -> [u8; 3] {
    let h = hue.rem_euclid(360.0);
    let s = saturation.clamp(0.0, 1.0);
    let v = value.clamp(0.0, 1.0);
    let c = v * s;
    let x = c * (1.0 - (((h / 60.0).rem_euclid(2.0)) - 1.0).abs());
    let m = v - c;

    let (r1, g1, b1) = if h < 60.0 {
        (c, x, 0.0)
    } else if h < 120.0 {
        (x, c, 0.0)
    } else if h < 180.0 {
        (0.0, c, x)
    } else if h < 240.0 {
        (0.0, x, c)
    } else if h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    [
        ((r1 + m) * 255.0).round().clamp(0.0, 255.0) as u8,
        ((g1 + m) * 255.0).round().clamp(0.0, 255.0) as u8,
        ((b1 + m) * 255.0).round().clamp(0.0, 255.0) as u8,
    ]
}

fn apply_mathop(op_code: u64, value: f64) -> f64 {
    match op_code {
        0 => value.abs(),
        1 => value.floor(),
        2 => value.ceil(),
        3 => value.sqrt(),
        4 => value.to_radians().sin(),
        5 => value.to_radians().cos(),
        6 => value.to_radians().tan(),
        7 => value.asin().to_degrees(),
        8 => value.acos().to_degrees(),
        9 => value.atan().to_degrees(),
        10 => value.ln(),
        11 => value.log10(),
        12 => value.exp(),
        13 => 10_f64.powf(value),
        _ => 0.0,
    }
}

fn js_round(value: f64) -> f64 {
    if !value.is_finite() {
        return value;
    }
    (value + 0.5).floor()
}
