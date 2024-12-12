use indoc::formatdoc;
use std::collections::HashSet;

use crate::harmonic::get_harmonic;

const ICON_STYLE: &str = r#"
icon-l{
    display: inline-block;
    width: fit-content;
    vertical-align: middle;
}

icon-l > :nth-child(1) {
    height: 0.75em;
    width: auto;
}
"#;

fn icon_scale_style(value: &str, harmonic: f64) -> String {
    formatdoc!(
        r#"
        icon-l[layout*="scale:{value}"] >:nth-child(1){{
            height: {harmonic:.2}rem;
        }}
        "#,
    )
}

fn icon_align_style(value: &str) -> String {
    formatdoc!(
        r#"
        icon-l[layout~="align:{value}"]{{
            vertical-align: {value};
        }}
        "#,
    )
}

fn icon_group_style(
    value: &str,
    gap_dir_selector: &str,
    gap_dir: &str,
    opposite_dir: &str,
    harmonic: f64,
) -> String {
    formatdoc!(
        r#"
        icon-l{gap_dir_selector}[layout*="gap:{value}"] >:nth-child(1){{
            margin-inline-{gap_dir}: {harmonic:.2}rem;
            margin-inline-{opposite_dir}: initial;
        }}
        "#,
    )
}

pub fn icon_css(
    scale: Option<&str>,
    align: Option<&str>,
    gap_dir: Option<&str>,
    gap: Option<&str>,
    harmonic_ratio: f64,
    set: &mut HashSet<String>,
) {
    set.insert(ICON_STYLE.to_string());
    if let Some(value) = scale {
        let harmonic_value = get_harmonic(&value, harmonic_ratio);
        set.insert(icon_scale_style(value, harmonic_value));
    }
    if let Some(value) = align {
        set.insert(icon_align_style(value));
    }
    if let Some(value) = gap {
        let harmonic_value = get_harmonic(value, harmonic_ratio);
        let gap_dir = match gap_dir {
            Some("end") => "end",
            _ => "start",
        };
        let gap_dir_selector = if gap_dir == "end" {
            r#"[layout*="gap-dir:end"]"#
        } else {
            ""
        };
        let opposite_dir = if gap_dir == "end" { "start" } else { "end" };
        set.insert(icon_group_style(
            value,
            gap_dir_selector,
            gap_dir,
            opposite_dir,
            harmonic_value,
        ));
    }
}
