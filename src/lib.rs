use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

pub mod distributions;
pub mod planet;
pub mod solar_system;
pub mod small_bodies;
pub mod small_body_generation;

#[cfg(test)]
mod tests;

/// Represents a 3D position in space
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// Chemical composition of a celestial body
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Composition {
    /// Percentage of hydrogen
    pub hydrogen: f64,
    /// Percentage of helium
    pub helium: f64,
    /// Percentage of metals (elements heavier than helium)
    pub metallicity: f64,
    /// Percentage of other elements
    pub other: f64,
}

/// Physical characteristics shared by celestial bodies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicalProperties {
    /// Mass in kilograms
    pub mass: f64,
    /// Radius in meters
    pub radius: f64,
    /// Surface temperature in Kelvin
    pub surface_temperature: f64,
    /// Density in kg/m³
    pub density: f64,
    /// Surface gravity in m/s²
    pub surface_gravity: f64,
    /// Escape velocity in m/s
    pub escape_velocity: f64,
}

impl PhysicalProperties {
    /// Calculate surface gravity based on mass and radius
    pub fn calculate_surface_gravity(&self) -> f64 {
        const G: f64 = 6.67430e-11; // gravitational constant
        G * self.mass / (self.radius * self.radius)
    }

    /// Calculate escape velocity based on mass and radius
    pub fn calculate_escape_velocity(&self) -> f64 {
        const G: f64 = 6.67430e-11;
        (2.0 * G * self.mass / self.radius).sqrt()
    }

    /// Calculate density based on mass and radius
    pub fn calculate_density(&self) -> f64 {
        self.mass / (4.0/3.0 * PI * self.radius.powi(3))
    }
}

/// Trait for objects that can be procedurally generated
pub trait Generate {
    /// Generate a new instance with default parameters
    fn generate() -> Self;
    /// Generate a new instance with a specific seed
    fn generate_with_seed(seed: u64) -> Self;
}

// Re-export commonly used types
pub use planet::{Planet, PlanetType, Atmosphere};
pub use solar_system::{SolarSystem, Star, StellarType};
pub use small_bodies::{SmallBody, SmallBodyType, ElementDistribution};
pub use small_body_generation::SmallBodyGeneration;

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_physical_properties_calculations() {
        let props = PhysicalProperties {
            mass: 5.972e24, // Earth's mass
            radius: 6.371e6, // Earth's radius
            surface_temperature: 288.0,
            density: 5514.0,
            surface_gravity: 9.81,
            escape_velocity: 11200.0,
        };

        assert!((props.calculate_surface_gravity() - 9.81).abs() < 0.1);
        assert!((props.calculate_escape_velocity() - 11200.0).abs() < 100.0);
        assert!((props.calculate_density() - 5514.0).abs() < 100.0);
    }
}
