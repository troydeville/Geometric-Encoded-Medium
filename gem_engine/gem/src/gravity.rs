use std::f64::consts::PI;
use crate::constants::{G, EPSILON_0, C};

pub struct Body {
    pub mass: f64, // kg
}

impl Body {
    pub fn new(mass: f64) -> Self {
        Body { mass }
    }
}

/// Verifies the Mathematica derivation: G / Go == curvature^2
pub fn verify_emergent_gravity(b1: &Body, b2: &Body, d: f64) -> bool {
    let m1 = b1.mass;
    let m2 = b2.mass;
    
    // Mathematica: mr = (2 G (m1+m2))/c^2;
    let mr = (2.0 * G.val * (m1 + m2)) / C.val.powi(2);

    // Mathematica: q1 = Sqrt[ ((8Pi G m1^2 epsilon)/Sqrt[1-mr/d]) ]
    // We must calculate the geometric term (Sqrt[1-mr/d])
    let geometric_term = (1.0 - (mr / d)).sqrt();
    
    // Calculate Charges
    let q1 = ((8.0 * PI * G.val * m1.powi(2) * EPSILON_0.val) / geometric_term).sqrt();
    let q2 = ((8.0 * PI * G.val * m2.powi(2) * EPSILON_0.val) / geometric_term).sqrt();

    // Mathematica: curvature = (2 (m1+m2) Sqrt[2 Pi G epsilon])/(q1+q2);
    let curvature_numerator = 2.0 * (m1 + m2) * (2.0 * PI * G.val * EPSILON_0.val).sqrt();
    let curvature = curvature_numerator / (q1 + q2);

    // Mathematica: Go = ((q1+q2)/(2(m1+m2)))^2 * 1/(2Pi epsilon);
    let term_a = (q1 + q2) / (2.0 * (m1 + m2));
    let go = term_a.powi(2) * (1.0 / (2.0 * PI * EPSILON_0.val));

    // Mathematica: G / Go == curvature^2
    let ratio = G.val / go;
    let curv_sq = curvature.powi(2);

    println!("G_calc: {:.5e}, Go: {:.5e}", G.val, go);
    println!("Ratio (G/Go): {:.9}, Curvature^2: {:.9}", ratio, curv_sq);

    // Verify equality with tolerance
    (ratio - curv_sq).abs() < 1.0e-9
}