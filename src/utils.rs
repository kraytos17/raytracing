use lazy_static::lazy_static;
use std::{f64::consts::PI, sync::Mutex};

pub fn _degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

lazy_static! {
    static ref RNG: Mutex<fastrand::Rng> = Mutex::new(fastrand::Rng::new());
}

pub fn random_double() -> f64 {
    let mut rng = RNG.lock().unwrap();
    rng.f64()
}

pub fn random_double_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
}
