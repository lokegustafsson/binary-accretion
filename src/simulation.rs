use crate::constants::TWO_PI;
use crate::particle::Particle;
use crate::vector::{Float, Vector3};
use rand;
use rand::Rng;
use rayon::prelude::*;

pub struct Simulation {
    particles: Vec<Particle>,
    densities: Vec<Float>,
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
                pos_unit.rotated_around_x(TWO_PI / 4.0) * speed,
                total_mass / count as Float,
                temperature,
                molar_mass,
            )
        };
        let particles: Vec<Particle> = (0..count).map(|_| particle()).collect();
        let densities = compute_densities(&particles);
        Simulation {
            particles,
            densities,
        }
    }
    pub fn step(&mut self, dt: Float) {
        let particles_with_densities: Vec<(&Particle, Float)> =
            self.particles.iter().zip(self.densities.clone()).collect();

        // O(n^2) time derivatives of particle properties
        let time_derivatives: Vec<(Vector3, Float)> = particles_with_densities
            .par_iter()
            .map(|particle_with_density| {
                let &(particle, density) = particle_with_density;
                // Acceleration from pressure forces and gravity
                let acceleration = total_gravitational_acceleration(particle, &self.particles)
                    + particle.pressure_acceleration(density, &particles_with_densities);

                // Compute dh/dt = -h / dim / density * d\rho/dt = -h / dim / density * (- density * div_vel) = div_vel * h / dim
                let delta_smoothing_length =
                    particle.div_vel(density, &self.particles) * particle.smoothing_length / 3.0;

                (acceleration, delta_smoothing_length)
            })
            .collect();
        // O(n) particle updates
        self.particles
            .iter_mut()
            .zip(time_derivatives)
            .for_each(|(particle, deltas)| particle.update_properties(deltas, dt));
        // O(n^2) densities update
        self.densities = compute_densities(&self.particles);
    }

    pub fn particles(&self) -> &Vec<Particle> {
        &self.particles
    }

    pub fn densities(&self) -> &Vec<Float> {
        &self.densities
    }
}

fn compute_densities(particles: &[Particle]) -> Vec<Float> {
    particles
        .par_iter()
        .map(|particle| particle.density(particles))
        .collect()
}

fn total_gravitational_acceleration(target: &Particle, particles: &[Particle]) -> Vector3 {
    let mut acceleration = Vector3::zero();
    for particle in particles {
        if (particle.pos - target.pos).norm_squared() > 1.0 {
            acceleration += target.gravitational_acceleration_from(particle);
        }
    }
    acceleration
}
