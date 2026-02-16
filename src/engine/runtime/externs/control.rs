//! Control flow runtime functions (wait, clone, stop, repeat)
//!
//! `control_wait` follows the scratch-vm Sequencer model:
//!   1. First call: record deadline, yield (STATUS_YIELD equivalent)
//!   2. Each subsequent tick: re-check deadline, yield if not yet reached
//!   3. Deadline reached: return → execution proceeds to next block
//! No thread::sleep is used; the cooperative scheduler drives timing.

use super::super::RuntimeState;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::time::{Duration, Instant};

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

        // scratch-vm: duration <= 0 still yields once (requestRedraw + yield)
        if seconds <= 0.0 {
            control_wait_yield_once(state);
            return;
        }

        let deadline = Instant::now() + Duration::from_secs_f64(seconds);

        // scratch-vm model: yield, then on each re-entry check the timer.
        // In fiber mode the scheduler resumes us each tick.
        // In non-fiber mode we busy-wait with short sleeps (since there's
        // no cooperative scheduler to return to).
        if state.active_fiber_control.is_some() {
            // Fiber mode: yield to scheduler each tick, re-check on resume.
            loop {
                control_wait_yield_once(state);
                if Instant::now() >= deadline {
                    break;
                }
                if state
                    .stop_requested
                    .as_ref()
                    .is_some_and(|stop| stop.load(Ordering::Relaxed))
                {
                    break;
                }
            }
        } else {
            // Non-fiber (legacy / test) mode: busy-wait until deadline.
            while Instant::now() < deadline {
                let remaining = deadline.saturating_duration_since(Instant::now());
                if remaining.is_zero() {
                    break;
                }
                std::thread::sleep(remaining.min(Duration::from_millis(1)));
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
}

/// Yield exactly one scheduler tick, matching scratch-vm's `util.yield()`
/// (STATUS_YIELD).  In fiber mode this yields to the scheduler; in legacy
/// mode it sleeps until the next frame.
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
pub extern "C" fn rt_repeat_count(state: *mut RuntimeState, value: f64) -> u64 {
    unsafe {
        let result = if let Some(state) = state.as_ref() {
            let numeric_value = state.value_to_number(value);
            if !numeric_value.is_finite() {
                0
            } else if numeric_value <= 0.0 {
                0
            } else {
                let rounded = numeric_value.round();
                if rounded > (u64::MAX as f64) {
                    u64::MAX
                } else {
                    rounded as u64
                }
            }
        } else {
            // Fallback when state is not available
            if !value.is_finite() {
                0
            } else if value <= 0.0 {
                0
            } else {
                let rounded = value.round();
                if rounded > (u64::MAX as f64) {
                    u64::MAX
                } else {
                    rounded as u64
                }
            }
        };

        if let Some(state) = state.as_ref() {
            if state.debug_mode {
                eprintln!(
                    "[debug][repeat] repeat count: input={}, result={}",
                    value, result
                );
            }
        }

        result
    }
}
