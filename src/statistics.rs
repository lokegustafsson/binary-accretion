use crate::constants::{
    COUNT, GAS_CONSTANT, GRAVITATIONAL_CONSTANT, MASS, MOLAR_MASS, PARTICLE_MASS,
};
use crate::vector::{Float, Vector3};
use rayon::prelude::*;

pub fn observe_movement(velocities: &[Vector3]) -> Vector3 {
    velocities.iter().map(|&v| v).sum()
}

pub fn observe_thermal_energy(energies: &[Float]) -> Float {
    energies.iter().map(|&e| e).sum()
}

pub fn observe_kinetic_energy(velocities: &[Vector3]) -> Float {
    velocities.iter().map(|&v| v.norm_squared()).sum::<Float>() * PARTICLE_MASS / 2.0
}

pub fn observe_potential_energy(positions: &[Vector3]) -> Float {
    (0..COUNT)
        .into_par_iter()
        .map(|i| {
            let mut energy = 0.0;
            for j in 0..COUNT {
                if i != j {
                    energy += (positions[i] - positions[j]).norm().powi(-1);
                }
            }
            energy
        })
        .sum::<Float>()
        * -0.5
        * GRAVITATIONAL_CONSTANT
        * PARTICLE_MASS.powi(2)
}

pub fn observe_average_temperature(energies: &[Float]) -> Float {
    let energy = observe_thermal_energy(energies);
    energy * MOLAR_MASS / 1.5 / GAS_CONSTANT / MASS
}

pub fn observe_average_pressure(energies: &[Float], densities: &[Float]) -> Float {
    (0..COUNT)
        .into_par_iter()
        .map(|i| energies[i] * densities[i])
        .sum::<Float>()
        / MASS
        / 1.5
}
