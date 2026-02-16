//! Core runtime control functions (warp, block counting)

use super::super::RuntimeState;

#[unsafe(no_mangle)]
pub extern "C" fn rt_count_executed_block(state: *mut RuntimeState) {
    unsafe {
        (*state).executed_block_count += 1;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_warp_enter(state: *mut RuntimeState) {
    unsafe {
        (*state).warp_depth += 1;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_warp_leave(state: *mut RuntimeState) {
    unsafe {
        (*state).warp_depth -= 1;
    }
}
