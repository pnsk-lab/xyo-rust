use super::*;
use std::ffi::CStr;
use std::io::{self, IsTerminal, Write};
use std::os::raw::c_char;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

#[unsafe(no_mangle)]
pub extern "C" fn rt_count_executed_block(state: *mut RuntimeState) {
    unsafe {
        if let Some(state) = state.as_mut() {
            state.executed_block_count = state.executed_block_count.saturating_add(1);
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_motion_move_steps(state: *mut RuntimeState, steps: f64) {
    unsafe {
        if let Some(state) = state.as_mut() {
            let steps = state.value_to_number(steps);
            if !steps.is_finite() {
                return;
            }
            let radians = (90.0 - state.direction_deg).to_radians();
            let new_x = state.sprite_x + steps * radians.cos();
            let new_y = state.sprite_y + steps * radians.sin();
            state.move_sprite_to(new_x, new_y);
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_motion_set_direction(state: *mut RuntimeState, direction: f64) {
    unsafe {
        if let Some(state) = state.as_mut() {
            let direction = state.value_to_number(direction);
            if !direction.is_finite() {
                return;
            }
            state.direction_deg = direction;
            state.live_canvas_dirty = true;
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_motion_change_x(state: *mut RuntimeState, dx: f64) {
    unsafe {
        if let Some(state) = state.as_mut() {
            let dx = state.value_to_number(dx);
            if !dx.is_finite() {
                return;
            }
            state.move_sprite_to(state.sprite_x + dx, state.sprite_y);
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_motion_change_y(state: *mut RuntimeState, dy: f64) {
    unsafe {
        if let Some(state) = state.as_mut() {
            let dy = state.value_to_number(dy);
            if !dy.is_finite() {
                return;
            }
            state.move_sprite_to(state.sprite_x, state.sprite_y + dy);
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_motion_set_x(state: *mut RuntimeState, x: f64) {
    unsafe {
        if let Some(state) = state.as_mut() {
            let x = state.value_to_number(x);
            if !x.is_finite() {
                return;
            }
            state.move_sprite_to(x, state.sprite_y);
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_motion_set_y(state: *mut RuntimeState, y: f64) {
    unsafe {
        if let Some(state) = state.as_mut() {
            let y = state.value_to_number(y);
            if !y.is_finite() {
                return;
            }
            state.move_sprite_to(state.sprite_x, y);
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_motion_goto_xy(state: *mut RuntimeState, x: f64, y: f64) {
    unsafe {
        if let Some(state) = state.as_mut() {
            let x = state.value_to_number(x);
            let y = state.value_to_number(y);
            if !x.is_finite() || !y.is_finite() {
                return;
            }
            state.move_sprite_to(x, y);
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_motion_x_position(state: *mut RuntimeState) -> f64 {
    unsafe { state.as_ref().map(|state| state.sprite_x).unwrap_or(0.0) }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_motion_y_position(state: *mut RuntimeState) -> f64 {
    unsafe { state.as_ref().map(|state| state.sprite_y).unwrap_or(0.0) }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_get_var(state: *mut RuntimeState, index: u64) -> f64 {
    unsafe {
        state
            .as_ref()
            .and_then(|s| s.variables.get(index as usize))
            .copied()
            .unwrap_or(0.0)
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_set_var(state: *mut RuntimeState, index: u64, value: f64) {
    unsafe {
        if let Some(state) = state.as_mut() {
            if let Some(slot) = state.variables.get_mut(index as usize) {
                *slot = value;
            }
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_change_var(state: *mut RuntimeState, index: u64, delta: f64) {
    unsafe {
        if let Some(state) = state.as_mut() {
            let base = state
                .variables
                .get(index as usize)
                .copied()
                .map(|value| state.value_to_number(value))
                .unwrap_or(0.0);
            let delta = state.value_to_number(delta);
            if let Some(slot) = state.variables.get_mut(index as usize) {
                *slot = base + delta;
            }
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_sensing_mouse_x(state: *mut RuntimeState) -> f64 {
    unsafe { state.as_ref().map(|state| state.mouse_x()).unwrap_or(0.0) }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_sensing_mouse_y(state: *mut RuntimeState) -> f64 {
    unsafe { state.as_ref().map(|state| state.mouse_y()).unwrap_or(0.0) }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_sensing_mouse_down(state: *mut RuntimeState) -> f64 {
    unsafe {
        if state.as_ref().is_some_and(|state| state.mouse_down()) {
            1.0
        } else {
            0.0
        }
    }
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

#[unsafe(no_mangle)]
pub extern "C" fn rt_sensing_key_pressed(state: *mut RuntimeState, key_option: f64) -> f64 {
    unsafe {
        let Some(state) = state.as_ref() else {
            return 0.0;
        };
        let key_name = normalize_key_name(&state.value_as_string(key_option));
        let result = if state.is_key_down(&key_name) {
            1.0
        } else {
            0.0
        };
        if state.debug_mode {
            eprintln!(
                "[debug][key] keypressed('{}') = {} (input_state={})",
                key_name,
                result,
                if state.input_state.is_some() {
                    "attached"
                } else {
                    "none"
                }
            );
        }
        result
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_sensing_ask_and_wait(state: *mut RuntimeState, question: f64) {
    unsafe {
        let Some(state) = state.as_mut() else {
            return;
        };
        state.flush_live_canvas();
        let question_text = state.value_as_string(question);

        let answer = if let Ok(env_answer) = std::env::var("SCRATCH_ANSWER") {
            env_answer
        } else if let Some(prompt_state) = state.ask_prompt_state.as_ref() {
            prompt_state.prompt_and_wait(question_text.clone(), state.stop_requested.as_ref())
        } else if io::stdin().is_terminal() {
            println!("[ask] {question_text}");
            let _ = io::stdout().flush();
            print!("> ");
            let _ = io::stdout().flush();
            let mut line = String::new();
            if io::stdin().read_line(&mut line).is_ok() {
                line.trim_end_matches(['\r', '\n']).to_string()
            } else {
                String::new()
            }
        } else {
            String::new()
        };
        let answer_id = state.intern_string(&answer);
        state.answer_value = encode_string_id(answer_id);
        state.flush_live_canvas();
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_sensing_answer(state: *mut RuntimeState) -> f64 {
    unsafe {
        state
            .as_ref()
            .map(|state| state.answer_value)
            .unwrap_or(0.0)
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_sensing_timer(state: *mut RuntimeState) -> f64 {
    unsafe {
        state
            .as_ref()
            .map(|state| state.timer_start.elapsed().as_secs_f64())
            .unwrap_or(0.0)
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_sensing_days_since_2000(_state: *mut RuntimeState) -> f64 {
    const UNIX_DAYS_TO_2000_01_01: f64 = 10_957.0;
    let now = SystemTime::now();
    let since_epoch = now.duration_since(UNIX_EPOCH).unwrap_or_default();
    since_epoch.as_secs_f64() / 86_400.0 - UNIX_DAYS_TO_2000_01_01
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_sensing_of(state: *mut RuntimeState, object: f64, property: f64) -> f64 {
    unsafe {
        let Some(state) = state.as_mut() else {
            return 0.0;
        };
        state.sensing_of(object, property)
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_sensing_touching_object(state: *mut RuntimeState, object: f64) -> f64 {
    unsafe {
        let Some(state) = state.as_mut() else {
            return 0.0;
        };
        let selector = state.value_as_string(object).trim().to_ascii_lowercase();
        let touching = match selector.as_str() {
            "_edge_" | "edge" => {
                state.sprite_x <= -240.0
                    || state.sprite_x >= 240.0
                    || state.sprite_y <= -180.0
                    || state.sprite_y >= 180.0
            }
            "_mouse_" | "mouse-pointer" | "mouse pointer" => {
                let dx = state.sprite_x - state.mouse_x();
                let dy = state.sprite_y - state.mouse_y();
                dx * dx + dy * dy <= 9.0
            }
            _ => state
                .resolve_target_index_by_name(&selector)
                .and_then(|target_index| {
                    state
                        .base_actor_by_target
                        .get(target_index as usize)
                        .copied()
                        .and_then(|actor_id| state.actor_snapshot(actor_id))
                })
                .is_some_and(|other| {
                    let dx = state.sprite_x - other.sprite_x;
                    let dy = state.sprite_y - other.sprite_y;
                    dx * dx + dy * dy <= 1.0
                }),
        };
        if touching { 1.0 } else { 0.0 }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_sensing_touching_color(state: *mut RuntimeState, color: f64) -> f64 {
    unsafe {
        let Some(state) = state.as_mut() else {
            return 0.0;
        };
        let rgb = state.parse_color_value(color);
        if state.touching_color(rgb) { 1.0 } else { 0.0 }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_sensing_current(_state: *mut RuntimeState, menu_code: u64) -> f64 {
    let now = SystemTime::now();
    let since_epoch = now.duration_since(UNIX_EPOCH).unwrap_or_default();
    let unix_seconds = since_epoch.as_secs_f64();
    let unix_days = (unix_seconds / 86_400.0).floor() as i64;
    let seconds_in_day = unix_seconds.rem_euclid(86_400.0);

    let hour = (seconds_in_day / 3600.0).floor() as u32;
    let minute = ((seconds_in_day / 60.0).floor() as u32) % 60;
    let second = (seconds_in_day.floor() as u32) % 60;
    let (year, month, day) = civil_from_days(unix_days);
    let day_of_week = ((unix_days + 4).rem_euclid(7) + 1) as u32;

    match menu_code {
        0 => year as f64,
        1 => month as f64,
        2 => day as f64,
        3 => day_of_week as f64,
        4 => hour as f64,
        5 => minute as f64,
        6 => second as f64,
        _ => 0.0,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_sensing_reset_timer(state: *mut RuntimeState) {
    unsafe {
        let Some(state) = state.as_mut() else {
            return;
        };
        state.timer_start = Instant::now();
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_looks_say_number(state: *mut RuntimeState, value: f64) {
    unsafe {
        if let Some(state) = state.as_ref() {
            if decode_string_id(value).is_some() {
                println!("[say] {}", state.value_as_string(value));
                return;
            }
        }
    }
    println!("[say] {}", value);
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_looks_say_text(_state: *mut RuntimeState, text: *const c_char) {
    if text.is_null() {
        println!("[say] ");
        return;
    }
    let rendered = unsafe { CStr::from_ptr(text) }.to_string_lossy();
    println!("[say] {}", rendered);
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_looks_hide(state: *mut RuntimeState) {
    unsafe {
        if let Some(state) = state.as_mut() {
            state.visible = false;
            state.live_canvas_dirty = true;
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_looks_show(state: *mut RuntimeState) {
    unsafe {
        if let Some(state) = state.as_mut() {
            state.visible = true;
            state.live_canvas_dirty = true;
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_looks_set_size(state: *mut RuntimeState, size: f64) {
    unsafe {
        let Some(state) = state.as_mut() else {
            return;
        };
        let numeric = state.value_to_number(size);
        if !numeric.is_finite() {
            return;
        }
        state.size_percent = numeric.max(0.0);
        state.live_canvas_dirty = true;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_looks_set_effect_to(state: *mut RuntimeState, effect: f64, value: f64) {
    unsafe {
        let Some(state) = state.as_mut() else {
            return;
        };
        state.set_effect_to(effect, value);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_looks_switch_costume_to(state: *mut RuntimeState, costume: f64) {
    unsafe {
        let Some(state) = state.as_mut() else {
            return;
        };
        state.switch_costume_to(costume);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_looks_switch_backdrop_to(state: *mut RuntimeState, backdrop: f64) {
    unsafe {
        let Some(state) = state.as_mut() else {
            return;
        };
        state.switch_backdrop_to(backdrop);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_looks_costume_number(state: *mut RuntimeState) -> f64 {
    unsafe {
        state
            .as_ref()
            .map(|state| state.costume_number_value())
            .unwrap_or(1.0)
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_looks_costume_name(state: *mut RuntimeState) -> f64 {
    unsafe {
        let Some(state) = state.as_mut() else {
            return encode_string_id(EMPTY_STRING_ID);
        };
        state.costume_name_value()
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_pen_down(state: *mut RuntimeState) {
    unsafe {
        if let Some(state) = state.as_mut() {
            state.pen_down = true;
            state.draw_disc(state.sprite_x, state.sprite_y);
            state.live_canvas_dirty = true;
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_pen_up(state: *mut RuntimeState) {
    unsafe {
        if let Some(state) = state.as_mut() {
            state.pen_down = false;
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_pen_clear(state: *mut RuntimeState) {
    unsafe {
        if let Some(state) = state.as_mut() {
            state.clear_canvas();
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_pen_set_size(state: *mut RuntimeState, size: f64) {
    unsafe {
        if let Some(state) = state.as_mut() {
            let numeric = state.value_to_number(size);
            if !numeric.is_finite() {
                return;
            }
            state.pen_size = numeric.max(1.0);
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_pen_set_color(state: *mut RuntimeState, color: f64) {
    unsafe {
        let Some(state) = state.as_mut() else {
            return;
        };

        if let Some(index) = decode_string_id(color) {
            let raw = state.strings.get(index).cloned().unwrap_or_default();
            // '#rrggbb' or '0xrrggbb' → direct hex-to-RGB
            if let Some(rgb) = parse_hex_color(&raw) {
                state.pen_color = rgb;
                return;
            }
            // Otherwise interpret the string as a number and extract RGB
            // from the decimal value, matching scratch-vm's
            // Color.decimalToRgb(Cast.toNumber(value)).
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
            state.pen_color = decimal_to_rgb(number);
            return;
        }

        // Numeric value → extract RGB components from the integer,
        // matching scratch-vm's Color.decimalToRgb().
        state.pen_color = decimal_to_rgb(state.value_to_number(color));
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_pen_stamp(state: *mut RuntimeState) {
    unsafe {
        let Some(state) = state.as_mut() else {
            return;
        };
        state.stamp_active_sprite_to_pen_layer();
        state.live_canvas_dirty = true;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_pen_set_color_param(state: *mut RuntimeState, param_code: u64, value: f64) {
    unsafe {
        let Some(state) = state.as_mut() else {
            return;
        };
        let numeric = state.value_to_number(value);
        match param_code {
            // color
            0 => {
                state.pen_color = hue_to_rgb(numeric);
            }
            // saturation
            1 => {
                let (h, _s, v) = rgb_to_hsv(state.pen_color);
                let saturation = (numeric / 100.0).clamp(0.0, 1.0);
                state.pen_color = hsv_to_rgb(h, saturation, v);
            }
            // brightness
            2 => {
                let (h, s, _v) = rgb_to_hsv(state.pen_color);
                let brightness = (numeric / 100.0).clamp(0.0, 1.0);
                state.pen_color = hsv_to_rgb(h, s, brightness);
            }
            // transparency
            3 => {
                state.pen_alpha = (1.0 - (numeric / 100.0)).clamp(0.0, 1.0);
            }
            _ => {}
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_music_set_tempo(state: *mut RuntimeState, tempo: f64) {
    unsafe {
        let Some(state) = state.as_mut() else {
            return;
        };
        state.tempo_bpm = state.value_to_number(tempo);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_control_wait(state: *mut RuntimeState, duration: f64) {
    unsafe {
        let Some(state) = state.as_mut() else {
            return;
        };
        let seconds = state.value_to_number(duration);
        if !seconds.is_finite() {
            return;
        }
        if state.debug_mode {
            eprintln!("[debug][wait] control_wait duration={:.3}s", seconds);
        }

        if seconds <= 0.0 {
            control_wait_yield_once(state);
            return;
        }

        let deadline = Instant::now() + Duration::from_secs_f64(seconds);
        while Instant::now() < deadline {
            if state.frame_duration.is_none() && state.active_fiber_control.is_none() {
                let remaining = deadline.saturating_duration_since(Instant::now());
                std::thread::sleep(remaining.min(Duration::from_millis(1)));
            } else {
                control_wait_yield_once(state);
            }
            if state
                .stop_requested
                .as_ref()
                .is_some_and(|stop| stop.load(Ordering::Relaxed))
            {
                break;
            }
        }
    }
}

fn control_wait_yield_once(state: &mut RuntimeState) {
    if state.active_fiber_control.is_some() {
        if state.frame_duration.is_some() {
            state.wait_for_next_frame();
        } else if let Some(control) = state.active_fiber_control.as_ref().map(Arc::clone) {
            control.yield_to_scheduler();
        }
    } else {
        state.wait_for_next_frame();
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_control_create_clone_of(state: *mut RuntimeState, target_selector: i64) {
    unsafe {
        let Some(state) = state.as_mut() else {
            return;
        };
        state.create_clone(target_selector);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_control_delete_this_clone(state: *mut RuntimeState) {
    unsafe {
        let Some(state) = state.as_mut() else {
            return;
        };
        state.delete_active_clone();
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_control_stop(state: *mut RuntimeState, mode_code: u64) {
    unsafe {
        let Some(state) = state.as_mut() else {
            return;
        };
        if state.debug_mode {
            eprintln!("[debug][stop] control_stop mode={}", mode_code);
        }
        match mode_code {
            // "stop this script" – JIT emits an immediate return for the
            // current function after this extern call, so avoid mutating the
            // global step budget (which would incorrectly impact other scripts).
            0 => {}
            1 => state.stop_all_scripts(),
            2 => state.stop_other_scripts_in_active_target(),
            _ => {}
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_repeat_count(value: f64) -> u64 {
    if !value.is_finite() {
        return 0;
    }
    if value <= 0.0 {
        return 0;
    }
    let floored = value.floor();
    if floored > (u64::MAX as f64) {
        u64::MAX
    } else {
        floored as u64
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_operator_length(state: *mut RuntimeState, value: f64) -> f64 {
    unsafe {
        let Some(state) = state.as_ref() else {
            return value.to_string().chars().count() as f64;
        };
        state.value_as_string(value).chars().count() as f64
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_operator_join(state: *mut RuntimeState, left: f64, right: f64) -> f64 {
    unsafe {
        let Some(state) = state.as_mut() else {
            return 0.0;
        };
        let merged = format!(
            "{}{}",
            state.value_as_string(left),
            state.value_as_string(right)
        );
        let id = state.intern_string(&merged);
        encode_string_id(id)
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_operator_round(state: *mut RuntimeState, value: f64) -> f64 {
    unsafe {
        let Some(state) = state.as_ref() else {
            return js_round(value);
        };
        js_round(state.value_to_number(value))
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_operator_contains(state: *mut RuntimeState, text: f64, part: f64) -> f64 {
    unsafe {
        let Some(state) = state.as_ref() else {
            return 0.0;
        };
        let haystack = state.value_as_string(text).to_lowercase();
        let needle = state.value_as_string(part).to_lowercase();
        if haystack.contains(&needle) { 1.0 } else { 0.0 }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_operator_mathop(state: *mut RuntimeState, op: f64, value: f64) -> f64 {
    unsafe {
        let Some(state) = state.as_ref() else {
            return 0.0;
        };
        let op_code = state.value_to_number(op).round().max(0.0) as u64;
        let numeric = state.value_to_number(value);
        apply_mathop(op_code, numeric)
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_operator_letter_of(
    state: *mut RuntimeState,
    letter: f64,
    string_value: f64,
) -> f64 {
    unsafe {
        let Some(state) = state.as_mut() else {
            return encode_string_id(EMPTY_STRING_ID);
        };
        let index = rt_repeat_count(state.value_to_number(letter));
        if index == 0 {
            return encode_string_id(EMPTY_STRING_ID);
        }

        let source = state.value_as_string(string_value);
        let character = source
            .chars()
            .nth(index as usize - 1)
            .map(|c| c.to_string())
            .unwrap_or_default();
        let id = state.intern_string(&character);
        encode_string_id(id)
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_operator_equals(state: *mut RuntimeState, left: f64, right: f64) -> f64 {
    unsafe {
        let Some(state) = state.as_ref() else {
            return if left == right { 1.0 } else { 0.0 };
        };
        if state.values_equal(left, right) {
            1.0
        } else {
            0.0
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_operator_greater_than(state: *mut RuntimeState, left: f64, right: f64) -> f64 {
    unsafe {
        let Some(state) = state.as_ref() else {
            return 0.0;
        };
        if state.compare_values(left, right) > 0 {
            1.0
        } else {
            0.0
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_operator_less_than(state: *mut RuntimeState, left: f64, right: f64) -> f64 {
    unsafe {
        let Some(state) = state.as_ref() else {
            return 0.0;
        };
        if state.compare_values(left, right) < 0 {
            1.0
        } else {
            0.0
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_data_item_of_list(
    state: *mut RuntimeState,
    list_index: u64,
    index: f64,
) -> f64 {
    unsafe {
        let Some(state) = state.as_mut() else {
            return 0.0;
        };
        state.list_item(list_index as usize, index)
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_data_add_to_list(state: *mut RuntimeState, list_index: u64, item: f64) {
    unsafe {
        let Some(state) = state.as_mut() else {
            return;
        };
        state.list_add_item(list_index as usize, item);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_data_length_of_list(state: *mut RuntimeState, list_index: u64) -> f64 {
    unsafe {
        let Some(state) = state.as_ref() else {
            return 0.0;
        };
        state.list_length(list_index as usize)
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_data_delete_all_of_list(state: *mut RuntimeState, list_index: u64) {
    unsafe {
        let Some(state) = state.as_mut() else {
            return;
        };
        state.list_delete_all(list_index as usize);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_data_delete_of_list(state: *mut RuntimeState, list_index: u64, index: f64) {
    unsafe {
        let Some(state) = state.as_mut() else {
            return;
        };
        state.list_delete_item(list_index as usize, index);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_data_replace_item_of_list(
    state: *mut RuntimeState,
    list_index: u64,
    index: f64,
    item: f64,
) {
    unsafe {
        let Some(state) = state.as_mut() else {
            return;
        };
        state.list_replace_item(list_index as usize, index, item);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_data_item_num_of_list(
    state: *mut RuntimeState,
    list_index: u64,
    item: f64,
) -> f64 {
    unsafe {
        let Some(state) = state.as_ref() else {
            return 0.0;
        };
        state.list_item_num(list_index as usize, item)
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_data_list_contains_item(
    state: *mut RuntimeState,
    list_index: u64,
    item: f64,
) -> f64 {
    unsafe {
        let Some(state) = state.as_ref() else {
            return 0.0;
        };
        if state.list_contains_item(list_index as usize, item) {
            1.0
        } else {
            0.0
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_event_broadcast_value(state: *mut RuntimeState, message: f64) {
    unsafe {
        let Some(state_ref) = state.as_mut() else {
            return;
        };
        let scripts = state_ref.broadcast_script_ids_for_message_value(message);
        let message_text = state_ref.value_as_string(message);
        state_ref.break_on_broadcast_message(message_text.as_str(), false);
        if state_ref.should_trace_events() {
            eprintln!(
                "[debug][event] broadcast '{}' handlers={}",
                message_text,
                scripts.len()
            );
        } else if state_ref.should_trace_broadcasts() {
            eprintln!(
                "[broadcast] message='{}' handlers={}",
                message_text,
                scripts.len()
            );
        }
        let event_name = format!("broadcast '{}'", message_text);
        dispatch_broadcast(state_ref, scripts, false, event_name.as_str());
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_event_broadcast_and_wait_value(state: *mut RuntimeState, message: f64) {
    unsafe {
        let Some(state_ref) = state.as_mut() else {
            return;
        };
        let scripts = state_ref.broadcast_script_ids_for_message_value(message);
        let message_text = state_ref.value_as_string(message);
        state_ref.break_on_broadcast_message(message_text.as_str(), true);
        if state_ref.should_trace_events() {
            eprintln!(
                "[debug][event] broadcast and wait '{}' handlers={}",
                message_text,
                scripts.len()
            );
        } else if state_ref.should_trace_broadcasts() {
            eprintln!(
                "[broadcast_wait] message='{}' handlers={}",
                message_text,
                scripts.len()
            );
        }
        let event_name = format!("broadcast and wait '{}'", message_text);
        dispatch_broadcast(state_ref, scripts, true, event_name.as_str());
    }
}

fn dispatch_broadcast(
    state_ref: &mut RuntimeState,
    target_scripts: Vec<u64>,
    wait: bool,
    event_name: &str,
) {
    // Collect the (script_id, actor_id) pairs that THIS broadcast should handle.
    let mut tasks: Vec<(u64, u64)> = Vec::new();
    for script_id in target_scripts {
        let Some(target_index) = state_ref
            .script_target_by_id
            .get(script_id as usize)
            .copied()
        else {
            continue;
        };
        let actor_ids = state_ref.actor_ids_for_target(target_index);
        for actor_id in actor_ids {
            if state_ref.should_trace_events() {
                eprintln!(
                    "[debug][queue] event={} script={} (id={}) actor={}",
                    event_name,
                    state_ref.script_name_for_id(script_id),
                    script_id,
                    state_ref.actor_label(actor_id)
                );
            }
            tasks.push((script_id, actor_id));
        }
    }

    if !wait {
        for (script_id, actor_id) in tasks {
            state_ref.enqueue_task(script_id, actor_id, Some(event_name), None);
        }
        return;
    }

    // Broadcast-and-wait must run all handlers concurrently and wait until all
    // launched scripts complete.
    let wait_group_id = state_ref.create_wait_group();
    for (script_id, actor_id) in tasks {
        state_ref.enqueue_task(script_id, actor_id, Some(event_name), Some(wait_group_id));
    }

    if state_ref.active_fiber_control.is_some() {
        while state_ref.wait_group_has_pending_tasks(wait_group_id) {
            if state_ref.frame_duration.is_some() {
                state_ref.wait_for_next_frame();
            } else if let Some(control) = state_ref.active_fiber_control.as_ref().map(Arc::clone) {
                control.yield_to_scheduler();
            } else {
                break;
            }
            if state_ref
                .stop_requested
                .as_ref()
                .is_some_and(|stop| stop.load(Ordering::Relaxed))
            {
                break;
            }
        }
        if state_ref.frame_duration.is_some() {
            // Keep each frame/tick responsive by refreshing the current fiber's
            // script budget after broadcast-and-wait handlers.
            state_ref.remaining_steps = state_ref.step_budget;
        }
        return;
    }

    // Legacy non-fiber mode: process queued handlers inline until this wait
    // group drains.
    while state_ref.wait_group_has_pending_tasks(wait_group_id) {
        if let Some(task) = state_ref.dequeue_script() {
            state_ref.processing_queued_script = true;
            state_ref.run_script(task.script_id, task.actor_id);
            state_ref.processing_queued_script = false;
            state_ref.complete_wait_group_task(task.wait_group_id);
        } else {
            state_ref.wait_for_next_frame();
        }
        if state_ref
            .stop_requested
            .as_ref()
            .is_some_and(|stop| stop.load(Ordering::Relaxed))
        {
            break;
        }
    }
}

fn loop_should_continue(
    state: &mut RuntimeState,
    pace_frames: bool,
    present_live_canvas: bool,
    consume_step_budget: bool,
) -> bool {
    if state
        .stop_requested
        .as_ref()
        .is_some_and(|stop| stop.load(Ordering::Relaxed))
    {
        return false;
    }
    if consume_step_budget {
        if state.remaining_steps == 0 {
            if state.debug_mode {
                eprintln!("[debug][loop] step budget exhausted, exiting loop");
            }
            return false;
        }
        state.remaining_steps -= 1;
    }
    let first_guard_this_resume = if pace_frames && state.active_fiber_control.is_some() {
        state.note_paced_loop_guard()
    } else {
        false
    };
    if pace_frames && !first_guard_this_resume && state.should_yield_for_work_time() {
        state.wait_for_next_frame();
    }
    // In fiber mode the scheduler handles canvas sync, key-press polling,
    // and script interleaving.  In legacy (non-fiber) mode we do it inline.
    if state.active_fiber_control.is_none() {
        if present_live_canvas {
            state.sync_live_canvas_if_due(false);
        }
        state.enqueue_key_pressed_scripts();
        if !state.processing_queued_script {
            if let Some(task) = state.dequeue_script() {
                state.processing_queued_script = true;
                state.run_script(task.script_id, task.actor_id);
                state.processing_queued_script = false;
                state.complete_wait_group_task(task.wait_group_id);
            }
        }
    }
    true
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_forever_should_continue(state: *mut RuntimeState) -> bool {
    unsafe {
        let Some(state) = state.as_mut() else {
            return false;
        };
        loop_should_continue(state, true, true, true)
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_forever_should_continue_warp(state: *mut RuntimeState) -> bool {
    unsafe {
        let Some(state) = state.as_mut() else {
            return false;
        };
        // Warp-mode `forever` loops still consume step budget because they
        // have no condition that will eventually become true – budget
        // exhaustion is the only exit mechanism (besides `stop`).
        // Even in warp mode, periodically yield in fiber execution so a
        // single long-running custom block cannot starve other scripts.
        loop_should_continue(state, true, false, true)
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_loop_should_continue(state: *mut RuntimeState) -> bool {
    unsafe {
        let Some(state) = state.as_mut() else {
            return false;
        };
        let consume_step_budget = !state.relax_procedure_loop_budget;
        loop_should_continue(state, true, true, consume_step_budget)
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_loop_should_continue_warp(state: *mut RuntimeState) -> bool {
    unsafe {
        let Some(state) = state.as_mut() else {
            return false;
        };
        // Warp-mode (run without screen refresh) loops run without budget
        // limits. Still periodically yield in fiber mode to avoid starving
        // other scripts when large warp procedures execute.
        loop_should_continue(state, true, false, false)
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_random(state: *mut RuntimeState, from: f64, to: f64) -> f64 {
    unsafe {
        let Some(state) = state.as_mut() else {
            return from;
        };

        let mut lo = state.value_to_number(from);
        let mut hi = state.value_to_number(to);
        if lo > hi {
            std::mem::swap(&mut lo, &mut hi);
        }
        if !lo.is_finite() || !hi.is_finite() {
            return 0.0;
        }
        if (lo - hi).abs() <= f64::EPSILON {
            return lo;
        }

        let unit = next_random_unit(state);
        let lo_int = lo.fract() == 0.0;
        let hi_int = hi.fract() == 0.0;
        if lo_int && hi_int {
            let span = (hi - lo + 1.0).max(1.0);
            let stepped = (unit * span).floor();
            (lo + stepped).min(hi)
        } else {
            lo + (hi - lo) * unit
        }
    }
}

fn civil_from_days(days_since_unix_epoch: i64) -> (i32, u32, u32) {
    let z = days_since_unix_epoch + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = mp + if mp < 10 { 3 } else { -9 };
    let year = y + if month <= 2 { 1 } else { 0 };
    (year as i32, month as u32, day as u32)
}
