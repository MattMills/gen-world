# gen-world

A Rust library for procedurally generating realistic galaxies, solar systems, and planets with accurate astronomical distributions.

## Features

### Galactic Structure
- Realistic spiral arm generation with configurable parameters
- Population types (thin disk, thick disk, bulge, halo)
- Metallicity gradients based on galactic position
- Accurate stellar density distributions
- Deterministic, position-based generation for infinite exploration

### Star Generation
- Diverse star types with realistic frequencies:
  - Main sequence stars (Brown Dwarfs to Blue Supergiants)
  - Giant stars (Red Giants, Supergiants, Hypergiants)
  - Stellar remnants (White Dwarfs, Neutron Stars, Black Holes)
  - Exotic objects (Quark Stars, Pulsars, Magnetars)

### Planet Generation
- Realistic mass distributions based on orbital distance
- Proper orbital spacing using modified Titius-Bode law
- Temperature calculations based on stellar luminosity
- Atmospheric composition and greenhouse effects
- Habitability assessment based on multiple factors

### Small Body Generation
- Deterministic, position-based asteroid generation
- Realistic composition based on formation region
- Element distribution influenced by stellar history
- Support for sparse, infinite asteroid fields
- Different types (rocky, metallic, icy) with proper distributions

## Usage

### Galaxy Generation
```rust
use gen_world::{Galaxy, PopulationType};

// Create a new galaxy
let galaxy = Galaxy::new();

// Generate a region at specific coordinates (in parsecs)
let region = galaxy.generate_region(8000.0, 0.0, 0.0); // Solar neighborhood

println!("Population: {:?}", region.population);
println!("Metallicity: {:.2} [Fe/H]", region.metallicity);
println!("Star Density: {:.3} stars/pc³", region.star_density);

// Generate solar systems in this region
if let Some(system) = region.generate_solar_system(42) {
    println!("Star Type: {:?}", system.star.stellar_type);
    println!("Planets: {}", system.planets.len());
}
```

### Solar System Generation
```rust
use gen_world::{Generate, SolarSystem};

// Generate a random solar system
let system = SolarSystem::generate();

// Access star properties
println!("Star type: {:?}", system.star.stellar_type);
println!("Mass: {:.1} solar masses", system.star.physical.mass / 1.989e30);
println!("Luminosity: {:.1} solar", system.star.luminosity);

// Get habitable zone range in AU
let (inner, outer) = system.habitable_zone;
println!("Habitable zone: {:.2} AU to {:.2} AU", inner, outer);

// Examine planets
for planet in &system.planets {
    println!("Planet type: {:?}", planet.planet_type);
    println!("Mass: {:.1} Earth masses", planet.physical.mass / 5.972e24);
    println!("Habitable: {}", planet.habitable);
}
```

### Small Body Generation
```rust
use gen_world::{Position, SmallBodyGeneration};

// Generate asteroids in a specific region
let main_belt_center = Position { x: 2.7, y: 0.0, z: 0.0 };
let asteroids = system.generate_small_bodies(main_belt_center, 0.5, 10.0);

for asteroid in &asteroids {
    println!("Type: {:?}", asteroid.body_type);
    println!("Iron content: {:.1}%", asteroid.elements.iron * 100.0);
    println!("Water ice: {:.1}%", asteroid.elements.water_ice * 100.0);
}
```

## Statistical Properties

The generator produces systems with the following characteristics:

### Galactic
- Realistic spiral arm structure with 12.5° pitch angle
- Exponential disk with 2.6 kpc scale length
- Metallicity gradient of -0.07 dex/kpc
- Proper stellar population ratios

### Stellar
- Star type frequencies matching observed distributions
- ~50% red dwarfs, decreasing percentages for larger stars
- Realistic metallicity distribution for each population

### Planetary
- Planet type ratios based on Kepler mission data
- Realistic orbital architectures
- ~2% habitable planet frequency
- Mass distributions following known exoplanet statistics

### Small Bodies
- Dense main asteroid belt (2.2-3.2 AU)
- Sparse scattered disk (30-50 AU)
- Moderate density Kuiper belt (40-100 AU)
- Realistic composition gradients

See the examples directory for more detailed demonstrations.

## License

MIT License
