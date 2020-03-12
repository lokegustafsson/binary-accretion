## Physical TODOs
- [x] Stable smoothing length calculation
- [x] Statistics: conservation of momentum
- [ ] Statistics: conservation of angular momentum
- [ ] Statistics: conservation of energy
- [ ] Electron degeneracy pressure
- [ ] Artificial viscosity
- [ ] Adiabatic: gas temperature
- [ ] Adiabatic: dissipation in shocks
- [ ] Initial conditions

## Algorithmic TODOs
- [x] O(n^2) gravity simulation
- [x] O(n^2) isothermal SPH pressure forces
- [ ] Well-separated tree construction
- [ ] O(n*k) SPH using neighbors from tree
- [ ] O(n) gravity using first-order almost-FMM
- [ ] Higher-order FMM gravity

## Program feature TODOs
- [x] 2D projection camera
- [x] Control attitude
- [ ] Zoom and scale indicator
- [ ] Attitude indicator
- [ ] 3D camera
- [ ] Command line simulation configuration
- [ ] Capture video file
- [ ] Pause/unpause simulation
- [ ] Toggleable: view different fields

## Complexity overview
- `O(n)` statistics and rendering
- `O(n log n)` tree construction (perhaps sped up through tree reuse?)
- `O(n)` gravity (also dependent on precision)
- `O(kn)` SPH: pressure, viscosity, thermal energy