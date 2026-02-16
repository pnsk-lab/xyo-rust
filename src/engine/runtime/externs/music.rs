//! Music block runtime functions

use super::super::RuntimeState;

#[unsafe(no_mangle)]
pub extern "C" fn rt_music_set_tempo(state: *mut RuntimeState, tempo: f64) {
    unsafe {
        let Some(state) = state.as_mut() else {
            return;
        };
        state.tempo_bpm = state.value_to_number(tempo);
    }
}
