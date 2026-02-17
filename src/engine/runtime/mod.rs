use std::cell::UnsafeCell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::env;
use std::fs::File;
use std::io::{self, IsTerminal, Write};
use std::path::Path;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
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
// Concurrency mode selection
// ---------------------------------------------------------------------------

/// Controls how fibers (cooperative Scratch script threads) are implemented.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConcurrencyMode {
    /// Each fiber gets a real OS thread (original behaviour).
    /// May exhaust OS resources when many clones are created.
    NativeThreads,
    /// Fibers use userspace context switching (no OS threads).
    /// Much lighter and avoids thread-exhaustion crashes.
    #[cfg(target_arch = "x86_64")]
    Userspace,
}

impl Default for ConcurrencyMode {
    fn default() -> Self {
        #[cfg(target_arch = "x86_64")]
        {
            Self::Userspace
        }
        #[cfg(not(target_arch = "x86_64"))]
        {
            Self::NativeThreads
        }
    }
}

impl ConcurrencyMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NativeThreads => "native-threads",
            #[cfg(target_arch = "x86_64")]
            Self::Userspace => "userspace",
        }
    }
}

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

// ---------------------------------------------------------------------------
// Userspace context switching (x86_64 only)
// ---------------------------------------------------------------------------

/// Saved callee-saved CPU registers for userspace cooperative context switching.
#[cfg(target_arch = "x86_64")]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct MachineContext {
    rbx: u64,
    rbp: u64,
    r12: u64,
    r13: u64,
    r14: u64,
    r15: u64,
    rsp: u64,
}

#[cfg(target_arch = "x86_64")]
impl MachineContext {
    const fn zeroed() -> Self {
        Self {
            rbx: 0,
            rbp: 0,
            r12: 0,
            r13: 0,
            r14: 0,
            r15: 0,
            rsp: 0,
        }
    }
}

/// Stack size for userspace fiber stacks (256 KiB).
#[cfg(target_arch = "x86_64")]
const USERSPACE_FIBER_STACK_SIZE: usize = 256 * 1024;

// Assembly routines for context switching on x86_64.
//
// `asm_swap_context(save, restore)`: saves the current callee-saved registers
// and RSP into `save`, then restores them from `restore` and returns to the
// saved program counter (via `ret`).
//
// `asm_fiber_trampoline`: initial entry point placed on a new fiber's stack.
// Moves the data pointer from r12 into rdi (first argument) and calls the
// Rust entry function `asm_fiber_entry`.
#[cfg(target_arch = "x86_64")]
std::arch::global_asm!(
    ".globl asm_swap_context",
    ".type asm_swap_context, @function",
    "asm_swap_context:",
    "mov [rdi + 0x00], rbx",
    "mov [rdi + 0x08], rbp",
    "mov [rdi + 0x10], r12",
    "mov [rdi + 0x18], r13",
    "mov [rdi + 0x20], r14",
    "mov [rdi + 0x28], r15",
    "mov [rdi + 0x30], rsp",
    "mov rbx, [rsi + 0x00]",
    "mov rbp, [rsi + 0x08]",
    "mov r12, [rsi + 0x10]",
    "mov r13, [rsi + 0x18]",
    "mov r14, [rsi + 0x20]",
    "mov r15, [rsi + 0x28]",
    "mov rsp, [rsi + 0x30]",
    "ret",
    ".size asm_swap_context, . - asm_swap_context",
    "",
    ".globl asm_fiber_trampoline",
    ".type asm_fiber_trampoline, @function",
    "asm_fiber_trampoline:",
    // r12 = FiberStartInfo pointer (set up when the context was created)
    "mov rdi, r12",
    // Align stack to 16 bytes before the call (ABI requirement).
    "and rsp, -16",
    "call asm_fiber_entry",
    // Should never return – trap.
    "ud2",
    ".size asm_fiber_trampoline, . - asm_fiber_trampoline",
);

#[cfg(target_arch = "x86_64")]
unsafe extern "C" {
    fn asm_swap_context(save: *mut MachineContext, restore: *const MachineContext);
    fn asm_fiber_trampoline();
}

// Thread-local scheduler context for userspace cooperative switching.
#[cfg(target_arch = "x86_64")]
thread_local! {
    static SCHEDULER_CTX: UnsafeCell<MachineContext> =
        const { UnsafeCell::new(MachineContext::zeroed()) };
}

/// Data passed from the scheduler to the userspace fiber trampoline.
/// Must live on the heap (via `Box`) so that its address is stable even when
/// the owning `Fiber` struct is moved (e.g. when the `Vec<Fiber>` grows).
#[cfg(target_arch = "x86_64")]
struct FiberStartInfo {
    function: ScriptEntry,
    state_ptr: *mut RuntimeState,
    control: *const FiberControl,
}

/// Rust entry point called from `asm_fiber_trampoline`.
#[cfg(target_arch = "x86_64")]
#[unsafe(no_mangle)]
unsafe extern "C" fn asm_fiber_entry(info_ptr: *mut FiberStartInfo) {
    let info = unsafe { &*info_ptr };
    // Execute the JIT-compiled Scratch script.
    unsafe {
        (info.function)(info.state_ptr);
    }
    // Mark this fiber as finished.
    unsafe {
        (*info.control).uspace_done.store(true, Ordering::Relaxed);
    }
    // Switch back to the scheduler.
    #[cfg(target_arch = "x86_64")]
    SCHEDULER_CTX.with(|sched| unsafe {
        asm_swap_context((*info.control).fiber_ctx.get(), sched.get() as *const _);
    });
    // Should never reach here.
    unreachable!("fiber entry returned after final context switch");
}

// ---------------------------------------------------------------------------
// FiberControl – synchronisation between scheduler and fiber
// ---------------------------------------------------------------------------

/// Synchronisation primitive shared between a fiber and the scheduler.
/// Supports both OS-thread mode and userspace context-switching mode.
struct FiberControl {
    mode: ConcurrencyMode,
    // --- Native-thread mode fields ---
    state: Mutex<FiberSyncState>,
    condvar: Condvar,
    // --- Userspace mode fields ---
    #[cfg(target_arch = "x86_64")]
    fiber_ctx: UnsafeCell<MachineContext>,
    #[cfg(target_arch = "x86_64")]
    uspace_done: AtomicBool,
}

// Safety: In NativeThreads mode the userspace fields are never accessed.
// In Userspace mode everything runs on a single OS thread, so concurrent
// access cannot occur.
unsafe impl Sync for FiberControl {}

impl std::fmt::Debug for FiberControl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let state = self
            .state
            .lock()
            .map(|g| *g)
            .unwrap_or(FiberSyncState::Done);
        f.debug_struct("FiberControl")
            .field("mode", &self.mode)
            .field("state", &state)
            .finish()
    }
}

impl FiberControl {
    fn new(mode: ConcurrencyMode) -> Self {
        Self {
            mode,
            state: Mutex::new(FiberSyncState::WaitingToStart),
            condvar: Condvar::new(),
            #[cfg(target_arch = "x86_64")]
            fiber_ctx: UnsafeCell::new(MachineContext::zeroed()),
            #[cfg(target_arch = "x86_64")]
            uspace_done: AtomicBool::new(false),
        }
    }

    /// Called by the **fiber thread** (native-thread mode only) – block
    /// until the scheduler sets state to `Running`.
    fn wait_for_resume(&self) {
        let mut guard = self.state.lock().unwrap();
        while *guard != FiberSyncState::Running {
            guard = self.condvar.wait(guard).unwrap();
        }
    }

    /// Called at a yield-point inside JIT code (both modes).
    fn yield_to_scheduler(&self) {
        match self.mode {
            ConcurrencyMode::NativeThreads => {
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
            #[cfg(target_arch = "x86_64")]
            ConcurrencyMode::Userspace => {
                // Save fiber context and jump back to the scheduler.
                SCHEDULER_CTX.with(|sched| unsafe {
                    asm_swap_context(self.fiber_ctx.get(), sched.get() as *const _);
                });
            }
        }
    }

    /// Called by the native-thread fiber when the JIT function returns.
    fn signal_done(&self) {
        let mut guard = self.state.lock().unwrap();
        *guard = FiberSyncState::Done;
        self.condvar.notify_all();
    }

    /// Called by the **scheduler** – resume the fiber.
    /// In native-thread mode this wakes the blocked thread.
    /// In userspace mode this performs a context switch (blocking until
    /// the fiber yields or finishes).
    fn resume(&self) {
        match self.mode {
            ConcurrencyMode::NativeThreads => {
                let mut guard = self.state.lock().unwrap();
                *guard = FiberSyncState::Running;
                self.condvar.notify_all();
            }
            #[cfg(target_arch = "x86_64")]
            ConcurrencyMode::Userspace => {
                // Switch to the fiber context.  Returns when the fiber
                // yields or finishes.
                SCHEDULER_CTX.with(|sched| unsafe {
                    asm_swap_context(sched.get(), self.fiber_ctx.get() as *const _);
                });
            }
        }
    }

    /// Called by the **scheduler** – block until the fiber yields or finishes.
    /// In userspace mode `resume()` already does this, so this is a no-op
    /// that simply returns the current state.
    fn wait_for_yield_or_done(&self) -> FiberSyncState {
        match self.mode {
            ConcurrencyMode::NativeThreads => {
                let mut guard = self.state.lock().unwrap();
                loop {
                    match *guard {
                        FiberSyncState::Yielded | FiberSyncState::Done => return *guard,
                        _ => guard = self.condvar.wait(guard).unwrap(),
                    }
                }
            }
            #[cfg(target_arch = "x86_64")]
            ConcurrencyMode::Userspace => {
                // resume() already blocked until the fiber switched back.
                if self.uspace_done.load(Ordering::Relaxed) {
                    FiberSyncState::Done
                } else {
                    FiberSyncState::Yielded
                }
            }
        }
    }

    fn is_done(&self) -> bool {
        match self.mode {
            ConcurrencyMode::NativeThreads => {
                matches!(*self.state.lock().unwrap(), FiberSyncState::Done)
            }
            #[cfg(target_arch = "x86_64")]
            ConcurrencyMode::Userspace => self.uspace_done.load(Ordering::Relaxed),
        }
    }
}

/// A single cooperatively-scheduled "thread" that runs one Scratch script.
pub struct Fiber {
    #[allow(dead_code)]
    script_id: u64,
    #[allow(dead_code)]
    actor_id: u64,
    /// Tracks which actor is currently loaded in the RuntimeState fields.
    current_actor_id: u64,
    /// Depth of active warp procedure calls for this fiber.
    warp_depth: u32,
    wait_group_id: Option<u64>,
    control: Arc<FiberControl>,
    /// OS thread handle (native-thread mode only).
    handle: Option<thread::JoinHandle<()>>,
    /// Heap-allocated stack for the fiber (userspace mode only).
    #[cfg(target_arch = "x86_64")]
    _userspace_stack: Option<Box<[u8]>>,
    /// Start-info kept alive while the fiber runs (userspace mode only).
    #[cfg(target_arch = "x86_64")]
    _start_info: Option<Box<FiberStartInfo>>,
}

impl Fiber {
    /// Create a new fiber.
    ///
    /// * `NativeThreads` mode: spawns a real OS thread that blocks until
    ///   `resume()` is called.
    /// * `Userspace` mode: allocates a small stack and sets up a
    ///   `MachineContext` for context-switch–based execution.
    fn spawn(
        state_ptr: *mut RuntimeState,
        function: ScriptEntry,
        script_id: u64,
        actor_id: u64,
        wait_group_id: Option<u64>,
        mode: ConcurrencyMode,
    ) -> Self {
        let control = Arc::new(FiberControl::new(mode));

        match mode {
            ConcurrencyMode::NativeThreads => {
                let control_for_thread = Arc::clone(&control);
                let raw = state_ptr as usize; // usize is Send

                let handle = thread::Builder::new()
                    .stack_size(256 * 1024)
                    .spawn(move || {
                        let state_ptr = raw as *mut RuntimeState;
                        control_for_thread.wait_for_resume();
                        unsafe {
                            function(state_ptr);
                        }
                        control_for_thread.signal_done();
                    })
                    .expect("failed to spawn fiber thread");

                Fiber {
                    script_id,
                    actor_id,
                    current_actor_id: actor_id,
                    warp_depth: 0,
                    wait_group_id,
                    control,
                    handle: Some(handle),
                    #[cfg(target_arch = "x86_64")]
                    _userspace_stack: None,
                    #[cfg(target_arch = "x86_64")]
                    _start_info: None,
                }
            }
            #[cfg(target_arch = "x86_64")]
            ConcurrencyMode::Userspace => {
                // Allocate a stack for this fiber.
                let stack = vec![0u8; USERSPACE_FIBER_STACK_SIZE].into_boxed_slice();

                // Build the start-info on the heap so its address is stable.
                let start_info = Box::new(FiberStartInfo {
                    function,
                    state_ptr,
                    control: Arc::as_ptr(&control),
                });

                // Set up the initial MachineContext so that the first
                // `asm_swap_context` into this fiber lands in
                // `asm_fiber_trampoline`, which reads the FiberStartInfo
                // pointer from r12 and calls `asm_fiber_entry`.
                let stack_top = (stack.as_ptr() as usize + stack.len()) & !0xF;
                let rsp = stack_top - 8; // space for the "return address"
                unsafe {
                    // Place the trampoline address where `ret` will pop it.
                    std::ptr::write(
                        rsp as *mut usize,
                        asm_fiber_trampoline as *const () as usize,
                    );
                }

                let ctx = unsafe { &mut *control.fiber_ctx.get() };
                *ctx = MachineContext {
                    rbx: 0,
                    rbp: 0,
                    r12: &*start_info as *const FiberStartInfo as u64,
                    r13: 0,
                    r14: 0,
                    r15: 0,
                    rsp: rsp as u64,
                };

                Fiber {
                    script_id,
                    actor_id,
                    current_actor_id: actor_id,
                    warp_depth: 0,
                    wait_group_id,
                    control,
                    handle: None,
                    _userspace_stack: Some(stack),
                    _start_info: Some(start_info),
                }
            }
        }
    }

    fn is_done(&self) -> bool {
        self.control.is_done()
    }

    fn join(&mut self) {
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
        // In userspace mode there is no thread to join; dropping the fiber
        // frees the stack and start-info automatically.
    }
}
const EMPTY_STRING_ID: usize = 0;
pub const STRING_TAG_MASK: u64 = 0x7fff_0000_0000_0000;
pub const STRING_TAG_BITS: u64 = 0x7ff9_0000_0000_0000;
const STRING_PAYLOAD_MASK: u64 = 0x0000_ffff_ffff_ffff;
const DEFAULT_LIVE_CANVAS_SYNC_INTERVAL: Duration = Duration::from_millis(16);
const FRAME_SLEEP_COARSE_MARGIN: Duration = Duration::from_micros(800);
// scratch-vm Sequencer uses WORK_TIME = currentStepTime * 0.75.
const SCRATCH_VM_WORK_TIME_RATIO: f64 = 0.75;
// scratch-vm Sequencer uses WARP_TIME = 500ms.
const SCRATCH_VM_WARP_TIME: Duration = Duration::from_millis(500);

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
    pub name: String,
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

#[inline(always)]
pub fn encode_string_id(index: usize) -> f64 {
    let payload = (index as u64).saturating_add(1) & STRING_PAYLOAD_MASK;
    f64::from_bits(STRING_TAG_BITS | payload)
}

#[inline(always)]
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

/// Fast check whether an f64 value carries a NaN-boxed string tag.
/// This is the hot-path guard for arithmetic: if false, the value is a
/// plain IEEE-754 number and can be used directly.
#[inline(always)]
pub fn is_string_tagged(value: f64) -> bool {
    (value.to_bits() & STRING_TAG_MASK) == STRING_TAG_BITS
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PenRenderMode {
    CpuRealtime,
    GpuBatch,
}

impl PenRenderMode {
    pub fn as_str(self) -> &'static str {
        match self {
            PenRenderMode::CpuRealtime => "cpu",
            PenRenderMode::GpuBatch => "gpu-batch",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PenStrokeStyle {
    pub color: [u8; 3],
    pub alpha: f32,
    pub size: f32,
}

#[derive(Debug, Clone, Copy)]
pub enum PenBatchCommand {
    Clear,
    Line {
        x0: f32,
        y0: f32,
        x1: f32,
        y1: f32,
        style: PenStrokeStyle,
    },
    Disc {
        x: f32,
        y: f32,
        style: PenStrokeStyle,
    },
}

#[derive(Debug, Clone, Copy)]
struct ScriptTask {
    script_id: u64,
    actor_id: u64,
    wait_group_id: Option<u64>,
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
    variable_names: Vec<String>,
    variable_target_indices: Vec<u64>,
    pub lists: Vec<Vec<f64>>,
    pub executed_block_count: u64,
    pub remaining_steps: u64,
    step_budget: u64,
    relax_procedure_loop_budget: bool,
    pub warp_depth: u32,
    answer_value: f64,
    timer_start: Instant,
    strings: Vec<String>,
    string_index: HashMap<String, usize>,
    canvas_width: usize,
    canvas_height: usize,
    pen_rgba: Vec<u8>,
    pen_render_mode: PenRenderMode,
    pending_pen_batch: Vec<PenBatchCommand>,
    canvas_rgb: Vec<u8>,
    target_render_data: Vec<TargetRenderData>,
    target_initial_visuals: Vec<TargetInitialVisualState>,
    live_canvas: Option<Arc<Mutex<Vec<u8>>>>,
    live_pen_layer: Option<Arc<Mutex<Vec<u8>>>>,
    live_pen_batch: Option<Arc<Mutex<Vec<PenBatchCommand>>>>,
    pen_batch_sent_count: usize,
    live_canvas_dirty: bool,
    live_canvas_last_sync: Instant,
    live_canvas_sync_interval: Duration,
    live_canvas_generation: Option<Arc<AtomicU64>>,
    input_state: Option<Arc<Mutex<InputState>>>,
    ask_prompt_state: Option<Arc<AskPromptState>>,
    stop_requested: Option<Arc<AtomicBool>>,
    dump_vars_requested: Option<Arc<AtomicBool>>,
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
    broadcast_message_targets: HashMap<String, Vec<u64>>,
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
    wait_group_pending: HashMap<u64, u64>,
    next_wait_group_id: u64,
    processing_queued_script: bool,
    /// When set, the runtime is executing inside a fiber and yield-points
    /// should cooperatively yield to the scheduler instead of sleeping.
    active_fiber_control: Option<Arc<FiberControl>>,
    concurrency_mode: ConcurrencyMode,
    rng_state: u64,
    trace_broadcasts: bool,
    debug_mode: bool,
    break_on_messages: HashSet<String>,
}

impl RuntimeState {
    #[inline(always)]
    fn div_round_u32(value: u32, divisor: u32) -> u32 {
        (value + (divisor / 2)) / divisor
    }

    #[inline(always)]
    fn blend_unpremultiplied_rgba_pixel(
        pixels: &mut [u8],
        offset: usize,
        src_r: u8,
        src_g: u8,
        src_b: u8,
        src_alpha: u8,
    ) {
        let src_alpha_u32 = src_alpha as u32;
        if src_alpha_u32 == 0 {
            return;
        }
        if src_alpha_u32 >= 255 {
            pixels[offset] = src_r;
            pixels[offset + 1] = src_g;
            pixels[offset + 2] = src_b;
            pixels[offset + 3] = 255;
            return;
        }

        let dst_alpha = pixels[offset + 3] as u32;
        let inv_src_alpha = 255u32 - src_alpha_u32;

        let out_alpha = src_alpha_u32 + Self::div_round_u32(dst_alpha * inv_src_alpha, 255);
        if out_alpha == 0 {
            return;
        }

        let out_r_premul = (src_r as u32) * src_alpha_u32
            + Self::div_round_u32((pixels[offset] as u32) * dst_alpha * inv_src_alpha, 255);
        let out_g_premul = (src_g as u32) * src_alpha_u32
            + Self::div_round_u32((pixels[offset + 1] as u32) * dst_alpha * inv_src_alpha, 255);
        let out_b_premul = (src_b as u32) * src_alpha_u32
            + Self::div_round_u32((pixels[offset + 2] as u32) * dst_alpha * inv_src_alpha, 255);

        pixels[offset] = Self::div_round_u32(out_r_premul * 255, out_alpha).min(255) as u8;
        pixels[offset + 1] = Self::div_round_u32(out_g_premul * 255, out_alpha).min(255) as u8;
        pixels[offset + 2] = Self::div_round_u32(out_b_premul * 255, out_alpha).min(255) as u8;
        pixels[offset + 3] = out_alpha.min(255) as u8;
    }

    #[inline(always)]
    fn pen_brush_shape_for_size(&self, pen_size: f64) -> (i32, f64) {
        let canvas_scale = (self.canvas_width as f64) / (STAGE_WIDTH as f64);
        let scaled_pen_size = pen_size.max(1.0) * canvas_scale.max(1.0);
        let radius = ((scaled_pen_size - 1.0) / 2.0).max(0.0);
        let extent = radius.ceil() as i32;
        (extent, radius * radius)
    }

    #[inline(always)]
    fn current_pen_style(&self) -> PenStrokeStyle {
        PenStrokeStyle {
            color: self.pen_color,
            alpha: self.pen_alpha.clamp(0.0, 1.0) as f32,
            size: self.pen_size.max(1.0) as f32,
        }
    }

    fn push_pen_batch_command(&mut self, command: PenBatchCommand) {
        self.pending_pen_batch.push(command);
        self.live_canvas_dirty = true;
    }

    fn rasterize_pen_batch_for_cpu_output(&mut self) {
        if self.pen_render_mode != PenRenderMode::GpuBatch {
            return;
        }
        self.pen_rgba.fill(0);
        let mut commands = Vec::new();
        commands.extend_from_slice(&self.pending_pen_batch);
        for command in commands {
            self.apply_pen_batch_command_cpu(command);
        }
    }

    fn apply_pen_batch_command_cpu(&mut self, command: PenBatchCommand) {
        match command {
            PenBatchCommand::Clear => self.pen_rgba.fill(0),
            PenBatchCommand::Line {
                x0,
                y0,
                x1,
                y1,
                style,
            } => self.draw_line_with_style_cpu(x0 as f64, y0 as f64, x1 as f64, y1 as f64, style),
            PenBatchCommand::Disc { x, y, style } => {
                self.draw_disc_with_style_cpu(x as f64, y as f64, style)
            }
        }
    }

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
        variable_names: Vec<String>,
        variable_target_indices: Vec<u64>,
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
            variable_names,
            variable_target_indices,
            lists: initial_lists,
            executed_block_count: 0,
            remaining_steps: step_budget,
            step_budget,
            relax_procedure_loop_budget: false,
            warp_depth: 0,
            answer_value: encode_string_id(EMPTY_STRING_ID),
            timer_start: Instant::now(),
            strings,
            string_index,
            canvas_width: STAGE_WIDTH,
            canvas_height: STAGE_HEIGHT,
            pen_rgba: vec![0; STAGE_WIDTH * STAGE_HEIGHT * 4],
            pen_render_mode: PenRenderMode::CpuRealtime,
            pending_pen_batch: Vec::new(),
            canvas_rgb: vec![255; STAGE_WIDTH * STAGE_HEIGHT * 3],
            target_render_data: Vec::new(),
            target_initial_visuals: Vec::new(),
            live_canvas: None,
            live_pen_layer: None,
            live_pen_batch: None,
            pen_batch_sent_count: 0,
            live_canvas_dirty: false,
            live_canvas_last_sync: Instant::now(),
            live_canvas_sync_interval: DEFAULT_LIVE_CANVAS_SYNC_INTERVAL,
            live_canvas_generation: None,
            input_state: None,
            ask_prompt_state: None,
            stop_requested: None,
            dump_vars_requested: None,
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
            broadcast_message_targets: HashMap::new(),
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
            wait_group_pending: HashMap::new(),
            next_wait_group_id: 1,
            processing_queued_script: false,
            active_fiber_control: None,
            concurrency_mode: ConcurrencyMode::default(),
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
        self.broadcast_message_targets.clear();
        for (message, targets) in self
            .broadcast_messages
            .iter()
            .zip(self.broadcast_targets.iter())
        {
            let normalized = normalize_broadcast_message(message);
            if normalized.is_empty() {
                continue;
            }
            self.broadcast_message_targets
                .entry(normalized)
                .or_default()
                .extend(targets.iter().copied());
        }
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
        self.wait_group_pending.clear();
        self.next_wait_group_id = 1;
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

    fn dequeue_script(&mut self) -> Option<ScriptTask> {
        self.script_queue.pop_front()
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
        let previous_warp_depth = self.warp_depth;

        self.active_actor_id = actor_id;
        self.load_actor_from_snapshot(actor_snapshot);
        // Script invocations have independent warp state.
        self.warp_depth = 0;

        let state_ptr = self as *mut RuntimeState;
        unsafe {
            function(state_ptr);
        }

        self.persist_runtime_into_actor(actor_id);
        self.warp_depth = previous_warp_depth;

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
        self.enqueue_task(script_id, actor_id, reason, None);
    }

    fn enqueue_task(
        &mut self,
        script_id: u64,
        actor_id: u64,
        reason: Option<&str>,
        wait_group_id: Option<u64>,
    ) {
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
            wait_group_id,
        });
        if let Some(group_id) = wait_group_id {
            let entry = self.wait_group_pending.entry(group_id).or_insert(0);
            *entry = entry.saturating_add(1);
        }
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

    fn create_wait_group(&mut self) -> u64 {
        let id = self.next_wait_group_id;
        self.next_wait_group_id = self.next_wait_group_id.saturating_add(1);
        self.wait_group_pending.insert(id, 0);
        id
    }

    fn complete_wait_group_task(&mut self, wait_group_id: Option<u64>) {
        let Some(group_id) = wait_group_id else {
            return;
        };
        let mut remove_entry = false;
        if let Some(remaining) = self.wait_group_pending.get_mut(&group_id) {
            if *remaining > 0 {
                *remaining -= 1;
            }
            if *remaining == 0 {
                remove_entry = true;
            }
        }
        if remove_entry {
            self.wait_group_pending.remove(&group_id);
        }
    }

    fn wait_group_has_pending_tasks(&self, wait_group_id: u64) -> bool {
        self.wait_group_pending
            .get(&wait_group_id)
            .copied()
            .unwrap_or(0)
            > 0
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
        if message.is_empty() {
            return Vec::new();
        }
        self.broadcast_message_targets
            .get(&message)
            .cloned()
            .unwrap_or_default()
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
            self.enqueue_task(script_id, new_actor_id, Some("clone start"), None);
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
        let mut removed_wait_groups = Vec::new();
        self.script_queue.retain(|task| {
            let keep = task.actor_id != actor_id;
            if !keep {
                removed_wait_groups.push(task.wait_group_id);
            }
            keep
        });
        for wait_group_id in removed_wait_groups {
            self.complete_wait_group_task(wait_group_id);
        }
        self.live_canvas_dirty = true;
    }

    pub fn write_canvas_ppm<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        self.rasterize_pen_batch_for_cpu_output();
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

    pub fn set_pen_render_mode(&mut self, mode: PenRenderMode) {
        self.pen_render_mode = mode;
        self.live_canvas_dirty = true;
    }

    pub fn canvas_dimensions(&self) -> (usize, usize) {
        (self.canvas_width, self.canvas_height)
    }

    pub fn canvas_rgb_copy(&self) -> Vec<u8> {
        self.canvas_rgb.clone()
    }

    pub fn pen_rgba_copy(&self) -> Vec<u8> {
        self.pen_rgba.clone()
    }

    pub fn pen_batch_copy(&self) -> Vec<PenBatchCommand> {
        self.pending_pen_batch.clone()
    }

    pub fn attach_live_canvas(&mut self, live_canvas: Arc<Mutex<Vec<u8>>>) {
        self.live_canvas = Some(live_canvas);
        self.live_canvas_dirty = true;
        self.sync_live_canvas_if_due(true);
    }

    pub fn attach_live_pen_layer(&mut self, live_pen_layer: Arc<Mutex<Vec<u8>>>) {
        self.live_pen_layer = Some(live_pen_layer);
        self.live_canvas_dirty = true;
        self.sync_live_canvas_if_due(true);
    }

    pub fn attach_live_pen_batch(&mut self, live_pen_batch: Arc<Mutex<Vec<PenBatchCommand>>>) {
        self.live_pen_batch = Some(live_pen_batch);
        self.live_canvas_dirty = true;
        self.sync_live_canvas_if_due(true);
    }

    pub fn attach_live_canvas_generation(&mut self, generation: Arc<AtomicU64>) {
        self.live_canvas_generation = Some(generation);
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

    pub fn attach_dump_vars_flag(&mut self, dump_vars_requested: Arc<AtomicBool>) {
        self.dump_vars_requested = Some(dump_vars_requested);
    }

    pub fn dump_variables(&self) {
        eprintln!("\n=== Variable Dump ===");
        for (i, name) in self.variable_names.iter().enumerate() {
            if i < self.variables.len() {
                let value = self.variables[i];
                eprintln!("  {} = {}", name, self.debug_value(value));
            }
        }
        eprintln!("=====================\n");
    }

    pub fn set_debug_mode(&mut self, enabled: bool) {
        self.debug_mode = enabled;
    }

    pub fn set_concurrency_mode(&mut self, mode: ConcurrencyMode) {
        self.concurrency_mode = mode;
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
        // In turbo mode (no frame pacing), give fibers an effectively
        // unlimited step budget so they don't yield unnecessarily.
        // The loop guard's time-slice checks and stop_requested flags are
        // still in effect, so infinite loops remain interruptible.
        if self.frame_duration.is_none() {
            self.step_budget = u64::MAX;
            self.remaining_steps = u64::MAX;
        }
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

    #[inline(always)]
    fn intern_string(&mut self, text: &str) -> usize {
        if let Some(index) = self.string_index.get(text).copied() {
            return index;
        }
        let index = self.strings.len();
        self.strings.push(text.to_string());
        self.string_index.insert(text.to_string(), index);
        index
    }

    #[inline(always)]
    fn value_as_string(&self, value: f64) -> String {
        if !is_string_tagged(value) {
            if !value.is_finite() {
                return String::new();
            }
            return value.to_string();
        }
        let payload = value.to_bits() & STRING_PAYLOAD_MASK;
        if payload == 0 {
            return String::new();
        }
        let index = (payload - 1) as usize;
        self.strings.get(index).cloned().unwrap_or_default()
    }

    #[inline(always)]
    fn value_to_number(&self, value: f64) -> f64 {
        // Fast path: if the value is not a NaN-boxed string, return directly.
        // This avoids the full decode_string_id bit-manipulation for the
        // overwhelmingly common case of a plain IEEE-754 number.
        if !is_string_tagged(value) {
            return value;
        }
        self.value_to_number_slow(value)
    }

    #[cold]
    #[inline(never)]
    fn value_to_number_slow(&self, value: f64) -> f64 {
        let payload = value.to_bits() & STRING_PAYLOAD_MASK;
        if payload == 0 {
            return 0.0;
        }
        let index = (payload - 1) as usize;
        self.strings
            .get(index)
            .map(|text| text.trim().parse::<f64>().unwrap_or(0.0))
            .unwrap_or(0.0)
    }

    fn parse_scratch_number_for_compare(&self, text: &str) -> Option<f64> {
        // Scratch comparison number parsing, matching the official VM's
        // `Cast.compare` + `isNotActuallyZero` behaviour:
        //
        // 1. Try to parse the string as a number.
        // 2. If the result is 0.0, apply `isNotActuallyZero`: if the original
        //    text does NOT contain '0' (0x30) or '\t' (0x09), treat the value
        //    as non-numeric (→ fall back to string comparison).
        //
        // Empty / whitespace-only strings are handled here too:
        //   - ""  → Rust parse fails; contains neither '0' nor '\t' → None
        //   - "\t"→ Rust parse fails; but contains '\t' → Some(0.0)

        let trimmed = text.trim();

        if trimmed.is_empty() {
            // JavaScript: Number("") === 0 and Number("\t") === 0.
            // Check whether the *original* text is "actually zero".
            return if text.bytes().any(|b| b == b'0' || b == b'\t') {
                Some(0.0)
            } else {
                None
            };
        }

        match trimmed.parse::<f64>() {
            Ok(num) if num == 0.0 => {
                // Parsed to zero – apply isNotActuallyZero on original text.
                if text.bytes().any(|b| b == b'0' || b == b'\t') {
                    Some(0.0)
                } else {
                    None
                }
            }
            Ok(num) if num.is_finite() => Some(num),
            _ => None,
        }
    }

    #[inline(always)]
    fn value_to_number_for_compare(&self, value: f64) -> Option<f64> {
        if !is_string_tagged(value) {
            return if value.is_nan() { None } else { Some(value) };
        }
        let payload = value.to_bits() & STRING_PAYLOAD_MASK;
        if payload == 0 {
            return None;
        }
        let index = (payload - 1) as usize;
        self.strings
            .get(index)
            .and_then(|text| self.parse_scratch_number_for_compare(text))
    }

    #[inline(always)]
    fn compare_values(&self, left: f64, right: f64) -> i8 {
        // Fast path: both are plain numbers (not NaN-boxed strings)
        if !is_string_tagged(left) && !is_string_tagged(right) {
            // Both are raw f64 — compare directly (NaN handled by Scratch rules)
            if left.is_nan() || right.is_nan() {
                // Fall through to slow path for NaN edge cases
            } else {
                return if left == right {
                    0
                } else if left < right {
                    -1
                } else {
                    1
                };
            }
        }
        self.compare_values_slow(left, right)
    }

    #[cold]
    #[inline(never)]
    fn compare_values_slow(&self, left: f64, right: f64) -> i8 {
        // Scratch comparison follows the official VM's Cast.compare() rules:
        // 1. Try to interpret both values as numbers (with isNotActuallyZero).
        // 2. If both succeed → numeric comparison.
        // 3. Otherwise → case-insensitive string comparison.

        let left_as_number = self.value_to_number_for_compare(left);
        let right_as_number = self.value_to_number_for_compare(right);

        match (left_as_number, right_as_number) {
            (Some(l), Some(r)) => {
                if l == r {
                    0
                } else if l < r {
                    -1
                } else {
                    1
                }
            }
            _ => {
                let left_str = self.value_as_string(left).to_lowercase();
                let right_str = self.value_as_string(right).to_lowercase();

                if left_str == right_str {
                    0
                } else if left_str < right_str {
                    -1
                } else {
                    1
                }
            }
        }
    }

    fn values_equal(&self, left: f64, right: f64) -> bool {
        self.compare_values(left, right) == 0
    }

    fn clear_canvas(&mut self) {
        self.pen_rgba.fill(0);
        self.pending_pen_batch.clear();
        self.pending_pen_batch.push(PenBatchCommand::Clear);
        self.pen_batch_sent_count = 0;
        self.live_canvas_dirty = true;
    }

    fn move_sprite_to(&mut self, new_x: f64, new_y: f64) {
        if !new_x.is_finite() || !new_y.is_finite() {
            return;
        }
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
        let use_split_pen_layers = self.live_pen_layer.is_some() || self.live_pen_batch.is_some();
        if use_split_pen_layers {
            self.compose_canvas_rgb_without_pen();
        } else {
            self.rasterize_pen_batch_for_cpu_output();
            self.compose_canvas_rgb();
        }
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
        drop(guard);

        if use_split_pen_layers {
            if let Some(live_pen_layer) = &self.live_pen_layer {
                if let Ok(mut pen_guard) = live_pen_layer.lock() {
                    if pen_guard.len() != self.pen_rgba.len() {
                        *pen_guard = vec![0; self.pen_rgba.len()];
                    }
                    pen_guard.copy_from_slice(&self.pen_rgba);
                }
            }
        }

        if let Some(live_pen_batch) = &self.live_pen_batch {
            if let Ok(mut batch_guard) = live_pen_batch.lock() {
                if self.pen_batch_sent_count > self.pending_pen_batch.len() {
                    batch_guard.clear();
                    self.pen_batch_sent_count = 0;
                }
                if self.pen_batch_sent_count < self.pending_pen_batch.len() {
                    batch_guard
                        .extend_from_slice(&self.pending_pen_batch[self.pen_batch_sent_count..]);
                    self.pen_batch_sent_count = self.pending_pen_batch.len();
                }
            }
        }

        if let Some(generation) = &self.live_canvas_generation {
            generation.fetch_add(1, Ordering::Release);
        }
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
        let work_time =
            Duration::from_secs_f64(frame_duration.as_secs_f64() * SCRATCH_VM_WORK_TIME_RATIO);
        self.should_yield_for_time_slice(work_time)
    }

    fn should_yield_for_warp_time(&mut self) -> bool {
        self.should_yield_for_time_slice(SCRATCH_VM_WARP_TIME)
    }

    fn should_yield_for_time_slice(&mut self, slice: Duration) -> bool {
        let Some(_) = self.frame_duration else {
            return false;
        };
        let now = Instant::now();
        let tick_started_at = *self.current_tick_started_at.get_or_insert(now);
        now.duration_since(tick_started_at) >= slice
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
        let tick_started_at = self.current_tick_started_at.unwrap_or(now);
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
        while let Some(task) = self.dequeue_script() {
            if self
                .stop_requested
                .as_ref()
                .is_some_and(|stop| stop.load(Ordering::Relaxed))
            {
                break;
            }
            self.processing_queued_script = true;
            self.run_script(task.script_id, task.actor_id);
            self.processing_queued_script = false;
            self.complete_wait_group_task(task.wait_group_id);
        }
        self.flush_live_canvas();
    }

    /// Execute the program concurrently: each script runs as a cooperative
    /// fiber and all active fibers advance one yield-step per tick.
    pub fn execute_concurrent(&mut self) {
        let state_ptr = self as *mut RuntimeState;
        let mode = self.concurrency_mode;

        let mut fibers: Vec<Fiber> = Vec::new();

        // Spawn fibers for all initially-queued scripts.
        while let Some(task) = self.dequeue_script() {
            let function = match self.script_functions.get(task.script_id as usize).copied() {
                Some(f) => f,
                None => {
                    self.complete_wait_group_task(task.wait_group_id);
                    continue;
                }
            };
            let Some(actor) = self.actor_snapshot(task.actor_id) else {
                self.complete_wait_group_task(task.wait_group_id);
                continue;
            };
            if !actor.alive {
                self.complete_wait_group_task(task.wait_group_id);
                continue;
            }
            fibers.push(Fiber::spawn(
                state_ptr,
                function,
                task.script_id,
                task.actor_id,
                task.wait_group_id,
                mode,
            ));
        }

        // Main tick loop.
        loop {
            // Anchor this scheduler iteration to a stable tick start so both
            // time-slice checks and frame pacing use the same origin.
            self.current_tick_started_at = Some(Instant::now());
            let mut any_active = false;

            // --- step each active fiber one yield ---
            for fiber in fibers.iter_mut() {
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
                // When a GUI is attached (turbo + GUI), also reset the
                // budget so that forever loops keep running until the user
                // closes the window or triggers "stop all".
                //
                // Without frame pacing AND without a GUI (turbo CLI), the
                // global step budget acts as a termination bound – loops
                // consume budget at full speed and exit when budget is
                // exhausted.
                if self.frame_duration.is_some() || self.live_canvas.is_some() {
                    self.remaining_steps = self.step_budget;
                }
                self.warp_depth = fiber.warp_depth;

                // Install the fiber's control so yield-points use it.
                self.active_fiber_control = Some(Arc::clone(&fiber.control));
                self.paced_loop_guards_in_resume = 0;

                // Resume the fiber thread.
                fiber.control.resume();

                // Wait until it yields or finishes.
                let result = fiber.control.wait_for_yield_or_done();

                // Save actor state back – persist into whichever actor is
                // currently loaded (may differ from the original if the
                // fiber is mid-broadcast-and-wait handler execution).
                self.persist_runtime_into_actor(self.active_actor_id);
                fiber.current_actor_id = self.active_actor_id;
                fiber.warp_depth = self.warp_depth;
                self.active_fiber_control = None;
                if matches!(result, FiberSyncState::Done) {
                    self.complete_wait_group_task(fiber.wait_group_id);
                    fiber.wait_group_id = None;
                }
            }

            // --- spawn fibers for newly-queued scripts (broadcasts, clones) ---
            let mut new_tasks: Vec<ScriptTask> = Vec::new();
            while let Some(task) = self.dequeue_script() {
                new_tasks.push(task);
            }
            for task in new_tasks {
                let function = match self.script_functions.get(task.script_id as usize).copied() {
                    Some(f) => f,
                    None => {
                        self.complete_wait_group_task(task.wait_group_id);
                        continue;
                    }
                };
                let Some(actor) = self.actor_snapshot(task.actor_id) else {
                    self.complete_wait_group_task(task.wait_group_id);
                    continue;
                };
                if !actor.alive {
                    self.complete_wait_group_task(task.wait_group_id);
                    continue;
                }
                fibers.push(Fiber::spawn(
                    state_ptr,
                    function,
                    task.script_id,
                    task.actor_id,
                    task.wait_group_id,
                    mode,
                ));
                any_active = true;
            }

            // --- reap completed fibers to free OS thread resources ---
            // Without this, clone-heavy projects can accumulate thousands of
            // finished-but-unjoined threads and hit the OS thread/memory
            // limit (EAGAIN / "Resource temporarily unavailable").
            fibers.retain_mut(|fiber| {
                if fiber.is_done() {
                    fiber.join();
                    false
                } else {
                    true
                }
            });

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
            self.sync_live_canvas_if_due(true);
            self.enqueue_key_pressed_scripts();

            // --- frame pacing (applied once per tick) ---
            self.pace_frame();
        }

        // Clean up: in native-thread mode, signal remaining fibers so their
        // threads can exit, then join them.  In userspace mode, simply drop
        // the fiber structs (freeing the stacks).
        if mode == ConcurrencyMode::NativeThreads {
            for fiber in fibers.iter() {
                if !fiber.is_done() {
                    fiber.control.resume();
                }
            }
        }
        for fiber in &mut fibers {
            fiber.join();
        }

        self.flush_live_canvas();
    }

    fn clamp01(&mut self, v: f64) -> f64 {
        if v < 0.0 {
            0.0
        } else if v > 1.0 {
            1.0
        } else {
            v
        }
    }

    // self.pixels: Vec<u8> (RGBA), canvas_width/height: i32 を想定
    fn blend_pixel_coverage(
        &mut self,
        x: i32,
        y: i32,
        src: [u8; 3],
        src_alpha: f64,
        coverage: f64,
    ) {
        if x < 0 || y < 0 || x >= self.canvas_width as i32 || y >= self.canvas_height as i32 {
            return;
        }

        let cov = self.clamp01(coverage);
        if cov <= 0.0 {
            return;
        }

        let idx = ((y as usize) * (self.canvas_width) + (x as usize)) * 4;

        let sa = src_alpha * cov;
        if sa <= 0.0 {
            return;
        }

        let da = self.pen_rgba[idx + 3] as f64 / 255.0;

        let sr = src[0] as f64 / 255.0;
        let sg = src[1] as f64 / 255.0;
        let sb = src[2] as f64 / 255.0;

        let dr = self.pen_rgba[idx] as f64 / 255.0;
        let dg = self.pen_rgba[idx + 1] as f64 / 255.0;
        let db = self.pen_rgba[idx + 2] as f64 / 255.0;

        let out_a = sa + da * (1.0 - sa);
        if out_a <= 0.0 {
            return;
        }

        let out_r = (sr * sa + dr * da * (1.0 - sa)) / out_a;
        let out_g = (sg * sa + dg * da * (1.0 - sa)) / out_a;
        let out_b = (sb * sa + db * da * (1.0 - sa)) / out_a;

        self.pen_rgba[idx] = (out_r * 255.0).round().clamp(0.0, 255.0) as u8;
        self.pen_rgba[idx + 1] = (out_g * 255.0).round().clamp(0.0, 255.0) as u8;
        self.pen_rgba[idx + 2] = (out_b * 255.0).round().clamp(0.0, 255.0) as u8;
        self.pen_rgba[idx + 3] = (out_a * 255.0).round().clamp(0.0, 255.0) as u8;
    }

    fn draw_line_with_style_cpu(
        &mut self,
        x0: f64,
        y0: f64,
        x1: f64,
        y1: f64,
        style: PenStrokeStyle,
    ) {
        if !x0.is_finite() || !y0.is_finite() || !x1.is_finite() || !y1.is_finite() {
            return;
        }

        // 例: pen_brush_shapeから半径取得
        let (_extent, radius_sq) = self.pen_brush_shape_for_size(style.size as f64);
        let radius = radius_sq.sqrt();
        if radius <= 0.0 {
            let px = scratch_to_pixel_x(x0, self.canvas_width);
            let py = scratch_to_pixel_y(y0, self.canvas_height);
            self.blend_pixel_coverage(px, py, style.color, style.alpha as f64, 1.0);
            return;
        }

        let x0p = scratch_to_pixel_x(x0, self.canvas_width) as f64;
        let y0p = scratch_to_pixel_y(y0, self.canvas_height) as f64;
        let x1p = scratch_to_pixel_x(x1, self.canvas_width) as f64;
        let y1p = scratch_to_pixel_y(y1, self.canvas_height) as f64;

        let min_x = ((x0p.min(x1p) - radius - 1.0).floor() as i32).max(0);
        let max_x =
            ((x0p.max(x1p) + radius + 1.0).ceil() as i32).min((self.canvas_width - 1) as i32);
        let min_y = ((y0p.min(y1p) - radius - 1.0).floor() as i32).max(0);
        let max_y =
            ((y0p.max(y1p) + radius + 1.0).ceil() as i32).min((self.canvas_width - 1) as i32);

        let vx = x1p - x0p;
        let vy = y1p - y0p;
        let len2 = vx * vx + vy * vy;

        let inner = (radius - 0.5).max(0.0);
        let outer = radius + 0.5;
        let inner2 = inner * inner;
        let outer2 = outer * outer;

        for py in min_y..=max_y {
            let cy = py as f64 + 0.5;
            for px in min_x..=max_x {
                let cx = px as f64 + 0.5;

                let t = if len2 > 1e-12 {
                    (((cx - x0p) * vx + (cy - y0p) * vy) / len2).clamp(0.0, 1.0)
                } else {
                    0.0
                };

                let nx = x0p + t * vx;
                let ny = y0p + t * vy;
                let dx = cx - nx;
                let dy = cy - ny;
                let d2 = dx * dx + dy * dy;

                if d2 >= outer2 {
                    continue;
                }

                let coverage = if d2 <= inner2 {
                    1.0
                } else {
                    let d = d2.sqrt();
                    (radius + 0.5 - d).clamp(0.0, 1.0)
                };

                self.blend_pixel_coverage(px, py, style.color, style.alpha as f64, coverage);
            }
        }
    }

    fn draw_line(&mut self, x0: f64, y0: f64, x1: f64, y1: f64) {
        let style = self.current_pen_style();
        match self.pen_render_mode {
            PenRenderMode::CpuRealtime => self.draw_line_with_style_cpu(x0, y0, x1, y1, style),
            PenRenderMode::GpuBatch => self.push_pen_batch_command(PenBatchCommand::Line {
                x0: x0 as f32,
                y0: y0 as f32,
                x1: x1 as f32,
                y1: y1 as f32,
                style,
            }),
        }
    }

    fn draw_disc_with_brush_cpu(
        &mut self,
        x: f64,
        y: f64,
        extent: i32,
        radius_sq: f64,
        style: PenStrokeStyle,
    ) {
        if !x.is_finite() || !y.is_finite() {
            return;
        }
        let extent = extent as f64;
        let center_x = scratch_to_pixel_x(x, self.canvas_width) as f64;
        let center_y = scratch_to_pixel_y(y, self.canvas_height) as f64;
        let radius = radius_sq.sqrt();
        let min_x = ((center_x - radius - 1.0).floor() as i32).max(0);
        let max_x = ((center_x + radius + 1.0).ceil() as i32).min((self.canvas_width - 1) as i32);
        let min_y = ((center_y - radius - 1.0).floor() as i32).max(0);
        let max_y = ((center_y + radius + 1.0).ceil() as i32).min((self.canvas_height - 1) as i32);

        for py in min_y..=max_y {
            let cy = py as f64 + 0.5;
            for px in min_x..=max_x {
                let cx = px as f64 + 0.5;
                let dx = cx - center_x;
                let dy = cy - center_y;
                let d2 = dx * dx + dy * dy;

                if d2 >= radius_sq {
                    continue;
                }

                let coverage = if extent > 1e-6 {
                    (extent - (d2 / radius_sq)).clamp(0.0, 1.0)
                } else {
                    1.0
                };

                self.blend_pixel_coverage(px, py, style.color, style.alpha as f64, coverage);
            }
        }
    }

    fn draw_disc_with_style_cpu(&mut self, x: f64, y: f64, style: PenStrokeStyle) {
        let (extent, radius_sq) = self.pen_brush_shape_for_size(style.size as f64);
        self.draw_disc_with_brush_cpu(x, y, extent, radius_sq, style);
    }

    fn draw_disc(&mut self, x: f64, y: f64) {
        let style = self.current_pen_style();
        match self.pen_render_mode {
            PenRenderMode::CpuRealtime => self.draw_disc_with_style_cpu(x, y, style),
            PenRenderMode::GpuBatch => {
                self.push_pen_batch_command(PenBatchCommand::Disc {
                    x: x as f32,
                    y: y as f32,
                    style,
                });
            }
        }
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

    fn compose_canvas_rgb_without_pen(&mut self) {
        self.canvas_rgb.fill(255);
        self.compose_backdrop();
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
            let alpha = self.pen_rgba[src_offset + 3] as u32;
            if alpha == 0 {
                continue;
            }
            if alpha >= 255 {
                self.canvas_rgb[dst_offset] = self.pen_rgba[src_offset];
                self.canvas_rgb[dst_offset + 1] = self.pen_rgba[src_offset + 1];
                self.canvas_rgb[dst_offset + 2] = self.pen_rgba[src_offset + 2];
                continue;
            }
            let inv_alpha = 255u32 - alpha;
            self.canvas_rgb[dst_offset] = Self::div_round_u32(
                (self.pen_rgba[src_offset] as u32) * alpha
                    + (self.canvas_rgb[dst_offset] as u32) * inv_alpha,
                255,
            )
            .min(255) as u8;
            self.canvas_rgb[dst_offset + 1] = Self::div_round_u32(
                (self.pen_rgba[src_offset + 1] as u32) * alpha
                    + (self.canvas_rgb[dst_offset + 1] as u32) * inv_alpha,
                255,
            )
            .min(255) as u8;
            self.canvas_rgb[dst_offset + 2] = Self::div_round_u32(
                (self.pen_rgba[src_offset + 2] as u32) * alpha
                    + (self.canvas_rgb[dst_offset + 2] as u32) * inv_alpha,
                255,
            )
            .min(255) as u8;
        }
    }

    fn compose_sprites(&mut self) {
        self.compose_sprites_internal(None);
    }

    fn compose_sprites_internal(&mut self, excluded_actor: Option<u64>) {
        let mut draw_order = self
            .actors
            .iter()
            .enumerate()
            .filter_map(|(actor_id, actor)| {
                if excluded_actor == Some(actor_id as u64) {
                    return None;
                }
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
        let index = self.resolve_costume_index_for_target(target_index, costume_number)?;
        self.target_render_data
            .get(target_index)?
            .costumes
            .get(index)
    }

    fn resolve_costume_index_for_target(
        &self,
        target_index: usize,
        costume_number: f64,
    ) -> Option<usize> {
        let target = self.target_render_data.get(target_index)?;
        if target.costumes.is_empty() {
            return None;
        }
        let count = target.costumes.len() as i64;
        let raw = (costume_number.floor() as i64).saturating_sub(1);
        Some(raw.rem_euclid(count) as usize)
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
                let src_alpha = costume.pixels_rgba[src_offset + 3];
                if src_alpha == 0 {
                    continue;
                }
                let dst_offset = ((py as usize) * self.canvas_width + (px as usize)) * 4;
                Self::blend_unpremultiplied_rgba_pixel(
                    &mut self.pen_rgba,
                    dst_offset,
                    costume.pixels_rgba[src_offset],
                    costume.pixels_rgba[src_offset + 1],
                    costume.pixels_rgba[src_offset + 2],
                    src_alpha,
                );
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

        let resolved_index = if is_string_tagged(index) {
            if let Some(string_index) = decode_string_id(index) {
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
        } else {
            None
        }
        .or_else(|| {
            let numeric_index =
                rt_repeat_count(self as *mut RuntimeState, self.value_to_number(index)) as usize;
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

        let resolved_index = if is_string_tagged(index) {
            if let Some(string_index) = decode_string_id(index) {
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
        } else {
            None
        }
        .or_else(|| {
            let numeric_index =
                rt_repeat_count(self as *mut RuntimeState, self.value_to_number(index)) as usize;
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

        let item_index =
            rt_repeat_count(self as *mut RuntimeState, self.value_to_number(index)) as usize;
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
        while let Some(task) = self.script_queue.pop_front() {
            self.complete_wait_group_task(task.wait_group_id);
        }
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
        let mut removed_wait_groups = Vec::new();
        self.script_queue.retain(|task| {
            let keep = actor_targets
                .get(task.actor_id as usize)
                .is_none_or(|actor_target| *actor_target != target_index);
            if !keep {
                removed_wait_groups.push(task.wait_group_id);
            }
            keep
        });
        for wait_group_id in removed_wait_groups {
            self.complete_wait_group_task(wait_group_id);
        }
    }

    fn switch_costume_to(&mut self, costume: f64) {
        let target_index = self.active_target_index() as usize;
        self.switch_costume_for_target(target_index, costume);
    }

    fn switch_backdrop_to(&mut self, backdrop: f64) {
        let target_index = self.stage_target_index();
        self.switch_costume_for_target(target_index, backdrop);
    }

    fn switch_costume_for_target(&mut self, target_index: usize, costume: f64) {
        if let Some(string_index) = decode_string_id(costume) {
            let raw = self.strings.get(string_index).cloned().unwrap_or_default();
            let selector = raw.trim().to_ascii_lowercase();
            if selector == "next costume" || selector == "next backdrop" {
                let current = self
                    .base_actor_by_target
                    .get(target_index)
                    .and_then(|actor_id| self.actor_snapshot(*actor_id))
                    .map(|actor| actor.costume_number.floor().max(1.0))
                    .unwrap_or(1.0);
                self.set_target_costume_by_zero_based_index(target_index, current);
                return;
            }
            if selector == "previous costume" || selector == "previous backdrop" {
                let current = self
                    .base_actor_by_target
                    .get(target_index)
                    .and_then(|actor_id| self.actor_snapshot(*actor_id))
                    .map(|actor| actor.costume_number.floor().max(1.0))
                    .unwrap_or(1.0);
                self.set_target_costume_by_zero_based_index(target_index, current - 2.0);
                return;
            }
            if let Some(index) = self.costume_index_by_name_for_target(target_index, &raw) {
                self.set_target_costume_by_zero_based_index(target_index, index as f64);
                return;
            }
            let trimmed = raw.trim();
            if trimmed.is_empty() {
                return;
            }
            if let Ok(number) = trimmed.parse::<f64>() {
                self.set_target_costume_by_zero_based_index(target_index, number - 1.0);
            }
            return;
        }

        self.set_target_costume_by_zero_based_index(
            target_index,
            self.value_to_number(costume) - 1.0,
        );
    }

    fn stage_target_index(&self) -> usize {
        self.target_render_data
            .iter()
            .position(|target| target.is_stage)
            .unwrap_or(0)
    }

    fn costume_number_value(&self) -> f64 {
        self.costume_number.floor().max(1.0)
    }

    fn costume_name_value(&mut self) -> f64 {
        let name = self
            .costume_name_for_target(self.active_target_index() as usize, self.costume_number)
            .map(ToOwned::to_owned)
            .unwrap_or_default();
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

    fn costume_index_by_name_for_target(&self, target_index: usize, name: &str) -> Option<usize> {
        self.target_render_data
            .get(target_index)?
            .costumes
            .iter()
            .position(|costume| costume.name == name)
    }

    fn set_target_costume_by_zero_based_index(&mut self, target_index: usize, index: f64) {
        let Some(target) = self.target_render_data.get(target_index) else {
            return;
        };
        let costume_count = target.costumes.len();
        if costume_count == 0 {
            return;
        }

        let mut rounded = index.round();
        if !rounded.is_finite() || rounded == 0.0 {
            rounded = 0.0;
        }
        let wrapped = (rounded as i64).rem_euclid(costume_count as i64) as usize;
        let costume_number = wrapped as f64 + 1.0;
        if let Some(actor_id) = self.base_actor_by_target.get(target_index).copied() {
            if let Some(actor) = self.actors.get_mut(actor_id as usize) {
                actor.costume_number = costume_number;
            }
        }
        if self.active_target_index() as usize == target_index {
            self.costume_number = costume_number;
        }
        self.live_canvas_dirty = true;
    }

    fn set_effect_to(&mut self, effect: f64, value: f64) {
        let effect_name = self.value_as_string(effect).trim().to_ascii_lowercase();
        let numeric = self.value_to_number(value);
        if !numeric.is_finite() {
            return;
        }
        match effect_name.as_str() {
            "color" | "fisheye" | "whirl" | "pixelate" | "mosaic" | "brightness" | "ghost" => {
                self.live_canvas_dirty = true;
            }
            _ => {}
        }
    }

    fn parse_color_value(&self, color: f64) -> [u8; 3] {
        if let Some(index) = decode_string_id(color) {
            let raw = self.strings.get(index).cloned().unwrap_or_default();
            if let Some(rgb) = parse_hex_color(&raw) {
                return rgb;
            }
            let trimmed = raw.trim();
            let number = if let Some(hex) = trimmed
                .strip_prefix("0x")
                .or_else(|| trimmed.strip_prefix("0X"))
            {
                u64::from_str_radix(hex, 16)
                    .ok()
                    .map(|v| v as f64)
                    .unwrap_or(0.0)
            } else {
                trimmed.parse::<f64>().unwrap_or(0.0)
            };
            return decimal_to_rgb(number);
        }
        decimal_to_rgb(self.value_to_number(color))
    }

    fn touching_color(&mut self, color: [u8; 3]) -> bool {
        let Some(actor) = self.actor_snapshot(self.active_actor_id) else {
            return false;
        };
        if !actor.alive || !actor.visible || actor.size_percent <= 0.0 {
            return false;
        }
        let Some(costume) = self
            .resolve_costume_for_target(actor.target_index as usize, actor.costume_number)
            .cloned()
        else {
            return false;
        };
        let Some(transform) = CostumeTransform::new(
            &costume,
            actor.sprite_x,
            actor.sprite_y,
            actor.direction_deg,
            actor.size_percent,
        ) else {
            return false;
        };

        let saved_canvas = self.canvas_rgb.clone();
        self.canvas_rgb.fill(255);
        self.compose_backdrop();
        self.blend_pen_layer_into_canvas();
        self.compose_sprites_internal(Some(self.active_actor_id));

        let (min_x, max_x, min_y, max_y) =
            costume_pixel_bounds(&costume, self.canvas_width, self.canvas_height, &transform);
        let mut hit = false;
        'scan: for py in min_y..=max_y {
            for px in min_x..=max_x {
                let world_x = pixel_to_scratch_x(px as usize, self.canvas_width);
                let world_y = pixel_to_scratch_y(py as usize, self.canvas_height);
                let Some((src_x, src_y)) =
                    sample_costume_coordinates(&costume, world_x, world_y, &transform)
                else {
                    continue;
                };
                let src_offset = (src_y * costume.width + src_x) * 4;
                if costume.pixels_rgba[src_offset + 3] == 0 {
                    continue;
                }
                let dst_offset = ((py as usize) * self.canvas_width + (px as usize)) * 3;
                let pixel = [
                    self.canvas_rgb[dst_offset],
                    self.canvas_rgb[dst_offset + 1],
                    self.canvas_rgb[dst_offset + 2],
                ];
                if pixel == color {
                    hit = true;
                    break 'scan;
                }
            }
        }
        self.canvas_rgb = saved_canvas;
        hit
    }

    fn costume_name_for_target(&self, target_index: usize, costume_number: f64) -> Option<&str> {
        let index = self.resolve_costume_index_for_target(target_index, costume_number)?;
        self.target_render_data
            .get(target_index)?
            .costumes
            .get(index)
            .map(|costume| costume.name.as_str())
    }

    fn lookup_variable_value_for_target(&self, target_index: u64, name: &str) -> Option<f64> {
        for (index, variable_name) in self.variable_names.iter().enumerate() {
            let owner = self.variable_target_indices.get(index).copied();
            if owner == Some(target_index) && variable_name == name {
                return self.variables.get(index).copied();
            }
        }
        None
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
        let property_text = self.value_as_string(property);
        let is_stage = self
            .target_render_data
            .get(target_index as usize)
            .map(|target| target.is_stage)
            .unwrap_or(false);

        if is_stage {
            match property_text.as_str() {
                "background #" | "backdrop #" | "backdrop number" => actor
                    .map(|state| state.costume_number.floor().max(1.0))
                    .unwrap_or(1.0),
                "backdrop name" => actor
                    .map(|state| {
                        let name = self
                            .costume_name_for_target(target_index as usize, state.costume_number)
                            .map(ToOwned::to_owned)
                            .unwrap_or_default();
                        let id = self.intern_string(&name);
                        encode_string_id(id)
                    })
                    .unwrap_or_else(|| encode_string_id(EMPTY_STRING_ID)),
                "volume" => 0.0,
                _ => self
                    .lookup_variable_value_for_target(target_index, &property_text)
                    .unwrap_or(0.0),
            }
        } else {
            match property_text.as_str() {
                "x position" => actor.map(|state| state.sprite_x).unwrap_or(0.0),
                "y position" => actor.map(|state| state.sprite_y).unwrap_or(0.0),
                "direction" => actor.map(|state| state.direction_deg).unwrap_or(90.0),
                "size" => actor.map(|state| state.size_percent).unwrap_or(100.0),
                "costume #" | "costume number" => actor
                    .map(|state| state.costume_number.floor().max(1.0))
                    .unwrap_or(1.0),
                "costume name" => actor
                    .map(|state| {
                        let name = self
                            .costume_name_for_target(target_index as usize, state.costume_number)
                            .map(ToOwned::to_owned)
                            .unwrap_or_default();
                        let id = self.intern_string(&name);
                        encode_string_id(id)
                    })
                    .unwrap_or_else(|| encode_string_id(EMPTY_STRING_ID)),
                "volume" => 0.0,
                _ => self
                    .lookup_variable_value_for_target(target_index, &property_text)
                    .unwrap_or(0.0),
            }
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

pub(super) fn next_random_unit(state: &mut RuntimeState) -> f64 {
    // Numerical Recipes LCG (deterministic and cheap for runtime integration).
    state.rng_state = state
        .rng_state
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1);
    let mantissa = state.rng_state >> 11;
    mantissa as f64 / ((1_u64 << 53) as f64)
}

pub(super) fn parse_hex_color_with_alpha(raw: &str) -> Option<([u8; 3], Option<f64>)> {
    let hex = raw
        .trim()
        .trim_start_matches('#')
        .trim_start_matches("0x")
        .trim_start_matches("0X");

    match hex.len() {
        6 => {
            // Standard 6-digit hex color: RRGGBB
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            Some(([r, g, b], None))
        }
        8 => {
            // 8-digit hex color with alpha: AARRGGBB
            let a = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let r = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let g = u8::from_str_radix(&hex[4..6], 16).ok()?;
            let b = u8::from_str_radix(&hex[6..8], 16).ok()?;
            Some(([r, g, b], Some((a as f64) / 255.0)))
        }
        _ => None,
    }
}

pub(super) fn parse_hex_color(raw: &str) -> Option<[u8; 3]> {
    parse_hex_color_with_alpha(raw).map(|(rgb, _)| rgb)
}

pub(super) fn hue_to_rgb(color: f64) -> [u8; 3] {
    let hue = color.rem_euclid(200.0) * 360.0 / 200.0;
    hsv_to_rgb(hue, 1.0, 1.0)
}

/// Convert a decimal colour value to RGB, matching scratch-vm's
/// `Color.decimalToRgb`.  The integer is interpreted as 0xRRGGBB.
pub(super) fn decimal_to_rgb(decimal: f64) -> [u8; 3] {
    let decimal = decimal as i64;
    let r = ((decimal >> 16) & 0xFF) as u8;
    let g = ((decimal >> 8) & 0xFF) as u8;
    let b = (decimal & 0xFF) as u8;
    [r, g, b]
}

pub(super) fn rgb_to_hsv(rgb: [u8; 3]) -> (f64, f64, f64) {
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

pub(super) fn hsv_to_rgb(hue: f64, saturation: f64, value: f64) -> [u8; 3] {
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

pub(super) fn apply_mathop(op_code: u64, value: f64) -> f64 {
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

pub(super) fn js_round(value: f64) -> f64 {
    if !value.is_finite() {
        return value;
    }
    (value + 0.5).floor()
}

#[cfg(test)]
mod tests {
    use super::*;

    unsafe extern "C" fn script_capture_and_mutate_warp(state: *mut RuntimeState) {
        unsafe {
            let Some(state_ref) = state.as_mut() else {
                return;
            };
            if let Some(slot) = state_ref.variables.get_mut(0) {
                *slot = state_ref.warp_depth as f64;
            }
            state_ref.warp_depth = 3;
        }
    }

    unsafe extern "C" fn script_yield_in_warp(state: *mut RuntimeState) {
        rt_warp_enter(state);
        rt_control_wait(state, 0.0);
        rt_warp_leave(state);
    }

    unsafe extern "C" fn script_record_warp_depth(state: *mut RuntimeState) {
        unsafe {
            let Some(state_ref) = state.as_mut() else {
                return;
            };
            if let Some(slot) = state_ref.variables.get_mut(0) {
                *slot = state_ref.warp_depth as f64;
            }
        }
    }

    fn runtime_with_scripts(script_functions: Vec<ScriptEntry>) -> RuntimeState {
        let mut state = RuntimeState::new(
            vec![0.0],
            vec!["v".to_string()],
            vec![0],
            Vec::new(),
            Vec::new(),
            100,
        );
        let script_names = (0..script_functions.len())
            .map(|index| format!("script{index}"))
            .collect::<Vec<_>>();
        let script_target_by_id = vec![0; script_functions.len()];
        state.install_scheduler(
            script_functions,
            script_names,
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            script_target_by_id,
            vec!["Sprite1".to_string()],
            1,
        );
        state
    }

    #[test]
    fn run_script_resets_and_restores_warp_depth() {
        let mut state = runtime_with_scripts(vec![script_capture_and_mutate_warp]);
        state.warp_depth = 7;

        state.run_script(0, 0);

        assert_eq!(state.variables[0], 0.0);
        assert_eq!(state.warp_depth, 7);
    }

    #[test]
    fn concurrent_fibers_do_not_share_warp_depth() {
        let mut state = runtime_with_scripts(vec![script_yield_in_warp, script_record_warp_depth]);
        state.enqueue_scripts(&[0, 1]);

        state.execute_concurrent();

        assert_eq!(state.variables[0], 0.0);
    }

    #[test]
    fn warp_uses_longer_yield_time_slice_than_normal_mode() {
        let mut state = runtime_with_scripts(Vec::new());
        state.set_target_fps(Some(30.0));

        state.current_tick_started_at = Some(Instant::now() - Duration::from_millis(100));
        assert!(state.should_yield_for_work_time());
        assert!(!state.should_yield_for_warp_time());

        state.current_tick_started_at = Some(Instant::now() - Duration::from_millis(600));
        assert!(state.should_yield_for_warp_time());
    }

    #[test]
    fn parse_argb_hex_color_exposes_alpha() {
        let parsed = parse_hex_color_with_alpha("#40ffffff").expect("valid ARGB hex");
        assert_eq!(parsed.0, [255, 255, 255]);
        let alpha = parsed.1.expect("alpha should be present");
        assert!((alpha - (0x40 as f64 / 255.0)).abs() < 1e-9);
    }

    #[test]
    fn parse_rgb_hex_color_has_no_alpha() {
        let parsed = parse_hex_color_with_alpha("#112233").expect("valid RGB hex");
        assert_eq!(parsed.0, [0x11, 0x22, 0x33]);
        assert!(parsed.1.is_none());
    }

    #[test]
    fn control_wait_short_duration_is_not_clamped_to_full_frame() {
        let mut state = runtime_with_scripts(Vec::new());
        state.set_target_fps(Some(1.0));

        let started_at = Instant::now();
        rt_control_wait(&mut state as *mut RuntimeState, 0.03);
        let elapsed = started_at.elapsed();

        assert!(
            elapsed < Duration::from_millis(600),
            "control_wait(0.03) should not consume a full 1s frame; elapsed={elapsed:?}"
        );
    }

    #[test]
    fn pen_set_color_argb_updates_pen_alpha() {
        let mut state = runtime_with_scripts(Vec::new());
        state.pen_alpha = 1.0;
        let color_id = state.intern_string("#40ffffff");

        rt_pen_set_color(&mut state as *mut RuntimeState, encode_string_id(color_id));

        assert_eq!(state.pen_color, [255, 255, 255]);
        assert!((state.pen_alpha - (0x40 as f64 / 255.0)).abs() < 1e-9);
    }

    #[test]
    fn pen_set_color_rgb_keeps_existing_pen_alpha() {
        let mut state = runtime_with_scripts(Vec::new());
        state.pen_alpha = 0.25;
        let color_id = state.intern_string("#abcdef");

        rt_pen_set_color(&mut state as *mut RuntimeState, encode_string_id(color_id));

        assert_eq!(state.pen_color, [0xab, 0xcd, 0xef]);
        assert!((state.pen_alpha - 0.25).abs() < 1e-9);
    }
}
