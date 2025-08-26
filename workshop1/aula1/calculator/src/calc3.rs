pub fn pot(base: f64, exponent: i32) -> f64 {
    base.powi(exponent)
}

pub fn log(base: f64, value: f64) -> f64 {
    value.ln() / base.ln()
}