use std::f64;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * f64::consts::PI
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}
