use crate::constants::{
    COUNT, DELTA_T, ENABLE_GAS_DYNAMICS, ENABLE_GRAVITY, INITIAL_THERMAL_ENERGY, TWO_PI,
    VELOCITY_AVERAGING,
};
use crate::neighbors::*;
use crate::particle;
use crate::vector::{Float, Vector3};
use rand;
use rand::Rng;
use rayon::prelude::*;

pub struct Simulation {
    positions: Box<[Vector3; COUNT]>,
    velocities: Box<[Vector3; COUNT]>,
    thermal_energies: Box<[Float; COUNT]>,
}

impl Simulation {
    pub fn new(max_radius: Float, speed: Float) -> Self {
        let mut rng = rand::thread_rng();
        let mut positions = Vec::with_capacity(COUNT);
        let mut velocities = Vec::with_capacity(COUNT);
        for _ in 0..COUNT {
            let pos_unit = Vector3::from_polar(
                (2.0 * rng.gen::<Float>() - 1.0).acos(),
                TWO_PI * rng.gen::<Float>(),
                rng.gen::<Float>(),
            );
            positions.push(pos_unit * max_radius);
            velocities.push(
                (pos_unit - Vector3::unit_z() * Vector3::unit_z().dot(pos_unit))
                    .rotated(Vector3::unit_z(), TWO_PI / 4.0)
                    * speed,
            );
        }
        let average_movement: Vector3 =
            velocities.iter().map(|&p| p).sum::<Vector3>() / COUNT as f64;
        for v in velocities.iter_mut() {
            *v -= average_movement;
        }

        Simulation {
            positions: unsafe {
                Box::from_raw(Box::into_raw(positions.into_boxed_slice()) as *mut [Vector3; COUNT])
            },
            velocities: unsafe {
                Box::from_raw(Box::into_raw(velocities.into_boxed_slice()) as *mut [Vector3; COUNT])
            },
            thermal_energies: Box::new([INITIAL_THERMAL_ENERGY; COUNT]),
        }
    }
    pub fn step(&mut self) -> Vec<Float> {
        let neighbor_indices = nearest_neighbors(&self.positions);
        // Get smoothing lengths
        let surround_pos: Vec<Vec<Vector3>> = neighbor_indices
            .par_iter()
            .map(|indices| indices.iter().map(|&idx| self.positions[idx]).collect())
            .collect();
        let smoothing_lengths: Vec<Float> = (0..COUNT)
            .into_par_iter()
            .map(|i| particle::smoothing_length(self.positions[i], &*surround_pos[i]))
            .collect();
        // Get densities
        let surround_smooth: Vec<Vec<Float>> = neighbor_indices
            .par_iter()
            .map(|indices| indices.iter().map(|&idx| smoothing_lengths[idx]).collect())
            .collect();
        let densities: Vec<Float> = (0..COUNT)
            .into_par_iter()
            .map(|i| {
                particle::density(
                    self.positions[i],
                    smoothing_lengths[i],
                    &*surround_pos[i],
                    &*surround_smooth[i],
                )
            })
            .collect();
        // Update positions and velocities
        let deltas: Vec<(Vector3, Vector3, Float)> = (0..COUNT)
            .into_par_iter()
            .map(|i| {
                let surround_density: Vec<Float> = neighbor_indices[i]
                    .iter()
                    .map(|&idx| densities[idx])
                    .collect();
                let surround_energy: Vec<Float> = neighbor_indices[i]
                    .iter()
                    .map(|&idx| self.thermal_energies[idx])
                    .collect();
                let surround_vel: Vec<Vector3> = neighbor_indices[i]
                    .iter()
                    .map(|&idx| self.velocities[idx])
                    .collect();
                let accel = if ENABLE_GRAVITY {
                    particle::gravitational_acceleration(self.positions[i], &*self.positions)
                } else {
                    Vector3::zero()
                } + if ENABLE_GAS_DYNAMICS {
                    particle::pressure_acceleration(
                        self.positions[i],
                        self.thermal_energies[i],
                        smoothing_lengths[i],
                        densities[i],
                        &*surround_pos[i],
                        &*surround_energy,
                        &*surround_smooth[i],
                        &*surround_density,
                    )
                } else {
                    Vector3::zero()
                };
                let neigh_vel = particle::neighborhood_velocity(
                    self.positions[i],
                    self.velocities[i],
                    smoothing_lengths[i],
                    densities[i],
                    &*surround_pos[i],
                    &*surround_vel,
                    &*surround_smooth[i],
                    &*surround_density,
                );

                let delta_energy = particle::time_derivative_thermal_energy(
                    self.positions[i],
                    self.velocities[i],
                    self.thermal_energies[i],
                    smoothing_lengths[i],
                    densities[i],
                    &*surround_pos[i],
                    &*surround_vel,
                    &*surround_energy,
                    &*surround_smooth[i],
                    &*surround_density,
                );
                (
                    self.velocities[i] * DELTA_T
                        + accel * DELTA_T * DELTA_T / 2.0
                        + VELOCITY_AVERAGING * neigh_vel,
                    accel * DELTA_T,
                    delta_energy,
                )
            })
            .collect();
        for i in 0..COUNT {
            self.positions[i] += deltas[i].0;
            self.velocities[i] += deltas[i].1;
            self.thermal_energies[i] += deltas[i].2;
        }
        // Translate to place center of mass at origo
        let center_of_mass: Vector3 =
            self.positions.iter().map(|&p| p).sum::<Vector3>() / COUNT as f64;
        for p in self.positions.iter_mut() {
            *p -= center_of_mass;
        }
        // Assert valid floats
        assert!(self.positions.iter().all(|p| p.is_finite()));
        assert!(self.velocities.iter().all(|p| p.is_finite()));
        assert!(self.thermal_energies.iter().all(|p| p.is_sign_positive()));
        // Return densities to aid computing statistics
        densities
    }

    pub fn positions(&self) -> &[Vector3] {
        &*self.positions
    }

    pub fn velocities(&self) -> &[Vector3] {
        &*self.velocities
    }
    pub fn thermal_energies(&self) -> &[Float] {
        &*self.thermal_energies
    }
}
