use rand_distr::{Distribution, Normal};
use std::f64::consts::{E, PI};

pub fn distribution_density(mean: f64, stddev: f64, state: f64) -> f64 {
    if stddev == 0.0 {
        panic!("Divide by zero for stddev");
    }

    let pi_const = (2.0 * PI).sqrt();
    let denom: f64 = 1.0 / (stddev * pi_const);
    let power = ((state - mean) / stddev).powf(2.0) * -0.5;
    E.powf(power) * denom
}

pub fn sample() -> f64 {
    let normal = Normal::new(2.0, 0.2).unwrap();
    normal.sample(&mut rand::thread_rng())
}

pub fn sample_custom(mean: f64, dev: f64) -> f64 {
    let normal = Normal::new(mean, dev).unwrap();
    normal.sample(&mut rand::thread_rng())
}
