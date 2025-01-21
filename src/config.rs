use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
pub struct LayoutStyleConfig {
    pub harmonic_ratio: f64,
    pub base_value: String,
    pub dev: bool,
}


