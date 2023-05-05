use mcmc::{acceptance, gaussian_sample};
use rand::Rng;

pub fn main() {
    /* CAUTION project assumes
     * the PDF provided is proportional to the Gaussian Distribution!!
     * WIP for other symmetrical distributions. Not going to worry about
     * assymetrical.
     */
    let mut position = gaussian_sample();
    let samples: i32 = 20;

    let mut rng = rand::thread_rng();
    for _ in 0..samples {
        println!("current position: {}", position);
        loop {
            // hardcoded to gaussian for now (these should be custom eventually)
            let candidate = gaussian_sample();
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
