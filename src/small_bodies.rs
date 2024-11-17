use serde::{Deserialize, Serialize};
use rand::prelude::*;
use crate::{Composition, PhysicalProperties, Position, solar_system::StellarType};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SmallBodyType {
    RockyAsteroid,      // Silicate-rich main belt asteroid
    MetallicAsteroid,   // Iron-nickel rich asteroid
    IcyAsteroid,        // Volatile-rich outer system asteroid
    ShortPeriodComet,   // Jupiter-family comets
    LongPeriodComet,    // Oort cloud comets
    Centaur,            // Outer system bodies with chaotic orbits
    KuiperBeltObject,   // Trans-Neptunian objects
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementDistribution {
    pub iron: f64,
    pub nickel: f64,
    pub gold: f64,
    pub platinum: f64,
    pub rare_earth: f64,
    pub water_ice: f64,
    pub methane_ice: f64,
    pub silicates: f64,
    pub carbon: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmallBody {
    pub name: String,
    pub body_type: SmallBodyType,
    pub physical: PhysicalProperties,
    pub position: Position,
    pub composition: Composition,
    pub elements: ElementDistribution,
    pub orbital_period: f64,
    pub rotation_period: f64,
}

impl SmallBody {
    /// Generate a small body at a specific position with deterministic properties
    pub fn generate_at_position(system_seed: u64, position: Position, stellar_type: &StellarType, system_age: f64) -> Self {
        // Use position components to create a deterministic seed
        let x_seed = (position.x * 1e6) as i64;
        let y_seed = (position.y * 1e6) as i64;
        let z_seed = (position.z * 1e6) as i64;
        let position_seed = (x_seed.wrapping_mul(73856093) ^ 
                           y_seed.wrapping_mul(19349663) ^ 
                           z_seed.wrapping_mul(83492791)) as u64;
        let seed = system_seed.wrapping_add(position_seed);
        
        let mut rng = StdRng::seed_from_u64(seed);
        
        // Calculate distance from star
        let distance = (position.x.powi(2) + position.y.powi(2) + position.z.powi(2)).sqrt() / 1.496e11;
        
        // Determine body type based on distance and random factor
        let body_type = match distance {
            d if d < 2.0 => {
                // Inner system - mostly rocky and metallic asteroids
                if rng.gen::<f64>() < 0.7 { SmallBodyType::RockyAsteroid }
                else { SmallBodyType::MetallicAsteroid }
            },
            d if d < 5.0 => {
                // Main belt - mix of all asteroid types
                let roll = rng.gen::<f64>();
                if roll < 0.5 { SmallBodyType::RockyAsteroid }
                else if roll < 0.8 { SmallBodyType::MetallicAsteroid }
                else { SmallBodyType::IcyAsteroid }
            },
            d if d < 30.0 => {
                // Outer system - icy bodies and centaurs
                let roll = rng.gen::<f64>();
                if roll < 0.4 { SmallBodyType::IcyAsteroid }
                else if roll < 0.7 { SmallBodyType::Centaur }
                else { SmallBodyType::ShortPeriodComet }
            },
            _ => {
                // Far outer system - KBOs and long-period comets
                if rng.gen::<f64>() < 0.7 { SmallBodyType::KuiperBeltObject }
                else { SmallBodyType::LongPeriodComet }
            }
        };

        // Generate mass based on type and position
        let mass = match body_type {
            SmallBodyType::RockyAsteroid | SmallBodyType::MetallicAsteroid => 
                rng.gen_range(1e13..1e19),  // 10km to 100km diameter
            SmallBodyType::IcyAsteroid | SmallBodyType::Centaur => 
                rng.gen_range(1e15..1e20),  // Larger icy bodies
            SmallBodyType::ShortPeriodComet | SmallBodyType::LongPeriodComet => 
                rng.gen_range(1e12..1e15),  // Typical comet masses
            SmallBodyType::KuiperBeltObject => 
                rng.gen_range(1e18..1e22),  // Large KBOs
        };

        // Calculate element distribution based on type and stellar history
        let mut elements = match body_type {
            SmallBodyType::MetallicAsteroid => ElementDistribution {
                iron: rng.gen_range(0.5..0.8),
                nickel: rng.gen_range(0.1..0.2),
                gold: rng.gen_range(1e-6..1e-5),
                platinum: rng.gen_range(1e-6..1e-5),
                rare_earth: rng.gen_range(1e-4..1e-3),
                water_ice: 0.0,
                methane_ice: 0.0,
                silicates: rng.gen_range(0.05..0.2),
                carbon: rng.gen_range(0.01..0.05),
            },
            SmallBodyType::RockyAsteroid => ElementDistribution {
                iron: rng.gen_range(0.1..0.3),
                nickel: rng.gen_range(0.01..0.05),
                gold: rng.gen_range(1e-7..1e-6),
                platinum: rng.gen_range(1e-7..1e-6),
                rare_earth: rng.gen_range(1e-5..1e-4),
                water_ice: 0.0,
                methane_ice: 0.0,
                silicates: rng.gen_range(0.6..0.8),
                carbon: rng.gen_range(0.05..0.1),
            },
            SmallBodyType::IcyAsteroid | SmallBodyType::ShortPeriodComet |
            SmallBodyType::LongPeriodComet => ElementDistribution {
                iron: rng.gen_range(0.01..0.05),
                nickel: rng.gen_range(0.001..0.01),
                gold: rng.gen_range(1e-8..1e-7),
                platinum: rng.gen_range(1e-8..1e-7),
                rare_earth: rng.gen_range(1e-6..1e-5),
                water_ice: rng.gen_range(0.3..0.6),
                methane_ice: rng.gen_range(0.1..0.3),
                silicates: rng.gen_range(0.1..0.3),
                carbon: rng.gen_range(0.05..0.15),
            },
            SmallBodyType::Centaur | SmallBodyType::KuiperBeltObject => ElementDistribution {
                iron: rng.gen_range(0.05..0.15),
                nickel: rng.gen_range(0.01..0.03),
                gold: rng.gen_range(1e-7..1e-6),
                platinum: rng.gen_range(1e-7..1e-6),
                rare_earth: rng.gen_range(1e-5..1e-4),
                water_ice: rng.gen_range(0.2..0.4),
                methane_ice: rng.gen_range(0.1..0.2),
                silicates: rng.gen_range(0.2..0.4),
                carbon: rng.gen_range(0.1..0.2),
            },
        };

        // Adjust element distribution based on stellar type
        elements = match stellar_type {
            StellarType::NeutronStar | StellarType::BlackHole => {
                // Enriched in heavy elements due to supernova
                ElementDistribution {
                    iron: elements.iron * 1.5,
                    nickel: elements.nickel * 1.5,
                    gold: elements.gold * 2.0,
                    platinum: elements.platinum * 2.0,
                    rare_earth: elements.rare_earth * 2.0,
                    ..elements
                }
            },
            StellarType::RedGiant | StellarType::SuperGiant | StellarType::HyperGiant => {
                // More volatile depletion in inner system
                if distance < 5.0 {
                    ElementDistribution {
                        water_ice: elements.water_ice * 0.5,
                        methane_ice: elements.methane_ice * 0.5,
                        ..elements
                    }
                } else {
                    elements
                }
            },
            _ => elements,
        };

        // Normalize element distribution to sum to 1.0
        let total = elements.iron + elements.nickel + elements.gold + 
                   elements.platinum + elements.rare_earth + elements.water_ice + 
                   elements.methane_ice + elements.silicates + elements.carbon;
        
        elements.iron /= total;
        elements.nickel /= total;
        elements.gold /= total;
        elements.platinum /= total;
        elements.rare_earth /= total;
        elements.water_ice /= total;
        elements.methane_ice /= total;
        elements.silicates /= total;
        elements.carbon /= total;

        // Calculate physical properties
        let density = match body_type {
            SmallBodyType::MetallicAsteroid => rng.gen_range(4500.0..8000.0),  // kg/m³
            SmallBodyType::RockyAsteroid => rng.gen_range(2500.0..4000.0),
            SmallBodyType::IcyAsteroid | SmallBodyType::Centaur => rng.gen_range(1000.0..2000.0),
            SmallBodyType::ShortPeriodComet | SmallBodyType::LongPeriodComet => rng.gen_range(500.0..1000.0),
            SmallBodyType::KuiperBeltObject => rng.gen_range(1500.0..2500.0),
        };

        let radius = (3.0 * mass / (4.0 * std::f64::consts::PI * density)).powf(1.0/3.0);

        let mut physical = PhysicalProperties {
            mass,
            radius,
            density,
            surface_temperature: 0.0,  // Will be set by the system
            surface_gravity: 0.0,      // Will be calculated
            escape_velocity: 0.0,      // Will be calculated
        };

        physical.surface_gravity = physical.calculate_surface_gravity();
        physical.escape_velocity = physical.calculate_escape_velocity();

        let composition = Composition {
            hydrogen: 0.0,
            helium: 0.0,
            metallicity: elements.iron + elements.nickel + elements.gold + elements.platinum + elements.rare_earth,
            other: 1.0 - (elements.iron + elements.nickel + elements.gold + elements.platinum + elements.rare_earth),
        };

        SmallBody {
            name: format!("SB-{}", seed % 1000000),
            body_type,
            physical,
            position,
            composition,
            elements,
            orbital_period: 0.0,  // Will be calculated by the system
            rotation_period: rng.gen_range(0.1..100.0),  // Hours
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deterministic_generation() {
        let pos1 = Position { x: 1.0, y: 2.0, z: 3.0 };
        let pos2 = Position { x: 1.0, y: 2.0, z: 3.0 };
        let pos3 = Position { x: 1.1, y: 2.0, z: 3.0 };

        let body1 = SmallBody::generate_at_position(42, pos1, &StellarType::YellowDwarf, 4.5);
        let body2 = SmallBody::generate_at_position(42, pos2, &StellarType::YellowDwarf, 4.5);
        let body3 = SmallBody::generate_at_position(42, pos3, &StellarType::YellowDwarf, 4.5);

        // Same position and seed should generate identical bodies
        assert_eq!(body1.body_type, body2.body_type);
        assert_eq!(body1.physical.mass, body2.physical.mass);
        
        // Different position should generate different body
        assert_ne!(body1.body_type, body3.body_type);
    }

    #[test]
    fn test_element_distribution() {
        let pos = Position { x: 2.0, y: 0.0, z: 0.0 };
        let body = SmallBody::generate_at_position(42, pos, &StellarType::NeutronStar, 10.0);

        // Check element ratios sum to approximately 1.0
        let total = body.elements.iron + body.elements.nickel + body.elements.gold + 
                   body.elements.platinum + body.elements.rare_earth + body.elements.water_ice + 
                   body.elements.methane_ice + body.elements.silicates + body.elements.carbon;
        assert!((total - 1.0).abs() < 0.01);

        // Neutron star system should have enhanced heavy elements
        assert!(body.elements.gold > 1e-7);
        assert!(body.elements.platinum > 1e-7);
    }

    #[test]
    fn test_distance_based_types() {
        // Inner system should favor rocky/metallic asteroids
        let inner_pos = Position { x: 1.496e11, y: 0.0, z: 0.0 }; // 1 AU
        let inner_body = SmallBody::generate_at_position(42, inner_pos, &StellarType::YellowDwarf, 4.5);
        assert!(matches!(inner_body.body_type, 
            SmallBodyType::RockyAsteroid | SmallBodyType::MetallicAsteroid));

        // Outer system should favor icy bodies
        let outer_pos = Position { x: 30.0 * 1.496e11, y: 0.0, z: 0.0 }; // 30 AU
        let outer_body = SmallBody::generate_at_position(42, outer_pos, &StellarType::YellowDwarf, 4.5);
        assert!(matches!(outer_body.body_type, 
            SmallBodyType::KuiperBeltObject | SmallBodyType::LongPeriodComet));
    }
}
