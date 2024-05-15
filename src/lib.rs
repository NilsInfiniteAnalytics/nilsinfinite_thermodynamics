pub const STANDARD_PRESSURE: f64 = 101325.0; // Pa
pub const STANDARD_TEMPERATURE: f64 = 273.15; // K
pub const GAS_CONSTANT: f64 = 8.3144598; // J mol-1 K-1
pub const MOLAR_MASS_DRY_AIR: f64 = 0.028964; // kg mol-1
pub const MOLAR_MASS_WATER: f64 = 0.01801528; // kg mol-1

// Derived parameters
pub const R_D: f64 = GAS_CONSTANT / MOLAR_MASS_DRY_AIR;
pub const R_V: f64 = GAS_CONSTANT / MOLAR_MASS_WATER;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derived_parameters() {
        // Expected values
        let expected_r_d = 287.06; // J kg-1 K-1
        let expected_r_v = 461.52; // J kg-1 K-1

        // Allow a small tolerance for floating-point comparisons
        let tolerance = 1e-2;

        assert!((R_D - expected_r_d).abs() < tolerance);
        assert!((R_V - expected_r_v).abs() < tolerance);
    }
}
