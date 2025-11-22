// #![allow(clippy::excessive_precision)]


// use std::f64::consts::{PI, SQRT_2};

// /// A physical constant with (1) an exact symbolic form and
// /// (2) an already‑evaluated f64 for fast numerics.
// pub struct Const {
//     pub val: f64,            // numeric
//     pub repr: &'static str,  // exact string
// }

// macro_rules! const_def {
//     ($name:ident, $repr:expr, $eval:expr) => {
//         pub const $name: Const = Const { val: $eval, repr: $repr };
//     };
// }


// const_def!(ALPHA_GAMMA, "αγ = 4580703784999263461548761·π",
//     4580703784999263461548761_f64 * PI);

// const_def!(ALPHA_DELTA, "αδ = 1972044687500000000000000000",
// 1972044687500000000000000000.0);

// const_def!(ALPHA, "α = αγ / αδ",
//     ALPHA_GAMMA.val / ALPHA_DELTA.val);

// const_def!(C, "c = 299792458 m·s⁻¹", 299792458e0);



// const_def!(E, "e = 1.602176634x10⁻¹⁹ C", 1.602176634e-19);

// const_def!(H, "h = 6.62607015x10⁻³⁴ J·s", 662607015e-42);

// const_def!(GAMMA, "Γ = (α h) / (2π c)", (ALPHA.val * H.val) / (2.0 * PI * C.val));

// const_def!(
//     OMEGA,
//     "Ω = 100·(4π)^(1/8) kg·s m⁻¹ C⁻¹",
//     137.2148872226854845674725056999147645813166521821855440478943555
// );



// const_def!(PHI, "Φ = Γ / e²", 1e-7_f64);

// const_def!(G, "G = (4π x Φ)/(Ω x Ω)", (4.0*PI*PHI.val) / (OMEGA.val * OMEGA.val));

// const_def!(MP, "mP = Sqrt[( h c)/(2 [Pi] G)]",
// 2.176430147259907557993883326292928736158505386632112365867775695e-8);

// const_def!(ME, "mₑ = m_P / 23892177732494625341440",
//     MP.val / 23_892_177_732_494_625_341_440_f64);
    

// // GEM Geometric Constants
// // The Gravitational Scalar derived from Horn Torus geometry
// // S = Sqrt[2] * Pi^(1/4)
// pub fn GEM_SCALAR_S() -> f64 {
//     SQRT_2 * PI.powf(0.25)
// }
// pub fn HORN_TORUS_VOLUME_FACTOR() -> f64 {
//     2.0 * PI.powi(2)
// }

// // Geometric Checks
// pub fn is_horn_torus(major_r: f64, minor_r: f64) -> bool {
//     let tolerance = 1e-9;
//     (major_r - minor_r).abs() < tolerance
// }

// pub fn calculate_spin_path(radius: f64) -> f64 {
//     // Path length = 4 * Pi * S
//     4.0 * PI * GEM_SCALAR_S() * radius
// }

#![allow(clippy::excessive_precision)]

use std::f64::consts::PI;

/// A physical constant with (1) an exact symbolic form and
/// (2) an already‑evaluated f64 for fast numerics.
pub struct Const {
    pub val: f64,            // numeric
    pub repr: &'static str,  // exact string
}

macro_rules! const_def {
    ($name:ident, $repr:expr, $eval:expr) => {
        pub const $name: Const = Const { val: $eval, repr: $repr };
    };
}

// ==========================================
// FUNDAMENTAL CONSTANTS
// ==========================================

const_def!(ALPHA_GAMMA, "αγ = 4580703784999263461548761·π",
    4580703784999263461548761_f64 * PI);

const_def!(ALPHA_DELTA, "αδ = 1972044687500000000000000000",
    1972044687500000000000000000.0);

const_def!(ALPHA, "α = αγ / αδ",
    ALPHA_GAMMA.val / ALPHA_DELTA.val);

const_def!(C, "c = 299792458 m·s⁻¹", 299792458e0);

const_def!(E, "e = 1.602176634x10⁻¹⁹ C", 1.602176634e-19);

const_def!(H, "h = 6.62607015x10⁻³⁴ J·s", 662607015e-42);

const_def!(GAMMA, "Γ = (α h) / (2π c)", (ALPHA.val * H.val) / (2.0 * PI * C.val));

const_def!(
    OMEGA,
    "Ω = 100·(4π)^(1/8) kg·s m⁻¹ C⁻¹",
    137.2148872226854845674725056999147645813166521821855440478943555
);

const_def!(PHI, "Φ = Γ / e²", 1e-7_f64);

const_def!(G, "G = (4π x Φ)/(Ω x Ω)", (4.0*PI*PHI.val) / (OMEGA.val * OMEGA.val));

const_def!(MP, "mP = Sqrt[( h c)/(2 [Pi] G)]",
    2.176430147259907557993883326292928736158505386632112365867775695e-8);

const_def!(ME, "mₑ = m_P / 23892177732494625341440",
    MP.val / 23_892_177_732_494_625_341_440_f64);

    const_def!(EPSILON_0, "1 / (c Zo)",
    625_000_f64 / (22_468_879_468_420_441_f64 * PI));


// ==========================================
// GEM GEOMETRIC FUNCTIONS
// ==========================================

/// The Gravitational Scalar derived from Horn Torus geometry
/// S = Sqrt[2] * Pi^(1/4)
/// Returns the scaling factor required to derive G from Vacuum Impedance.
pub fn gem_scalar_s() -> f64 {
    2.0_f64.sqrt() * PI.powf(0.25)
}

/// The volume factor for a Horn Torus
/// V_factor = 2 * Pi^2
pub fn horn_torus_volume_factor() -> f64 {
    2.0 * PI.powi(2)
}

/// Calculates the path length of a spinor completing a 720 degree rotation
/// on the Horn Torus surface.
/// Path = 4 * Pi * S * radius
pub fn calculate_spin_path(radius: f64) -> f64 {
    4.0 * PI * gem_scalar_s() * radius
}

/// Geometric Checks
/// Verifies if a geometry qualifies as a Horn Torus (R == r)
pub fn is_horn_torus(major_r: f64, minor_r: f64) -> bool {
    let tolerance = 1e-9;
    (major_r - minor_r).abs() < tolerance
}