#[derive(Debug, Clone, Copy)]
pub struct MathHostAddresses {
    pub abs: usize,
    pub floor: usize,
    pub ceil: usize,
    pub sqrt: usize,
    pub sin: usize,
    pub cos: usize,
    pub tan: usize,
    pub asin: usize,
    pub acos: usize,
    pub atan: usize,
    pub loge: usize,
    pub log10: usize,
    pub exp: usize,
    pub pow10: usize,
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct SpriteState {
    pub sprite_x: f64,
    pub sprite_y: f64,
    pub sprite_rotate: f64,
}

pub fn math_host_addresses() -> MathHostAddresses {
    MathHostAddresses {
        abs: xyo_abs as *const () as usize,
        floor: xyo_floor as *const () as usize,
        ceil: xyo_ceil as *const () as usize,
        sqrt: xyo_sqrt as *const () as usize,
        sin: xyo_sin as *const () as usize,
        cos: xyo_cos as *const () as usize,
        tan: xyo_tan as *const () as usize,
        asin: xyo_asin as *const () as usize,
        acos: xyo_acos as *const () as usize,
        atan: xyo_atan as *const () as usize,
        loge: xyo_loge as *const () as usize,
        log10: xyo_log10 as *const () as usize,
        exp: xyo_exp as *const () as usize,
        pow10: xyo_pow10 as *const () as usize,
    }
}

extern "C" fn xyo_abs(value: f64) -> f64 {
    value.abs()
}

extern "C" fn xyo_floor(value: f64) -> f64 {
    value.floor()
}

extern "C" fn xyo_ceil(value: f64) -> f64 {
    value.ceil()
}

extern "C" fn xyo_sqrt(value: f64) -> f64 {
    value.sqrt()
}

extern "C" fn xyo_sin(value: f64) -> f64 {
    value.sin()
}

extern "C" fn xyo_cos(value: f64) -> f64 {
    value.cos()
}

extern "C" fn xyo_tan(value: f64) -> f64 {
    value.tan()
}

extern "C" fn xyo_asin(value: f64) -> f64 {
    value.asin()
}

extern "C" fn xyo_acos(value: f64) -> f64 {
    value.acos()
}

extern "C" fn xyo_atan(value: f64) -> f64 {
    value.atan()
}

extern "C" fn xyo_loge(value: f64) -> f64 {
    value.ln()
}

extern "C" fn xyo_log10(value: f64) -> f64 {
    value.log10()
}

extern "C" fn xyo_exp(value: f64) -> f64 {
    value.exp()
}

extern "C" fn xyo_pow10(value: f64) -> f64 {
    10.0_f64.powf(value)
}
