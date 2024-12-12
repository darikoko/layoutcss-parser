use indoc::formatdoc;
use crate::harmonic::get_harmonic;

use std::collections::HashSet;
pub fn p_recursive_css(value: &str, harmonic_ratio: f64, set: &mut HashSet<String>) {
    let harmonic_value = get_harmonic(&value, harmonic_ratio);
    set.insert(formatdoc!(
        r#"
        [layout~="p-recursive:{value}"] *{{
            padding: {harmonic_value:.2}rem;
            --pl: padding: {harmonic_value:.2}rem;
            --pr: padding: {harmonic_value:.2}rem;
        }}
        "#
    ));
}

pub fn pt_recursive_css(value: &str, harmonic_ratio: f64, set: &mut HashSet<String>) {
    let harmonic_value = get_harmonic(&value, harmonic_ratio);
    set.insert(formatdoc!(
        r#"
        [layout~="pt-recursive:{value}"] *{{
            padding-top: {harmonic_value:.2}rem;
        }}
        "#
    ));
}

pub fn pb_recursive_css(value: &str, harmonic_ratio: f64, set: &mut HashSet<String>) {
    let harmonic_value = get_harmonic(&value, harmonic_ratio);
    set.insert(formatdoc!(
        r#"
        [layout~="pb-recursive:{value}"] *{{
            padding-bottom: {harmonic_value:.2}rem;
        }}
        "#
    ));
}

pub fn pl_recursive_css(value: &str, harmonic_ratio: f64, set: &mut HashSet<String>) {
    let harmonic_value = get_harmonic(&value, harmonic_ratio);
    set.insert(formatdoc!(
        r#"
        [layout~="pl-recursive:{value}"] *{{
            padding-left: {harmonic_value:.2}rem;
            --pl: {harmonic_value:.2}rem;
        }}
        "#
    ));
}

pub fn pr_recursive_css(value: &str, harmonic_ratio: f64, set: &mut HashSet<String>) {
    let harmonic_value = get_harmonic(&value, harmonic_ratio);
    set.insert(formatdoc!(
        r#"
        [layout~="pr-recursive:{value}"] *{{
            padding-right: {harmonic_value:.2}rem;
            --pr: {harmonic_value:.2}rem;
        }}
        "#
    ));
}

pub fn px_recursive_css(value: &str, harmonic_ratio: f64, set: &mut HashSet<String>) {
    let harmonic_value = get_harmonic(&value, harmonic_ratio);
    set.insert(formatdoc!(
        r#"
        [layout~="px-recursive:{value}"] *{{
            padding-left: {harmonic_value:.2}rem;
            padding-right: {harmonic_value:.2}rem;
            --pl: {harmonic_value:.2}rem;
            --pr: {harmonic_value:.2}rem;
        }}
        "#
    ));
}

pub fn py_recursive_css(value: &str, harmonic_ratio: f64, set: &mut HashSet<String>) {
    let harmonic_value = get_harmonic(&value, harmonic_ratio);
    set.insert(formatdoc!(
        r#"
        [layout~="py-recursive:{value}"] *{{
            padding-top: {harmonic_value:.2}rem;
            padding-bottom: {harmonic_value:.2}rem;
        }}
        "#
    ));
}
