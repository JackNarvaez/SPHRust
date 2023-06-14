// Initialize a set of particle to simulate a Toy Star system in 2D.

use std::{
    error::Error,
    process,
};

use sphfunctions;

fn main() -> Result<(), Box<dyn Error>> {
    let path = "./Data/initial_distribution/toy_star_2D.csv";
    let n:u32 = 100; // Number of Particles
    let m:f64 = 2.0; // Star's mass
    let r:f64 = 0.75; // Star's radius
    let rho:f64 = 1.0; // density
    let dm:f64 = m/n as f64; // Particle's mass
    let h = 0.04 /(n as f64 /1000.).sqrt(); // Smoothing length
    let (x0, y0) = (0.0, 0.0); // Circle's center
    if let Err(err) = sphfunctions::init_random_circle(path, n, r, dm, rho, h, x0, y0) {
        println!("{}", err);
        process::exit(1);
    }
    Ok(())
}