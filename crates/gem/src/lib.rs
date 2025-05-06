mod constants;
mod quantum;
mod helper;

pub use constants::*;
pub use quantum::*;
pub use helper::diff;

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

    #[test]
    fn bohr_energy_matches_rydberg_n2() {
        
        let e = energy_n(ME.val, 2);           // n = 2
        let expected = -3.401_423_281_7;       // CODATA eV
        println!("\nGEM: {:.9e} eV, expected: {:.9e} eV", e, expected);
        let (abs, rel) = diff(e, expected);
        println!("\nabs: {:.9e}, rel: {:.9e}\n", abs, rel);

        assert!(rel < 1e-8, "rel err = {rel:e}");   // 10 ppb margin
    }

    #[test]
    fn compare_all_observed() -> anyhow::Result<()> {
        let rows = load_csv("../data/observed.csv")?;
        for r in rows {
            let pred = match r.label.as_str() {
                "Bohr_E_n2_H" => energy_n(ME.val, 2),
                _ => continue,
            };
            let (_, rel) = diff(pred, r.value);
            assert!(rel < 1e-8, "{} rel_err={rel:e}", r.label);
        }
        Ok(())
    }

    #[test]
    pub fn get_muonic_lamb_shift() {
        let result = gem_muonic_lamb_shift();
        println!("gem_muonic_lamb_shift: {:.9}", result);
    }
    
}
