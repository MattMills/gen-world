use serde::{Deserialize, Serialize};
use rand::prelude::*;
use crate::{SolarSystem, Generate};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GalacticPosition {
    pub x: f64,  // parsecs from galactic center
    pub y: f64,  // parsecs from galactic center
    pub z: f64,  // parsecs from galactic plane
    pub r: f64,  // cylindrical radius from center
    pub theta: f64, // angle in galactic plane
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PopulationType {
    ThinDisk,    // Young stars, high metallicity
    ThickDisk,   // Intermediate age stars
    Bulge,       // Old stars, varied metallicity
    Halo,        // Very old stars, low metallicity
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GalacticRegion {
    pub position: GalacticPosition,
    pub population: PopulationType,
    pub metallicity: f64,      // [Fe/H] relative to solar
    pub star_density: f64,     // stars per cubic parsec
    pub spiral_phase: f64,     // phase in spiral arm
}

impl GalacticRegion {
    pub fn generate_at_position(x: f64, y: f64, z: f64) -> Self {
        let r = (x * x + y * y).sqrt();
        let theta = y.atan2(x);

        // Determine population type based on position
        let population = if r < 3000.0 && z.abs() < 1000.0 {
            PopulationType::Bulge
        } else if z.abs() > 1000.0 {
            PopulationType::Halo
        } else if z.abs() > 400.0 {
            PopulationType::ThickDisk
        } else {
            PopulationType::ThinDisk
        };

        // Calculate metallicity gradient
        // Decreases with radius and distance from plane
        let base_metallicity = match population {
            PopulationType::ThinDisk => 0.0,  // Solar metallicity at solar radius
            PopulationType::ThickDisk => -0.5,
            PopulationType::Bulge => 0.3,
            PopulationType::Halo => -1.5,
        };
        
        // Add radial gradient (-0.07 dex/kpc)
        let r_kpc = r / 1000.0;
        let solar_r = 8.0; // kpc
        let metallicity = base_metallicity - 0.07 * (r_kpc - solar_r);

        // Calculate star density
        let density = match population {
            PopulationType::ThinDisk => {
                // Exponential disk with 2.6 kpc scale length
                let scale_height = 300.0; // pc
                let scale_length = 2600.0; // pc
                0.1 * (-r/scale_length - z.abs()/scale_height).exp()
            },
            PopulationType::ThickDisk => {
                let scale_height = 900.0;
                let scale_length = 3600.0;
                0.02 * (-r/scale_length - z.abs()/scale_height).exp()
            },
            PopulationType::Bulge => {
                // de Vaucouleurs profile
                let r_eff = 2500.0; // pc
                0.5 * (-7.67 * ((r*r + (z/0.5)*(z/0.5)).sqrt()/r_eff).powf(0.25)).exp()
            },
            PopulationType::Halo => {
                // r^-3.5 profile
                let r_spherical = (r*r + z*z).sqrt();
                1e-4 * (r_spherical/8000.0).powf(-3.5)
            },
        };

        // Calculate spiral arm phase
        // Using 4-arm logarithmic spiral with pitch angle 12.5°
        let pitch_angle = 12.5f64.to_radians();
        let spiral_phase = if population == PopulationType::ThinDisk {
            let k = pitch_angle.tan();
            let base_phase = theta - (r.ln() / k);
            // Normalize to [0, 2π)
            (base_phase % (2.0 * std::f64::consts::PI) + 2.0 * std::f64::consts::PI) 
                % (2.0 * std::f64::consts::PI)
        } else {
            0.0
        };

        GalacticRegion {
            position: GalacticPosition { x, y, z, r, theta },
            population,
            metallicity,
            star_density: density,
            spiral_phase,
        }
    }

    pub fn generate_solar_system(&self, seed: u64) -> Option<SolarSystem> {
        let mut rng = StdRng::seed_from_u64(seed);
        
        // Probability of star generation based on density
        let prob = self.star_density / 0.1; // Normalize to thin disk maximum
        
        if rng.gen::<f64>() > prob {
            return None;
        }

        // Generate system with appropriate metallicity
        let system = SolarSystem::generate_with_seed(seed);
        Some(system)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Galaxy {
    pub radius: f64,           // parsecs
    pub disk_height: f64,      // parsecs
    pub bulge_radius: f64,     // parsecs
    pub spiral_arms: usize,
    pub pitch_angle: f64,      // degrees
}

impl Galaxy {
    pub fn new() -> Self {
        Galaxy {
            radius: 50000.0,       // 50 kpc
            disk_height: 1000.0,    // 1 kpc
            bulge_radius: 3000.0,   // 3 kpc
            spiral_arms: 4,
            pitch_angle: 12.5,
        }
    }

    pub fn generate_region(&self, x: f64, y: f64, z: f64) -> GalacticRegion {
        GalacticRegion::generate_at_position(x, y, z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_population_assignment() {
        // Test bulge
        let region = GalacticRegion::generate_at_position(0.0, 0.0, 0.0);
        assert_eq!(region.population, PopulationType::Bulge);

        // Test thin disk
        let region = GalacticRegion::generate_at_position(8000.0, 0.0, 0.0);
        assert_eq!(region.population, PopulationType::ThinDisk);

        // Test thick disk
        let region = GalacticRegion::generate_at_position(8000.0, 0.0, 500.0);
        assert_eq!(region.population, PopulationType::ThickDisk);

        // Test halo
        let region = GalacticRegion::generate_at_position(8000.0, 0.0, 2000.0);
        assert_eq!(region.population, PopulationType::Halo);
    }

    #[test]
    fn test_metallicity_gradient() {
        let inner = GalacticRegion::generate_at_position(4000.0, 0.0, 0.0);
        let solar = GalacticRegion::generate_at_position(8000.0, 0.0, 0.0);
        let outer = GalacticRegion::generate_at_position(12000.0, 0.0, 0.0);

        // Test metallicity decreases with radius
        assert!(inner.metallicity > solar.metallicity);
        assert!(solar.metallicity > outer.metallicity);
    }

    #[test]
    fn test_density_distribution() {
        let center = GalacticRegion::generate_at_position(0.0, 0.0, 0.0);
        let solar = GalacticRegion::generate_at_position(8000.0, 0.0, 0.0);
        let outer = GalacticRegion::generate_at_position(20000.0, 0.0, 0.0);

        // Test density decreases with radius
        assert!(center.star_density > solar.star_density);
        assert!(solar.star_density > outer.star_density);

        // Test vertical density decrease
        let plane = GalacticRegion::generate_at_position(8000.0, 0.0, 0.0);
        let above = GalacticRegion::generate_at_position(8000.0, 0.0, 500.0);
        assert!(plane.star_density > above.star_density);
    }

    #[test]
    fn test_spiral_structure() {
        // Test points at same radius but different angles
        let p1 = GalacticRegion::generate_at_position(8000.0, 0.0, 0.0);
        let p2 = GalacticRegion::generate_at_position(0.0, 8000.0, 0.0);
        
        // Should have different spiral phases
        assert!((p1.spiral_phase - p2.spiral_phase).abs() > 0.1);

        // Test spiral phase is 0 for non-disk populations
        let halo = GalacticRegion::generate_at_position(8000.0, 0.0, 2000.0);
        assert_eq!(halo.spiral_phase, 0.0);
    }

    #[test]
    fn test_deterministic_generation() {
        let region = GalacticRegion::generate_at_position(8000.0, 0.0, 0.0);
        
        let system1 = region.generate_solar_system(42);
        let system2 = region.generate_solar_system(42);
        let system3 = region.generate_solar_system(43);

        // Same seed should give same result
        assert_eq!(system1.is_some(), system2.is_some());
        if let (Some(s1), Some(s2)) = (&system1, system2) {
            assert_eq!(s1.star.stellar_type, s2.star.stellar_type);
        }

        // Different seeds should usually give different results
        if let (Some(s1), Some(s3)) = (&system1, system3) {
            assert!(s1.star.stellar_type != s3.star.stellar_type || 
                   s1.planets.len() != s3.planets.len());
        }
    }
}
