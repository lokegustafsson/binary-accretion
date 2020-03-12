use crate::constants::TWO_PI;
use crate::particle::Particle;
use crate::vector::{Float, Vector3};
use rand;
use rand::Rng;
use rayon::prelude::*;

pub struct Simulation {
    particles: Vec<Particle>,
}

impl Simulation {
    pub fn new(count: usize, max_radius: Float, speed: Float, total_mass: Float) -> Self {
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
                total_mass / count as Float,
                temperature,
                molar_mass,
            )
        };
        let particles: Vec<Particle> = (0..count).map(|_| particle()).collect();
        let mut sim = Simulation { particles };
        sim.recompute_densities_and_smoothing_lengths();
        return sim;
    }
    pub fn step(&mut self, dt: Float) {
        // O(n^2) time derivatives of particle properties
        let acceleration: Vec<Vector3> = self
            .particles
            .par_iter()
            .map(|particle| {
                // Acceleration from pressure forces and gravity
                self.total_gravitational_acceleration(particle)
                    + particle.pressure_acceleration(&self.particles)
            })
            .collect();
        // O(n) particle updates
        self.particles
            .iter_mut()
            .zip(acceleration)
            .for_each(|(particle, accel)| particle.update_properties(accel, dt));
        // O(n^2) densities update
        self.recompute_densities_and_smoothing_lengths();
    }

    pub fn particles(&self) -> &[Particle] {
        &self.particles
    }

    fn recompute_densities_and_smoothing_lengths(&mut self) {
        let lengths: Vec<Float> = self
            .particles
            .par_iter()
            .map(|p| p.smoothing_length(&self.particles))
            .collect();
        for i in 0..self.particles.len() {
            self.particles[i].smoothing_length = lengths[i];
        }
        // Computing densities depends on having smoothing lengths
        let densities: Vec<Float> = self
            .particles
            .par_iter()
            .map(|p| p.density(&self.particles))
            .collect();
        for i in 0..self.particles.len() {
            self.particles[i].density = densities[i];
        }
    }

    fn total_gravitational_acceleration(&self, target: &Particle) -> Vector3 {
        self.particles()
            .iter()
            .filter(|p| (target.pos - p.pos).norm_squared() > 1.0)
            .fold(Vector3::zero(), |acc, p| {
                acc + target.gravitational_acceleration_from(p)
            })
    }
}
