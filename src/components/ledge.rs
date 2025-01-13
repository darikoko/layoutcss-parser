use indoc::formatdoc;
use crate::harmonic::get_harmonic;
use std::collections::HashSet;
const LEDGE_STYLE: &str = r#"
  ledge-l{
    display:flex;
    flex-wrap: wrap;
  }
  ledge-l > * {
    min-width: 0;
  }
"#;

const LEDGE_NO_WRAP_STYLE: &str = r#"
  ledge-l[layout~="nowrap"]  {
      flex-wrap:nowrap;
  }
"#;

const LEDGE_TWIN_WIDTH_STYLE: &str = r#"
  ledge-l[layout~="twin-width"] > * {
      flex-grow:1;
      flex-basis:0;
      min-width: 0;
  }
"#;

fn ledge_direction_style(value: &str) -> String {
    formatdoc!(
        r#"
        ledge-l[layout~="direction:{value}"]{{
            flex-direction: {value};
        }}
        "#,
    )
}

fn ledge_justify_style(value: &str) -> String {
    formatdoc!(
        r#"
        ledge-l[layout~="justify:{value}"]{{
            justify-content: {value};
        }}
        "#,
    )
}

fn ledge_align_style(value: &str) -> String {
    formatdoc!(
        r#"
        ledge-l[layout~="align:{value}"]{{
            align-items: {value};
        }}
        "#,
    )
}
fn ledge_gap_style(value: &str, harmonic: String) -> String {
    formatdoc!(
        r#"
        ledge-l[layout~="gap:{value}"]{{
            gap: {harmonic};
        }}
        "#,
    )
}

fn ledge_gap_x_style(value: &str, harmonic: String) -> String {
    formatdoc!(
        r#"
        ledge-l[layout~="gap-x:{value}"]{{
            column-gap: {harmonic};
        }}
        "#,
    )
}

fn ledge_gap_y_style(value: &str, harmonic: String) -> String {
    formatdoc!(
        r#"
        ledge-l[layout~="gap-y:{value}"]{{
            row-gap: {harmonic};
        }}
        "#,
    )
}

pub fn ledge_css(
    nowrap: bool,
    twin_width: bool,
    direction: Option<&str>,
    justify: Option<&str>,
    align: Option<&str>,
    gap: Option<&str>,
    gap_x: Option<&str>,
    gap_y: Option<&str>,
    harmonic_ratio: f64,
    set: &mut HashSet<String>,
) {
    set.insert(LEDGE_STYLE.to_string());

    if nowrap {
        set.insert(LEDGE_NO_WRAP_STYLE.to_string());
    }
    if twin_width {
        set.insert(LEDGE_TWIN_WIDTH_STYLE.to_string());
    }
    if let Some(value) = direction {
        set.insert(ledge_direction_style(value));
    }
    if let Some(value) = justify {
        set.insert(ledge_justify_style(value));
    }
    if let Some(value) = align {
        set.insert(ledge_align_style(value));
    }
    if let Some(value) = gap {
        let harmonic_value = get_harmonic(value, harmonic_ratio);
        set.insert(ledge_gap_style(value, harmonic_value));
    }
    if let Some(ref value) = gap_x {
        let harmonic_value = get_harmonic(value, harmonic_ratio);
        set.insert(ledge_gap_x_style(value, harmonic_value));
    }
    if let Some(ref value) = gap_y {
        let harmonic_value = get_harmonic(value, harmonic_ratio);
        set.insert(ledge_gap_y_style(value, harmonic_value));
    }
}
