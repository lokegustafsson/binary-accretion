mod camera;
mod constants;
mod statistics;
mod particle;
mod simulation;
mod vector;

use crate::camera::{Camera, FlatProjectionCamera};
use crate::constants::{COUNT, DELTA_T, HEIGHT, MASS, RADIUS, SPEED, WIDTH};
use statistics::Statistics;
use crate::simulation::Simulation;
use crate::vector::{Float, Vector3};
use minifb::{Window, WindowOptions};
use std::cmp;
use std::time::{Duration, Instant};

pub fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut simulation = Simulation::new(COUNT, RADIUS, SPEED, MASS);
    let mut stats = Statistics::new();
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

    window.limit_update_rate(None);

    let mut elapsed_next_print = Duration::from_secs(1);
    let time_start = Instant::now();
    let mut last_time = Instant::now();
    let mut seconds_per_tick = 0.0;

    while window.is_open() {
        // Simulation step and display
        simulation.step(DELTA_T);
        camera.view(&mut buffer, WIDTH, HEIGHT, simulation.particles());
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();

        // Observables
        if let Some(movement) = stats.observe_movement(simulation.particles()) {
            println!("Movement: {}", movement);
        }
        if let Some(radius) =
            stats.observe_idealised_radius(simulation.particles(), simulation.densities())
        {
            println!(
                "Idealised radius: {} (Initial actual: {:e})",
                radius, RADIUS
            );
        }

        // Looping meta stuff
        {
            let now = Instant::now();
            seconds_per_tick =
                (9.0 * seconds_per_tick + now.duration_since(last_time).as_secs_f64()) / 10.0;
            if now.duration_since(time_start) > elapsed_next_print {
                elapsed_next_print *= 2;
                println!("Ticks per second: {}", seconds_per_tick.powi(-1) as u32);
            }
            last_time = now;
        }
    }
}
