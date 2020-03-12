use crate::constants::PI;
use crate::particle::Particle;
use crate::vector::{Float, Vector3};

pub struct Statistics {
    formatted_movement: String,
    formatted_ideal_radius: String,
}

impl Statistics {
    pub fn new() -> Self {
        Statistics {
            formatted_movement: String::from(""),
            formatted_ideal_radius: String::from(""),
        }
    }

    pub fn observe_movement(&mut self, particles: &[Particle]) -> Option<String> {
        let total_momentum = particles
            .iter()
            .map(|p| p.vel * p.mass)
            .sum::<Vector3>()
            .norm();
        let total_mass: Float = particles.iter().map(|p| p.mass).sum();

        let res = format!("{:.2e}", total_momentum / total_mass);
        if res != self.formatted_movement {
            self.formatted_movement = res.clone();
            Some(res)
        } else {
            None
        }
    }

    pub fn observe_idealised_radius(&mut self, particles: &[Particle]) -> Option<String> {
        let total_volume: Float = particles
            .iter()
            .map(|particle| particle.mass / particle.density)
            .sum();

        let res = format!("{:.0e}", (total_volume * 3.0 / 4.0 / PI).powf(1.0 / 3.0));
        if res != self.formatted_ideal_radius {
            self.formatted_ideal_radius = res.clone();
            Some(res)
        } else {
            None
        }
    }
}
