mod constants;
mod engine;
mod frontend;
mod project;
mod utils;

use constants::*;
use engine::{ir, jit, runtime};
use frontend::gui;
use project::sb3;
use utils::{embedded_project, image};

use anyhow::{Context, Result, bail};
use std::env;
use std::path::PathBuf;
use std::time::Instant;

struct CliOptions {
    sb3_path: Option<PathBuf>,
    output_path: Option<PathBuf>,
    emit_native_object_path: Option<PathBuf>,
    emit_executable_path: Option<PathBuf>,
    emit_only: bool,
    gui_enabled: bool,
    window_scale: usize,
    vsync_enabled: bool,
    vsync_fps: usize,
    target_fps: Option<f64>,
    turbo: bool,
    debug_enabled: bool,
    break_on_messages: Vec<String>,
}

fn main() {
    if let Err(error) = run() {
        eprintln!("error: {error:#}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = parse_cli()?;
    let current_executable = env::current_exe().context("failed to resolve current executable")?;
    if let Some(output_executable_path) = &cli.emit_executable_path {
        let sb3_path = cli
            .sb3_path
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("--emit-executable requires a project.sb3 path"))?;
        if !sb3_path.exists() {
            bail!("sb3 file not found: {}", sb3_path.display());
        }
        embedded_project::emit_embedded_project_executable(
            &current_executable,
            sb3_path,
            output_executable_path,
        )
        .with_context(|| {
            format!(
                "failed to emit executable with embedded project: {}",
                output_executable_path.display()
            )
        })?;
        println!(
            "Emitted executable: {} (embedded project: {})",
            output_executable_path.display(),
            sb3_path.display()
        );
        if cli.emit_only {
            return Ok(());
        }
    }

    let (project, default_output_path) = if let Some(sb3_path) = &cli.sb3_path {
        if !sb3_path.exists() {
            bail!("sb3 file not found: {}", sb3_path.display());
        }
        let project = sb3::load_project_from_sb3(sb3_path)
            .with_context(|| format!("failed to load sb3 project from {}", sb3_path.display()))?;
        (project, sb3_path.with_extension("ppm"))
    } else {
        let embedded_project_bytes = embedded_project::read_embedded_project_bytes(
            &current_executable,
        )
        .with_context(|| {
            format!(
                "failed to inspect embedded project payload in {}",
                current_executable.display()
            )
        })?;
        let Some(embedded_project_bytes) = embedded_project_bytes else {
            bail!("missing sb3 file path and no embedded project was found");
        };
        let project = sb3::load_project_from_sb3_bytes(
            &embedded_project_bytes,
            &current_executable.display().to_string(),
        )
        .with_context(|| {
            format!(
                "failed to load embedded sb3 project from {}",
                current_executable.display()
            )
        })?;
        (project, current_executable.with_extension("ppm"))
    };
    let output_path = cli.output_path.unwrap_or(default_output_path);
    let program = ir::lower_project(&project);

    if program.scripts.is_empty() {
        println!("No `when green flag clicked` scripts were found.");
        return Ok(());
    }

    println!(
        "Loaded {} scripts, {} procedures, {} variables, and {} lists.",
        program.scripts.len(),
        program.procedures.len(),
        program.variables.len(),
        program.lists.len()
    );
    for script in &program.scripts {
        let trigger = match &script.trigger {
            ir::ScriptTrigger::GreenFlag => "green flag".to_string(),
            ir::ScriptTrigger::Broadcast(message) => format!("broadcast '{}'", message),
            ir::ScriptTrigger::KeyPressed(key) => format!("key pressed '{}'", key),
            ir::ScriptTrigger::CloneStart => "clone start".to_string(),
        };
        println!(
            "  script: {} (target: {}, trigger: {})",
            script.name, script.target_name, trigger
        );
    }
    for procedure in &program.procedures {
        println!(
            "  procedure: {} (target: {}, proccode: {}, args: {})",
            procedure.name,
            procedure.target_name,
            procedure.proccode,
            procedure.arg_names.len()
        );
    }
    for list in &program.lists {
        println!(
            "  list: {} ({}) items={}",
            list.name,
            list.id,
            list.initial_values.len()
        );
    }

    if !program.warnings.is_empty() {
        println!("Warnings:");
        for warning in &program.warnings {
            println!("  - {}", warning);
        }
    }

    if let Some(native_output_path) = &cli.emit_native_object_path {
        jit::emit_native_object(&program, native_output_path).with_context(|| {
            format!(
                "failed to emit native object file: {}",
                native_output_path.display()
            )
        })?;
        println!("Emitted native object: {}", native_output_path.display());
        if cli.emit_only {
            return Ok(());
        }
    }

    let initial_variables = program
        .variables
        .iter()
        .map(|variable| variable.initial_value)
        .collect::<Vec<_>>();
    let initial_lists = program
        .lists
        .iter()
        .map(|list| {
            list.initial_values
                .iter()
                .map(|value| match value {
                    ir::ScalarValue::Number(number) => *number,
                    ir::ScalarValue::String(index) => runtime::encode_string_id(*index),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let step_budget = env::var(ENV_SCRATCH_STEP_BUDGET)
        .ok()
        .and_then(|raw| {
            if raw.eq_ignore_ascii_case("unlimited") {
                Some(u64::MAX)
            } else {
                raw.parse::<u64>().ok()
            }
        })
        .unwrap_or(DEFAULT_STEP_BUDGET as u64);
    let mut runtime_state = runtime::RuntimeState::new(
        initial_variables,
        initial_lists,
        program.strings.clone(),
        step_budget,
    );
    runtime_state.set_debug_mode(cli.debug_enabled);
    runtime_state.set_break_on_messages(cli.break_on_messages.clone());
    let (target_render_data, target_initial_visuals) =
        build_render_configuration(&project).context("failed to build render configuration")?;
    runtime_state.configure_render_targets(target_render_data, target_initial_visuals);

    let default_fps = if cli.gui_enabled {
        if cli.vsync_enabled {
            Some(cli.vsync_fps as f64)
        } else {
            Some(30.0)
        }
    } else {
        None
    };
    let target_fps = if cli.turbo {
        None
    } else {
        cli.target_fps.or(default_fps)
    };
    if let Some(fps) = target_fps {
        println!("Execution pacing: {:.2} FPS", fps);
    } else {
        println!("Execution pacing: turbo (unlimited)");
    }
    if cli.debug_enabled {
        println!("Debug trace: enabled");
    }
    if !cli.break_on_messages.is_empty() {
        println!(
            "Broadcast breakpoints: {}",
            cli.break_on_messages.join(", ")
        );
    }
    if cli.gui_enabled {
        println!(
            "Stage resolution scale: {}x (window size fixed)",
            cli.window_scale
        );
        if cli.vsync_enabled {
            println!("Present sync: vsync {} Hz", cli.vsync_fps);
        } else {
            println!("Present sync: off");
        }
    }
    let execution_started_at = Instant::now();
    let mut runtime_state = execute_with_optional_gui(
        &program,
        runtime_state,
        cli.gui_enabled,
        cli.window_scale,
        target_fps,
        cli.vsync_enabled,
        cli.vsync_fps,
    )?;
    let execution_elapsed = execution_started_at.elapsed();
    let executed_operations = step_budget.saturating_sub(runtime_state.remaining_steps);
    let elapsed_seconds = execution_elapsed.as_secs_f64();
    if elapsed_seconds > 0.0 {
        println!(
            "Execution throughput: {} operations in {:.3}s ({:.2} Operation/s)",
            executed_operations,
            elapsed_seconds,
            executed_operations as f64 / elapsed_seconds
        );
    } else {
        println!(
            "Execution throughput: {} operations in <0.001s",
            executed_operations
        );
    }

    if let Some(target) = target_fps {
        if let Some(actual) = runtime_state.measured_fps() {
            let delta = actual - target;
            let ratio = if target > 0.0 {
                (actual / target) * 100.0
            } else {
                0.0
            };
            println!(
                "FPS comparison: target={:.2}, actual={:.2}, delta={:+.2} ({:.1}%)",
                target, actual, delta, ratio
            );
        } else {
            println!(
                "FPS comparison: target={:.2}, actual=unavailable (insufficient paced frames)",
                target
            );
        }
    }

    println!(
        "Final sprite state: x={:.2}, y={:.2}, direction={:.2} (remaining_steps={})",
        runtime_state.sprite_x,
        runtime_state.sprite_y,
        runtime_state.direction_deg,
        runtime_state.remaining_steps
    );
    if !program.variables.is_empty() {
        println!("Variables:");
        for (index, variable) in program.variables.iter().enumerate() {
            let value = runtime_state.variables.get(index).copied().unwrap_or(0.0);
            println!(
                "  {} ({}) = {}",
                variable.name,
                variable.id,
                runtime_state.debug_value(value)
            );
        }
    }

    runtime_state
        .write_canvas_ppm(&output_path)
        .with_context(|| format!("failed to write {}", output_path.display()))?;
    println!("Rendered pen image: {}", output_path.display());

    Ok(())
}

fn parse_cli() -> Result<CliOptions> {
    let mut args = env::args();
    let bin_name = args
        .next()
        .unwrap_or_else(|| "scratch-native-runtime".to_string());
    let usage = format!(
        "usage: {bin_name} [project.sb3] [output.ppm] [--emit-native <output.o>] [--emit-executable <output-bin>] [--emit-only] [--gui|--no-gui] [--scale <1|2|4|8|16>] [--vsync|--no-vsync] [--vsync-fps <value>] [--fps <value>|--turbo] [--debug|--no-debug] [--break-on-message <message>]"
    );

    let mut positional = Vec::new();
    let mut output_path: Option<PathBuf> = None;
    let mut emit_native_object_path: Option<PathBuf> = None;
    let mut emit_executable_path: Option<PathBuf> = None;
    let mut emit_only = false;
    let mut gui_enabled = true;
    let mut window_scale: usize = DEFAULT_WINDOW_SCALE;
    let mut vsync_enabled = true;
    let mut vsync_fps: usize = DEFAULT_VSYNC_FPS;
    let mut target_fps: Option<f64> = None;
    let mut turbo = false;
    let mut debug_enabled = env_flag_enabled(ENV_SCRATCH_DEBUG);
    let mut break_on_messages = env_message_list(ENV_SCRATCH_BREAK_ON_MESSAGE);
    let rest = args.collect::<Vec<_>>();
    let mut index = 0;

    while index < rest.len() {
        match rest[index].as_str() {
            "--gui" => gui_enabled = true,
            "--no-gui" => gui_enabled = false,
            "--vsync" => vsync_enabled = true,
            "--no-vsync" => vsync_enabled = false,
            "--debug" => debug_enabled = true,
            "--no-debug" => debug_enabled = false,
            "--break-on-message" => {
                index += 1;
                let Some(raw) = rest.get(index) else {
                    bail!("--break-on-message requires a message value");
                };
                let message = raw.trim();
                if message.is_empty() {
                    bail!("--break-on-message requires a non-empty message");
                }
                break_on_messages.push(message.to_string());
            }
            "--emit-only" => emit_only = true,
            "--emit-native" => {
                index += 1;
                let Some(raw) = rest.get(index) else {
                    bail!("--emit-native requires a file path");
                };
                emit_native_object_path = Some(PathBuf::from(raw));
            }
            "--emit-executable" => {
                index += 1;
                let Some(raw) = rest.get(index) else {
                    bail!("--emit-executable requires a file path");
                };
                emit_executable_path = Some(PathBuf::from(raw));
            }
            "--scale" => {
                index += 1;
                let Some(raw) = rest.get(index) else {
                    bail!("--scale requires a numeric value");
                };
                let parsed = raw
                    .parse::<usize>()
                    .with_context(|| format!("invalid --scale value: {}", raw))?;
                if !matches!(parsed, 1 | 2 | 4 | 8 | 16) {
                    bail!("--scale must be one of: 1, 2, 4, 8, 16");
                }
                window_scale = parsed;
            }
            "--vsync-fps" => {
                index += 1;
                let Some(raw) = rest.get(index) else {
                    bail!("--vsync-fps requires a numeric value");
                };
                let parsed = raw
                    .parse::<usize>()
                    .with_context(|| format!("invalid --vsync-fps value: {}", raw))?;
                if parsed == 0 {
                    bail!("--vsync-fps must be greater than 0");
                }
                vsync_fps = parsed;
            }
            "--turbo" => {
                target_fps = None;
                turbo = true;
            }
            "--fps" => {
                index += 1;
                let Some(raw) = rest.get(index) else {
                    bail!("--fps requires a numeric value");
                };
                let fps = raw
                    .parse::<f64>()
                    .with_context(|| format!("invalid --fps value: {}", raw))?;
                if !fps.is_finite() || fps <= 0.0 {
                    bail!("--fps must be a positive number");
                }
                target_fps = Some(fps);
                turbo = false;
            }
            _ => {
                if rest[index].starts_with("--") {
                    bail!("unknown option: {}", rest[index]);
                }
                if positional.len() < 2 {
                    positional.push(rest[index].clone());
                } else {
                    bail!("too many positional arguments\n{usage}");
                }
            }
        }
        index += 1;
    }

    let sb3_path = positional.first().map(PathBuf::from);
    if let Some(raw) = positional.get(1) {
        output_path = Some(PathBuf::from(raw));
    }

    if emit_only && emit_native_object_path.is_none() && emit_executable_path.is_none() {
        bail!(
            "--emit-only requires --emit-native <output.o> and/or --emit-executable <output-bin>"
        );
    }
    if emit_executable_path.is_some() && sb3_path.is_none() {
        eprintln!("{usage}");
        bail!("--emit-executable requires project.sb3 positional argument");
    }
    if sb3_path.is_none() && emit_native_object_path.is_some() {
        eprintln!("{usage}");
        bail!("--emit-native requires project.sb3 positional argument");
    }
    break_on_messages.retain(|message| !message.trim().is_empty());

    Ok(CliOptions {
        sb3_path,
        output_path,
        emit_native_object_path,
        emit_executable_path,
        emit_only,
        gui_enabled,
        window_scale,
        vsync_enabled,
        vsync_fps,
        target_fps,
        turbo,
        debug_enabled,
        break_on_messages,
    })
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

fn execute_with_optional_gui(
    program: &ir::Program,
    mut runtime_state: runtime::RuntimeState,
    gui_enabled: bool,
    window_scale: usize,
    target_fps: Option<f64>,
    vsync_enabled: bool,
    vsync_fps: usize,
) -> Result<runtime::RuntimeState> {
    if !gui_enabled {
        runtime_state.set_target_fps(target_fps);
        jit::execute_program(program, &mut runtime_state)
            .context("failed to execute native-compiled Scratch program")?;
        return Ok(runtime_state);
    }

    gui::execute_program_with_gui(
        program.clone(),
        runtime_state,
        window_scale,
        target_fps,
        vsync_enabled,
        vsync_fps,
    )
}

fn build_render_configuration(
    project: &sb3::Project,
) -> Result<(
    Vec<runtime::TargetRenderData>,
    Vec<runtime::TargetInitialVisualState>,
)> {
    let mut targets = Vec::with_capacity(project.targets.len());
    let mut initial_visuals = Vec::with_capacity(project.targets.len());

    for target in &project.targets {
        let mut costumes = Vec::new();
        for costume in &target.costumes {
            let Some(bytes) = project.assets.get(&costume.md5ext) else {
                continue;
            };
            match image::decode_costume_rgba(costume, bytes) {
                Ok((width, height, pixels_rgba)) => costumes.push(runtime::CostumeBitmap {
                    width,
                    height,
                    pixels_rgba,
                    rotation_center_x: costume.rotation_center_x,
                    rotation_center_y: costume.rotation_center_y,
                }),
                Err(error) => {
                    eprintln!(
                        "warning: failed to decode costume '{}' ({}): {error}",
                        costume.name, costume.md5ext
                    );
                }
            }
        }

        targets.push(runtime::TargetRenderData {
            is_stage: target.is_stage,
            layer_order: target.layer_order,
            costumes,
        });
        initial_visuals.push(runtime::TargetInitialVisualState {
            x: target.x,
            y: target.y,
            direction_deg: target.direction,
            costume_number: (target.current_costume.saturating_add(1)) as f64,
            visible: if target.is_stage {
                true
            } else {
                target.visible
            },
            size_percent: target.size.max(0.0),
        });
    }

    Ok((targets, initial_visuals))
}

// Image processing functions moved to utils::image module
