use indoc::formatdoc;
use crate::harmonic::get_harmonic;

use std::collections::HashSet;
pub fn p_css(value: &str, harmonic_ratio: f64, set: &mut HashSet<String>) {
    let harmonic_value = get_harmonic(value, harmonic_ratio);
    set.insert(formatdoc!(
        r#"
        [layout~="p:{value}"]{{
            padding: {harmonic_value};
            --pl: {harmonic_value};
            --pr: {harmonic_value};
        }}
        "#
    ));
}

pub fn pt_css(value: &str, harmonic_ratio: f64, set: &mut HashSet<String>) {
    let harmonic_value = get_harmonic(value, harmonic_ratio);
    set.insert(formatdoc!(
        r#"
        [layout~="pt:{value}"]{{
            padding-top: {harmonic_value};
        }}
        "#
    ));
}

pub fn pb_css(value: &str, harmonic_ratio: f64, set: &mut HashSet<String>) {
    let harmonic_value = get_harmonic(value, harmonic_ratio);
    set.insert(formatdoc!(
        r#"
        [layout~="pb:{value}"]{{
            padding-bottom: {harmonic_value};
        }}
        "#
    ));
}

pub fn pl_css(value: &str, harmonic_ratio: f64, set: &mut HashSet<String>) {
    let harmonic_value = get_harmonic(&value, harmonic_ratio);
    set.insert(formatdoc!(
        r#"
        [layout~="pl:{value}"]{{
            padding-left: {harmonic_value};
            --pl: {harmonic_value};
        }}
        "#
    ));
}

pub fn pr_css(value: &str, harmonic_ratio: f64, set: &mut HashSet<String>) {
    let harmonic_value = get_harmonic(&value, harmonic_ratio);
    set.insert(formatdoc!(
        r#"
        [layout~="pr:{value}"]{{
            padding-right: {harmonic_value};
            --pr: {harmonic_value};
        }}
        "#
    ));
}

pub fn px_css(value: &str, harmonic_ratio: f64, set: &mut HashSet<String>) {
    let harmonic_value = get_harmonic(&value, harmonic_ratio);
    set.insert(formatdoc!(
        r#"
        [layout~="px:{value}"]{{
            padding-left: {harmonic_value};
            padding-right: {harmonic_value};
            --pl: {harmonic_value};
            --pr: {harmonic_value};
        }}
        "#
    ));
}

pub fn py_css(value: &str, harmonic_ratio: f64, set: &mut HashSet<String>) {
    let harmonic_value = get_harmonic(&value, harmonic_ratio);
    set.insert(formatdoc!(
        r#"
        [layout~="py:{value}"]{{
            padding-top: {harmonic_value};
            padding-bottom: {harmonic_value};
        }}
        "#
    ));
}
