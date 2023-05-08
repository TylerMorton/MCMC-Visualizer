pub mod bellcurve;
pub mod gaussian;
pub mod metropolis;
pub mod stage;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dist_density_works() {
        // given norm dist of (mean:4, std. dev.: 0.4)
        let sample = gaussian::distribution_density(4.0, 0.4, 3.4);
        assert_eq!((sample * 100.0).floor() / 100.0, 0.32);
    }
}
