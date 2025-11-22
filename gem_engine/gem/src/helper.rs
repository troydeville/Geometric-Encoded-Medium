
pub fn diff(pred: f64, obs: f64) -> f64 {        // (absolute, relative)
    let abs = (pred - obs).abs();
    let rel = abs / obs.abs();
    rel
}
