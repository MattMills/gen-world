use gen_world::{Galaxy, PopulationType};

fn main() {
    let galaxy = Galaxy::new();
    
    println!("Exploring the Milky Way...\n");

    // Explore different regions of the galaxy
    let regions = [
        // Galactic Center (bulge)
        (0.0, 0.0, 0.0, "Galactic Core"),
        // Solar neighborhood
        (8000.0, 0.0, 0.0, "Solar Neighborhood"),
        // Outer disk
        (15000.0, 0.0, 0.0, "Outer Rim"),
        // Above disk (halo)
        (8000.0, 0.0, 2000.0, "Galactic Halo"),
        // Inner spiral arm
        (4000.0, 4000.0, 0.0, "Inner Spiral Arm"),
    ];

    for (x, y, z, name) in regions {
        println!("=== {} ===", name);
        let region = galaxy.generate_region(x, y, z);
        
        println!("Population: {:?}", region.population);
        println!("Metallicity: {:.2} [Fe/H]", region.metallicity);
        println!("Star Density: {:.3} stars/pc³", region.star_density);
        
        if region.population == PopulationType::ThinDisk {
            println!("Spiral Phase: {:.2}π", region.spiral_phase / std::f64::consts::PI);
        }

        // Generate some example systems in this region
        println!("\nExample Systems:");
        for i in 0..3 {
            let seed = ((x + y + z) as u64).wrapping_mul(1000).wrapping_add(i);
            if let Some(system) = region.generate_solar_system(seed) {
                println!("\nStar System {}:", i + 1);
                println!("Star Type: {:?}", system.star.stellar_type);
                println!("Mass: {:.2} solar masses", system.star.physical.mass / 1.989e30);
                println!("Planets: {}", system.planets.len());
                
                // Show habitable planets if any
                let habitable = system.habitable_planets();
                if !habitable.is_empty() {
                    println!("Habitable Planets: {}", habitable.len());
                }
            }
        }
        println!();
    }

    // Show a cross-section of star density
    println!("=== Galactic Density Cross-Section ===");
    println!("(from galactic center to 20kpc, marking high density regions)\n");

    let mut cross_section = String::new();
    for x in (0..=40).rev() {
        let kpc = x as f64 * 0.5; // 0 to 20 kpc in 0.5 kpc steps
        let region = galaxy.generate_region(kpc * 1000.0, 0.0, 0.0);
        
        // Use different characters for density levels
        let char = if region.star_density > 0.1 { "█" }
        else if region.star_density > 0.01 { "▓" }
        else if region.star_density > 0.001 { "▒" }
        else if region.star_density > 0.0001 { "░" }
        else { " " };
        
        cross_section.push_str(char);
    }
    println!("{}", cross_section);
    println!("0kpc {: >39}20kpc", "");
    
    // Print legend
    println!("\nDensity Legend:");
    println!("█ > 0.1 stars/pc³    (Core)");
    println!("▓ > 0.01 stars/pc³   (Inner Disk)");
    println!("▒ > 0.001 stars/pc³  (Outer Disk)");
    println!("░ > 0.0001 stars/pc³ (Sparse Regions)");
}
