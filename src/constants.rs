use crate::vector::Float;

// Program
pub const WIDTH: usize = 800;
pub const HEIGHT: usize = 800;
pub const SECONDS_PER_REVOLUTION: Float = 2.0;

// Simulation
pub const COUNT: usize = 500;
pub const NEIGHBORS: usize = 50;
pub const RADIUS: Float = 1e16; // About one light year
pub const SPEED: Float = 100.0;
pub const MASS: Float = 2e30; // About one solar mass
const YEAR: Float = 3e7;
pub const DELTA_T: Float = 5000.0 * YEAR;

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
