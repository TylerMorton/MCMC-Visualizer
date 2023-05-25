use crate::gaussian;

use rand::Rng;

pub struct Candidate {
    prob_accept: f64,
    pub value: f64,
}

pub fn acceptance(mean: f64, dev: f64, current: f64, candidate: f64) -> f64 {
    let mean = mean;
    let dev = dev;

    let f_a = gaussian::distribution_density(mean, dev, candidate);
    let f_b = gaussian::distribution_density(mean, dev, current);
    println!("densities: cur {} can {}", f_b, f_a);
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
    let mean = 2.0;
    let dev = 0.2;

    let mut rng = rand::thread_rng();
    for _ in 0..samples {
        println!("current position: {}", position);
        loop {
            // hardcoded to gaussian for now (these should be custom eventually)
            let candidate = gaussian::sample();
            let prob_accept = acceptance(mean, dev, position, candidate);
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

pub fn derive_candidate(mean: f64, dev: f64, position: f64) -> Candidate {
    let value = gaussian::sample_custom(position, 0.2 /*dev*/);
    println!("current {} candidate {}", position, value);
    let prob_accept = acceptance(mean, dev, position, value);
    Candidate { value, prob_accept }
}

pub fn metropolis_state(mean: f64, position: f64, candidate: Candidate) -> f64 {
    /* CAUTION project assumes
     * the PDF provided is proportional to the Gaussian Distribution!!
     * WIP for other symmetrical distributions. Not going to worry about
     * assymetrical.
     */
    let mut rng = rand::thread_rng();
    println!("mean within metro state is: {}", mean);
    // hardcoded to gaussian for now (these should be custom eventually)
    let gen: f64 = rng.gen();
    println!("prob {} gen {}", candidate.prob_accept, gen);
    if candidate.prob_accept > gen {
        println!("candidate: {:.2} was accepted", candidate.value);
        return candidate.value;
    }
    position
}
