use crate::vector::Float;

// Program
pub const WIDTH: usize = 800;
pub const HEIGHT: usize = 800;

// Computational
pub const COUNT: usize = 2000;
pub const DELTA_T: Float = 1000.0 * YEAR;
// General initial conditions
pub const RADIUS: Float = 10_000.0 * AU;
pub const ROTATIONAL_PERIOD: Float = 1e9 * YEAR;
pub const DENSITY_CURVE: DensityCurve = DensityCurve::Uniform;
// Gravity
pub const ENABLE_GRAVITY: bool = true;
pub const MASS: Float = 1.0 * SOLAR_MASS;
// SPH
pub const ENABLE_GAS_DYNAMICS: bool = true;
const INITIAL_TEMPERATURE: Float = 5.0;
pub const MOLAR_MASS: Float = 0.002016;
pub const NEIGHBORS: usize = 30;
pub const SMOOTHING_DIST_FACTOR: Float = 2.0;
pub const VELOCITY_AVERAGING: Float = 1.0;

// Convenience
#[allow(dead_code)]
const LIGHT_YEAR: Float = 1e16;
#[allow(dead_code)]
const AU: Float = 1.5e11;
#[allow(dead_code)]
const SOLAR_MASS: Float = 2e30;
pub const YEAR: Float = 3e7;
#[allow(dead_code)]
pub enum DensityCurve {
    Uniform,
    InverseLinear,
    InverseQuadratic,
}

// Derived constants
pub const PARTICLE_MASS: Float = MASS / COUNT as Float;
pub const INITIAL_THERMAL_ENERGY: Float =
    PARTICLE_MASS / MOLAR_MASS * GAS_CONSTANT * INITIAL_TEMPERATURE * 1.5;

// Mathematical
pub const PI: Float = std::f64::consts::PI as Float;
pub const TWO_PI: Float = 2.0 * std::f64::consts::PI as Float;

// Physical
pub const GRAVITATIONAL_CONSTANT: Float = 6.674e-11;
pub const GAS_CONSTANT: Float = 8.314;
