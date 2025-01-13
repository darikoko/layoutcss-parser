use crate::harmonic::get_harmonic;
use indoc::formatdoc;

use std::collections::HashSet;
const SWITCHER_STYLE: &str = r#"
 switcher-l{
    display: flex;
    flex-wrap: wrap;
  }

  switcher-l > *:not(outsider-l){
      flex-grow: 1;
  }
"#;

const SWITCHER_REVERSE_STYLE: &str = r#"
  switcher-l[layout~="reverse"]{
    flex-wrap: wrap-reverse;
  }
"#;

fn switcher_threshold_style(value: &str) -> String {
    formatdoc!(
        r#"
        switcher-l[layout~="threshold:{value}"] > *:not(outsider-l) {{
            flex-basis: calc(({value} - 100%) * 999);
        }}
        "#,
    )
}

fn switcher_limit_style(value: &str) -> String {
    formatdoc!(
        r#"
        switcher-l[layout~="limit:{value}"] > :nth-last-child(n+{value}):not(outsider-l),
        switcher-l[layout~="limit:{value}"] > :nth-last-child(n+{value}) ~ *:not(outsider-l){{
            flex-basis: 100%;
        }}
        "#,
    )
}

fn switcher_gap_style(value: &str, harmonic: String) -> String {
    formatdoc!(
        r#"
        switcher-l[layout~="gap:{value}"]{{
            gap: {harmonic};
        }}
        "#,
    )
}

fn switcher_gap_x_style(value: &str, harmonic: String) -> String {
    formatdoc!(
        r#"
        switcher-l[layout~="gap-x:{value}"]{{
            column-gap: {harmonic};
        }}
        "#,
    )
}

fn switcher_gap_y_style(value: &str, harmonic: String) -> String {
    formatdoc!(
        r#"
        switcher-l[layout~="gap-y:{value}"]{{
            row-gap: {harmonic};
        }}
        "#,
    )
}

pub fn switcher_css(
    threshold: Option<&str>,
    limit: Option<&str>,
    reverse: bool,
    gap: Option<&str>,
    gap_x: Option<&str>,
    gap_y: Option<&str>,
    harmonic_ratio: f64,
    set: &mut HashSet<String>,
) {
    set.insert(SWITCHER_STYLE.to_string());
    if let Some(value) = threshold {
        set.insert(switcher_threshold_style(value));
    }
    if let Some(value) = limit {
        set.insert(switcher_limit_style(value));
    }
    if reverse {
        set.insert(SWITCHER_REVERSE_STYLE.to_string());
    }
    if let Some(value) = gap {
        let harmonic_value = get_harmonic(value, harmonic_ratio);
        set.insert(switcher_gap_style(value, harmonic_value));
    }
    if let Some(ref value) = gap_x {
        let harmonic_value = get_harmonic(value, harmonic_ratio);
        set.insert(switcher_gap_x_style(value, harmonic_value));
    }
    if let Some(ref value) = gap_y {
        let harmonic_value = get_harmonic(value, harmonic_ratio);
        set.insert(switcher_gap_y_style(value, harmonic_value));
    }
}
