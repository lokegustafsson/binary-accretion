use crate::constants::{GRAVITATIONAL_CONSTANT, PARTICLE_MASS, PI, PRESSURE_FACTOR};
use crate::vector::{Float, Vector3};

#[derive(Copy, Clone)]
pub struct Particle {
    pub pos: Vector3,
    pub vel: Vector3,

    // Associated values that are recomputed each tick
    pub smoothing_length: Float,
    pub density: Float,
}

impl Particle {
    pub fn new(pos: Vector3, vel: Vector3) -> Particle {
        Particle {
            pos,
            vel,
            // Placeholders only for the first tick
            density: 1.0,
            smoothing_length: 1.0,
        }
    }

    pub fn update(&mut self, dt: Float, surrounding: &[Particle], gravity_field: &[Vector3]) {
        let acceleration = self.gravitational_acceleration(gravity_field)
            + self.pressure_acceleration(surrounding);
        self.smoothing_length = self.smoothing_length(surrounding);
        self.pos += self.vel * dt + acceleration * dt * dt / 2.0;
        self.vel += acceleration * dt;
    }

    fn smoothing_length(&self, surrounding: &[Particle]) -> Float {
        surrounding
            .iter()
            .map(|p| (p.pos - self.pos).norm())
            .sum::<Float>()
            / surrounding.len() as Float
    }

    fn gravitational_acceleration(&self, gravity_field: &[Vector3]) -> Vector3 {
        gravity_field
            .iter()
            .filter(|pos| (self.pos - **pos).norm_squared() > 1.0)
            .fold(Vector3::zero(), |acc, p| {
                acc + self.gravitational_acceleration_from(p)
            })
    }

    fn gravitational_acceleration_from(&self, other: Vector3) -> Vector3 {
        let v = other - self.pos;
        PARTICLE_MASS * GRAVITATIONAL_CONSTANT * v / v.norm().powi(3)
    }

    // The acceleration due to pressure according to the euler momentum equation
    fn pressure_acceleration(&self, surrounding: &[Particle]) -> Vector3 {
        -1.0 * self.grad_pressure(surrounding) / self.density
    }

    // The divergence of velocity at this particle
    fn div_vel(&self, surrounding: &[Particle]) -> Float {
        surrounding
            .iter()
            .map(|other| PARTICLE_MASS * (other.pos - self.pos).dot(self.grad_kernel(other)))
            .sum::<Float>()
            / self.density
    }

    // This particle's density
    fn density(&self, surrounding: &[Particle]) -> Float {
        surrounding
            .iter()
            .map(|other| PARTICLE_MASS * self.kernel(other))
            .sum()
    }

    // The gradient of pressure at this particle
    fn grad_pressure(&self, surrounding: &[Particle]) -> Vector3 {
        surrounding
            .iter()
            .map(|other| {
                PARTICLE_MASS
                    * PRESSURE_FACTOR
                    * (other.density - self.density)
                    * self.grad_kernel(other)
            })
            .sum::<Vector3>()
            / self.density
    }

    // The gaussian kernel
    fn kernel(&self, other: &Particle) -> Float {
        let h = (self.smoothing_length + other.smoothing_length) / 2.0;
        let d2 = (self.pos - other.pos).norm_squared();
        PI.powf(-0.5) * (-d2 / (h * h)).exp() / (h * h * h)
    }

    // The gradient of the gaussian kernel
    fn grad_kernel(&self, other: &Particle) -> Vector3 {
        let h = (self.smoothing_length + other.smoothing_length) / 2.0;
        (other.pos - self.pos) * 2.0 * self.kernel(other) / (h * h)
    }
}
