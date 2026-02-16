//! Operator block runtime functions

use super::super::{EMPTY_STRING_ID, RuntimeState, encode_string_id, is_string_tagged};

// Helper functions (defined in mod.rs)
use super::super::{apply_mathop, js_round};

#[unsafe(no_mangle)]
pub extern "C" fn rt_operator_length(state: *mut RuntimeState, value: f64) -> f64 {
    unsafe { (&*state).value_as_string(value).chars().count() as f64 }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_operator_join(state: *mut RuntimeState, left: f64, right: f64) -> f64 {
    unsafe {
        let state = &mut *state;
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
    if !is_string_tagged(value) {
        return js_round(value);
    }
    unsafe { js_round((&*state).value_to_number(value)) }
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
        let index = super::control::rt_repeat_count(state, state.value_to_number(letter));
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
    // Fast path: both are plain numbers — direct comparison
    if !is_string_tagged(left) && !is_string_tagged(right) {
        // Both are raw f64; use numeric comparison (NaN != NaN per IEEE)
        return if left == right { 1.0 } else { 0.0 };
    }
    unsafe {
        let state = &*state;
        if state.values_equal(left, right) {
            1.0
        } else {
            0.0
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_operator_greater_than(state: *mut RuntimeState, left: f64, right: f64) -> f64 {
    // Fast path: both plain numbers
    if !is_string_tagged(left) && !is_string_tagged(right) {
        return if !left.is_nan() && !right.is_nan() && left > right {
            1.0
        } else {
            0.0
        };
    }
    unsafe {
        let state = &*state;
        if state.compare_values(left, right) > 0 {
            1.0
        } else {
            0.0
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_operator_less_than(state: *mut RuntimeState, left: f64, right: f64) -> f64 {
    // Fast path: both plain numbers
    if !is_string_tagged(left) && !is_string_tagged(right) {
        return if !left.is_nan() && !right.is_nan() && left < right {
            1.0
        } else {
            0.0
        };
    }
    unsafe {
        let state = &*state;
        if state.compare_values(left, right) < 0 {
            1.0
        } else {
            0.0
        }
    }
}

// Arithmetic operators with proper string-to-number conversion

#[unsafe(no_mangle)]
pub extern "C" fn rt_operator_add(state: *mut RuntimeState, left: f64, right: f64) -> f64 {
    // Fast path: both are plain numbers (not NaN-boxed strings)
    if !is_string_tagged(left) && !is_string_tagged(right) {
        return left + right;
    }
    unsafe {
        let state = &*state;
        state.value_to_number(left) + state.value_to_number(right)
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_operator_subtract(state: *mut RuntimeState, left: f64, right: f64) -> f64 {
    if !is_string_tagged(left) && !is_string_tagged(right) {
        return left - right;
    }
    unsafe {
        let state = &*state;
        state.value_to_number(left) - state.value_to_number(right)
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_operator_multiply(state: *mut RuntimeState, left: f64, right: f64) -> f64 {
    if !is_string_tagged(left) && !is_string_tagged(right) {
        return left * right;
    }
    unsafe {
        let state = &*state;
        state.value_to_number(left) * state.value_to_number(right)
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_operator_divide(state: *mut RuntimeState, left: f64, right: f64) -> f64 {
    if !is_string_tagged(left) && !is_string_tagged(right) {
        return left / right;
    }
    unsafe {
        let state = &*state;
        state.value_to_number(left) / state.value_to_number(right)
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_operator_mod(state: *mut RuntimeState, left: f64, right: f64) -> f64 {
    let (left_num, right_num) = if !is_string_tagged(left) && !is_string_tagged(right) {
        (left, right)
    } else {
        unsafe {
            let state = &*state;
            (state.value_to_number(left), state.value_to_number(right))
        }
    };
    let mut result = left_num % right_num;
    // Scratch mod uses floored division instead of truncated division.
    if result / right_num < 0.0 {
        result += right_num;
    }
    result
}
