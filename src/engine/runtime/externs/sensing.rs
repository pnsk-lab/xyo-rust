//! Sensing block runtime functions

use super::super::{RuntimeState, encode_string_id};
use std::io::{self, IsTerminal, Write};
use std::time::{SystemTime, UNIX_EPOCH};

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
        state.timer_start = std::time::Instant::now();
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
