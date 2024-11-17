# gen-world

A Rust library for procedurally generating realistic solar systems with accurate astronomical distributions.

## Features

- Generates diverse star types with realistic frequencies:
  - Main sequence stars (Brown Dwarfs to Blue Supergiants)
  - Giant stars (Red Giants, Supergiants, Hypergiants)
  - Stellar remnants (White Dwarfs, Neutron Stars, Black Holes)
  - Exotic objects (Quark Stars, Pulsars, Magnetars)

- Creates planetary systems with:
  - Realistic mass distributions
  - Proper orbital spacing using modified Titius-Bode law
  - Temperature calculations based on stellar luminosity
  - Atmospheric composition and greenhouse effects
  - Habitability assessment based on multiple factors

- Supports small body generation:
  - Deterministic, position-based asteroid generation
  - Realistic composition based on formation region
  - Element distribution influenced by stellar history
  - Support for sparse, infinite asteroid fields
  - Different types (rocky, metallic, icy) with proper distributions

- Statistical distributions matching real astronomical observations:
  - Star type frequencies (e.g., 50% Red Dwarfs)
  - Planet type ratios (Terrestrial, Ice Giants, Gas Giants)
  - Orbital distances and planetary system architectures
  - Physical properties (mass, temperature, composition)
  - Small body densities in different regions (asteroid belt, Kuiper belt)

## Usage

```rust
use gen_world::{Generate, SolarSystem, Position, SmallBodyGeneration};

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
    let distance = (planet.position.x.powi(2) + 
                   planet.position.y.powi(2)).sqrt() / 1.496e11;
    println!("Planet type: {:?}", planet.planet_type);
    println!("Distance: {:.2} AU", distance);
    println!("Mass: {:.1} Earth masses", planet.physical.mass / 5.972e24);
    println!("Habitable: {}", planet.habitable);
}

// Generate small bodies in a region
let main_belt_center = Position { x: 2.7, y: 0.0, z: 0.0 };
let asteroids = system.generate_small_bodies(main_belt_center, 0.5, 10.0);

// Examine asteroid properties
for asteroid in &asteroids {
    println!("Type: {:?}", asteroid.body_type);
    println!("Iron content: {:.1}%", asteroid.elements.iron * 100.0);
    println!("Water ice: {:.1}%", asteroid.elements.water_ice * 100.0);
    println!("Precious metals: {:.4}%", 
        (asteroid.elements.gold + asteroid.elements.platinum) * 100.0);
}

// Get expected small body density at a distance
let density = system.small_body_density(2.7); // bodies per cubic AU
```

See the `examples/basic_system.rs` for a more detailed demonstration.

## Statistical Properties

The generator produces systems with the following characteristics:

- Star type frequencies matching observed distributions
- Planet type ratios based on Kepler mission data
- Realistic orbital architectures
- ~2% habitable planet frequency
- Mass distributions following known exoplanet statistics
- Small body distributions matching solar system observations:
  - Dense main asteroid belt (2.2-3.2 AU)
  - Sparse scattered disk (30-50 AU)
  - Moderate density Kuiper belt (40-100 AU)

## License

MIT License
