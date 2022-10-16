use crate::constants::{
    GRAVITATIONAL_CONSTANT, NEIGHBORS, PARTICLE_MASS, PI, SMOOTHING_DIST_FACTOR, FLOAT_ZERO,
};
use crate::vector::{Float, Vector3};

pub fn smoothing_length(self_pos: Vector3, surround_pos: &[Vector3]) -> Float {
    surround_pos
        .iter()
        .map(|&other_pos| (other_pos - self_pos).norm_squared())
        .fold(FLOAT_ZERO, |a, b| a.max(b))
        .sqrt()
        / SMOOTHING_DIST_FACTOR
}

pub fn density(
    self_pos: Vector3,
    self_smooth: Float,
    surround_pos: &[Vector3],
    surround_smooth: &[Float],
) -> Float {
    surround_pos
        .iter()
        .zip(surround_smooth.iter())
        .map(|(&pos, &smooth)| PARTICLE_MASS * kernel(self_pos, self_smooth, pos, smooth))
        .sum()
}

pub fn gravitational_acceleration(self_pos: Vector3, other_pos: &[Vector3]) -> Vector3 {
    other_pos
        .iter()
        .filter(|&&pos| self_pos != pos)
        .fold(Vector3::zero(), |acc, &pos| {
            acc + gravitational_acceleration_from(self_pos, pos)
        })
}

// The acceleration due to pressure according to the euler momentum equation
pub fn pressure_acceleration(
    self_pos: Vector3,
    self_energy: Float,
    self_smooth: Float,
    self_density: Float,
    surround_pos: &[Vector3],
    surround_energy: &[Float],
    surround_smooth: &[Float],
    surround_density: &[Float],
) -> Vector3 {
    -1.0 * grad_pressure(
        self_pos,
        self_energy,
        self_smooth,
        self_density,
        surround_pos,
        surround_energy,
        surround_smooth,
        surround_density,
    ) / self_density
}

pub fn time_derivative_thermal_energy(
    self_pos: Vector3,
    self_vel: Vector3,
    self_energy: Float,
    self_smooth: Float,
    self_density: Float,
    surround_pos: &[Vector3],
    surround_vel: &[Vector3],
    surround_energy: &[Float],
    surround_smooth: &[Float],
    surround_density: &[Float],
) -> Float {
    (0..NEIGHBORS)
        .into_iter()
        .map(|i| {
            (pressure(self_energy, self_density) / self_density.powi(2)
                + pressure(surround_energy[i], surround_density[i]) / surround_density[i].powi(2))
                * grad_kernel(self_pos, self_smooth, surround_pos[i], surround_smooth[i])
                    .dot(self_vel - surround_vel[i])
        })
        .sum::<Float>()
        * PARTICLE_MASS.powi(2)
        / 2.0
}

pub fn neighborhood_velocity(
    self_pos: Vector3,
    self_vel: Vector3,
    self_smooth: Float,
    self_density: Float,
    surround_pos: &[Vector3],
    surround_vel: &[Vector3],
    surround_smooth: &[Float],
    surround_density: &[Float],
) -> Vector3 {
    (0..NEIGHBORS)
        .into_iter()
        .map(|i| {
            (surround_vel[i] - self_vel) / (self_density + surround_density[i])
                * kernel(self_pos, self_smooth, surround_pos[i], surround_smooth[i])
        })
        .sum::<Vector3>()
        * PARTICLE_MASS
        * 2.0
}

pub fn pressure(energy: Float, density: Float) -> Float {
    energy * density / PARTICLE_MASS / 1.5
}

fn gravitational_acceleration_from(self_pos: Vector3, other_pos: Vector3) -> Vector3 {
    let v = other_pos - self_pos;
    PARTICLE_MASS * GRAVITATIONAL_CONSTANT * v / v.norm().powi(3)
}

// The gradient of pressure at this particle
fn grad_pressure(
    self_pos: Vector3,
    self_energy: Float,
    self_smooth: Float,
    self_density: Float,
    surround_pos: &[Vector3],
    surround_energy: &[Float],
    surround_smooth: &[Float],
    surround_density: &[Float],
) -> Vector3 {
    (0..NEIGHBORS)
        .into_iter()
        .map(|i| {
            (pressure(self_energy, self_density) / self_density.powi(2)
                + pressure(surround_energy[i], surround_density[i]) / surround_density[i].powi(2))
                * grad_kernel(self_pos, self_smooth, surround_pos[i], surround_smooth[i])
        })
        .sum::<Vector3>()
        * self_density
        * PARTICLE_MASS
}

// The gaussian kernel
fn kernel(self_pos: Vector3, self_smooth: Float, other_pos: Vector3, other_smooth: Float) -> Float {
    let h = self_smooth.min(other_smooth);
    let d2 = (self_pos - other_pos).norm_squared();
    if d2 > (h * SMOOTHING_DIST_FACTOR).powi(2) {
        0.0
    } else {
        PI.powf(-1.5) * (-d2 / (h * h)).exp() / h.powi(3)
    }
}

// The gradient of the gaussian kernel
fn grad_kernel(
    self_pos: Vector3,
    self_smooth: Float,
    other_pos: Vector3,
    other_smooth: Float,
) -> Vector3 {
    let h = self_smooth.min(other_smooth);
    let d2 = (self_pos - other_pos).norm_squared();
    if d2 > (h * SMOOTHING_DIST_FACTOR).powi(2) {
        Vector3::zero()
    } else {
        (other_pos - self_pos) * 2.0 * PI.powf(-1.5) * (-d2 / (h * h)).exp() / h.powi(5)
    }
}
