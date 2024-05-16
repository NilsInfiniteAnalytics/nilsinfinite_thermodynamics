pub const STANDARD_PRESSURE: f64 = 101325.0; // Pa
pub const STANDARD_TEMPERATURE: f64 = 273.15; // K
pub const GAS_CONSTANT: f64 = 8.3144598; // J mol-1 K-1
pub const MOLAR_MASS_DRY_AIR: f64 = 0.028964; // kg mol-1
pub const MOLAR_MASS_WATER: f64 = 0.01801528; // kg mol-1
pub const WATER_TRIPLE_POINT_TEMPERATURE: f64 = 273.16; // K
pub const WATER_TRIPLE_POINT_PRESSURE: f64 = 611.657; // Pa
pub const WATER_LATENT_HEAT_VAPORIZATION_REFERENCE: f64 = 2500800.0; // J kg-1
pub const WATER_LATENT_HEAT_SUBLIMATION_REFERENCE: f64 = 2834400.0; // J kg-1

// Derived parameters
pub const R_D: f64 = GAS_CONSTANT / MOLAR_MASS_DRY_AIR;
pub const R_V: f64 = GAS_CONSTANT / MOLAR_MASS_WATER;
pub const DRY_AIR_TO_WATER_MOLAR_MASS_RATIO: f64 = MOLAR_MASS_DRY_AIR / MOLAR_MASS_WATER;
pub const E_INTERNAL_WATER_VAPOR_TP: f64 = WATER_LATENT_HEAT_VAPORIZATION_REFERENCE - R_V * STANDARD_TEMPERATURE;
pub const E_INTERNAL_WATER_SOLID_TP: f64 = WATER_LATENT_HEAT_SUBLIMATION_REFERENCE - WATER_LATENT_HEAT_VAPORIZATION_REFERENCE;

// Models
pub struct MoistAirThermodynamicParameters {
    pub t_0: f64,
    pub mslp: f64,
    pub p_ref_theta: f64,
    pub cp_v: f64,
    pub cp_l: f64,
    pub cp_i: f64,
    pub lh_v0: f64,
    pub lh_s0: f64,
    pub press_triple: f64,
    pub t_triple: f64,
    pub t_freeze: f64,
    pub t_min: f64,
    pub t_max: f64,
    pub t_init_min: f64,
    pub entropy_reference_temperature: f64,
    pub entropy_dry_air: f64,
    pub entropy_water_vapor: f64,
    pub kappa_d: f64,
    pub gas_constant: f64,
    pub molmass_dryair: f64,
    pub molmass_water: f64,
    pub t_surf_ref: f64,
    pub t_min_ref: f64,
    pub grav: f64,
    pub t_icenuc: f64,
    pub pow_icenuc: f64,
}

impl MoistAirThermodynamicParameters {
    pub fn new() -> Self {
        Self {
            t_0: STANDARD_TEMPERATURE, // K
            mslp: STANDARD_PRESSURE, // Pa
            p_ref_theta: 100000.0,
            cp_v: 1870.0, // J kg-1 K-1
            cp_l: 4184.0, // J kg-1 K-1
            cp_i: 2108.0, // J kg-1 K-1 
            lh_v0: WATER_LATENT_HEAT_VAPORIZATION_REFERENCE, // J kg-1
            lh_s0: WATER_LATENT_HEAT_SUBLIMATION_REFERENCE, // J kg-1
            press_triple: WATER_TRIPLE_POINT_PRESSURE, // Pa
            t_triple: WATER_TRIPLE_POINT_TEMPERATURE, // K
            t_freeze: 273.15, // K
            t_min: 100.0,
            t_max: 400.0,
            t_init_min: 250.0,
            entropy_reference_temperature: 298.15,
            entropy_dry_air: 0.0,
            entropy_water_vapor: 0.0,
            kappa_d: 1.4,
            gas_constant: GAS_CONSTANT,
            molmass_dryair: MOLAR_MASS_DRY_AIR,
            molmass_water: MOLAR_MASS_WATER,
            t_surf_ref: 288.15,
            t_min_ref: 200.0,
            grav: 9.80665, // m/s^2
            t_icenuc: 233.15,
            pow_icenuc: 1.0,
        }
    }

    pub fn r_d(&self) -> f64 {
        self.gas_constant / self.molmass_dryair
    }

    pub fn r_v(&self) -> f64 {
        self.gas_constant / self.molmass_water
    }

    pub fn molmass_ratio(&self) -> f64 {
        self.molmass_dryair / self.molmass_water
    }

    pub fn lh_f0(&self) -> f64 {
        self.lh_s0 - self.lh_v0
    }

    pub fn e_int_v0(&self) -> f64 {
        self.lh_v0 - self.r_v() * self.t_0
    }

    pub fn e_int_i0(&self) -> f64 {
        self.lh_f0()
    }

    pub fn cp_d(&self) -> f64 {
        self.r_d() / self.kappa_d
    }

    pub fn cv_d(&self) -> f64 {
        self.cp_d() - self.r_d()
    }

    pub fn cv_v(&self) -> f64 {
        self.cp_v - self.r_v()
    }

    pub fn cv_l(&self) -> f64 {
        self.cp_l
    }

    pub fn cv_i(&self) -> f64 {
        self.cp_i
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derived_parameters() {
        let params = MoistAirThermodynamicParameters::new();
        
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
}
