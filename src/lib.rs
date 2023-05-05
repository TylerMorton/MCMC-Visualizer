use rand_distr::{Distribution, Normal};

pub fn normal_distribution_density(mean: f64, stddev: f64, state: f64) -> f64 {
    const E: f64 = 2.71828;
    let pi_const = (2.0 * 3.1415926 as f64).sqrt();
    let denom: f64 = 1.0 / (stddev * pi_const);
    let power = ((state - mean) / stddev).powf(2.0) * -0.5;
    E.powf(power) * denom
}

pub fn gaussian_sample() -> f64 {
    let normal = Normal::new(2.0, 3.0).unwrap();
    let v = normal.sample(&mut rand::thread_rng());
    v
}

pub fn acceptance(current: f64, candidate: f64) -> f64 {
    let mean = 4.0;
    let dev = 0.4;

    let f_a = normal_distribution_density(mean, dev, candidate);
    let f_b = normal_distribution_density(mean, dev, current);

    f64::min(f_a / f_b, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dist_density_works() {
        // given norm dist of (mean:4, std. dev.: 0.4)
        let sample = normal_distribution_density(4.0, 0.4, 3.4);
        assert_eq!((sample * 100.0).floor() / 100.0, 0.32);
    }
}
