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

    // Ratio of fluid alcohol to total liquid, +-0.5%
    vol_alcohol/vol
}

pub fn brix_to_abv(brix: f64, vol: f64) -> f64 {
    sug_to_abv(vol/100.0 * brix, vol)
}

pub fn boost_brix(vol: f64, brix: f64, target_brix: f64) -> f64 {

    // Brix = sugar / 100ml
    let multiplier = vol/100.0;

    let brix_diff = target_brix - brix;

    // Sugar required to boost brix
    brix_diff * multiplier
}

