use crate::gaussian;

use iced::Point;
use rand::Rng;

pub struct Candidate {
    prob_accept: (f64, f64),
    pub position: Point,
}

impl Candidate {
    pub fn new(prob_accept: (f64, f64), position: Point) -> Self {
        Candidate {
            prob_accept,
            position,
        }
    }
}

pub fn acceptance(mean: f64, dev: f64, current: f64, candidate: f64) -> f64 {
    let f_a = gaussian::distribution_density(mean, dev, candidate);
    let f_b = gaussian::distribution_density(mean, dev, current);
    f64::min(f_a / f_b, 1.0)
}

pub fn acceptance_2d(
    mean: (f64, f64),
    dev: (f64, f64),
    current: Point,
    candidate: Point,
) -> (f64, f64) {
    let f_a_x = gaussian::distribution_density(mean.0, dev.0, candidate.x as f64);
    let f_a_y = gaussian::distribution_density(mean.1, dev.1, candidate.y as f64);
    let f_b_x = gaussian::distribution_density(mean.0, dev.0, current.x as f64);
    let f_b_y = gaussian::distribution_density(mean.1, dev.1, current.y as f64);
    //println!("densities: cur {} can {}", f_b, f_a);
    let x = f64::min(f_a_x / f_b_x, 1.0);
    let y = f64::min(f_a_y / f_b_y, 1.0);
    (x, y)
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

// pub fn derive_candidate(mean: f64, dev: f64, position: f64) -> Candidate {
//     let candidate_position = Point {
//         x: gaussian::sample_custom(position, 0.2) as f32,
//         y: 5.0,
//     };
//     let prob_accept = acceptance(mean, dev, position, candidate_position.x as f64);
//     Candidate {
//         position: candidate_position,
//         prob_accept,
//     }
// }

pub fn derive_candidate_2d(mean: (f64, f64), dev: (f64, f64), position: Point) -> Candidate {
    let candidate_position = Point {
        x: gaussian::sample_custom(position.x as f64, 0.2) as f32,
        y: gaussian::sample_custom(position.y as f64, 0.2) as f32,
    };
    let prob_accept = acceptance_2d(mean, dev, position, candidate_position);
    Candidate {
        position: candidate_position,
        prob_accept,
    }
}

// pub fn metropolis_state(mean: f64, position: f64, candidate: &Candidate) -> f64 {
//     /* CAUTION project assumes
//      * the PDF provided is proportional to the Gaussian Distribution!!
//      * WIP for other symmetrical distributions. Not going to worry about
//      * assymetrical.
//      */
//     let mut rng = rand::thread_rng();
//     println!("mean within metro state is: {}", mean);
//     // hardcoded to gaussian for now (these should be custom eventually)
//     let gen: f64 = rng.gen();
//     println!("prob {} gen {}", candidate.prob_accept, gen);
//     if candidate.prob_accept > gen {
//         println!("candidate: {:.2} was accepted", candidate.position.x);
//         return candidate.position.x as f64;
//     }
//     position
// }

pub fn metropolis_state_2d(position: Point, candidate: &Candidate) -> Point {
    let mut rng = rand::thread_rng();
    let gen: f64 = rng.gen();
    let mut pos = position;
    if candidate.prob_accept.0 > gen {
        pos.x = candidate.position.x;
    }
    if candidate.prob_accept.1 > gen {
        pos.y = candidate.position.y;
    }
    pos
}
