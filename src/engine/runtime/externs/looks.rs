//! Looks block runtime functions

use super::super::{decode_string_id, encode_string_id, RuntimeState, EMPTY_STRING_ID};
use std::ffi::CStr;
use std::os::raw::c_char;

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
