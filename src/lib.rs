pub const STANDARD_PRESSURE: f64 = 101325.0; // Pa
pub const STANDARD_TEMPERATURE: f64 = 273.15; // K
pub const GAS_CONSTANT: f64 = 8.3144598; // J mol-1 K-1
pub const MOLAR_MASS_DRY_AIR: f64 = 0.028964; // kg mol-1
pub const MOLAR_MASS_WATER: f64 = 0.01801528; // kg mol-1
pub const WATER_TRIPLE_POINT_TEMPERATURE = 273.16; // K

// Derived parameters
pub const R_D: f64 = GAS_CONSTANT / MOLAR_MASS_DRY_AIR;
pub const R_V: f64 = GAS_CONSTANT / MOLAR_MASS_WATER;
pub const DRY_AIR_TO_WATER_MOLAR_MASS_RATIO: f64 = MOLAR_MASS_DRY_AIR / MOLAR_MASS_WATER;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derived_parameters() {
        // Expected values
        let expected_r_d = 287.061863; // J kg-1 K-1
        let expected_r_v = 461.522652; // J kg-1 K-1
        let expected_molar_mass_ratio = 1.60774;

        // Allow a small tolerance for floating-point comparisons
        let tolerance = 1e-3;

        assert!((R_D - expected_r_d).abs() < tolerance);
        assert!((R_V - expected_r_v).abs() < tolerance);
        assert!((DRY_AIR_TO_WATER_MOLAR_MASS_RATIO - expected_molar_mass_ratio).abs() < tolerance);
    }
}
