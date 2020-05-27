mod camera;
mod constants;
mod neighbors;
mod particle;
mod simulation;
mod statistics;
mod vector;

use crate::camera::Camera;
use crate::constants::{DELTA_T, HEIGHT, RADIUS, WIDTH, YEAR};
use crate::simulation::Simulation;
use crate::vector::{Float, Vector3};
use minifb::{Key, Window, WindowOptions};

use std::cmp;
use std::time::Instant;

pub fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut simulation = Simulation::new();
    let mut camera = Camera::new(
        Vector3::zero(),
        WIDTH as Float * 4.0 * RADIUS / cmp::min(WIDTH, HEIGHT) as Float,
        HEIGHT as Float * 4.0 * RADIUS / cmp::min(WIDTH, HEIGHT) as Float,
    );

    let mut window =
        Window::new("Test", WIDTH, HEIGHT, WindowOptions::default()).unwrap_or_else(|e| {
            panic!("{}", e);
        });

    window.limit_update_rate(None);

    let mut last_time = Instant::now();
    let mut seconds_per_tick = 1.0/30.0;
    let mut tick = 0;

    println!("UPS Years    Move    Energy    Poten   Kinetic  Temp Pressure");

    while window.is_open() {
        // Simulation step and display
        let densities = simulation.step();
        camera.take_input(
            (1.0 / seconds_per_tick) as f64,
            window.is_key_down(Key::A),
            window.is_key_down(Key::D),
            window.is_key_down(Key::W),
            window.is_key_down(Key::S),
            window.is_key_down(Key::Q),
            window.is_key_down(Key::E),
            window.is_key_down(Key::X),
            window.is_key_down(Key::Z),
        );
        camera.view(&mut buffer, WIDTH, HEIGHT, simulation.positions());
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();

        // Statistics
        let now = Instant::now();
        seconds_per_tick =
            0.9 * seconds_per_tick + 0.1 * now.duration_since(last_time).as_secs_f64();
        last_time = now;

        if tick % 100 == 0 {
            let movement = statistics::observe_movement(simulation.velocities());
            let kinetic_energy = statistics::observe_kinetic_energy(simulation.velocities());
            let thermal_energy = statistics::observe_thermal_energy(simulation.thermal_energies());
            let potential_energy = statistics::observe_potential_energy(simulation.positions());
            let temp = statistics::observe_average_temperature(simulation.thermal_energies());
            let pressure =
                statistics::observe_average_pressure(simulation.thermal_energies(), &*densities);

            println!(
                "{:2} {:7} {:8.1e} {:8.2e} {:8.2e} {:8.2e} {:5.2} {:8.1e}",
                seconds_per_tick.powi(-1) as u32,
                (tick as f64 * DELTA_T / YEAR) as usize,
                movement.norm(),
                potential_energy + kinetic_energy + thermal_energy,
                potential_energy,
                kinetic_energy,
                temp,
                pressure,
            );
        }
        tick += 1;
    }
}
