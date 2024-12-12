use indoc::formatdoc;
use crate::harmonic::get_harmonic;

use std::collections::HashSet;
pub fn w_css(value: &str, harmonic_ratio: f64, set: &mut HashSet<String>) {
    let harmonic_value = get_harmonic(value, harmonic_ratio);
    set.insert(formatdoc!(
        r#"
        [layout~="w:{value}"]{{
            width: {harmonic_value:.2}rem;
        }}
        "#
    ));
}
