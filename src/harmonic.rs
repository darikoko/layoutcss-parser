pub fn get_harmonic(value: &str, harmonic: f64) -> f64 {
    if value == "none" {
        return 0.0;
    }
    if let Ok(x) = value.parse::<f64>() {
        return harmonic.powf(x);
    };
    -1.0
}
