use crate::{Galaxy, GalacticRegion, SolarSystem};

pub trait GalaxyGeneration {
    fn generate_region(&self, x: f64, y: f64, z: f64) -> GalacticRegion;
    fn generate_solar_system(&self, region: &GalacticRegion, seed: u64) -> Option<SolarSystem>;
}

impl GalaxyGeneration for Galaxy {
    fn generate_region(&self, x: f64, y: f64, z: f64) -> GalacticRegion {
        GalacticRegion::generate_at_position(x, y, z)
    }

    fn generate_solar_system(&self, region: &GalacticRegion, seed: u64) -> Option<SolarSystem> {
        region.generate_solar_system(seed)
    }
}
