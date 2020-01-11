use crate::particle::Particle;
use crate::vector::{Float, Vector3};

pub trait Camera {
    fn view(&self, buffer: &mut Vec<u32>, width: usize, height: usize, particles: &[Particle]);
}

pub struct FlatProjectionCamera {
    pos: Vector3,
    horizontal: Vector3,
    vertical: Vector3,
}

impl FlatProjectionCamera {
    pub fn new(
        pos: Vector3,
        width: Float,
        height: Float,
        angle_x: Float,
        angle_y: Float,
        angle_z: Float,
    ) -> Self {
        FlatProjectionCamera {
            pos,
            horizontal: width * Vector3::unit_y().rotated_around_xyz(angle_x, angle_y, angle_z),
            vertical: height * Vector3::unit_z().rotated_around_xyz(angle_x, angle_y, angle_z),
        }
    }
}

impl Camera for FlatProjectionCamera {
    fn view(&self, buffer: &mut Vec<u32>, width: usize, height: usize, particles: &[Particle]) {
        assert_eq!(buffer.len(), width * height);
        for pixel in buffer.iter_mut() {
            *pixel = 0x000000;
        }

        for particle in particles {
            let pos = particle.pos - self.pos;
            let x = 0.5 + pos.dot(&self.horizontal) / self.horizontal.norm_squared();
            let y = 0.5 + pos.dot(&self.vertical) / self.vertical.norm_squared();
            if 0.0 <= x && x < 1.0 && 0.0 <= y && y < 1.0 {
                let x = (width as Float * x) as usize;
                let y = (height as Float * y) as usize;
                buffer[y * width + x] = 0xFFFFFF;
            }
        }
    }
}
