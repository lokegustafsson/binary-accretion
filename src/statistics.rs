use crate::constants::{GAS_CONSTANT, MASS, MOLAR_MASS, PARTICLE_MASS};
use crate::vector::{Float, Vector3};

pub fn observe_movement(velocities: &[Vector3]) -> Vector3 {
    velocities.iter().map(|&v| v).sum()
}
pub fn observe_thermal_energy(energies: &[Float]) -> Float {
    energies.iter().map(|&e| e).sum()
}
pub fn observe_kinetic_energy(velocities: &[Vector3]) -> Float {
    velocities.iter().map(|&v| v.norm_squared()).sum::<Float>() * PARTICLE_MASS / 2.0
}
pub fn observe_average_temperature(energies: &[Float]) -> Float {
    let energy = observe_thermal_energy(energies);
    energy * MOLAR_MASS / 1.5 / GAS_CONSTANT / MASS
}
pub fn observe_overall_density(densities: &[Float]) -> Float {
    let count = densities.len() as Float;
    densities
        .into_iter()
        .map(|x| 1.0 / x)
        .sum::<Float>()
        .powi(-1)
        * count
}
