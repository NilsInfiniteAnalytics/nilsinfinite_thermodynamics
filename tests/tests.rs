use nilsinfinite_thermodynamics::thermodynamics::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derived_parameters() {
        let params = ThermodynamicParameterSet::new();
        
        let expected_r_d = 287.061863; // J kg-1 K-1
        let expected_r_v = 461.522652; // J kg-1 K-1
        let expected_molar_mass_ratio = 1.60774; // Unitless
        let expected_e_internal_solid_tp = 333600.0; // J kg-1
        let expected_e_internal_vapor_tp = 2374735.088; // J kg-1

        let tolerance = 1e-3;

        assert!((params.r_d() - expected_r_d).abs() < tolerance);
        assert!((params.r_v() - expected_r_v).abs() < tolerance);
        assert!((params.molmass_ratio() - expected_molar_mass_ratio).abs() < tolerance);
        assert!((params.e_int_v0() - expected_e_internal_vapor_tp).abs() < tolerance);
        assert!((params.e_int_i0() - expected_e_internal_solid_tp).abs() < tolerance);
    }

    #[test]
    fn test_gas_constant_air() {
        let params = ThermodynamicParameterSet::new();
        let phase_partition = MoistAirPhasePartition::new(0.1, 0.02, 0.01, 0.0, 0.87);
        
        let expected_gas_constant = params.r_d() * (1.0 + (params.molmass_ratio() - 1.0) * 0.1 - params.molmass_ratio() * (0.02 + 0.01));
        let calculated_gas_constant = params.gas_constant_air(&phase_partition);

        let tolerance = 1e-3;
        assert!((calculated_gas_constant - expected_gas_constant).abs() < tolerance);
    }

    #[test]
    fn test_cp_m() {
        let params = ThermodynamicParameterSet::new();
        let phase_partition = MoistAirPhasePartition::new(0.1, 0.02, 0.01, 0.0, 0.87);

        let expected_cp_m = params.cp_d()
            + (params.cp_v - params.cp_d()) * phase_partition.total
            + (params.cp_l - params.cp_v) * phase_partition.liquid
            + (params.cp_i - params.cp_v) * phase_partition.ice;

        let calculated_cp_m = cp_m(&params, &phase_partition);

        let tolerance = 1e-3;
        assert!((calculated_cp_m - expected_cp_m).abs() < tolerance);
    }
}
