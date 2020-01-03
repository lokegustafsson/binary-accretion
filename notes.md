## Features
- [x] Barebones O(n^2) gravity simulation
- [ ] Save video file

## Components
- `Vector3` is used fot all linear algebra
- `Particle`: Base-bones struct for a particle used in simulation. All forces acting upon it are defined in specific simulations.
- `Simulation`: Updates the particles according to a step function. Defines the forces in the simulation
- `Camera`: Reads the simulation and writes a representation to a screen buffer
- `Observer`: Provides stats on the simulation and prints these to the terminal

## Simulation methods
- Simple quadratic-gravity-only: Single-threaded and multithreaded through rayon
- Gravity can be made O(nlogn) and O(n) through FMM
- SPH will provide pressure-forces and viscosity
- This can be integrated by using the tree code to find the closest neighbors
- Should we care about magnetism and/or radiative transfer?