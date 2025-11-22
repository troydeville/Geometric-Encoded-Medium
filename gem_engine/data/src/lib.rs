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
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
