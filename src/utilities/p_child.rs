use indoc::formatdoc;
use crate::harmonic::get_harmonic;

use std::collections::HashSet;
pub fn p_child_css(value: &str, harmonic_ratio: f64, set: &mut HashSet<String>) {
    let harmonic_value = get_harmonic(value, harmonic_ratio);
    set.insert(formatdoc!(
        r#"
        [layout~="p-child:{value}"]> *{{
            padding: {harmonic_value};
            --pl: padding: {harmonic_value};
            --pr: padding: {harmonic_value};
        }}
        "#
    ));
}

pub fn pt_child_css(value: &str, harmonic_ratio: f64, set: &mut HashSet<String>) {
    let harmonic_value = get_harmonic(value, harmonic_ratio);
    set.insert(formatdoc!(
        r#"
        [layout~="pt-child:{value}"]> *{{
            padding-top: {harmonic_value};
        }}
        "#
    ));
}

pub fn pb_child_css(value: &str, harmonic_ratio: f64, set: &mut HashSet<String>) {
    let harmonic_value = get_harmonic(value, harmonic_ratio);
    set.insert(formatdoc!(
        r#"
        [layout~="pb-child:{value}"]> *{{
            padding-bottom: {harmonic_value};
        }}
        "#
    ));
}

pub fn pl_child_css(value: &str, harmonic_ratio: f64, set: &mut HashSet<String>) {
    let harmonic_value = get_harmonic(value, harmonic_ratio);
    set.insert(formatdoc!(
        r#"
        [layout~="pl-child:{value}"]> *{{
            padding-left: {harmonic_value};
            --pl: {harmonic_value};
        }}
        "#
    ));
}

pub fn pr_child_css(value: &str, harmonic_ratio: f64, set: &mut HashSet<String>) {
    let harmonic_value = get_harmonic(value, harmonic_ratio);
    set.insert(formatdoc!(
        r#"
        [layout~="pr-child:{value}"]> *{{
            padding-right: {harmonic_value};
            --pr: {harmonic_value};
        }}
        "#
    ));
}

pub fn px_child_css(value: &str, harmonic_ratio: f64, set: &mut HashSet<String>) {
    let harmonic_value = get_harmonic(value, harmonic_ratio);
    set.insert(formatdoc!(
        r#"
        [layout~="px-child:{value}"]> *{{
            padding-left: {harmonic_value};
            padding-right: {harmonic_value};
            --pl: {harmonic_value};
            --pr: {harmonic_value};
        }}
        "#
    ));
}

pub fn py_child_css(value: &str, harmonic_ratio: f64, set: &mut HashSet<String>) {
    let harmonic_value = get_harmonic(value, harmonic_ratio);
    set.insert(formatdoc!(
        r#"
        [layout~="py-child:{value}"]> *{{
            padding-top: {harmonic_value};
            padding-bottom: {harmonic_value};
        }}
        "#
    ));
}
