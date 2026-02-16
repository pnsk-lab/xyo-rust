//! Data (list) operation runtime functions

use super::super::RuntimeState;

#[unsafe(no_mangle)]
pub extern "C" fn rt_data_item_of_list(
    state: *mut RuntimeState,
    list_index: u64,
    index: f64,
) -> f64 {
    unsafe {
        let state = &mut *state;
        state.list_item(list_index as usize, index)
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_data_add_to_list(state: *mut RuntimeState, list_index: u64, item: f64) {
    unsafe {
        let state = &mut *state;
        state.list_add_item(list_index as usize, item);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_data_length_of_list(state: *mut RuntimeState, list_index: u64) -> f64 {
    unsafe { (*state).list_length(list_index as usize) }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_data_delete_all_of_list(state: *mut RuntimeState, list_index: u64) {
    unsafe {
        (&mut *state).list_delete_all(list_index as usize);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_data_delete_of_list(state: *mut RuntimeState, list_index: u64, index: f64) {
    unsafe {
        (&mut *state).list_delete_item(list_index as usize, index);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_data_replace_item_of_list(
    state: *mut RuntimeState,
    list_index: u64,
    index: f64,
    item: f64,
) {
    unsafe {
        (&mut *state).list_replace_item(list_index as usize, index, item);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_data_item_num_of_list(
    state: *mut RuntimeState,
    list_index: u64,
    item: f64,
) -> f64 {
    unsafe { (*state).list_item_num(list_index as usize, item) }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_data_list_contains_item(
    state: *mut RuntimeState,
    list_index: u64,
    item: f64,
) -> f64 {
    unsafe {
        if (*state).list_contains_item(list_index as usize, item) {
            1.0
        } else {
            0.0
        }
    }
}
