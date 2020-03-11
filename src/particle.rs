use crate::constants::{GAS_CONSTANT, GRAVITATIONAL_CONSTANT, PI};
use crate::vector::{Float, Vector3};

#[derive(Copy, Clone)]
pub struct Particle {
    pub pos: Vector3,
    pub vel: Vector3,
    pub mass: Float,

    // Pressure divided by density, depends on temperature and some other properties according to the ideal gas law
    pub pressure_factor: Float,

    pub smoothing_length: Float,
    pub density: Float,
}

impl Particle {
    pub fn new(
        pos: Vector3,
        vel: Vector3,
        mass: Float,
        temperature: Float,
        molar_mass: Float,
    ) -> Particle {
        Particle {
            pos,
            vel,
            mass,
            // Pressure factor definition (using ideal gas law):
            //  pressure_factor = pressure / density = molarity * GAS_CONSTANT * temperature / mass = GAS_CONSTANT * temperature / molar_mass
            pressure_factor: GAS_CONSTANT * temperature / molar_mass,
            smoothing_length: pos.norm(),
            density: std::f64::NAN,
        }
    }

    pub fn update_properties(&mut self, deltas: (Vector3, Float), dt: Float) {
        let (acceleration, delta_smoothing_length) = deltas;
        self.pos += self.vel * dt + acceleration * dt * dt / 2.0;
        self.vel += acceleration * dt;
        self.smoothing_length += delta_smoothing_length * dt;
    }

    pub fn gravitational_acceleration_from(&self, other: &Particle) -> Vector3 {
        let v = other.pos - self.pos;
        other.mass * GRAVITATIONAL_CONSTANT * v / v.norm().powi(3)
    }

    // The acceleration due to pressure according to the euler momentum equation
    pub fn pressure_acceleration(&self, surrounding: &[Particle]) -> Vector3 {
        -1.0 * self.grad_pressure(surrounding) / self.density
    }

    // The divergence of velocity at this particle
    pub fn div_vel(&self, surrounding: &[Particle]) -> Float {
        surrounding
            .iter()
            .map(|other| other.mass * (other.pos - self.pos).dot(self.grad_kernel(other)))
            .sum::<Float>()
            / self.density
    }

    // This particle's density
    pub fn density(&self, surrounding: &[Particle]) -> Float {
        surrounding
            .iter()
            .map(|other| other.mass * self.kernel(other))
            .sum()
    }

    // The gradient of pressure at this particle
    fn grad_pressure(&self, surrounding: &[Particle]) -> Vector3 {
        surrounding
            .iter()
            .map(|other| {
                other.mass
                    * (other.pressure_factor * other.density - self.pressure_factor * self.density)
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
