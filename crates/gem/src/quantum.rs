use std::f64::consts::PI;

use crate::constants::*;

pub fn lambda_alpha(n: f64) -> f64 {
    let ag = ALPHA_GAMMA.val;
    let ad = ALPHA_DELTA.val;
    (ad * n) / (ag + 2.0 * ad * n * n)
}

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

pub fn gem_muonic_lamb_shift() -> f64 {
    let mu = 1.692895470097821831177364761e-28; // reduced mass of muon + proton
    let a_mu = 1.0 / (ALPHA.val * mu); // Bohr radius
    let x = OMEGA.val * a_mu;
    let delta = (625000.0 * C.val * H.val * 1.0 / (ALPHA.val)) / (3.0 * PI.powf(5.0/2.0)) * 1.000144e8;
    delta / 1.602_176_634e-13 // J → MeV
}