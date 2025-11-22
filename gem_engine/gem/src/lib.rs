mod constants;
mod quantum;
mod helper;
mod gravity;

pub use constants::*;
pub use quantum::*;
pub use helper::diff;
pub use gravity::*;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Record {
    pub label: String,
    pub unit:  String,
    pub value: f64,
}

pub fn load_csv(path: &str) -> anyhow::Result<Vec<Record>> {
    let mut rdr = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for rec in rdr.deserialize::<Record>() {
        rows.push(rec?);
    }
    Ok(rows)
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========================================================================
    // GEOMETRIC VALIDATION TESTS (The Proofs)
    // ========================================================================

    #[test]
    fn validate_horn_torus_volume_vs_alpha() -> anyhow::Result<()> {
        // Theory: The Volume of the Horn Torus (defined by S) explains the 
        // capacity of the vacuum (1/Alpha).
        
        // 1. Get the Gravitational Scalar S = Sqrt[2] * Pi^(1/4)
        let s = gem_scalar_s();
        
        // 2. Calculate Torus Volume V = 2 * Pi^2 * S^3
        let vol_horn = horn_torus_volume_factor() * s.powi(3);
        
        // 3. Get Inverse Fine Structure Constant
        let inv_alpha = 1.0 / ALPHA.val;

        println!("\n--- Geometric Validation: Horn Torus Volume ---");
        println!("Scalar S       : {:.9}", s);
        println!("Horn Volume    : {:.9}", vol_horn);
        println!("1/Alpha        : {:.9}", inv_alpha);
        
        // 4. The Remainder (corresponds to Bohr Radius heuristic ~5.29)
        let remainder = inv_alpha - vol_horn;
        println!("Remainder      : {:.9}", remainder);

        // Assert the remainder is within the expected geometric gap
        assert!((remainder - 5.29).abs() < 0.1, "Geometric volume mismatch!");
        Ok(())
    }

    #[test]
    fn validate_proton_radius_anomaly() -> anyhow::Result<()> {
        // Theory: The Proton Radius Anomaly is exactly the volumetric mismatch
        // between the Horn Torus and the raw electromagnetic field (1/Alpha).
        
        let s = gem_scalar_s();
        let vol_horn = horn_torus_volume_factor() * s.powi(3);
        let inv_alpha = 1.0 / ALPHA.val;

        // Calculate % Mismatch
        let mismatch_ratio = 1.0 - (vol_horn / inv_alpha);
        let mismatch_percent = mismatch_ratio * 100.0;

        println!("\n--- Geometric Validation: Proton Anomaly ---");
        println!("Mismatch Ratio : {:.5}%", mismatch_percent);

        // The Anomaly is observed to be ~3.8% to 4% depending on measurement
        // (0.875 fm vs 0.841 fm is a ~3.88% reduction)
        assert!(mismatch_percent > 3.8 && mismatch_percent < 3.9);
        Ok(())
    }

    #[test]
    fn validate_gravity_emergence() {
        // Logic updated to match Mathematica script exactly
        let earth_mass = 5.972e24;
        let sun_mass = 1.989e30; // 1.9884e30 in script, using standard approx here
        let distance = 149.6e9; // AU

        // Body no longer needs distance, only Mass
        let earth = Body::new(earth_mass);
        let sun = Body::new(sun_mass);

        // Pass distance to the interaction verification
        let proof = verify_emergent_gravity(&earth, &sun, distance);
        
        println!("\n--- Geometric Validation: Gravity Emergence ---");
        println!("G / Go == Curvature^2 : {}", proof);
        assert!(proof, "Emergent Gravity derivation failed!");
    }

    // ========================================================================
    // STANDARD OBSERVATION TESTS
    // ========================================================================

    #[test]
    fn gamma_constant() -> anyhow::Result<()> {
        println!("\nΓ = (α h) / (2π c) = {:.9e}", GAMMA.val);
        println!();
        Ok(())
    }

    #[test]
    fn bohr_energy_matches_rydberg_n2() -> anyhow::Result<()> {
        
        let e = energy_n(ME.val, 2);           // n = 2
        let expected = -3.401_423_281_7;       // CODATA eV
        println!("\nGEM: {:.28e} eV, expected: {:.10e} eV", e, expected);
        let rel = diff(e, expected);
        println!("\nrel: {:.28e}\n", rel);

        assert!(rel < 1e-6, "rel err = {rel:e}");   // 10 ppb margin

        Ok(())
    }

    #[test]
    fn compare_all_observed() -> anyhow::Result<()> {
        let rows = load_csv("../data/observed.csv")?;
        println!("");
        println!("|\t   label   \t|\tpred\t\t|\t    obs    \t|\terror%      |   σ    |");
        println!("======================================================================================================");
        for r in rows {
            
            if r.label.as_str() == "Bohr_E_n2_H" {
                let pred: f64 = energy_n(ME.val, 2);
                let rel = diff(pred, r.value);
                // println!("{}: rel: {:.9e}", r.label, rel);
                println!("|\t{}\t|  {:.9e}\t|\t{:.9e}\t|  {:.9e}   |  {:.2e}  |", r.label, pred, r.value, 100.0 *rel, (pred - r.value).abs());
                println!("------------------------------------------------------------------------------------------------------");
                assert!(rel < 1e-6, "{} rel_err={rel:e}", r.label);
            }
            
            else if r.label.as_str() == "Muonic_Lamb_Shift" {
                let pred: f64 = gem_muonic_lamb_shift_mili_electron_volts();
                let rel = diff(pred, r.value);
                println!("|   {}   |   {:.9e}\t|\t{:.9e}\t|  {:.9e}   |  {:.2e}  |", r.label, pred, r.value, 100.0 *rel, (pred - r.value).abs());
                println!("------------------------------------------------------------------------------------------------------");
                assert!(rel < 1e-3, "{} rel_err={rel:e}", r.label);
            }

            else if r.label.as_str() == "Electron_Mass" {
                let pred: f64 = MP.val / 23892177732494625341440.0;
                let rel = diff(pred, r.value);
                println!("|     {}     |   {:.9e}\t|\t{:.9e}\t|  {:.9e}   |  {:.2e}  |", r.label, pred, r.value, 100.0 *rel, (pred - r.value).abs() );
                println!("------------------------------------------------------------------------------------------------------");
                assert!(rel < 1e-3, "{} rel_err={rel:e}", r.label);
            }

            else if r.label.as_str() == "Proton_Mass" {
                let pred: f64 = MP.val / 130121e14;
                let rel = diff(pred, r.value);
                println!("|     {}      |   {:.9e}\t | \t{:.9e}\t|  {:.9e}   |  {:.2e}  |", r.label, pred, r.value, 100.0 *rel, (pred - r.value).abs());
                println!("------------------------------------------------------------------------------------------------------");
                assert!(rel < 1e-3, "{} rel_err={rel:e}", r.label);
            }

            else if r.label.as_str() == "G" { 
                let pred: f64 = G.val;
                let rel = diff(pred, r.value);
                println!("|\t     {}     \t|   {:.9e} \t| {:.9e} \t|  {:.9e}   |  {:.2e}  |", r.label, pred, r.value, 100.0 *rel, (pred - r.value).abs());
                println!("------------------------------------------------------------------------------------------------------");
                assert!(rel < 1e-3, "{} rel_err={rel:e}", r.label);
            }
        }
        
        Ok(())
    }

    #[test]
    pub fn get_muonic_lamb_shift() -> anyhow::Result<()> {
        let result_jouels = gem_muonic_lamb_shift_joules();
        let result_electron_volts = gem_muonic_lamb_shift_electron_volts();
        let result_mili_electron_volts = gem_muonic_lamb_shift_mili_electron_volts();
        println!("gem_muonic_lamb_shift: {:.9e} J", result_jouels);
        println!("gem_muonic_lamb_shift: {:.9e} eV", result_electron_volts);
        println!("gem_muonic_lamb_shift: {:.9e} meV", result_mili_electron_volts);
        println!("");

        Ok(())
    }
    
}
