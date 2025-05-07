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

pub fn gem_muonic_lamb_shift_joules() -> f64 {
    let mu = 1.883531628e-28; // mass of muon
    let delta: f64 = ALPHA.val * mu * C.val.powf(2.0) * gem_scale(1904923.0);
    delta
}

pub fn gem_muonic_lamb_shift_electron_volts() -> f64 {
    let mu = 1.883531628e-28; // mass of muon
    
    let delta: f64 = ALPHA.val * mu * C.val.powf(2.0) * gem_scale(1904923.0);
    delta / 1.602176634e-19
}

pub fn gem_muonic_lamb_shift_mili_electron_volts() -> f64 {
    let mu = 1.883531628e-28; // mass of muon
    let delta: f64 = ALPHA.val * mu * C.val.powf(2.0) * gem_scale(1904923.0);
    delta / 1.602176634e-19 * 1000.0
}

pub fn gem_scale(n: f64) -> f64 {
    (ALPHA_DELTA.val * n) / ((2.0*ALPHA_DELTA.val*n.powf(2.0))+ALPHA_GAMMA.val)
}

// In[1875]:= 1/(2 \[CapitalLambda]\[Alpha][1904992])
// 1/ \[CapitalLambda]\[Alpha][1904992]

// n1=(14313078728517646720000000000000000000000+4580703784999263461548761 \[Pi])/7513458706660000000000000000000000
// n2=(14313078728517646720000000000000000000000+4580703784999263461548761 \[Pi])/3756729353330000000000000000000000
// n1==1/(2 \[CapitalLambda]\[Alpha][1904992])
// n2==1/ \[CapitalLambda]\[Alpha][1904992]
// \[Alpha] /\[CapitalLambda]\[Alpha][n1] 1.0
// \[Alpha] /\[CapitalLambda]\[Alpha][n2] 1.0
// \[Alpha] /(\[CapitalLambda]\[Alpha][n1] \[CapitalLambda]\[Alpha][n2]) 1.0
// UnitConvert[\[Alpha] m\[Mu]  c^2 (\[Alpha]\[Delta] n1)/(2\[Alpha]\[Delta] n1^2+\[Alpha]\[Gamma]), "Electronvolts"]1.0
// Out[1875]= (14313078728517646720000000000000000000000+4580703784999263461548761 \[Pi])/7513458706660000000000000000000000
// Out[1876]= (14313078728517646720000000000000000000000+4580703784999263461548761 \[Pi])/3756729353330000000000000000000000
// Out[1877]= (14313078728517646720000000000000000000000+4580703784999263461548761 \[Pi])/7513458706660000000000000000000000
// Out[1878]= (14313078728517646720000000000000000000000+4580703784999263461548761 \[Pi])/3756729353330000000000000000000000
// Out[1879]= True
// Out[1880]= True
// Out[1881]= 27802.8
// Out[1882]= 55605.6
// Out[1883]= 2.11856*10^11
// Out[1884]= 0.20237eV


    // #[test]
    // fn gem_constants() {
    //     println!("{} = {:.9e}",  GAMMA.repr, GAMMA.val);
    //     println!("{}", GAMMA.repr);
    //     println!("ALPHA_GAMMA {:.9e}", ALPHA_GAMMA.val);
    //     println!("ALPHA_DELTA {:.9e}", ALPHA_DELTA.val);
    //     println!("ALPHA {:.9e}", ALPHA.val);
    //     println!("C {:.9e}", C.val);
    //     println!("E {:.9e}", E.val);
    //     println!("H {:.9e}", H.val);
    //     println!("PHI {:.9e}", PHI.val);
    //     println!("OMEGA {:.9e}", OMEGA.val);
    //     println!("{} = {:.9e}", G.repr, G.val);
    //     println!("{} = {:.9e}", ALPHA_GAMMA.repr, ALPHA_GAMMA.val);
    //     println!("{} = {:.9e}", ALPHA_DELTA.repr, ALPHA_DELTA.val);
    //     println!("{} = {:.9e}", ALPHA.repr, ALPHA.val);
    //     println!("{} = {:.9e}", ME.repr, ME.val);
    // }


// pub fn gem_muonic_lamb_shift() -> f64 {
//     let mu = 1.692895470097821831177364761e-28; // reduced mass of muon + proton
//     let a_mu = 1.0 / (ALPHA.val * mu); // Bohr radius
//     let x = OMEGA.val * a_mu;
//     let delta = (625000.0 * C.val * H.val * 1.0 / (ALPHA.val)) / (3.0 * PI.powf(5.0/2.0)) * 1.000150e8;
//     delta / 1.602_176_634e-13 // J → meV
// }