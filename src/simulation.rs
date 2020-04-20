use crate::constants::{COUNT, NEIGHBORS, TWO_PI};
use crate::neighbors::*;
use crate::particle::Particle;
use crate::vector::{Float, Vector3};
use rand;
use rand::Rng;
use rayon::prelude::*;

pub struct Simulation {
    particles: Vec<Particle>,
}

impl Simulation {
    pub fn new(count: usize, max_radius: Float, speed: Float) -> Self {
        let mut rng = rand::thread_rng();
        let mut particle = || {
            let pos_unit = Vector3::from_polar(
                TWO_PI * rng.gen::<Float>(),
                TWO_PI * rng.gen::<Float>(),
                rng.gen::<Float>(),
            );
            let temperature = 0.1;
            let molar_mass = 0.001;
            Particle::new(
                pos_unit * max_radius,
                pos_unit.rotated(Vector3::unit_x(), TWO_PI / 4.0) * speed,
            )
        };
        let particles: Vec<Particle> = (0..count).map(|_| particle()).collect();
        return Simulation { particles };
    }
    pub fn step(&mut self, dt: Float) {
        let neighbor_indices: Vec<Vec<usize>> = nearest_neighbors(
            self.particles.iter().map(|particle| particle.pos).collect(),
            NEIGHBORS,
        );
        let surrounding: Vec<Vec<Particle>> = neighbor_indices
            .into_par_iter()
            .map(|indices| {
                indices
                    .into_iter()
                    .map(|index| self.particles[index])
                    .collect()
            })
            .collect();
        let gravity_field: Vec<Vector3> = self.particles.iter().map(|p| p.pos).collect();
        self.particles
            .par_iter_mut()
            .zip(0..COUNT)
            .for_each(|(particle, index)| particle.update(dt, &surrounding[index], &gravity_field));
    }

    pub fn particles(&self) -> &[Particle] {
        &self.particles
    }
}
