//! Event (broadcast) runtime functions

use super::super::RuntimeState;
use std::sync::Arc;
use std::sync::atomic::Ordering;

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
            .dump_vars_requested
            .as_ref()
            .is_some_and(|dump| dump.swap(false, Ordering::Relaxed))
        {
            state_ref.dump_variables();
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
