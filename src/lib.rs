const DENSITY_OF_ALCOHOL: f64 = 0.78945;

mod molar_mass {
    pub const ALCOHOL: f64 = 46.069;
    pub const SUGAR: f64 = 180.156; // Glucose, Fructose
}

pub fn sug_to_abv(sugar_content: f64, vol: f64) -> f64 {

    // Mols of sugar = mass(g) / molar mass
    let mols_sug = sugar_content / crate::molar_mass::SUGAR;

    // One Sugar molecule releases 2 alcohol molecules
    let mass_alcohol = mols_sug * 2.0 * crate::molar_mass::ALCOHOL;

    // Convert to ML by dividing by the density
    let vol_alcohol = mass_alcohol / crate::DENSITY_OF_ALCOHOL;

    // Percent Alcohol by Volume, +-0.5%
    (vol_alcohol/vol) * 100.0
}

pub fn brix_to_abv(brix: f64) -> f64 {
    return sug_to_abv(brix, 100.0);
}

pub fn abv_to_brix(abv: f64) -> f64 {
    let abm = abv * crate::DENSITY_OF_ALCOHOL;
    let mols_alcohol = abm / crate::molar_mass::ALCOHOL;
    let brix = mols_alcohol / 2.0 * crate::molar_mass::SUGAR;
    return brix;
}

pub fn boost_brix(vol: f64, brix: f64, target_brix: f64) -> f64 {

    // Brix = sugar / 100ml
    let multiplier = vol/100.0;

    let brix_diff = target_brix - brix;

    // Sugar required to boost brix
    return brix_diff * multiplier
}
