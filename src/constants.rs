use crate::vector::Float;

// Program
pub const WIDTH: usize = 800;
pub const HEIGHT: usize = 800;

// Simulation
pub const COUNT: usize = 1000;
pub const NEIGHBORS: usize = 30;
pub const ENABLE_GRAVITY: bool = true;
pub const ENABLE_GAS_DYNAMICS: bool = true;
const LIGHT_YEAR: Float = 1e16;
pub const RADIUS: Float = 1.0 * LIGHT_YEAR;
pub const SPEED: Float = 5000.0;
const SOLAR_MASS: Float = 2e30;
pub const MASS: Float = 500.0 * SOLAR_MASS;
pub const YEAR: Float = 3e7;
pub const DELTA_T: Float = 50.0 * YEAR;
pub const VELOCITY_AVERAGING: Float = 0.1;
pub const BACKGROUND_PRESSURE: Float = 1e-11;

// Properties constant between all particles
const INITIAL_TEMPERATURE: Float = 10.0;
pub const MOLAR_MASS: Float = 0.001;
pub const PARTICLE_MASS: Float = MASS / COUNT as Float;
pub const INITIAL_THERMAL_ENERGY: Float =
    PARTICLE_MASS / MOLAR_MASS * GAS_CONSTANT * INITIAL_TEMPERATURE * 1.5;

// Mathematical
pub const PI: Float = std::f64::consts::PI as Float;
pub const TWO_PI: Float = 2.0 * std::f64::consts::PI as Float;

// Physical

// Newtons law of gravity: F = G*m1*m2*r/|r|^3
// Units: m^3 * kg^-1 * s^-2
pub const GRAVITATIONAL_CONSTANT: Float = 6.674e-11;

// Ideal gas law: PV = nRT
// Units J * K^-1 * mol^-1
pub const GAS_CONSTANT: Float = 8.314;
