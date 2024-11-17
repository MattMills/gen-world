use gen_world::{Generate, solar_system::{SolarSystem, StellarType}, Position, SmallBodyGeneration};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // Generate multiple systems to showcase variety
    for i in 0..5 {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() + i;
            
        let system = SolarSystem::generate_with_seed(seed);
        
        println!("\n{}", "=".repeat(80));
        println!("SYSTEM {}", i + 1);
        println!("{}", "=".repeat(80));
        
        // Print detailed star information
        println!("Star System: {}", system.star.name);
        println!("Stellar Type: {:?}", system.star.stellar_type);
        println!("Mass: {:.2} solar masses", system.star.physical.mass / 1.989e30);
        println!("Radius: {:.2} solar radii", system.star.physical.radius / 6.957e8);
        println!("Surface Temperature: {:.0}K", system.star.physical.surface_temperature);
        println!("Luminosity: {:.2e} solar luminosity", system.star.luminosity);
        println!("Age: {:.2} billion years", system.system_age);
        println!("Magnetic Field: {:.2e} Tesla", system.star.magnetic_field);
        println!("Rotation Period: {:.2} Earth days", system.star.rotation_period);
        
        // Print composition
        println!("\nStellar Composition:");
        println!("Hydrogen: {:.2}%", system.star.composition.hydrogen * 100.0);
        println!("Helium: {:.2}%", system.star.composition.helium * 100.0);
        println!("Metallicity: {:.2}%", system.star.composition.metallicity * 100.0);
        println!("Other: {:.2}%", system.star.composition.other * 100.0);

        // Print special characteristics based on stellar type
        println!("\nSpecial Characteristics:");
        match system.star.stellar_type {
            StellarType::BlackHole => {
                let schwarzschild_radius = 2.0 * 6.674e-11 * system.star.physical.mass / (299_792_458.0f64.powi(2));
                println!("Event Horizon Radius: {:.2e} meters", schwarzschild_radius);
                println!("Extreme Gravitational Effects");
                println!("No Habitable Zone Possible");
            },
            StellarType::NeutronStar | StellarType::PulsarStar | StellarType::MagnetarStar => {
                println!("Extreme Density: ~10¹⁴ g/cm³");
                println!("Magnetic Field Strength: {:.2e} Tesla", system.star.magnetic_field);
                println!("Intense Radiation Environment");
            },
            StellarType::WhiteDwarfRemnant => {
                println!("Degenerate Matter State");
                println!("Cooling Rate: Very slow");
                println!("Limited Habitable Zone Potential");
            },
            StellarType::BrownDwarf => {
                println!("Sub-stellar Object");
                println!("Ongoing Gravitational Contraction");
                println!("Minimal Fusion Processes");
            },
            _ => {
                if system.star.physical.surface_temperature > 10000.0 {
                    println!("Strong Stellar Wind");
                    println!("Significant UV Radiation");
                }
                if matches!(system.star.stellar_type, 
                    StellarType::RedGiant | StellarType::SuperGiant | StellarType::HyperGiant) {
                    println!("Extended Atmosphere");
                    println!("Significant Mass Loss");
                }
            }
        }

        if system.star.stellar_type.can_have_planets() {
            println!("\nHabitable Zone: {:.2} AU to {:.2} AU", system.habitable_zone.0, system.habitable_zone.1);
            
            // Print information about planets
            println!("\nPlanets: {}", system.planets.len());
            for (i, planet) in system.planets.iter().enumerate() {
                let distance = (planet.position.x.powi(2) + planet.position.y.powi(2)).sqrt() / 1.496e11;
                println!("\nPlanet {}: {}", i + 1, planet.name);
                println!("Type: {:?}", planet.planet_type);
                println!("Mass: {:.2} Earth masses", planet.physical.mass / 5.972e24);
                println!("Distance from star: {:.2} AU", distance);
                println!("Orbital Period: {:.2} Earth years", planet.orbital_period);
                println!("Surface Temperature: {:.0}K", planet.physical.surface_temperature);
                println!("Potentially Habitable: {}", planet.habitable);
            }
            
            // Print habitable planets
            let habitable = system.habitable_planets();
            println!("\nNumber of potentially habitable planets: {}", habitable.len());
            for planet in habitable {
                let distance = (planet.position.x.powi(2) + planet.position.y.powi(2)).sqrt() / 1.496e11;
                println!("- {} ({:?}) at {:.2} AU", planet.name, planet.planet_type, distance);
            }

            // Generate and print small bodies in the main asteroid belt
            let main_belt_center = Position { x: 2.7, y: 0.0, z: 0.0 };
            let small_bodies = system.generate_small_bodies(main_belt_center, 0.5, 10.0);
            println!("\nMain Belt Objects: {}", small_bodies.len());
            for (i, body) in small_bodies.iter().take(5).enumerate() {
                let distance = (body.position.x.powi(2) + body.position.y.powi(2)).sqrt() / 1.496e11;
                println!("\nAsteroid {}: {}", i + 1, body.name);
                println!("Type: {:?}", body.body_type);
                println!("Mass: {:.2e} kg", body.physical.mass);
                println!("Distance from star: {:.2} AU", distance);
                println!("Composition:");
                println!("  Iron: {:.2}%", body.elements.iron * 100.0);
                println!("  Nickel: {:.2}%", body.elements.nickel * 100.0);
                println!("  Precious Metals: {:.4}%", 
                    (body.elements.gold + body.elements.platinum) * 100.0);
                println!("  Water Ice: {:.2}%", body.elements.water_ice * 100.0);
            }
        } else {
            println!("\nThis type of stellar object typically cannot maintain a stable planetary system.");
        }
        
        // Add a small delay between systems for better readability
        thread::sleep(std::time::Duration::from_millis(100));
    }
}
