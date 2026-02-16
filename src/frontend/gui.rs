use crate::constants::{STAGE_HEIGHT, STAGE_WIDTH};
use crate::engine::{ir, jit, runtime};

use anyhow::{Result, anyhow};
use eframe::egui;
use std::collections::HashSet;
use std::env;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

const FIXED_WINDOW_SCALE: usize = 2;
const DEFAULT_GUI_CANVAS_SYNC_FPS: f64 = 60.0;

pub fn execute_program_with_gui(
    program: ir::Program,
    mut runtime_state: runtime::RuntimeState,
    stage_scale: usize,
    target_fps: Option<f64>,
    pen_render_mode: runtime::PenRenderMode,
    llvm_opt_level: jit::JitOptimizationLevel,
    vsync_enabled: bool,
    vsync_fps: usize,
) -> Result<runtime::RuntimeState> {
    // Compile and load the program in the main thread before GUI initialization
    let compiled_program =
        jit::compile_and_load_program_with_optimization(&program, llvm_opt_level)?;

    runtime_state.set_canvas_scale(stage_scale);
    runtime_state.set_target_fps(target_fps);
    let live_canvas_sync_fps = if vsync_enabled {
        Some(vsync_fps as f64)
    } else {
        Some(DEFAULT_GUI_CANVAS_SYNC_FPS)
    };
    runtime_state.set_live_canvas_sync_fps(live_canvas_sync_fps);

    let (canvas_width, canvas_height) = runtime_state.canvas_dimensions();

    let live_scene_canvas = Arc::new(Mutex::new(runtime_state.canvas_rgb_copy()));
    runtime_state.attach_live_canvas(Arc::clone(&live_scene_canvas));
    let live_pen_layer = Arc::new(Mutex::new(runtime_state.pen_rgba_copy()));
    runtime_state.attach_live_pen_layer(Arc::clone(&live_pen_layer));
    let live_pen_batch = Arc::new(Mutex::new(runtime_state.pen_batch_copy()));
    runtime_state.attach_live_pen_batch(Arc::clone(&live_pen_batch));
    let live_canvas_generation = Arc::new(AtomicU64::new(0));
    runtime_state.attach_live_canvas_generation(Arc::clone(&live_canvas_generation));
    let input_state = Arc::new(Mutex::new(runtime::InputState::default()));
    runtime_state.attach_input_state(Arc::clone(&input_state));
    let ask_prompt_state = Arc::new(runtime::AskPromptState::default());
    runtime_state.attach_ask_prompt_state(Arc::clone(&ask_prompt_state));
    let stop_requested = Arc::new(AtomicBool::new(false));
    runtime_state.attach_stop_flag(Arc::clone(&stop_requested));
    let dump_vars_requested = Arc::new(AtomicBool::new(false));
    runtime_state.attach_dump_vars_flag(Arc::clone(&dump_vars_requested));
    let worker_done = Arc::new(AtomicBool::new(false));
    let worker_handle_cell = Arc::new(Mutex::new(None));

    let window_width = (STAGE_WIDTH * FIXED_WINDOW_SCALE) as f32;
    let window_height = (STAGE_HEIGHT * FIXED_WINDOW_SCALE) as f32;
    let final_frame_hold_ms = env::var("SCRATCH_FINAL_FRAME_HOLD_MS")
        .ok()
        .and_then(|raw| raw.parse::<u64>().ok())
        .unwrap_or(0);

    let app = EguiRuntimeApp::new(
        Arc::clone(&live_scene_canvas),
        Arc::clone(&live_pen_layer),
        Arc::clone(&live_pen_batch),
        Arc::clone(&live_canvas_generation),
        Arc::clone(&input_state),
        Arc::clone(&ask_prompt_state),
        Arc::clone(&stop_requested),
        Arc::clone(&dump_vars_requested),
        Arc::clone(&worker_done),
        canvas_width,
        canvas_height,
        target_fps,
        vsync_enabled,
        vsync_fps,
        pen_render_mode,
        final_frame_hold_ms,
        Some((compiled_program, runtime_state)),
        Arc::clone(&worker_handle_cell),
    );
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("scratch-native-runtime")
            .with_inner_size([window_width, window_height])
            .with_min_inner_size([window_width, window_height])
            .with_resizable(false),
        renderer: eframe::Renderer::Glow,
        vsync: vsync_enabled,
        ..Default::default()
    };

    let run_result = eframe::run_native(
        "scratch-native-runtime",
        native_options,
        Box::new(move |cc| {
            if cc.gl.is_none() {
                return Err(anyhow!(
                    "OpenGL context is unavailable (expected eframe::Renderer::Glow)"
                )
                .into());
            }
            Ok(Box::new(app))
        }),
    );
    if let Err(error) = run_result {
        eprintln!("warning: failed to open egui window: {error}");
        eprintln!("warning: GUI is unavailable; continuing in headless mode");
    }

    let worker_handle = Arc::try_unwrap(worker_handle_cell)
        .ok()
        .and_then(|mutex| mutex.into_inner().ok())
        .flatten();

    if let Some(handle) = worker_handle {
        let worker_result = handle
            .join()
            .map_err(|_| anyhow!("GUI execution worker thread panicked"))?;
        worker_result
    } else {
        Err(anyhow!("worker thread was not started"))
    }
}

struct EguiRuntimeApp {
    live_scene_canvas: Arc<Mutex<Vec<u8>>>,
    live_pen_layer: Arc<Mutex<Vec<u8>>>,
    live_pen_batch: Arc<Mutex<Vec<runtime::PenBatchCommand>>>,
    live_canvas_generation: Arc<AtomicU64>,
    last_displayed_generation: u64,
    input_state: Arc<Mutex<runtime::InputState>>,
    ask_prompt_state: Arc<runtime::AskPromptState>,
    stop_requested: Arc<AtomicBool>,
    dump_vars_requested: Arc<AtomicBool>,
    worker_done: Arc<AtomicBool>,
    canvas_width: usize,
    canvas_height: usize,
    simulation_fps: Option<f64>,
    vsync_enabled: bool,
    vsync_fps: usize,
    scene_frame_snapshot: Vec<u8>,
    pen_frame_snapshot: Vec<u8>,
    scene_texture: Option<egui::TextureHandle>,
    pen_texture: Option<egui::TextureHandle>,
    pen_render_mode: runtime::PenRenderMode,
    pen_batch_cursor: usize,
    gpu_pen_draw_commands: Vec<runtime::PenBatchCommand>,
    stage_rect: Option<egui::Rect>,
    presented_frames: u64,
    fps_window_start: Instant,
    close_after: Option<Instant>,
    final_frame_hold_ms: u64,
    ask_answer_buffer: String,
    active_prompt_serial: Option<u64>,
    focus_ask_input: bool,
    stop_on_drop: bool,
    start_payload: Option<(jit::CompiledProgram, runtime::RuntimeState)>,
    worker_handle_cell: Arc<Mutex<Option<thread::JoinHandle<Result<runtime::RuntimeState>>>>>,
    worker_started: bool,
}

impl EguiRuntimeApp {
    fn new(
        live_scene_canvas: Arc<Mutex<Vec<u8>>>,
        live_pen_layer: Arc<Mutex<Vec<u8>>>,
        live_pen_batch: Arc<Mutex<Vec<runtime::PenBatchCommand>>>,
        live_canvas_generation: Arc<AtomicU64>,
        input_state: Arc<Mutex<runtime::InputState>>,
        ask_prompt_state: Arc<runtime::AskPromptState>,
        stop_requested: Arc<AtomicBool>,
        dump_vars_requested: Arc<AtomicBool>,
        worker_done: Arc<AtomicBool>,
        canvas_width: usize,
        canvas_height: usize,
        simulation_fps: Option<f64>,
        vsync_enabled: bool,
        vsync_fps: usize,
        pen_render_mode: runtime::PenRenderMode,
        final_frame_hold_ms: u64,
        start_payload: Option<(jit::CompiledProgram, runtime::RuntimeState)>,
        worker_handle_cell: Arc<Mutex<Option<thread::JoinHandle<Result<runtime::RuntimeState>>>>>,
    ) -> Self {
        Self {
            live_scene_canvas,
            live_pen_layer,
            live_pen_batch,
            live_canvas_generation,
            last_displayed_generation: 0,
            input_state,
            ask_prompt_state,
            stop_requested,
            dump_vars_requested,
            worker_done,
            canvas_width,
            canvas_height,
            simulation_fps,
            vsync_enabled,
            vsync_fps: vsync_fps.max(1),
            scene_frame_snapshot: vec![255; canvas_width * canvas_height * 3],
            pen_frame_snapshot: vec![0; canvas_width * canvas_height * 4],
            scene_texture: None,
            pen_texture: None,
            pen_render_mode,
            pen_batch_cursor: 0,
            gpu_pen_draw_commands: Vec::new(),
            stage_rect: None,
            presented_frames: 0,
            fps_window_start: Instant::now(),
            close_after: None,
            final_frame_hold_ms,
            ask_answer_buffer: String::new(),
            active_prompt_serial: None,
            focus_ask_input: false,
            stop_on_drop: false,
            start_payload,
            worker_handle_cell,
            worker_started: false,
        }
    }

    fn start_worker_if_needed(&mut self) {
        if self.worker_started {
            return;
        }
        self.worker_started = true;

        if let Some((compiled_program, runtime_state)) = self.start_payload.take() {
            let worker_done = Arc::clone(&self.worker_done);
            let stop_requested = Arc::clone(&self.stop_requested);

            let handle = thread::spawn(move || -> Result<runtime::RuntimeState> {
                let mut worker_state = runtime_state;
                let result = (|| -> Result<runtime::RuntimeState> {
                    compiled_program.execute(&mut worker_state);
                    worker_state.flush_live_canvas();
                    Ok(worker_state)
                })();
                worker_done.store(true, Ordering::Relaxed);
                if result.is_err() {
                    stop_requested.store(true, Ordering::Relaxed);
                }
                result
            });

            if let Ok(mut cell) = self.worker_handle_cell.lock() {
                *cell = Some(handle);
            }
        }
    }

    fn render_interval(&self) -> Duration {
        if self.vsync_enabled {
            Duration::from_secs_f64(1.0 / self.vsync_fps as f64)
        } else {
            Duration::from_millis(1)
        }
    }

    fn poll_input(&self, ctx: &egui::Context) -> runtime::InputState {
        let mut keys_down = HashSet::new();
        let mut mouse_down = false;
        let mut mouse_position = None;

        ctx.input(|input| {
            mouse_down = input.pointer.primary_down();
            mouse_position = input.pointer.hover_pos();
            for key in &input.keys_down {
                if let Some(name) = key_to_scratch_name(key) {
                    keys_down.insert(name);
                }
            }
        });

        let (mouse_x, mouse_y) = match (mouse_position, self.stage_rect) {
            (Some(position), Some(stage_rect))
                if stage_rect.width() > 0.0 && stage_rect.height() > 0.0 =>
            {
                let x = ((position.x - stage_rect.min.x) / stage_rect.width()).clamp(0.0, 1.0)
                    * STAGE_WIDTH as f32;
                let y = ((position.y - stage_rect.min.y) / stage_rect.height()).clamp(0.0, 1.0)
                    * STAGE_HEIGHT as f32;
                (
                    x as f64 - (STAGE_WIDTH as f64 / 2.0),
                    (STAGE_HEIGHT as f64 / 2.0) - y as f64,
                )
            }
            _ => (0.0, 0.0),
        };

        runtime::InputState {
            mouse_x,
            mouse_y,
            mouse_down,
            keys_down,
        }
    }

    fn update_texture(&mut self, ctx: &egui::Context) -> bool {
        let current_generation = self.live_canvas_generation.load(Ordering::Acquire);
        if current_generation == self.last_displayed_generation
            && self.scene_texture.is_some()
            && self.pen_texture.is_some()
        {
            return false;
        }

        let Ok(scene_frame) = self.live_scene_canvas.lock() else {
            return false;
        };
        if self.scene_frame_snapshot.len() != scene_frame.len() {
            self.scene_frame_snapshot.resize(scene_frame.len(), 255);
        }
        self.scene_frame_snapshot
            .copy_from_slice(scene_frame.as_slice());
        drop(scene_frame);

        let Ok(pen_frame) = self.live_pen_layer.lock() else {
            return false;
        };
        if self.pen_frame_snapshot.len() != pen_frame.len() {
            self.pen_frame_snapshot.resize(pen_frame.len(), 0);
        }
        self.pen_frame_snapshot
            .copy_from_slice(pen_frame.as_slice());
        drop(pen_frame);

        self.sync_pen_batch_from_runtime();

        self.last_displayed_generation = current_generation;

        let scene_image = egui::ColorImage::from_rgb(
            [self.canvas_width, self.canvas_height],
            &self.scene_frame_snapshot,
        );
        let pen_image = egui::ColorImage::from_rgba_unmultiplied(
            [self.canvas_width, self.canvas_height],
            &self.pen_frame_snapshot,
        );

        if let Some(texture) = self.scene_texture.as_mut() {
            texture.set(scene_image, egui::TextureOptions::NEAREST);
        } else {
            self.scene_texture = Some(ctx.load_texture(
                "scratch-stage-scene-canvas",
                scene_image,
                egui::TextureOptions::NEAREST,
            ));
        }

        if let Some(texture) = self.pen_texture.as_mut() {
            texture.set(pen_image, egui::TextureOptions::NEAREST);
        } else {
            self.pen_texture = Some(ctx.load_texture(
                "scratch-stage-pen-layer",
                pen_image,
                egui::TextureOptions::NEAREST,
            ));
        }
        true
    }

    fn sync_pen_batch_from_runtime(&mut self) {
        if self.pen_render_mode != runtime::PenRenderMode::GpuBatch {
            return;
        }
        let Ok(batch) = self.live_pen_batch.lock() else {
            return;
        };
        if self.pen_batch_cursor > batch.len() {
            self.pen_batch_cursor = 0;
            self.gpu_pen_draw_commands.clear();
        }
        if self.pen_batch_cursor >= batch.len() {
            return;
        }
        for command in &batch[self.pen_batch_cursor..] {
            match command {
                runtime::PenBatchCommand::Clear => {
                    self.gpu_pen_draw_commands.clear();
                }
                other => {
                    self.gpu_pen_draw_commands.push(*other);
                }
            }
        }
        self.pen_batch_cursor = batch.len();
    }

    fn paint_gpu_pen_batch(&self, ui: &mut egui::Ui, rect: egui::Rect) {
        if self.pen_render_mode != runtime::PenRenderMode::GpuBatch {
            return;
        }
        let painter = ui.painter();
        for command in &self.gpu_pen_draw_commands {
            match *command {
                runtime::PenBatchCommand::Clear => {}
                runtime::PenBatchCommand::Line {
                    x0,
                    y0,
                    x1,
                    y1,
                    style,
                } => {
                    let start = scratch_to_screen_pos(rect, x0 as f64, y0 as f64);
                    let end = scratch_to_screen_pos(rect, x1 as f64, y1 as f64);
                    let stroke = egui::Stroke::new(
                        pen_width_screen_px(rect, style.size as f64),
                        pen_style_to_color(style),
                    );
                    painter.line_segment([start, end], stroke);
                }
                runtime::PenBatchCommand::Disc { x, y, style } => {
                    let center = scratch_to_screen_pos(rect, x as f64, y as f64);
                    let radius = (pen_width_screen_px(rect, style.size as f64) * 0.5).max(0.5);
                    painter.circle_filled(center, radius, pen_style_to_color(style));
                }
            }
        }
    }

    fn update_title(&mut self, ctx: &egui::Context, force: bool) {
        let elapsed = self.fps_window_start.elapsed();
        if !force && elapsed < Duration::from_millis(500) {
            return;
        }
        let render_fps = if elapsed.as_secs_f64() > 0.0 {
            self.presented_frames as f64 / elapsed.as_secs_f64()
        } else {
            0.0
        };
        let simulation_text = self
            .simulation_fps
            .map(|fps| format!("{fps:.1}"))
            .unwrap_or_else(|| "turbo".to_string());
        let present_text = if self.vsync_enabled {
            format!("vsync {}Hz", self.vsync_fps)
        } else {
            "no-vsync".to_string()
        };
        let title = format!(
            "scratch-native-runtime | render {:.1} FPS | sim {} | {}",
            render_fps, simulation_text, present_text
        );
        ctx.send_viewport_cmd(egui::ViewportCommand::Title(title));
        self.presented_frames = 0;
        self.fps_window_start = Instant::now();
    }

    fn sync_ask_prompt_state(&mut self) -> Option<String> {
        let prompt = self.ask_prompt_state.prompt_snapshot();
        let prompt_serial = prompt.as_ref().map(|(serial, _)| *serial);
        if prompt_serial != self.active_prompt_serial {
            self.active_prompt_serial = prompt_serial;
            self.ask_answer_buffer.clear();
            self.focus_ask_input = prompt.is_some();
        }
        prompt.map(|(_, question)| question)
    }

    fn show_ask_prompt(&mut self, ctx: &egui::Context, question: &str) {
        egui::Window::new("scratch-ask-prompt")
            .title_bar(false)
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_BOTTOM, [0.0, -8.0])
            .show(ctx, |ui| {
                ui.set_min_width((STAGE_WIDTH * FIXED_WINDOW_SCALE) as f32 * 0.85);
                ui.label(question);
                ui.add_space(4.0);
                let response = ui.add(
                    egui::TextEdit::singleline(&mut self.ask_answer_buffer)
                        .hint_text("Type answer and press Enter"),
                );
                if self.focus_ask_input {
                    response.request_focus();
                    self.focus_ask_input = false;
                }
                let submit_with_enter =
                    response.has_focus() && ui.input(|input| input.key_pressed(egui::Key::Enter));
                let submit_with_button = ui.button("OK").clicked();
                if submit_with_enter || submit_with_button {
                    let answer = std::mem::take(&mut self.ask_answer_buffer);
                    let _ = self.ask_prompt_state.submit_answer(answer);
                }
            });
    }
}

impl eframe::App for EguiRuntimeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Start worker thread after EGL/OpenGL is fully initialized
        self.start_worker_if_needed();

        self.stop_on_drop = true;
        ctx.request_repaint_after(self.render_interval());

        if ctx.input(|input| {
            input
                .keys_down
                .iter()
                .any(|key| format!("{key:?}") == "Escape")
        }) {
            self.stop_requested.store(true, Ordering::Relaxed);
            self.close_after = Some(Instant::now());
        }

        if ctx.input(|input| input.key_pressed(egui::Key::F9)) {
            self.dump_vars_requested.store(true, Ordering::Relaxed);
        }

        if let Ok(mut input_state) = self.input_state.lock() {
            *input_state = self.poll_input(ctx);
        }
        let ask_question = self.sync_ask_prompt_state();

        let presented = self.update_texture(ctx);

        egui::CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ctx, |ui| {
                let desired_size = egui::vec2(
                    (STAGE_WIDTH * FIXED_WINDOW_SCALE) as f32,
                    (STAGE_HEIGHT * FIXED_WINDOW_SCALE) as f32,
                );
                ui.set_min_size(desired_size);
                ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
                    let (rect, _) = ui.allocate_exact_size(desired_size, egui::Sense::hover());
                    if let Some(texture) = &self.scene_texture {
                        ui.painter().image(
                            texture.id(),
                            rect,
                            egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                            egui::Color32::WHITE,
                        );
                    }
                    if let Some(texture) = &self.pen_texture {
                        ui.painter().image(
                            texture.id(),
                            rect,
                            egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                            egui::Color32::WHITE,
                        );
                    }
                    self.paint_gpu_pen_batch(ui, rect);
                    self.stage_rect = Some(rect);
                });
            });
        if let Some(question) = ask_question.as_deref() {
            self.show_ask_prompt(ctx, question);
        }

        if presented {
            self.presented_frames += 1;
        }
        self.update_title(ctx, false);

        if self.worker_done.load(Ordering::Relaxed) && self.close_after.is_none() {
            self.update_title(ctx, true);
            self.close_after =
                Some(Instant::now() + Duration::from_millis(self.final_frame_hold_ms));
        }
        if self
            .close_after
            .is_some_and(|deadline| Instant::now() >= deadline)
        {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
    }
}

impl Drop for EguiRuntimeApp {
    fn drop(&mut self) {
        if self.stop_on_drop {
            self.stop_requested.store(true, Ordering::Relaxed);
        }
    }
}

fn scratch_to_screen_pos(rect: egui::Rect, scratch_x: f64, scratch_y: f64) -> egui::Pos2 {
    let nx = ((scratch_x + (STAGE_WIDTH as f64 / 2.0)) / STAGE_WIDTH as f64).clamp(0.0, 1.0);
    let ny = (((STAGE_HEIGHT as f64 / 2.0) - scratch_y) / STAGE_HEIGHT as f64).clamp(0.0, 1.0);
    egui::pos2(
        rect.min.x + rect.width() * nx as f32,
        rect.min.y + rect.height() * ny as f32,
    )
}

fn pen_width_screen_px(rect: egui::Rect, pen_size: f64) -> f32 {
    let stage_scale = rect.width() / STAGE_WIDTH as f32;
    ((pen_size.max(1.0) as f32) * stage_scale.max(1.0)).max(1.0)
}

fn pen_style_to_color(style: runtime::PenStrokeStyle) -> egui::Color32 {
    let alpha = (style.alpha.clamp(0.0, 1.0) * 255.0).round() as u8;
    egui::Color32::from_rgba_unmultiplied(style.color[0], style.color[1], style.color[2], alpha)
}

fn key_to_scratch_name(key: &egui::Key) -> Option<String> {
    let debug_name = format!("{key:?}");
    let mapped = match debug_name.as_str() {
        "ArrowLeft" | "Left" => "left arrow",
        "ArrowRight" | "Right" => "right arrow",
        "ArrowUp" | "Up" => "up arrow",
        "ArrowDown" | "Down" => "down arrow",
        "Space" | "Spacebar" => "space",
        _ => {
            if debug_name.len() == 1 {
                let ch = debug_name.chars().next()?;
                if ch.is_ascii_alphabetic() {
                    return Some(ch.to_ascii_lowercase().to_string());
                }
                if ch.is_ascii_digit() {
                    return Some(ch.to_string());
                }
            }
            if let Some(rest) = debug_name.strip_prefix("Num") {
                if rest.len() == 1 {
                    let ch = rest.chars().next()?;
                    if ch.is_ascii_digit() {
                        return Some(ch.to_string());
                    }
                }
            }
            return None;
        }
    };
    Some(mapped.to_string())
}
