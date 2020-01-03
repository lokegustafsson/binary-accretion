mod observe;
mod particle;
mod simulation;
mod vector;
mod camera;

use crate::observe::Observer;
use crate::simulation::{SimpleSimulation, Simulation};
use crate::vector::{Float, Vector3};
use crate::camera::{Camera, FlatProjectionCamera};
use minifb::{Window, WindowOptions};
use std::cmp;
use std::time::Duration;

const WIDTH: usize = 800;
const HEIGHT: usize = 800;

const COUNT: usize = 2000;
const RADIUS: Float = 100.0;
const SPEED: Float = 1.8;
const MASS: Float = 5000000000.0;

pub fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut simulation = SimpleSimulation::new(COUNT, RADIUS, SPEED, MASS);
    let mut observer = Observer::new();
    let camera = FlatProjectionCamera::new(
        Vector3::zero(),
        WIDTH as Float * 4.0 * RADIUS / cmp::min(WIDTH, HEIGHT) as Float,
        HEIGHT as Float * 4.0 * RADIUS / cmp::min(WIDTH, HEIGHT) as Float,
        0.0,
        0.0,
        0.0,
    );

    let mut window =
        Window::new("Test", WIDTH, HEIGHT, WindowOptions::default()).unwrap_or_else(|e| {
            panic!("{}", e);
        });

    window.limit_update_rate(Some(Duration::from_millis(8)));
    while window.is_open() {
        simulation.step();
        camera.view(&mut buffer, WIDTH, HEIGHT, simulation.particles(), MASS);
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
        observer.observe(simulation.particles());
    }
}
