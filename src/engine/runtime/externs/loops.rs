//! Loop control and random runtime functions

use super::super::{next_random_unit, RuntimeState};

#[unsafe(no_mangle)]
pub extern "C" fn rt_forever_should_continue(state: *mut RuntimeState) -> bool {
    unsafe {
        let Some(state) = state.as_mut() else {
            return false;
        };
        let in_warp = state.warp_depth > 0;
        // In warp mode, don't consume step budget – the loop should only
        // exit via explicit "stop this script" / "stop all", matching how
        // the official Scratch VM handles warp-mode forever loops.
        // Outside warp, consume budget so that infinite loops in no-gui
        // mode still terminate.
        loop_should_continue(state, true, in_warp, !in_warp, !in_warp)
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_forever_should_continue_warp(state: *mut RuntimeState) -> bool {
    unsafe {
        let Some(state) = state.as_mut() else {
            return false;
        };
        // Legacy static-warp path; kept for backward compatibility.
        // In warp mode, don't consume step budget (matches dynamic path).
        loop_should_continue(state, true, true, false, false)
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_loop_should_continue(state: *mut RuntimeState) -> bool {
    unsafe {
        let Some(state) = state.as_mut() else {
            return false;
        };
        let in_warp = state.warp_depth > 0;
        // In warp mode, skip canvas sync and don't consume step budget.
        // Outside warp, respect `relax_procedure_loop_budget`.
        let consume_step_budget = !in_warp && !state.relax_procedure_loop_budget;
        loop_should_continue(state, true, in_warp, !in_warp, consume_step_budget)
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_loop_should_continue_warp(state: *mut RuntimeState) -> bool {
    unsafe {
        let Some(state) = state.as_mut() else {
            return false;
        };
        // Legacy static-warp path; kept for backward compatibility.
        loop_should_continue(state, true, true, false, false)
    }
}

fn loop_should_continue(
    state: &mut RuntimeState,
    pace_frames: bool,
    in_warp: bool,
    present_live_canvas: bool,
    consume_step_budget: bool,
) -> bool {
    use std::sync::atomic::Ordering;
    
    if state
        .dump_vars_requested
        .as_ref()
        .is_some_and(|dump| dump.swap(false, Ordering::Relaxed))
    {
        state.dump_variables();
    }
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
    if pace_frames && !first_guard_this_resume {
        let should_yield = if in_warp {
            state.should_yield_for_warp_time()
        } else {
            state.should_yield_for_work_time()
        };
        if should_yield {
            state.wait_for_next_frame();
        }
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
