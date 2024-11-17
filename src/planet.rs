use crate::{Composition, Generate, PhysicalProperties, Position};
use rand::prelude::*;
use serde::{Deserialize, Serialize};

use crate::distributions::{random_planet_mass, random_orbital_period};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PlanetType {
    Terrestrial,
    GasGiant,
    IceGiant,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Atmosphere {
    pub pressure: f64,  // in atmospheres
    pub composition: Composition,
    pub greenhouse_effect: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Planet {
    pub name: String,
    pub planet_type: PlanetType,
    pub physical: PhysicalProperties,
    pub position: Position,
    pub orbital_period: f64,  // in Earth years
    pub rotation_period: f64, // in Earth days
    pub atmosphere: Option<Atmosphere>,
    pub composition: Composition,
    pub habitable: bool,
}

impl Planet {
    pub fn generate_at_distance(seed: u64, distance: f64) -> Self {
        let mut rng = StdRng::seed_from_u64(seed);
        
        // Generate basic properties
        let mass = random_planet_mass(&mut rng, distance);
        let orbital_period = random_orbital_period(&mut rng);
        
        // Determine planet type based on mass and distance
        let planet_type = match (mass, distance) {
            (m, d) if m < 2.0 && d < 4.0 => PlanetType::Terrestrial,
            (m, d) if m < 50.0 && d > 2.0 => PlanetType::IceGiant,
            _ => PlanetType::GasGiant,
        };

        // Calculate radius based on mass and type
        let radius = match planet_type {
            PlanetType::Terrestrial => (mass / 5.51).powf(1.0/3.0) * 6.371e6, // Earth density
            PlanetType::GasGiant => (mass / 1.33).powf(1.0/3.0) * 6.371e6,    // Jupiter density
            PlanetType::IceGiant => (mass / 1.64).powf(1.0/3.0) * 6.371e6,    // Neptune density
        };

        let physical = PhysicalProperties {
            mass: mass * 5.972e24, // Convert to kg (Earth mass)
            radius,
            surface_temperature: 288.0, // Will be adjusted based on position
            density: 0.0,  // Will be calculated
            surface_gravity: 0.0, // Will be calculated
            escape_velocity: 0.0, // Will be calculated
        };

        // Initialize with placeholder position
        let position = Position { x: 0.0, y: 0.0, z: 0.0 };

        let composition = match planet_type {
            PlanetType::Terrestrial => Composition {
                hydrogen: 0.0,
                helium: 0.0,
                metallicity: 0.9,
                other: 0.1,
            },
            PlanetType::GasGiant => Composition {
                hydrogen: 0.75,
                helium: 0.24,
                metallicity: 0.01,
                other: 0.0,
            },
            PlanetType::IceGiant => Composition {
                hydrogen: 0.20,
                helium: 0.15,
                metallicity: 0.15,
                other: 0.50, // ice and volatiles
            },
        };

        // Atmosphere more likely for larger planets and at appropriate distances
        let atmosphere = match planet_type {
            PlanetType::Terrestrial if mass > 0.1 && mass < 5.0 => {
                let greenhouse = if distance < 2.0 { 1.2 } else { 1.0 };
                Some(Atmosphere {
                    pressure: mass.powf(1.5),
                    composition: Composition {
                        hydrogen: 0.0,
                        helium: 0.0,
                        metallicity: 0.01,
                        other: 0.99,
                    },
                    greenhouse_effect: greenhouse,
                })
            },
            PlanetType::GasGiant | PlanetType::IceGiant => Some(Atmosphere {
                pressure: mass.powf(2.0),
                composition: composition.clone(),
                greenhouse_effect: 1.5,
            }),
            _ => None,
        };

        let mut planet = Planet {
            name: format!("Planet-{}", seed % 1000),
            planet_type,
            physical,
            position,
            orbital_period,
            rotation_period: rng.gen_range(0.1..100.0),
            atmosphere,
            composition,
            habitable: false,
        };

        // Calculate derived properties
        planet.physical.density = planet.physical.calculate_density();
        planet.physical.surface_gravity = planet.physical.calculate_surface_gravity();
        planet.physical.escape_velocity = planet.physical.calculate_escape_velocity();

        planet
    }

    /// Check if the planet could potentially support life
    pub fn assess_habitability(&mut self, distance_from_star: f64, star_mass: f64) {
        // First, set habitable to false by default
        self.habitable = false;

        // Only terrestrial planets can be habitable
        if !matches!(self.planet_type, PlanetType::Terrestrial) {
            return;
        }

        // Calculate habitable zone based on star mass
        let inner_zone = 0.95 * star_mass.powf(0.5);
        let outer_zone = 1.37 * star_mass.powf(0.5);
        let in_habitable_zone = distance_from_star >= inner_zone && distance_from_star <= outer_zone;

        // Check for conditions suitable for liquid water and Earth-like life
        let has_atmosphere = self.atmosphere.is_some();
        
        // More lenient mass range (0.1 to 5 Earth masses)
        let good_mass = self.physical.mass > 0.1 * 5.972e24 && self.physical.mass < 5.0 * 5.972e24;
        
        // Wider temperature range for potential life (250K to 400K)
        let good_temp = self.physical.surface_temperature > 250.0 && self.physical.surface_temperature < 400.0;
        
        // More lenient gravity range (0.2 to 3.0 Earth gravities)
        let good_gravity = self.physical.surface_gravity > 2.0 && self.physical.surface_gravity < 30.0;
        
        // Check for appropriate atmospheric pressure (0.1 to 10 Earth atmospheres)
        let good_pressure = self.atmosphere.as_ref()
            .map(|atm| atm.pressure >= 0.1 && atm.pressure <= 10.0)
            .unwrap_or(false);

        // Check for reasonable rotation period (0.1 to 100 Earth days)
        let good_rotation = self.rotation_period >= 0.1 && self.rotation_period <= 100.0;
        
        // Only set to true if all conditions are met
        self.habitable = has_atmosphere && good_mass && good_temp && good_gravity && 
                        good_pressure && good_rotation && in_habitable_zone;
    }
}

impl Generate for Planet {
    fn generate() -> Self {
        Self::generate_with_seed(thread_rng().gen())
    }

    fn generate_with_seed(seed: u64) -> Self {
        // Use a default distance for initial generation
        Self::generate_at_distance(seed, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_planet_generation() {
        let planet = Planet::generate();
        assert!(planet.physical.mass > 0.0);
        assert!(planet.physical.radius > 0.0);
        assert!(planet.orbital_period > 0.0);
    }

    #[test]
    fn test_planet_types() {
        let mut small_planet = Planet::generate_at_distance(1, 0.5); // Close to star, more likely terrestrial
        let mut giant_planet = Planet::generate_at_distance(999999, 5.0); // Far from star, more likely giant
        
        // Test habitability assessment
        small_planet.assess_habitability(0.5, 1.0);
        giant_planet.assess_habitability(5.0, 1.0);
        
        // Debug prints
        println!("Giant planet type: {:?}", giant_planet.planet_type);
        println!("Giant planet mass: {} Earth masses", giant_planet.physical.mass / 5.972e24);
        println!("Giant planet habitable: {}", giant_planet.habitable);
        
        // Verify planet types
        assert!(matches!(giant_planet.planet_type, PlanetType::GasGiant | PlanetType::IceGiant), 
            "Expected gas or ice giant, got {:?}", giant_planet.planet_type);
        
        // Gas giants should never be habitable
        assert!(!giant_planet.habitable, "Gas/Ice giants should not be habitable");
    }
}
