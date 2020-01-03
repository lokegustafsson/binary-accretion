use crate::particle::Particle;
use crate::vector::{Float, Vector3};
use rand;
use rand::Rng;
use rayon::prelude::*;

const TWO_PI: Float = 2.0 * std::f64::consts::PI as Float;

pub trait Simulation {
    fn step(&mut self);

    fn particles(&self) -> Vec<&Particle>;
}

pub struct SimpleSimulation {
    particles: Vec<Particle>,
}

impl SimpleSimulation {
    pub fn new(count: usize, max_radius: Float, speed: Float, max_mass: Float) -> Self {
        let mut rng = rand::thread_rng();
        let mut particle = || {
            let pos_unit = Vector3::from_polar(
                TWO_PI * rng.gen::<Float>(),
                TWO_PI * rng.gen::<Float>(),
                rng.gen::<Float>(),
            );
            Particle {
                pos: pos_unit * max_radius,
                vel: pos_unit.rotated_around_x(TWO_PI / 4.0) * speed,
                mass: max_mass * rng.gen::<Float>(),
            }
        };

        SimpleSimulation {
            particles: (0..count).map(|_| particle()).collect(),
        }
    }
}

impl Simulation for SimpleSimulation {
    fn step(&mut self) {
        const DELTA_T: Float = 1.0;

        let mut new_particles: Vec<Particle> = self.particles.clone();

        new_particles
            .par_iter_mut()
            .for_each(|particle| particle.update(&self.particles, DELTA_T));
        self.particles = new_particles;
    }

    fn particles(&self) -> Vec<&Particle> {
        self.particles.iter().collect()
    }
}
