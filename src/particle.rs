use crate::vector::{Float, Vector3};

// Newtons law of gravity: F = G*m1*m2*r/|r|^3
// In units: m^3 * kg^-1 * s^-2
const GRAVITATIONAL_CONSTANT: Float = 6.674e-11;

#[derive(Copy, Clone)]
pub struct Particle {
    pub pos: Vector3,
    pub vel: Vector3,
    pub mass: Float,
}

impl Particle {
    pub fn update(&mut self, surrounding: &Vec<Particle>, dt: Float) {
        let mut acceleration = Vector3::zero();
        for particle in surrounding {
            // vector from self to particle
            let v = particle.pos - self.pos;
            if v.norm_squared() < 1.0 {
                continue;
            }

            acceleration += particle.mass * GRAVITATIONAL_CONSTANT * v / v.norm().powi(3);
        }

        self.pos += self.vel * dt + acceleration * dt * dt / 2.0;
        self.vel += acceleration * dt;
    }
}
