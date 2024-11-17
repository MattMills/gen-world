use crate::{
    Generate, Position, SolarSystem,
    small_body_generation::SmallBodyGeneration
};

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
