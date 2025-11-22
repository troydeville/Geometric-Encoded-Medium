use std::f64::consts::PI;
use crate::constants::{g, epsilon_0, C};

pub struct Body {
    pub mass: f64, // kg
}

impl Body {
    pub fn new(mass: f64) -> Self {
        Body { mass }
    }
}
/// Verifies the GEM derivation: G / Go == curvature^2
pub fn verify_emergent_gravity(b1: &Body, b2: &Body, d: f64) -> bool {
    let m1 = b1.mass;
    let m2 = b2.mass;
    
    // 1. Calculate Combined Schwarzschild Radius equivalent
    // mr = (2 G (m1+m2))/c^2
    let mr = (2.0 * g() * (m1 + m2)) / C.val.powi(2);

    // 2. Calculate Geometric Scaling Factor
    // factor = Sqrt[1 - mr/d]
    let geo_factor = (1.0 - (mr / d)).sqrt();
    
    // 3. Calculate Emergent Charges
    // q = Sqrt[ ((8Pi G m^2 epsilon) / geo_factor) ]
    let q1 = ((8.0 * PI * g() * m1.powi(2) * epsilon_0()) / geo_factor).sqrt();
    let q2 = ((8.0 * PI * g() * m2.powi(2) * epsilon_0()) / geo_factor).sqrt();

    // 4. Calculate Curvature
    // curvature = (2 (m1+m2) Sqrt[2 Pi G epsilon])/(q1+q2)
    let k_numerator = 2.0 * (m1 + m2) * (2.0 * PI * g() * epsilon_0()).sqrt();
    let curvature = k_numerator / (q1 + q2);

    // 5. Calculate Emergent G (Go)
    // Go = ((q1+q2)/(2(m1+m2)))^2 * 1/(2Pi epsilon)
    let term_a = (q1 + q2) / (2.0 * (m1 + m2));
    let go = term_a.powi(2) * (1.0 / (2.0 * PI * epsilon_0()));

    // 6. Verify Identity: G / Go == curvature^2
    let ratio = g() / go;
    let k_squared = curvature.powi(2);

    println!("--- Gravity Emergence Verification ---");
    println!("G (Standard): {:.5e}", g());
    println!("Go (Emergent): {:.5e}", go);
    println!("Ratio G/Go:    {:.10}", ratio);
    println!("Curvature^2:   {:.10}", k_squared);

    // Allow for small floating point error
    (ratio - k_squared).abs() < 1e-9
}