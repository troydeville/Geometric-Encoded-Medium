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
    fn bohr_energy_matches_rydberg_n2() -> anyhow::Result<()> {
        
        let e = energy_n(ME.val, 2);           // n = 2
        let expected = -3.401_423_281_7;       // CODATA eV
        println!("\nGEM: {:.9e} eV, expected: {:.9e} eV", e, expected);
        let rel = diff(e, expected);
        println!("\nrel: {:.9e}\n", rel);

        assert!(rel < 1e-8, "rel err = {rel:e}");   // 10 ppb margin

        Ok(())
    }

    #[test]
    fn compare_all_observed() -> anyhow::Result<()> {
        let rows = load_csv("../data/observed.csv")?;
        println!("");
        println!("|\t   label   \t|\tpred\t\t|\t    obs    \t|\terror%      |   σ    |");
        println!("======================================================================================================");
        for r in rows {
            
            if (r.label.as_str() == "Bohr_E_n2_H") {
                let pred: f64 = energy_n(ME.val, 2);
                let rel = diff(pred, r.value);
                // println!("{}: rel: {:.9e}", r.label, rel);
                println!("|\t{}\t|  {:.9e}\t|\t{:.9e}\t|  {:.9e}   |  {:.2}  |", r.label, pred, r.value, 100.0 *rel, (pred - r.value).abs() / 0.0025);
                println!("------------------------------------------------------------------------------------------------------");
                assert!(rel < 1e-6, "{} rel_err={rel:e}", r.label);
            }
            
            else if (r.label.as_str() == "Muonic_Lamb_Shift") {
                let pred: f64 = gem_muonic_lamb_shift_mili_electron_volts();
                let rel = diff(pred, r.value);
                println!("|   {}   |   {:.9e}\t|\t{:.9e}\t|  {:.9e}   |  {:.2}  |", r.label, pred, r.value, 100.0 *rel, (pred - r.value).abs() / 0.0025);
                println!("------------------------------------------------------------------------------------------------------");
                assert!(rel < 1e-3, "{} rel_err={rel:e}", r.label);
            }

            else if (r.label.as_str() == "Electron_Mass") {
                let pred: f64 = MP.val / 23892177732494625341440.0;
                let rel = diff(pred, r.value);
                println!("|     {}     |   {:.9e}\t|\t{:.9e}\t|  {:.9e}   |  {:.2}  |", r.label, pred, r.value, 100.0 *rel, (pred - r.value).abs() / 0.0025);
                println!("------------------------------------------------------------------------------------------------------");
                assert!(rel < 1e-3, "{} rel_err={rel:e}", r.label);
            }

            else if (r.label.as_str() == "Proton_Mass") {
                let pred: f64 = MP.val / 130121e14;
                let rel = diff(pred, r.value);
                println!("|     {}     |   {:.9e}\t|\t{:.9e}\t|  {:.9e}   |  {:.2}  |", r.label, pred, r.value, 100.0 *rel, (pred - r.value).abs() / 0.0025);
                println!("------------------------------------------------------------------------------------------------------");
                assert!(rel < 1e-3, "{} rel_err={rel:e}", r.label);
            }

            else if (r.label.as_str() == "G") { 
                let pred: f64 = G.val;
                let rel = diff(pred, r.value);
                println!("|\t     {}     \t|   {:.9e} \t| {:.9e} \t|  {:.9e}   |  {:.2}  |", r.label, pred, r.value, 100.0 *rel, (pred - r.value).abs() / 0.0025);
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
