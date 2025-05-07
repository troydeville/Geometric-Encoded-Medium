#![allow(clippy::excessive_precision)]

// use gem::constants as k;

/// CODATA 2022 exact & recommended values
// pub mod codata {
//     pub const C:      f64 = 2.997_924_58e8;          // m s⁻¹ (exact)
//     pub const H:      f64 = 6.626_070_15e-34;        // J s  (exact)
//     pub const E:      f64 = 1.602_176_634e-19;       // C    (exact)
//     pub const MU_0:   f64 = 4.0 * std::f64::consts::PI * 1e-7; // H m⁻¹ (exact)
//     pub const EPS_0:  f64 = 1.0 / (MU_0 * C * C);    // F m⁻¹
//     pub const ALPHA:  f64 = 7.297_352_5693_571e-3;   // α (2022)
//     // ─── GEM‑specific ─────────────────────────────
//     pub const GAMMA:  f64 = 2.566_970_000e-45;       // kg m  (your Γ)
//     pub const OMEGA:  f64 = 137.2148872226854845674725056999147645813166521821855440478943555;// m⁻¹
//     pub const PHI:    f64 = 1.0e-7;                  // H m⁻¹ (vacuum Φ)
// }

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
    

