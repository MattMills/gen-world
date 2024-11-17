use lazy_static::lazy_static;
use rand::Rng;
use rand_distr::{Distribution, LogNormal, Normal};

lazy_static! {
    // Star mass distribution (in solar masses)
    static ref STAR_MASS_DISTRIBUTION: LogNormal<f64> = LogNormal::new(0.0, 0.5).unwrap();
    
    // Planet mass distributions for different types (in Earth masses)
    static ref TERRESTRIAL_MASS_DISTRIBUTION: LogNormal<f64> = LogNormal::new(-0.5, 0.5).unwrap();
    static ref ICE_GIANT_MASS_DISTRIBUTION: LogNormal<f64> = LogNormal::new(2.5, 0.3).unwrap();
    static ref GAS_GIANT_MASS_DISTRIBUTION: LogNormal<f64> = LogNormal::new(5.0, 0.4).unwrap();
    
    // Orbital period distribution (in Earth years)
    static ref ORBITAL_PERIOD_DISTRIBUTION: LogNormal<f64> = LogNormal::new(0.5, 1.0).unwrap();
    
    // Metallicity distribution (centered around solar metallicity)
    static ref METALLICITY_DISTRIBUTION: Normal<f64> = Normal::new(0.0, 0.2).unwrap();
}

/// Generate a random planet mass in Earth masses based on desired type
pub fn random_planet_mass<R: Rng + ?Sized>(rng: &mut R, distance_from_star: f64) -> f64 {
    // Probability of different planet types varies with distance
    // Based on Kepler data and exoplanet observations
    let terrestrial_prob = match distance_from_star {
        d if d < 0.5 => 0.6,  // High chance of terrestrial planets close to star
        d if d < 2.0 => 0.5,  // Still favors terrestrial in inner system
        d if d < 5.0 => 0.2,  // Reduced in middle system
        _ => 0.1,             // Rare in outer system
    };

    let ice_giant_prob = match distance_from_star {
        d if d < 0.5 => 0.2,  // Less common very close to star
        d if d < 2.0 => 0.3,  // More common in inner-mid system
        d if d < 10.0 => 0.4, // Peak in mid system
        _ => 0.3,             // Common in outer system
    };
    
    // This makes gas giants very common in outer system since
    // gas_giant_prob = 1.0 - (terrestrial_prob + ice_giant_prob)
    
    let roll = rng.gen::<f64>();
    
    if roll < terrestrial_prob {
        // Terrestrial planet (0.1-2 Earth masses)
        TERRESTRIAL_MASS_DISTRIBUTION.sample(rng).max(0.1).min(2.0)
    } else if roll < (terrestrial_prob + ice_giant_prob) {
        // Ice giant (10-50 Earth masses)
        ICE_GIANT_MASS_DISTRIBUTION.sample(rng).max(10.0).min(50.0)
    } else {
        // Gas giant (50-1000 Earth masses)
        // Increase minimum mass for outer system gas giants
        let min_mass = if distance_from_star > 5.0 { 100.0 } else { 50.0 };
        GAS_GIANT_MASS_DISTRIBUTION.sample(rng).max(min_mass).min(1000.0)
    }
}

/// Generate a random orbital period in Earth years
pub fn random_orbital_period<R: Rng + ?Sized>(rng: &mut R) -> f64 {
    ORBITAL_PERIOD_DISTRIBUTION.sample(rng)
}

/// Generate a random metallicity value (relative to solar)
pub fn random_metallicity<R: Rng + ?Sized>(rng: &mut R) -> f64 {
    METALLICITY_DISTRIBUTION.sample(rng)
}

/// Calculate habitable zone range for a star (in AU)
pub fn habitable_zone_range(star_mass: f64, luminosity: f64) -> (f64, f64) {
    // Using more accurate calculation based on both mass and luminosity
    // Inner edge: where runaway greenhouse effect occurs
    // Outer edge: where CO2 condensation occurs
    let inner_bound = (luminosity / 1.1).sqrt() * star_mass.powf(0.25);
    let outer_bound = (luminosity / 0.53).sqrt() * star_mass.powf(0.25);
    (inner_bound, outer_bound)
}

/// Calculate surface temperature for a planet at given distance from star
pub fn calculate_surface_temperature(distance_au: f64, stellar_luminosity: f64, greenhouse_effect: f64) -> f64 {
    // Using the Stefan-Boltzmann law and inverse square law
    // Base temperature for Earth-like albedo
    let base_temp = 278.0 * (stellar_luminosity.powf(0.25) / distance_au.powf(0.5));
    
    // Apply greenhouse effect
    let temp = base_temp * greenhouse_effect;
    
    // Add some random variation (±5%)
    temp * (0.95 + rand::thread_rng().gen::<f64>() * 0.1)
}

/// Calculate probability of planet having a moon
pub fn moon_probability(planet_mass: f64) -> f64 {
    // Higher probability for larger planets, with a sigmoid curve
    (1.0 - (-planet_mass / 10.0).exp()).min(0.95)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::thread_rng;

    #[test]
    fn test_distributions_positive() {
        let mut rng = thread_rng();
        for _ in 0..1000 {
            assert!(random_planet_mass(&mut rng, 1.0) > 0.0);
            assert!(random_orbital_period(&mut rng) > 0.0);
        }
    }

    #[test]
    fn test_habitable_zone() {
        let (inner, outer) = habitable_zone_range(1.0, 1.0); // Solar mass, solar luminosity
        assert!(inner < outer);
        assert!(inner > 0.0);
        assert!(outer > inner);
        
        // Test that more massive/luminous stars have wider habitable zones
        let (inner2, outer2) = habitable_zone_range(2.0, 10.0);
        assert!(inner2 > inner);
        assert!(outer2 > outer);
    }

    #[test]
    fn test_surface_temperature() {
        // Test Earth-like conditions
        let temp = calculate_surface_temperature(1.0, 1.0, 1.0);
        assert!((temp - 278.0).abs() < 30.0); // Allow for random variation
        
        // Test temperature decreases with distance
        let temp_far = calculate_surface_temperature(2.0, 1.0, 1.0);
        assert!(temp_far < temp);
        
        // Test temperature increases with luminosity
        let temp_bright = calculate_surface_temperature(1.0, 2.0, 1.0);
        assert!(temp_bright > temp);
    }

    #[test]
    fn test_moon_probability() {
        assert!(moon_probability(1.0) > 0.0);
        assert!(moon_probability(1.0) < 1.0);
        assert!(moon_probability(10.0) > moon_probability(1.0));
        
        // Test that very massive planets have high moon probability
        assert!(moon_probability(100.0) > 0.9);
    }

    #[test]
    fn test_planet_mass_distribution() {
        let mut rng = thread_rng();
        let mut inner_terrestrial = 0;
        let mut outer_giants = 0;
        
        // Test inner system favors terrestrial planets
        for _ in 0..1000 {
            let mass = random_planet_mass(&mut rng, 0.3);
            if mass < 2.0 {
                inner_terrestrial += 1;
            }
        }
        assert!(inner_terrestrial > 500); // At least 50% terrestrial
        
        // Test outer system favors giants
        for _ in 0..1000 {
            let mass = random_planet_mass(&mut rng, 10.0);
            if mass > 10.0 {
                outer_giants += 1;
            }
        }
        assert!(outer_giants > 700); // At least 70% giants
    }
}
