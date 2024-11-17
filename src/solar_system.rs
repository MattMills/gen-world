use crate::{Composition, Generate, PhysicalProperties, Position};
use crate::distributions::{habitable_zone_range, calculate_surface_temperature};
use crate::planet::Planet;
use crate::small_bodies::SmallBody;
use rand::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum StellarType {
    // Main Sequence Stars
    BrownDwarf,     // Failed star, < 0.08 solar masses
    RedDwarf,       // M-type, 0.08-0.45 solar masses
    OrangeDwarf,    // K-type, 0.45-0.8 solar masses
    YellowDwarf,    // G-type, 0.8-1.2 solar masses (like our Sun)
    WhiteDwarf,     // F-type, 1.2-1.4 solar masses
    BlueDwarf,      // A-type, 1.4-2.1 solar masses
    BlueGiant,      // B-type, 2.1-16 solar masses
    BlueSupergiant, // O-type, > 16 solar masses

    // Giant Stars
    RedGiant,       // Late life stage of low/medium mass stars
    SuperGiant,     // Late life stage of massive stars
    HyperGiant,     // Extremely massive evolved stars

    // Stellar Remnants
    WhiteDwarfRemnant,    // Dead low/medium mass stars
    NeutronStar,          // Dead massive stars
    BlackHole,            // Dead very massive stars
    
    // Exotic Objects
    QuarkStar,            // Hypothetical ultra-dense star
    PulsarStar,           // Rotating neutron star
    MagnetarStar,         // Highly magnetized neutron star
}

impl StellarType {
    fn mass_range(&self) -> (f64, f64) {
        match self {
            StellarType::BrownDwarf => (0.01, 0.08),
            StellarType::RedDwarf => (0.08, 0.45),
            StellarType::OrangeDwarf => (0.45, 0.8),
            StellarType::YellowDwarf => (0.8, 1.2),
            StellarType::WhiteDwarf => (1.2, 1.4),
            StellarType::BlueDwarf => (1.4, 2.1),
            StellarType::BlueGiant => (2.1, 6.0),         // Reduced from 8.0
            StellarType::BlueSupergiant => (6.0, 15.0),   // Reduced from 25.0
            StellarType::RedGiant => (0.3, 3.0),          // Reduced from 4.0
            StellarType::SuperGiant => (3.0, 12.0),       // Reduced from 20.0
            StellarType::HyperGiant => (12.0, 30.0),      // Reduced from 50.0
            StellarType::WhiteDwarfRemnant => (0.17, 1.4),
            StellarType::NeutronStar => (1.4, 3.0),
            StellarType::BlackHole => (3.0, 20.0),        // Reduced from 30.0
            StellarType::QuarkStar => (1.4, 3.0),
            StellarType::PulsarStar => (1.4, 3.0),
            StellarType::MagnetarStar => (1.4, 3.0),
        }
    }

    fn generate_random<R: Rng + ?Sized>(rng: &mut R) -> Self {
        let roll = rng.gen::<f64>();
        match roll {
            x if x < 0.05 => StellarType::BrownDwarf,
            x if x < 0.55 => StellarType::RedDwarf,      // Keep at 50%
            x if x < 0.70 => StellarType::OrangeDwarf,   // Keep at 15%
            x if x < 0.80 => StellarType::YellowDwarf,   // Keep at 10%
            x if x < 0.85 => StellarType::WhiteDwarf,    // Keep at 5%
            x if x < 0.89 => StellarType::BlueDwarf,     // Reduced to 4%
            x if x < 0.91 => StellarType::BlueGiant,     // Reduced to 2%
            x if x < 0.92 => StellarType::BlueSupergiant,// Reduced to 1%
            x if x < 0.94 => StellarType::RedGiant,      // Keep at 2%
            x if x < 0.95 => StellarType::SuperGiant,    // Reduced to 1%
            x if x < 0.96 => StellarType::HyperGiant,    // Reduced to 1%
            x if x < 0.97 => StellarType::WhiteDwarfRemnant,
            x if x < 0.98 => StellarType::NeutronStar,   // Keep at 1%
            x if x < 0.99 => StellarType::BlackHole,     // Keep at 1%
            x if x < 0.995 => StellarType::QuarkStar,    // Keep at 0.5%
            x if x < 0.9975 => StellarType::PulsarStar,  // Keep at 0.25%
            _ => StellarType::MagnetarStar,              // Keep at 0.25%
        }
    }

    fn temperature_range(&self) -> (f64, f64) {
        match self {
            StellarType::BrownDwarf => (300.0, 2800.0),
            StellarType::RedDwarf => (2800.0, 3500.0),
            StellarType::OrangeDwarf => (3500.0, 5000.0),
            StellarType::YellowDwarf => (5000.0, 6000.0),
            StellarType::WhiteDwarf => (6000.0, 7500.0),
            StellarType::BlueDwarf => (7500.0, 10000.0),
            StellarType::BlueGiant => (10000.0, 30000.0),
            StellarType::BlueSupergiant => (30000.0, 50000.0),
            StellarType::RedGiant => (3000.0, 4500.0),
            StellarType::SuperGiant => (3500.0, 8000.0),
            StellarType::HyperGiant => (4000.0, 50000.0),
            StellarType::WhiteDwarfRemnant => (4000.0, 150000.0),
            StellarType::NeutronStar => (100000.0, 1000000.0),
            StellarType::BlackHole => (0.0, 0.0),
            StellarType::QuarkStar => (100000.0, 1000000.0),
            StellarType::PulsarStar => (100000.0, 1000000.0),
            StellarType::MagnetarStar => (100000.0, 1000000.0),
        }
    }

    fn luminosity_factor(&self, mass: f64) -> f64 {
        match self {
            StellarType::BrownDwarf => mass.powf(2.0) * 0.001,
            StellarType::RedDwarf => mass.powf(3.0) * 0.01,
            StellarType::OrangeDwarf => mass.powf(3.5) * 0.1,
            StellarType::YellowDwarf => mass.powf(3.5),
            StellarType::WhiteDwarf => mass.powf(3.5) * 2.0,
            StellarType::BlueDwarf => mass.powf(3.5) * 5.0,
            StellarType::BlueGiant => mass.powf(3.8) * 10.0,
            StellarType::BlueSupergiant => mass.powf(4.0) * 100.0,
            StellarType::RedGiant => mass.powf(3.0) * 1000.0,
            StellarType::SuperGiant => mass.powf(3.5) * 10000.0,
            StellarType::HyperGiant => mass.powf(4.0) * 100000.0,
            StellarType::WhiteDwarfRemnant => mass.powf(-3.0) * 0.0001,
            StellarType::NeutronStar => mass.powf(-2.0) * 0.00001,
            StellarType::BlackHole => 0.0,
            StellarType::QuarkStar => mass.powf(-2.0) * 0.00001,
            StellarType::PulsarStar => mass.powf(-2.0) * 0.00001,
            StellarType::MagnetarStar => mass.powf(-2.0) * 0.00001,
        }
    }

    pub fn can_have_planets(&self) -> bool {
        !matches!(self, 
            StellarType::BlackHole | 
            StellarType::NeutronStar | 
            StellarType::PulsarStar | 
            StellarType::MagnetarStar |
            StellarType::QuarkStar
        )
    }

    fn planet_count_range(&self) -> (usize, usize) {
        match self {
            StellarType::BrownDwarf => (0, 3),
            StellarType::RedDwarf => (0, 5),
            StellarType::OrangeDwarf | StellarType::YellowDwarf => (0, 12),
            StellarType::WhiteDwarf | StellarType::BlueDwarf => (0, 8),
            StellarType::BlueGiant | StellarType::BlueSupergiant => (0, 5),
            StellarType::RedGiant | StellarType::SuperGiant | StellarType::HyperGiant => (0, 3),
            StellarType::WhiteDwarfRemnant => (0, 2),
            _ => (0, 0),
        }
    }
}

// Star and SolarSystem implementations...
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Star {
    pub name: String,
    pub stellar_type: StellarType,
    pub physical: PhysicalProperties,
    pub composition: Composition,
    pub luminosity: f64,  // relative to Sol
    pub age: f64,        // in billions of years
    pub magnetic_field: f64, // in Tesla
    pub rotation_period: f64, // in Earth days
}

impl Generate for Star {
    fn generate() -> Self {
        Self::generate_with_seed(thread_rng().gen())
    }

    fn generate_with_seed(seed: u64) -> Self {
        let mut rng = StdRng::seed_from_u64(seed);
        
        let stellar_type = StellarType::generate_random(&mut rng);
        let (min_mass, max_mass) = stellar_type.mass_range();
        let (min_temp, max_temp) = stellar_type.temperature_range();
        
        let mass_solar = min_mass + rng.gen::<f64>() * (max_mass - min_mass);
        let luminosity = stellar_type.luminosity_factor(mass_solar);
        
        let radius = match stellar_type {
            StellarType::BlackHole => {
                2.0 * 6.674e-11 * (mass_solar * 1.989e30) / (299_792_458.0f64.powi(2))
            },
            _ => {
                match stellar_type {
                    StellarType::RedGiant | StellarType::SuperGiant | StellarType::HyperGiant =>
                        mass_solar.powf(0.5) * 100.0 * 6.957e8,
                    StellarType::WhiteDwarfRemnant | StellarType::NeutronStar |
                    StellarType::QuarkStar | StellarType::PulsarStar | StellarType::MagnetarStar =>
                        mass_solar.powf(0.5) * 0.01 * 6.957e8,
                    _ => mass_solar.powf(0.8) * 6.957e8,
                }
            }
        };

        let temp = min_temp + rng.gen::<f64>() * (max_temp - min_temp);
        
        let physical = PhysicalProperties {
            mass: mass_solar * 1.989e30,
            radius,
            surface_temperature: temp,
            density: 0.0,
            surface_gravity: 0.0,
            escape_velocity: 0.0,
        };

        let composition = match stellar_type {
            StellarType::NeutronStar | StellarType::QuarkStar | 
            StellarType::PulsarStar | StellarType::MagnetarStar => Composition {
                hydrogen: 0.0,
                helium: 0.0,
                metallicity: 1.0,
                other: 0.0,
            },
            StellarType::BlackHole => Composition {
                hydrogen: 0.0,
                helium: 0.0,
                metallicity: 0.0,
                other: 1.0,
            },
            _ => Composition {
                hydrogen: 0.7347,
                helium: 0.2483,
                metallicity: 0.0169,
                other: 0.0001,
            },
        };

        let magnetic_field = match stellar_type {
            StellarType::MagnetarStar => 1e11 + rng.gen::<f64>() * 1e12,
            StellarType::PulsarStar => 1e8 + rng.gen::<f64>() * 1e9,
            StellarType::NeutronStar => 1e7 + rng.gen::<f64>() * 1e8,
            _ => 1e-4 + rng.gen::<f64>() * 1e2,
        };

        let rotation_period = match stellar_type {
            StellarType::PulsarStar => rng.gen_range(0.001..10.0),
            StellarType::NeutronStar | StellarType::MagnetarStar => rng.gen_range(0.1..100.0),
            _ => rng.gen_range(0.5..50.0),
        };

        let mut star = Star {
            name: format!("Star-{}", seed % 1000),
            stellar_type,
            physical,
            composition,
            luminosity,
            age: rng.gen_range(0.1..13.8),
            magnetic_field,
            rotation_period,
        };

        star.physical.density = star.physical.calculate_density();
        star.physical.surface_gravity = star.physical.calculate_surface_gravity();
        star.physical.escape_velocity = star.physical.calculate_escape_velocity();

        star
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolarSystem {
    pub star: Star,
    pub planets: Vec<Planet>,
    pub total_mass: f64,
    pub system_age: f64,
    pub habitable_zone: (f64, f64),
}

impl Generate for SolarSystem {
    fn generate() -> Self {
        Self::generate_with_seed(thread_rng().gen())
    }

    fn generate_with_seed(seed: u64) -> Self {
        let mut rng = StdRng::seed_from_u64(seed);
        
        let star = Star::generate_with_seed(seed);
        let star_mass = star.physical.mass / 1.989e30; // Convert to solar masses
        let system_age = star.age;
        
        let habitable_zone = habitable_zone_range(star_mass, star.luminosity);
        
        let mut planets = Vec::new();
        
        if star.stellar_type.can_have_planets() {
            let (min_planets, max_planets) = star.stellar_type.planet_count_range();
            let num_planets = rng.gen_range(min_planets..=max_planets);
            
            if num_planets > 0 {
                // Modified Titius-Bode law with randomization
                let base_distance = match star.stellar_type {
                    StellarType::BrownDwarf | StellarType::RedDwarf => 0.05,
                    StellarType::WhiteDwarfRemnant => 0.1,
                    _ => 0.3, // Increased from 0.2 to spread out planets
                };

                // Calculate spacing factor based on star mass and luminosity
                let spacing_factor = match star.stellar_type {
                    StellarType::BrownDwarf | StellarType::RedDwarf => 1.4f64,
                    StellarType::WhiteDwarfRemnant => 1.5f64,
                    StellarType::BlueGiant | StellarType::BlueSupergiant => 2.0f64,
                    _ => 1.7f64,
                };

                for i in 0..num_planets {
                    // Modified Titius-Bode law with variable spacing
                    let bode_distance = base_distance * spacing_factor.powf(i as f64);
                    let distance_factor = rng.gen_range(0.8..1.2); // 20% randomization
                    let distance = bode_distance * distance_factor;
                    
                    let angle = rng.gen_range(0.0..2.0 * std::f64::consts::PI);
                    
                    // Generate planet appropriate for this distance
                    let mut planet = Planet::generate_at_distance(seed + i as u64, distance);
                    
                    // Set its position
                    planet.position = Position {
                        x: distance * angle.cos() * 1.496e11,
                        y: distance * angle.sin() * 1.496e11,
                        z: rng.gen_range(-0.1..0.1) * 1.496e11, // Small inclination
                    };
                    
                    // Calculate surface temperature based on star's properties
                    let greenhouse_effect = planet.atmosphere.as_ref()
                        .map(|atm| atm.greenhouse_effect)
                        .unwrap_or(1.0);
                    
                    planet.physical.surface_temperature = calculate_surface_temperature(
                        distance,
                        star.luminosity,
                        greenhouse_effect
                    );
                    
                    // Pass habitable zone information for better habitability assessment
                    planet.assess_habitability(distance, star_mass);
                    
                    planets.push(planet);
                }

                // Sort planets by distance from star
                planets.sort_by(|a, b| {
                    let dist_a = (a.position.x.powi(2) + a.position.y.powi(2)).sqrt();
                    let dist_b = (b.position.x.powi(2) + b.position.y.powi(2)).sqrt();
                    dist_a.partial_cmp(&dist_b).unwrap()
                });
            }
        }
        
        let total_mass = star.physical.mass + 
            planets.iter().map(|p| p.physical.mass).sum::<f64>();

        SolarSystem {
            star,
            planets,
            total_mass,
            system_age,
            habitable_zone,
        }
    }
}

impl SolarSystem {
    pub fn habitable_planets(&self) -> Vec<&Planet> {
        self.planets.iter().filter(|p| p.habitable).collect()
    }

    pub fn center_of_mass(&self) -> Position {
        let mut total_weighted_x = 0.0;
        let mut total_weighted_y = 0.0;
        let mut total_weighted_z = 0.0;
        let mut total_mass = 0.0;  // Start at 0 and add all masses
        
        // Add star's contribution (position 0,0,0)
        total_mass += self.star.physical.mass;
        
        // Add planets' contributions
        for planet in &self.planets {
            total_weighted_x += planet.physical.mass * planet.position.x;
            total_weighted_y += planet.physical.mass * planet.position.y;
            total_weighted_z += planet.physical.mass * planet.position.z;
            total_mass += planet.physical.mass;
        }
        
        Position {
            x: total_weighted_x / total_mass,
            y: total_weighted_y / total_mass,
            z: total_weighted_z / total_mass,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_generation() {
        let system = SolarSystem::generate();
        assert!(system.star.physical.mass > 0.0);
        
        if system.star.stellar_type.can_have_planets() {
            let (min, max) = system.star.stellar_type.planet_count_range();
            assert!(system.planets.len() >= min && system.planets.len() <= max);
        } else {
            assert!(system.planets.is_empty());
        }
        
        assert!(system.total_mass >= system.star.physical.mass);
    }

    #[test]
    fn test_habitable_zone() {
        let system = SolarSystem::generate();
        let (inner, outer) = system.habitable_zone;
        assert!(inner < outer);
        assert!(inner > 0.0);
    }

    #[test]
    fn test_center_of_mass() {
        let system = SolarSystem::generate();
        let com = system.center_of_mass();
        
        let system_size = system.planets.iter()
            .map(|p| (p.position.x.powi(2) + p.position.y.powi(2)).sqrt())
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(0.0);
            
        assert!(com.x.abs() < system_size / 10.0);
        assert!(com.y.abs() < system_size / 10.0);
        assert!(com.z.abs() < system_size / 10.0);
    }
}
