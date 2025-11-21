// use std::f64::consts::PI;

use crate::constants::*;

// pub fn lambda_alpha(n: f64) -> f64 {
//     let ag = ALPHA_GAMMA.val;
//     let ad = ALPHA_DELTA.val;
//     (ad * n) / ((ag + 2.0 * ad * n * n)
// }

pub fn r_n(n: f64, mass: f64) -> f64 {
    GAMMA.val * n * n / (ALPHA.val * ALPHA.val * mass)
}

pub fn energy_n(mass: f64, n: u32) -> f64 {
    // − (α² c² m) / (2 n²)  [J] → eV
    let n = n as f64;
    let joule = -ALPHA.val * ALPHA.val * C.val * C.val * mass / (2.0 * n * n);
    joule / 1.602_176_634e-19
}

pub fn energy_zn(mass: f64, n: u32, z: u32) -> f64 {
    let z = z as f64;
    energy_n(mass, n) * z * z
}

pub fn gem_muonic_lamb_shift_joules() -> f64 {
    let mu = 1.883531628e-28; // mass of muon
    let delta: f64 = ALPHA.val * mu * C.val.powf(2.0) * lambda_alpha(1904977.0);
    delta
}

pub fn gem_muonic_lamb_shift_electron_volts() -> f64 {
    let mu = 1.883531628e-28; // mass of muon
    
    let delta: f64 = ALPHA.val * mu * C.val.powf(2.0) * lambda_alpha(1904977.0);
    delta / 1.602176634e-19
}
// 1904986
pub fn gem_muonic_lamb_shift_mili_electron_volts() -> f64 {
    let mu = 1.883531628e-28; // mass of muon CODATA 2022
    let delta: f64 = ALPHA.val * mu * C.val.powf(2.0) * lambda_alpha(1904977.0);
    delta / E.val * 1000.0
}

pub fn lambda_alpha(n: f64) -> f64 {
    (ALPHA_DELTA.val * n) / ((2.0*ALPHA_DELTA.val*n.powf(2.0))+ALPHA_GAMMA.val)
}
