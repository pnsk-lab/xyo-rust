//! Variable get/set/change operations

use super::super::{RuntimeState, is_string_tagged};

/// Returns the base pointer of the variables array.
/// Called once at the start of each JIT-compiled function; subsequent
/// variable accesses use direct GEP+load/store from this pointer,
/// eliminating per-access function call overhead.
#[unsafe(no_mangle)]
pub extern "C" fn rt_get_variables_ptr(state: *mut RuntimeState) -> *mut f64 {
    unsafe { (*state).variables.as_mut_ptr() }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_get_var(state: *mut RuntimeState, index: u64) -> f64 {
    unsafe { *(&(*state).variables).get_unchecked(index as usize) }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_set_var(state: *mut RuntimeState, index: u64, value: f64) {
    unsafe {
        *(&mut (*state).variables).get_unchecked_mut(index as usize) = value;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_change_var(state: *mut RuntimeState, index: u64, delta: f64) {
    unsafe {
        let state = &mut *state;
        let current = *state.variables.get_unchecked(index as usize);
        let base = if is_string_tagged(current) {
            state.value_to_number(current)
        } else {
            current
        };
        let d = if is_string_tagged(delta) {
            state.value_to_number(delta)
        } else {
            delta
        };
        *state.variables.get_unchecked_mut(index as usize) = base + d;
    }
}
