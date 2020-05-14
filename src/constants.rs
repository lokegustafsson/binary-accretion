use crate::vector::Float;

// Program
pub const WIDTH: usize = 800;
pub const HEIGHT: usize = 800;

// Simulation
pub const COUNT: usize = 2000;
pub const NEIGHBORS: usize = 30;
pub const RADIUS: Float = 1.0 * LIGHT_YEAR;
pub const SPEED: Float = 5000.0;
pub const DELTA_T: Float = 50.0 * YEAR;

pub const ENABLE_GRAVITY: bool = true;
pub const MASS: Float = 500.0 * SOLAR_MASS;

pub const ENABLE_GAS_DYNAMICS: bool = true;
pub const VELOCITY_AVERAGING: Float = 0.1;
pub const BACKGROUND_PRESSURE: Float = 1e-11;
const INITIAL_TEMPERATURE: Float = 10.0;

// Convenience
const LIGHT_YEAR: Float = 1e16;
const SOLAR_MASS: Float = 2e30;
pub const YEAR: Float = 3e7;

// Derived constants
pub const MOLAR_MASS: Float = 0.001;
pub const PARTICLE_MASS: Float = MASS / COUNT as Float;
pub const INITIAL_THERMAL_ENERGY: Float =
    PARTICLE_MASS / MOLAR_MASS * GAS_CONSTANT * INITIAL_TEMPERATURE * 1.5;

// Mathematical
pub const PI: Float = std::f64::consts::PI as Float;
pub const TWO_PI: Float = 2.0 * std::f64::consts::PI as Float;

// Physical
pub const GRAVITATIONAL_CONSTANT: Float = 6.674e-11;
pub const GAS_CONSTANT: Float = 8.314;
