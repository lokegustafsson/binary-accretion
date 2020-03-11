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
        sim.recompute_density();
        return sim;
    }
    pub fn step(&mut self, dt: Float) {
        // O(n^2) time derivatives of particle properties
        let time_derivatives: Vec<(Vector3, Float)> = self
            .particles
            .par_iter()
            .map(|particle| {
                // Acceleration from pressure forces and gravity
                let acceleration = self.total_gravitational_acceleration(particle)
                    + particle.pressure_acceleration(&self.particles);

                // Compute dh/dt = -h / dim / density * d\rho/dt = -h / dim / density * (- density * div_vel) = div_vel * h / dim
                let delta_smoothing_length =
                    particle.div_vel(&self.particles) * particle.smoothing_length / 3.0;

                (acceleration, delta_smoothing_length)
            })
            .collect();
        // O(n) particle updates
        self.particles
            .iter_mut()
            .zip(time_derivatives)
            .for_each(|(particle, deltas)| particle.update_properties(deltas, dt));
        // O(n^2) densities update
        self.recompute_density();
    }

    pub fn particles(&self) -> &Vec<Particle> {
        &self.particles
    }

    fn recompute_density(&mut self) {
        let densities: Vec<Float> = self
            .particles
            .iter()
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
