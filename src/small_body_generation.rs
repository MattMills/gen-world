use crate::{Position, small_bodies::SmallBody, solar_system::SolarSystem};
use rand::prelude::*;

pub trait SmallBodyGeneration {
    fn generate_small_bodies(&self, region_center: Position, region_radius: f64, density: f64) -> Vec<SmallBody>;
    fn small_body_density(&self, distance_au: f64) -> f64;
}

impl SmallBodyGeneration for SolarSystem {
    fn generate_small_bodies(&self, region_center: Position, region_radius: f64, density: f64) -> Vec<SmallBody> {
        let mut rng = StdRng::seed_from_u64(
            self.star.name.split('-').nth(1)
                .and_then(|s| s.parse::<u64>().ok())
                .unwrap_or(0)
        );

        // Convert AU to meters
        let au_to_m = 1.496e11;
        let center_m = Position {
            x: region_center.x * au_to_m,
            y: region_center.y * au_to_m,
            z: region_center.z * au_to_m,
        };
        let radius_m = region_radius * au_to_m;

        // Calculate volume and number of bodies
        let volume = 4.0/3.0 * std::f64::consts::PI * region_radius.powi(3);
        let num_bodies = (volume * density) as usize;

        let mut bodies = Vec::with_capacity(num_bodies);
        
        for _ in 0..num_bodies {
            // Generate random position within sphere using spherical coordinates
            let r = radius_m * rng.gen::<f64>().powf(1.0/3.0); // Uniform distribution in volume
            let theta = rng.gen::<f64>() * 2.0 * std::f64::consts::PI;
            let phi = (2.0 * rng.gen::<f64>() - 1.0).acos();
            
            let pos = Position {
                x: center_m.x + r * phi.sin() * theta.cos(),
                y: center_m.y + r * phi.sin() * theta.sin(),
                z: center_m.z + r * phi.cos(),
            };

            let body = SmallBody::generate_at_position(
                self.star.name.split('-').nth(1)
                    .and_then(|s| s.parse::<u64>().ok())
                    .unwrap_or(0),
                pos,
                &self.star.stellar_type,
                self.system_age
            );

            bodies.push(body);
        }

        // Sort by distance from center for deterministic ordering
        bodies.sort_by(|a, b| {
            let dist_a = ((a.position.x - center_m.x).powi(2) + 
                         (a.position.y - center_m.y).powi(2) + 
                         (a.position.z - center_m.z).powi(2)).sqrt();
            let dist_b = ((b.position.x - center_m.x).powi(2) + 
                         (b.position.y - center_m.y).powi(2) + 
                         (b.position.z - center_m.z).powi(2)).sqrt();
            dist_a.partial_cmp(&dist_b).unwrap()
        });

        bodies
    }

    fn small_body_density(&self, distance_au: f64) -> f64 {
        match distance_au {
            // Inner asteroid belt (1.8-2.2 AU)
            d if (1.8..=2.2).contains(&d) => 5.0,
            
            // Main asteroid belt (2.2-3.2 AU)
            d if (2.2..=3.2).contains(&d) => 10.0,
            
            // Scattered disk (30-50 AU)
            d if (30.0..=50.0).contains(&d) => 0.1,
            
            // Kuiper belt (40-100 AU)
            d if (40.0..=100.0).contains(&d) => 0.5,
            
            // Sparse regions
            _ => 0.01,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Generate;

    #[test]
    fn test_small_body_generation() {
        let system = SolarSystem::generate();
        
        // Test main belt generation
        let main_belt_center = Position { x: 2.7, y: 0.0, z: 0.0 };
        let bodies = system.generate_small_bodies(main_belt_center, 0.5, 10.0);
        
        assert!(!bodies.is_empty());
        
        // Verify deterministic generation
        let bodies2 = system.generate_small_bodies(main_belt_center, 0.5, 10.0);
        assert_eq!(bodies.len(), bodies2.len());
        assert_eq!(bodies[0].body_type, bodies2[0].body_type);
    }

    #[test]
    fn test_density_distribution() {
        let system = SolarSystem::generate();
        
        // Main belt should have higher density than sparse regions
        let main_belt_density = system.small_body_density(2.7);
        let sparse_density = system.small_body_density(10.0);
        assert!(main_belt_density > sparse_density);
        
        // Kuiper belt should have moderate density
        let kuiper_density = system.small_body_density(45.0);
        assert!(kuiper_density > sparse_density);
        assert!(kuiper_density < main_belt_density);
    }
}
