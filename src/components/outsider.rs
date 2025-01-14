use indoc::formatdoc;
use std::collections::HashSet;
use crate::harmonic::get_harmonic;

const OUTSIDER_STYLE: &str = r#"
  outsider-l{
    display:block;
  }
"#;

fn outsider_position_style(value: &str) -> String {
    formatdoc!(
        r#"
        outsider-l[layout~="position:{value}"]{{
            position:{value};
        }}
        "#,
    )
}

fn outsider_top_style(value: &str, harmonic: String) -> String {
    formatdoc!(
        r#"
        outsider-l[layout~="top:{value}"]{{
            top:{harmonic};
        }}
        "#,
    )
}

fn outsider_bottom_style(value: &str, harmonic: String) -> String {
    formatdoc!(
        r#"
        outsider-l[layout~="bottom:{value}"]{{
            bottom:{harmonic};
        }}
        "#,
    )
}

fn outsider_left_style(value: &str, harmonic: String) -> String {
    formatdoc!(
        r#"
        outsider-l[layout~="left:{value}"]{{
            left:{harmonic};
        }}
        "#,
    )
}

fn outsider_right_style(value: &str, harmonic: String) -> String {
    formatdoc!(
        r#"
        outsider-l[layout~="right:{value}"]{{
            right:{harmonic};
        }}
        "#,
    )
}

pub fn outsider_css(
    position: Option<&str>,
    top: Option<&str>,
    bottom: Option<&str>,
    left: Option<&str>,
    right: Option<&str>,
    harmonic_ratio: f64,
    set: &mut HashSet<String>,
) {
    set.insert(OUTSIDER_STYLE.to_string());

    if let Some(value) = position {
        set.insert(outsider_position_style(value));
    }

    if let Some(value) = top {
        let harmonic_value = get_harmonic(value, harmonic_ratio);
        set.insert(outsider_top_style(value, harmonic_value));
    }
    if let Some(value) = bottom {
        let harmonic_value = get_harmonic(value, harmonic_ratio);
        set.insert(outsider_bottom_style(value, harmonic_value));
    }
    if let Some(value) = left {
        let harmonic_value = get_harmonic(value, harmonic_ratio);
        set.insert(outsider_left_style(value, harmonic_value));
    }
    if let Some(value) = right {
        let harmonic_value = get_harmonic(value, harmonic_ratio);
        set.insert(outsider_right_style(value, harmonic_value));
    }
}
