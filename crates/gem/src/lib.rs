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
        let result_jouels = gem_muonic_lamb_shift_joules();
        let result_electron_volts = gem_muonic_lamb_shift_electron_volts();
        let result_mili_electron_volts = gem_muonic_lamb_shift_mili_electron_volts();
        println!("gem_muonic_lamb_shift: {:.9e} J", result_jouels);
        println!("gem_muonic_lamb_shift: {:.9e} eV", result_electron_volts);
        println!("gem_muonic_lamb_shift: {:.9e} meV", result_mili_electron_volts);
        println!("");
    }
    
}
