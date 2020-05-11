use crate::constants::TWO_PI;
use crate::vector::{Float, Vector3};

pub struct Camera {
    pos: Vector3,
    horizontal: Vector3,
    vertical: Vector3,
    horizontal_length: Float,
    vertical_length: Float,
}

impl Camera {
    pub fn new(pos: Vector3, width: Float, height: Float) -> Self {
        Camera {
            pos,
            horizontal: Vector3::unit_x(),
            vertical: Vector3::unit_y(),
            horizontal_length: width,
            vertical_length: height,
        }
    }

    pub fn view(&self, buffer: &mut Vec<u32>, width: usize, height: usize, positions: &[Vector3]) {
        assert_eq!(buffer.len(), width * height);
        for pixel in buffer.iter_mut() {
            *pixel = 0x000000;
        }

        for &position in positions {
            let pos = position - self.pos;
            let x = 0.5 + pos.dot(self.horizontal) / self.horizontal_length;
            let y = 0.5 + pos.dot(self.vertical) / self.vertical_length;
            if 0.0 <= x && x < 1.0 && 0.0 <= y && y < 1.0 {
                let x = (width as Float * x) as usize;
                let y = (height as Float * y) as usize;
                buffer[y * width + x] = 0xFFFFFF;
            }
        }
    }

    pub fn take_input(
        &mut self,
        current_fps: f64,
        left: bool,
        right: bool,
        up: bool,
        down: bool,
        clockwise: bool,
        counterclockwise: bool,
        zoom_in: bool,
        zoom_out: bool,
    ) {
        let angle = TWO_PI / current_fps / 3.0;
        if left != right {
            self.horizontal
                .rotate(self.vertical, if right { angle } else { -angle });
        }
        if up != down {
            self.vertical
                .rotate(self.horizontal, if up { angle } else { -angle });
        }
        if clockwise != counterclockwise {
            let depth = self.horizontal.cross(self.vertical);
            self.horizontal
                .rotate(depth, if counterclockwise { angle } else { -angle });
            self.vertical
                .rotate(depth, if counterclockwise { angle } else { -angle });
        }
        // Gram-Schmidt, making the vectors orthogonal again after floating point imprecision
        self.vertical = self.vertical - self.horizontal.dot(self.vertical) * self.horizontal;
        self.vertical = self.vertical.normalized();
        self.horizontal = self.horizontal.normalized();

        if zoom_in != zoom_out {
            self.horizontal_length *= if zoom_out { 1.05 } else { 1.0 / 1.05 };
            self.vertical_length *= if zoom_out { 1.05 } else { 1.0 / 1.05 };
        }
    }
}
