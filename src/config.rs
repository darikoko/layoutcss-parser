use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
pub struct LayoutStyleConfig {
    pub harmonic_ratio: f64,
    pub min_screen: String,
    pub max_screen: String,
    pub base_value: String,
    pub resizing_ratio: f64,
    pub dev: bool,
}


