## Algorithmic TODOs
- [x] O(n^2) gravity simulation
- [x] O(n^2) isothermal SPH pressure forces
- [ ] Adiabatic SPH
- [ ] Well-separated tree construction
- [ ] O(n*k) SPH using neighbors from tree
- [ ] O(n) gravity using first-order almost-FMM
- [ ] Higher-order FMM gravity
- [ ] Initial conditions generator

## Program feature TODOs
- [ ] Command line simulation configuration
- [ ] Capture video file
- [x] 2D projection camera
- [ ] 3D camera
- [ ] Attitude indicator
- [ ] Camera movement control
- [ ] Pause/unpause simulation
- [ ] Toggleable: view different fields

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