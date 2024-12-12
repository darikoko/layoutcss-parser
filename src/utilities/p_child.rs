use indoc::formatdoc;
use crate::harmonic::get_harmonic;

use std::collections::HashSet;
pub fn p_child_css(value: &str, harmonic_ratio: f64, set: &mut HashSet<String>) {
    let harmonic_value = get_harmonic(value, harmonic_ratio);
    set.insert(formatdoc!(
        r#"
        [layout~="p-child:{value}"]> *{{
            padding: {harmonic_value:.2}rem;
            --pl: padding: {harmonic_value:.2}rem;
            --pr: padding: {harmonic_value:.2}rem;
        }}
        "#
    ));
}

pub fn pt_child_css(value: &str, harmonic_ratio: f64, set: &mut HashSet<String>) {
    let harmonic_value = get_harmonic(value, harmonic_ratio);
    set.insert(formatdoc!(
        r#"
        [layout~="pt-child:{value}"]> *{{
            padding-top: {harmonic_value:.2}rem;
        }}
        "#
    ));
}

pub fn pb_child_css(value: &str, harmonic_ratio: f64, set: &mut HashSet<String>) {
    let harmonic_value = get_harmonic(value, harmonic_ratio);
    set.insert(formatdoc!(
        r#"
        [layout~="pb-child:{value}"]> *{{
            padding-bottom: {harmonic_value:.2}rem;
        }}
        "#
    ));
}

pub fn pl_child_css(value: &str, harmonic_ratio: f64, set: &mut HashSet<String>) {
    let harmonic_value = get_harmonic(value, harmonic_ratio);
    set.insert(formatdoc!(
        r#"
        [layout~="pl-child:{value}"]> *{{
            padding-left: {harmonic_value:.2}rem;
            --pl: {harmonic_value:.2}rem;
        }}
        "#
    ));
}

pub fn pr_child_css(value: &str, harmonic_ratio: f64, set: &mut HashSet<String>) {
    let harmonic_value = get_harmonic(value, harmonic_ratio);
    set.insert(formatdoc!(
        r#"
        [layout~="pr-child:{value}"]> *{{
            padding-right: {harmonic_value:.2}rem;
            --pr: {harmonic_value:.2}rem;
        }}
        "#
    ));
}

pub fn px_child_css(value: &str, harmonic_ratio: f64, set: &mut HashSet<String>) {
    let harmonic_value = get_harmonic(value, harmonic_ratio);
    set.insert(formatdoc!(
        r#"
        [layout~="px-child:{value}"]> *{{
            padding-left: {harmonic_value:.2}rem;
            padding-right: {harmonic_value:.2}rem;
            --pl: {harmonic_value:.2}rem;
            --pr: {harmonic_value:.2}rem;
        }}
        "#
    ));
}

pub fn py_child_css(value: &str, harmonic_ratio: f64, set: &mut HashSet<String>) {
    let harmonic_value = get_harmonic(value, harmonic_ratio);
    set.insert(formatdoc!(
        r#"
        [layout~="py-child:{value}"]> *{{
            padding-top: {harmonic_value:.2}rem;
            padding-bottom: {harmonic_value:.2}rem;
        }}
        "#
    ));
}
