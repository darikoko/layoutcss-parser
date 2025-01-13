use indoc::formatdoc;
use crate::harmonic::get_harmonic;

use std::collections::HashSet;
const RACK_STYLE: &str = r#"
  rack-l{
    display:flex;
    flex-direction:column;
  }

  rack-l > [layout~="centered"]{
      margin-block: auto;
  }

  rack-l > :first-child:not([layout~="centered"]):not(outsider-l) {
      margin-block-start: 0;
  }

  rack-l > :last-child:not([layout~="centered"]):not(outsider-l) {
      margin-block-end: 0;
  }
"#;

fn rack_height_style(value: &str) -> String {
    formatdoc!(
        r#"
        rack-l[layout~="height:{value}"]{{
            height: {value};
            overflow-y: auto;
        }}
        "#,
    )
}

fn rack_min_height_style(value: &str) -> String {
    formatdoc!(
        r#"
        rack-l[layout~="min-height:{value}"]{{
            min-height: {value};
        }}
        "#,
    )
}

fn rack_max_height_style(value: &str) -> String {
    formatdoc!(
        r#"
        rack-l[layout~="max-height:{value}"]{{
            max-height: {value};
        }}
        "#,
    )
}

fn rack_gap_style(value: &str, harmonic: String) -> String {
    formatdoc!(
        r#"
        rack-l[layout~="gap:{value}"]{{
            gap: {harmonic};
        }}
        "#,
    )
}

pub fn rack_css(
    height: Option<&str>,
    min_height: Option<&str>,
    max_height: Option<&str>,
    gap: Option<&str>,
    harmonic_ratio: f64,
    set: &mut HashSet<String>,
) {
    set.insert(RACK_STYLE.to_string());
    if let Some(value) = height {
        set.insert(rack_height_style(&get_harmonic(value, harmonic_ratio)));
    }
    if let Some(value) = min_height {
        set.insert(rack_min_height_style(&get_harmonic(value, harmonic_ratio)));
    }
    if let Some(value) = max_height {
        set.insert(rack_max_height_style(&get_harmonic(value, harmonic_ratio)));
    }
    if let Some(value) = gap {
        let harmonic_value = get_harmonic(&value, harmonic_ratio);
        set.insert(rack_gap_style(value, harmonic_value));
    }
}
