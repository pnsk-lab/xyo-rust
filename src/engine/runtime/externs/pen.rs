//! Pen block runtime functions

use super::super::{RuntimeState, decode_string_id};

// Helper functions for pen color manipulation (defined in mod.rs)
use super::super::{
    decimal_to_rgb, hsv_to_rgb, hue_to_rgb, parse_hex_color_with_alpha, rgb_to_hsv,
};

#[unsafe(no_mangle)]
pub extern "C" fn rt_pen_down(state: *mut RuntimeState) {
    unsafe {
        if let Some(state) = state.as_mut() {
            state.pen_down = true;
            state.draw_disc(state.sprite_x, state.sprite_y);
            state.live_canvas_dirty = true;
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_pen_up(state: *mut RuntimeState) {
    unsafe {
        if let Some(state) = state.as_mut() {
            state.pen_down = false;
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_pen_clear(state: *mut RuntimeState) {
    unsafe {
        if let Some(state) = state.as_mut() {
            state.clear_canvas();
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_pen_set_size(state: *mut RuntimeState, size: f64) {
    unsafe {
        if let Some(state) = state.as_mut() {
            let numeric = state.value_to_number(size);
            if !numeric.is_finite() {
                return;
            }
            state.pen_size = numeric.max(1.0);
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_pen_set_color(state: *mut RuntimeState, color: f64) {
    unsafe {
        let Some(state) = state.as_mut() else {
            return;
        };

        if let Some(index) = decode_string_id(color) {
            let raw = state.strings.get(index).cloned().unwrap_or_default();
            // '#rrggbb', '#aarrggbb', '0xrrggbb', or '0xaarrggbb'
            if let Some((rgb, alpha)) = parse_hex_color_with_alpha(&raw) {
                state.pen_color = rgb;
                if let Some(alpha) = alpha {
                    state.pen_alpha = alpha;
                }
            } else {
                // Otherwise interpret the string as a number and extract RGB
                // from the decimal value, matching scratch-vm's
                // Color.decimalToRgb(Cast.toNumber(value)).
                let trimmed = raw.trim();
                let number = if let Some(hex) = trimmed
                    .strip_prefix("0x")
                    .or_else(|| trimmed.strip_prefix("0X"))
                {
                    u64::from_str_radix(hex, 16)
                        .ok()
                        .map(|v| v as f64)
                        .unwrap_or(0.0)
                } else {
                    trimmed.parse::<f64>().unwrap_or(0.0)
                };
                state.pen_color = decimal_to_rgb(number);
            }
        } else {
            // Numeric value → extract RGB components from the integer,
            // matching scratch-vm's Color.decimalToRgb().
            state.pen_color = decimal_to_rgb(state.value_to_number(color));
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_pen_stamp(state: *mut RuntimeState) {
    unsafe {
        let Some(state) = state.as_mut() else {
            return;
        };
        state.stamp_active_sprite_to_pen_layer();
        state.live_canvas_dirty = true;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_pen_set_color_param(state: *mut RuntimeState, param_code: u64, value: f64) {
    unsafe {
        let Some(state) = state.as_mut() else {
            return;
        };
        let numeric = state.value_to_number(value);
        if !numeric.is_finite() {
            return;
        }
        match param_code {
            // color
            0 => {
                state.pen_color = hue_to_rgb(numeric);
            }
            // saturation
            1 => {
                let (h, _s, v) = rgb_to_hsv(state.pen_color);
                let saturation = (numeric / 100.0).clamp(0.0, 1.0);
                state.pen_color = hsv_to_rgb(h, saturation, v);
            }
            // brightness
            2 => {
                let (h, s, _v) = rgb_to_hsv(state.pen_color);
                let brightness = (numeric / 100.0).clamp(0.0, 1.0);
                state.pen_color = hsv_to_rgb(h, s, brightness);
            }
            // transparency
            3 => {
                state.pen_alpha = (1.0 - (numeric / 100.0)).clamp(0.0, 1.0);
            }
            _ => {}
        }
    }
}
