//! Core runtime control functions (warp, block counting)

use super::super::RuntimeState;

/// Called from JIT loop guards. The JIT emits a call with a single
/// `RuntimeState*` argument and expects the runtime to bump the block
/// counter by the guard interval (amortized count). Historically this
/// function had a second `increment` parameter, but the LLVM side
/// declares it with only a single pointer parameter — keeping both
/// sides mismatched caused garbage values to be interpreted as the
/// increment. Use a fixed increment that matches the JIT's
/// `LOOP_GUARD_INTERVAL` (256) to maintain the previous behaviour.
#[unsafe(no_mangle)]
pub extern "C" fn rt_count_executed_block(state: *mut RuntimeState) {
    unsafe {
        // Must match JIT's LOOP_GUARD_INTERVAL constant.
        let step: u64 = 256;
        (*state).executed_block_count = (*state).executed_block_count.saturating_add(step);
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
