use crate::constants::{COUNT, NEIGHBORS, TWO_PI};
use crate::neighbors::*;
use crate::particle::Particle;
use crate::vector::{Float, Vector3};
use rand;
use rand::Rng;
use rayon::prelude::*;

pub struct Simulation {
    particles: Box<[Particle; COUNT]>,
}

impl Simulation {
    pub fn new(max_radius: Float, speed: Float) -> Self {
        let mut rng = rand::thread_rng();
        let vec: Vec<Particle> = (0..COUNT)
            .map(|_| {
                let pos_unit = Vector3::from_polar(
                    TWO_PI * rng.gen::<Float>(),
                    TWO_PI * rng.gen::<Float>(),
                    rng.gen::<Float>(),
                );
                Particle::new(
                    pos_unit * max_radius,
                    pos_unit.rotated(Vector3::unit_x(), TWO_PI / 4.0) * speed,
                )
            })
            .collect();

        Simulation {
            particles: unsafe {
                Box::from_raw(Box::into_raw(vec.into_boxed_slice()) as *mut [Particle; COUNT])
            },
        }
    }
    pub fn step(&mut self, dt: Float) {
        let particle_positions: Box<[Vector3; COUNT]> = {
            let vec: Vec<Vector3> = (0..COUNT).map(|i| self.particles[i].pos).collect();
            unsafe { Box::from_raw(Box::into_raw(vec.into_boxed_slice()) as *mut [Vector3; COUNT]) }
        };
        let neighbor_indices = nearest_neighbors(&particle_positions);
        let neighbors: Box<[[Particle; NEIGHBORS]; COUNT]> = {
            let mut vec = Vec::with_capacity(NEIGHBORS * COUNT);
            for i in 0..COUNT {
                for j in 0..NEIGHBORS {
                    let index = neighbor_indices[i][j];
                    vec.push(self.particles[index]);
                }
            }
            unsafe {
                Box::from_raw(
                    Box::into_raw(vec.into_boxed_slice()) as *mut [[Particle; NEIGHBORS]; COUNT]
                )
            }
        };
        let gravity_field = particle_positions;
        self.particles
            .par_iter_mut()
            .zip(0..COUNT)
            .for_each(|(particle, index)| particle.update(dt, &neighbors[index], &*gravity_field));
    }

    pub fn particles(&self) -> &[Particle] {
        &*self.particles
    }
}
