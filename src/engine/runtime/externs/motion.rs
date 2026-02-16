//! Motion block runtime functions

use super::super::RuntimeState;

#[unsafe(no_mangle)]
pub extern "C" fn rt_motion_move_steps(state: *mut RuntimeState, steps: f64) {
    unsafe {
        let state = &mut *state;
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

#[unsafe(no_mangle)]
pub extern "C" fn rt_motion_set_direction(state: *mut RuntimeState, direction: f64) {
    unsafe {
        let state = &mut *state;
        let direction = state.value_to_number(direction);
        if !direction.is_finite() {
            return;
        }
        state.direction_deg = direction;
        state.live_canvas_dirty = true;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_motion_change_x(state: *mut RuntimeState, dx: f64) {
    unsafe {
        let state = &mut *state;
        let dx = state.value_to_number(dx);
        if !dx.is_finite() {
            return;
        }
        state.move_sprite_to(state.sprite_x + dx, state.sprite_y);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_motion_change_y(state: *mut RuntimeState, dy: f64) {
    unsafe {
        let state = &mut *state;
        let dy = state.value_to_number(dy);
        if !dy.is_finite() {
            return;
        }
        state.move_sprite_to(state.sprite_x, state.sprite_y + dy);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_motion_set_x(state: *mut RuntimeState, x: f64) {
    unsafe {
        let state = &mut *state;
        let x = state.value_to_number(x);
        if !x.is_finite() {
            return;
        }
        state.move_sprite_to(x, state.sprite_y);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_motion_set_y(state: *mut RuntimeState, y: f64) {
    unsafe {
        let state = &mut *state;
        let y = state.value_to_number(y);
        if !y.is_finite() {
            return;
        }
        state.move_sprite_to(state.sprite_x, y);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_motion_goto_xy(state: *mut RuntimeState, x: f64, y: f64) {
    unsafe {
        let state = &mut *state;
        let x = state.value_to_number(x);
        let y = state.value_to_number(y);
        if !x.is_finite() || !y.is_finite() {
            return;
        }
        state.move_sprite_to(x, y);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_motion_x_position(state: *mut RuntimeState) -> f64 {
    unsafe { (*state).sprite_x }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_motion_y_position(state: *mut RuntimeState) -> f64 {
    unsafe { (*state).sprite_y }
}
