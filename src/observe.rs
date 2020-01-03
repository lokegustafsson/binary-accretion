use crate::particle::Particle;
use crate::vector::{Float, Vector3};

pub struct Observer {
    max_abs_total_movement_yet: Float,
}

impl Observer {
    pub fn new() -> Self {
        Observer {
            max_abs_total_movement_yet: 0.0,
        }
    }

    pub fn observe(&mut self, particles: Vec<&Particle>) {
        let momentum = particles
            .iter()
            .map(|p| p.vel * p.mass)
            .fold(Vector3::zero(), |acc, x| acc + x)
            .norm();
        let movement = momentum / particles.iter().fold(0.0, |acc, p| acc + p.mass);

        if movement > self.max_abs_total_movement_yet {
            self.max_abs_total_movement_yet = movement;
            println!("New max movement: {}", movement);
        }
    }
}
