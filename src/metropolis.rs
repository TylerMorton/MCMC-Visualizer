use crate::gaussian;

use rand::Rng;

pub fn acceptance(current: f64, candidate: f64) -> f64 {
    let mean = 10.0;
    let dev = 3.0;

    let f_a = gaussian::distribution_density(mean, dev, candidate);
    let f_b = gaussian::distribution_density(mean, dev, current);

    f64::min(f_a / f_b, 1.0)
}

pub fn metropolis() {
    /* CAUTION project assumes
     * the PDF provided is proportional to the Gaussian Distribution!!
     * WIP for other symmetrical distributions. Not going to worry about
     * assymetrical.
     */
    let mut position = gaussian::sample();
    let samples: i32 = 20;

    let mut rng = rand::thread_rng();
    for _ in 0..samples {
        println!("current position: {}", position);
        loop {
            // hardcoded to gaussian for now (these should be custom eventually)
            let candidate = gaussian::sample();
            let prob_accept = acceptance(position, candidate);
            if prob_accept > rng.gen() {
                println!("candidate: {:.2} was accepted", candidate);
                position = candidate;
                break;
            } else {
                println!("rejected");
            }
        }
    }
}

pub fn metropolis_state(dev: f64, position: f64) -> f64 {
    /* CAUTION project assumes
     * the PDF provided is proportional to the Gaussian Distribution!!
     * WIP for other symmetrical distributions. Not going to worry about
     * assymetrical.
     */
    let mut rng = rand::thread_rng();
        println!("current position: {}", position);
        loop {
            // hardcoded to gaussian for now (these should be custom eventually)
            let candidate = gaussian::sample_custom(position, dev);
            let prob_accept = acceptance(position, candidate);
            if prob_accept > rng.gen() {
                println!("candidate: {:.2} was accepted", candidate);
                return candidate;
            } else {
                println!("rejected");
            }
        }
}


