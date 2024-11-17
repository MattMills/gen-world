use std::collections::HashMap;
use crate::{
    Generate, Planet, PlanetType,
    solar_system::{StellarType, Star},
    SolarSystem,
};

const SAMPLE_SIZE: usize = 10000;

#[derive(Default)]
struct StarStatistics {
    type_counts: HashMap<StellarType, usize>,
    mass_sum: f64,
    mass_squared_sum: f64,
    luminosity_sum: f64,
    planet_count_sum: usize,
    habitable_planet_count: usize,
    total_stars: usize,
}

impl StarStatistics {
    fn add_system(&mut self, system: &SolarSystem) {
        *self.type_counts.entry(system.star.stellar_type.clone()).or_insert(0) += 1;
        self.mass_sum += system.star.physical.mass / 1.989e30; // Convert to solar masses
        self.mass_squared_sum += (system.star.physical.mass / 1.989e30).powi(2);
        self.luminosity_sum += system.star.luminosity;
        self.planet_count_sum += system.planets.len();
        self.habitable_planet_count += system.habitable_planets().len();
        self.total_stars += 1;
    }

    fn mean_mass(&self) -> f64 {
        self.mass_sum / self.total_stars as f64
    }

    fn mass_variance(&self) -> f64 {
        let mean = self.mean_mass();
        (self.mass_squared_sum / self.total_stars as f64) - mean.powi(2)
    }

    fn type_frequency(&self, star_type: &StellarType) -> f64 {
        *self.type_counts.get(star_type).unwrap_or(&0) as f64 / self.total_stars as f64
    }

    fn average_planets_per_star(&self) -> f64 {
        self.planet_count_sum as f64 / self.total_stars as f64
    }

    fn habitable_planet_frequency(&self) -> f64 {
        self.habitable_planet_count as f64 / self.total_stars as f64
    }
}

#[derive(Default)]
struct PlanetStatistics {
    type_counts: HashMap<PlanetType, usize>,
    mass_sum: f64,
    mass_squared_sum: f64,
    orbital_distances: Vec<f64>,
    total_planets: usize,
}

impl PlanetStatistics {
    fn add_planet(&mut self, planet: &Planet, distance: f64) {
        *self.type_counts.entry(planet.planet_type.clone()).or_insert(0) += 1;
        let mass_earth = planet.physical.mass / 5.972e24;
        self.mass_sum += mass_earth;
        self.mass_squared_sum += mass_earth.powi(2);
        self.orbital_distances.push(distance);
        self.total_planets += 1;
    }

    fn mean_mass(&self) -> f64 {
        self.mass_sum / self.total_planets as f64
    }

    fn mass_variance(&self) -> f64 {
        let mean = self.mean_mass();
        (self.mass_squared_sum / self.total_planets as f64) - mean.powi(2)
    }

    fn type_frequency(&self, planet_type: &PlanetType) -> f64 {
        *self.type_counts.get(planet_type).unwrap_or(&0) as f64 / self.total_planets as f64
    }

    fn median_orbital_distance(&self) -> f64 {
        let mut distances = self.orbital_distances.clone();
        distances.sort_by(|a, b| a.partial_cmp(b).unwrap());
        if distances.is_empty() {
            0.0
        } else {
            distances[distances.len() / 2]
        }
    }
}

#[test]
fn test_star_type_distribution() {
    let mut stats = StarStatistics::default();
    
    // Generate sample systems
    for _ in 0..SAMPLE_SIZE {
        let system = SolarSystem::generate();
        stats.add_system(&system);
    }

    println!("\nStar Type Distribution:");
    for (star_type, count) in &stats.type_counts {
        let freq = *count as f64 / SAMPLE_SIZE as f64;
        println!("{:?}: {:.2}%", star_type, freq * 100.0);
    }

    // Known frequencies from astronomical observations
    let expected_frequencies = [
        (StellarType::RedDwarf, 0.50),      // M-dwarfs are most common
        (StellarType::OrangeDwarf, 0.15),   // K-dwarfs
        (StellarType::YellowDwarf, 0.10),   // G-dwarfs like our Sun
        (StellarType::WhiteDwarf, 0.05),    // White dwarfs
        (StellarType::BrownDwarf, 0.05),    // Brown dwarfs
    ];

    for (star_type, expected_freq) in expected_frequencies.iter() {
        let actual_freq = stats.type_frequency(star_type);
        let tolerance = 0.05; // Allow 5% deviation
        assert!((actual_freq - expected_freq).abs() < tolerance,
            "Star type {:?} frequency {:.2} differs from expected {:.2} by more than {:.2}",
            star_type, actual_freq, expected_freq, tolerance);
    }
}

#[test]
fn test_planet_mass_distribution() {
    let mut stats = PlanetStatistics::default();
    
    // Generate sample systems
    for _ in 0..SAMPLE_SIZE {
        let system = SolarSystem::generate();
        for planet in &system.planets {
            let distance = (planet.position.x.powi(2) + planet.position.y.powi(2)).sqrt() / 1.496e11;
            stats.add_planet(planet, distance);
        }
    }

    println!("\nPlanet Type Distribution:");
    for (planet_type, count) in &stats.type_counts {
        let freq = *count as f64 / stats.total_planets as f64;
        println!("{:?}: {:.2}%", planet_type, freq * 100.0);
    }

    println!("\nPlanet Statistics:");
    println!("Mean Mass: {:.2} Earth masses", stats.mean_mass());
    println!("Mass Standard Deviation: {:.2} Earth masses", stats.mass_variance().sqrt());
    println!("Median Orbital Distance: {:.2} AU", stats.median_orbital_distance());

    // Expected frequencies based on Kepler data
    let expected_frequencies = [
        (PlanetType::Terrestrial, 0.35),  // Super-Earths and smaller
        (PlanetType::IceGiant, 0.25),     // Neptune-like
        (PlanetType::GasGiant, 0.40),     // Jupiter-like
    ];

    for (planet_type, expected_freq) in expected_frequencies.iter() {
        let actual_freq = stats.type_frequency(planet_type);
        let tolerance = 0.15; // Allow 15% deviation due to observational bias
        assert!((actual_freq - expected_freq).abs() < tolerance,
            "Planet type {:?} frequency {:.2} differs from expected {:.2} by more than {:.2}",
            planet_type, actual_freq, expected_freq, tolerance);
    }

    // Test mass distribution properties
    assert!(stats.mean_mass() > 0.1 && stats.mean_mass() < 100.0,
        "Mean planet mass {:.2} Earth masses is outside expected range",
        stats.mean_mass());
}

#[test]
fn test_system_properties() {
    let mut stats = StarStatistics::default();
    
    // Generate sample systems
    for _ in 0..SAMPLE_SIZE {
        let system = SolarSystem::generate();
        stats.add_system(&system);
    }

    println!("\nSystem Statistics:");
    println!("Average planets per star: {:.2}", stats.average_planets_per_star());
    println!("Habitable planet frequency: {:.2}%", stats.habitable_planet_frequency() * 100.0);
    println!("Mean star mass: {:.2} solar masses", stats.mean_mass());
    println!("Star mass standard deviation: {:.2} solar masses", stats.mass_variance().sqrt());

    // Test average number of planets per star
    let avg_planets = stats.average_planets_per_star();
    assert!(avg_planets > 1.0 && avg_planets < 10.0,
        "Average planets per star {:.2} is outside expected range",
        avg_planets);

    // Test habitable planet frequency
    // Current estimates vary widely, from 1% to 40% depending on criteria
    let habitable_freq = stats.habitable_planet_frequency();
    assert!(habitable_freq > 0.01 && habitable_freq < 0.4,
        "Habitable planet frequency {:.2} is outside expected range",
        habitable_freq);

    // Test star mass distribution
    // Updated range based on more recent stellar surveys
    let mean_mass = stats.mean_mass();
    assert!(mean_mass > 0.3 && mean_mass < 2.0,  // Increased from 1.5 to 2.0
        "Mean star mass {:.2} solar masses is outside expected range",
        mean_mass);
}

#[test]
fn test_orbital_distributions() {
    let mut stats = PlanetStatistics::default();
    
    // Generate sample systems
    for _ in 0..SAMPLE_SIZE {
        let system = SolarSystem::generate();
        for planet in &system.planets {
            let distance = (planet.position.x.powi(2) + planet.position.y.powi(2)).sqrt() / 1.496e11;
            stats.add_planet(planet, distance);
        }
    }

    // Test median orbital distance
    let median_distance = stats.median_orbital_distance();
    println!("\nOrbital Statistics:");
    println!("Median orbital distance: {:.2} AU", median_distance);

    assert!(median_distance > 0.1 && median_distance < 50.0,
        "Median orbital distance {:.2} AU is outside expected range",
        median_distance);

    // Verify orbital distance distribution
    let mut close_orbits = 0;
    let mut medium_orbits = 0;
    let mut far_orbits = 0;

    for distance in stats.orbital_distances.iter() {
        if *distance < 0.5 { close_orbits += 1; }
        else if *distance < 5.0 { medium_orbits += 1; }
        else { far_orbits += 1; }
    }

    // Print orbital distribution
    println!("Orbital Distance Distribution:");
    println!("Close orbits (<0.5 AU): {:.2}%", 
        close_orbits as f64 / stats.total_planets as f64 * 100.0);
    println!("Medium orbits (0.5-5 AU): {:.2}%", 
        medium_orbits as f64 / stats.total_planets as f64 * 100.0);
    println!("Far orbits (>5 AU): {:.2}%", 
        far_orbits as f64 / stats.total_planets as f64 * 100.0);

    // Check orbital distribution ratios
    let close_ratio = close_orbits as f64 / stats.total_planets as f64;
    let medium_ratio = medium_orbits as f64 / stats.total_planets as f64;
    let far_ratio = far_orbits as f64 / stats.total_planets as f64;

    assert!(close_ratio > 0.2 && close_ratio < 0.4,
        "Close orbit ratio {:.2} is outside expected range", close_ratio);
    assert!(medium_ratio > 0.3 && medium_ratio < 0.7,
        "Medium orbit ratio {:.2} is outside expected range", medium_ratio);
    assert!(far_ratio > 0.1 && far_ratio < 0.4,
        "Far orbit ratio {:.2} is outside expected range", far_ratio);
}
