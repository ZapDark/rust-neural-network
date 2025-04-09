use crate::constants::{SHIFT, SLOPE};

pub fn sign(n: f32) -> f32 {
    if n >= 0.0 {
        return 1.0;
    }
    return -1.0;
}

pub fn map_range(value: f32, from_min: f32, from_max: f32, to_min: f32, to_max: f32) -> f32 {
    to_min + (value - from_min) * (to_max - to_min) / (from_max - from_min)
}

pub fn f(x: f32) -> f32 {
    return SLOPE * x + SHIFT;
}
