pub const STANDARD_PRESSURE: f64 = 101325.0; // Pa
pub const STANDARD_TEMPERATURE: f64 = 273.15; // K
pub const GAS_CONSTANT: f64 = 8.3144598; // J mol-1 K-1
pub const MOLAR_MASS_DRY_AIR: f64 = 0.028964; // kg mol-1
pub const MOLAR_MASS_WATER: f64 = 0.01801528; // kg mol-1
pub const TEMPERATURE_WATER_TRIPLE_POINT: f64 = 273.16; // K
pub const WATER_TRIPLE_POINT_PRESSURE: f64 = 611.657; // Pa
pub const WATER_LATENT_HEAT_VAPORIZATION_REFERENCE: f64 = 2500800.0; // J kg-1
pub const WATER_LATENT_HEAT_SUBLIMATION_REFERENCE: f64 = 2834400.0; // J kg-1
pub const ADIABATIC_EXPONENT_DRY_AIR: f64 = 0.28571428571;
pub const TEMPERATURE_MIN_AT_REFERENCE: f64 = 220.0; // K
pub const ENTROPY_DRY_AIR: f64 = 6864.8; 
pub const ENTROPY_WATER_VAPOR: f64 = 10513.6;
pub const ENTROPY_REFERENCE_TEMPERATURE: f64 = 298.15; // K
pub const TEMPERATURE_WATER_FREEZE: f64 = 273.15; // K
pub const GRAVITATIONAL_ACCELERATION: f64 = 9.8067; // m s^-2
pub const TEMPERATURE_SATURATION_ADJUSTMENT_MIN: f64 = 1.0; // K
pub const TEMPERATURE_SATURATION_ADJUSTMENT_INIT_MIN: f64 = 150.0; // K
pub const TEMPERATURE_SATURATION_ADJUSTMENT_MAX: f64 = 1000.0; // K
pub const POTENTIAL_TEMPERATURE_REFERENCE_PRESSURE: f64 = 100000.0; // Pa
pub const POW_ICENUC: f64 = 1.0;
pub const TEMPERATURE_HOMOGENOUS_NUCLEATION: f64 = 233.0; // K
pub const THERMODYNAMICS_TEMPERATURE_REFERENCE: f64 = 273.16; // K
pub const ISOBARIC_SPECIFIC_HEAT_ICE: f64 = 2100.0; // J g-1 K-1
pub const ISOBARIC_SPECIFIC_HEAT_WATER_VAPOR: f64 = 1859.0; // J g-1 K-1
pub const ISOBARIC_SPECIFIC_HEAT_WATER_LIQUID: f64 = 4181.0; // J g-1 K-1
pub const MEAN_SEA_LEVEL_PRESSURE: f64 = 101325.0; // Pa
pub const TEMPERATURE_MEAN_AT_REFERENCE: f64 = 290.0; // K

// Derived parameters
pub const R_D: f64 = GAS_CONSTANT / MOLAR_MASS_DRY_AIR; // J mol-1 K-1 * mol kg-1 = J kg-1 K-1
pub const R_V: f64 = GAS_CONSTANT / MOLAR_MASS_WATER;
pub const DRY_AIR_TO_WATER_MOLAR_MASS_RATIO: f64 = MOLAR_MASS_DRY_AIR / MOLAR_MASS_WATER;
pub const E_INTERNAL_WATER_VAPOR_TP: f64 = WATER_LATENT_HEAT_VAPORIZATION_REFERENCE - R_V * STANDARD_TEMPERATURE;
pub const E_INTERNAL_WATER_SOLID_TP: f64 = WATER_LATENT_HEAT_SUBLIMATION_REFERENCE - WATER_LATENT_HEAT_VAPORIZATION_REFERENCE;


// Models
pub struct ThermodynamicParameterSet {
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

impl ThermodynamicParameterSet {
    pub fn new() -> Self {
        Self {
            t_0: THERMODYNAMICS_TEMPERATURE_REFERENCE,
            mslp: MEAN_SEA_LEVEL_PRESSURE,
            p_ref_theta: POTENTIAL_TEMPERATURE_REFERENCE_PRESSURE,
            cp_v: ISOBARIC_SPECIFIC_HEAT_WATER_VAPOR,
            cp_l: ISOBARIC_SPECIFIC_HEAT_WATER_LIQUID,
            cp_i: ISOBARIC_SPECIFIC_HEAT_ICE,
            lh_v0: WATER_LATENT_HEAT_VAPORIZATION_REFERENCE,
            lh_s0: WATER_LATENT_HEAT_SUBLIMATION_REFERENCE,
            press_triple: WATER_TRIPLE_POINT_PRESSURE,
            t_triple: TEMPERATURE_WATER_TRIPLE_POINT,
            t_freeze: TEMPERATURE_WATER_FREEZE,
            t_min: TEMPERATURE_SATURATION_ADJUSTMENT_MIN,
            t_max: TEMPERATURE_SATURATION_ADJUSTMENT_MAX,
            t_init_min: TEMPERATURE_SATURATION_ADJUSTMENT_INIT_MIN,
            entropy_reference_temperature: ENTROPY_REFERENCE_TEMPERATURE,
            entropy_dry_air: ENTROPY_DRY_AIR,
            entropy_water_vapor: ENTROPY_WATER_VAPOR,
            kappa_d: ADIABATIC_EXPONENT_DRY_AIR,
            gas_constant: GAS_CONSTANT,
            molmass_dryair: MOLAR_MASS_DRY_AIR,
            molmass_water: MOLAR_MASS_WATER,
            t_surf_ref: TEMPERATURE_MEAN_AT_REFERENCE,
            t_min_ref: TEMPERATURE_MIN_AT_REFERENCE,
            grav: GRAVITATIONAL_ACCELERATION,
            t_icenuc: TEMPERATURE_HOMOGENOUS_NUCLEATION,
            pow_icenuc: POW_ICENUC,
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
        let params = ThermodynamicParameterSet::new();
        
        let expected_r_d = 287.061863; // J kg-1 K-1
        let expected_r_v = 461.522652; // J kg-1 K-1
        let expected_molar_mass_ratio = 1.60774; // Unitless

        let tolerance = 1e-3;

        assert!((params.r_d() - expected_r_d).abs() < tolerance);
        assert!((params.r_v() - expected_r_v).abs() < tolerance);
        assert!((params.molmass_ratio() - expected_molar_mass_ratio).abs() < tolerance);
    }
}
