use indoc::formatdoc;
use crate::harmonic::get_harmonic;
use std::collections::HashSet;
const ROW_STYLE: &str = r#"
  row-l{
    display:flex;
    flex-wrap: wrap;
  }
  row-l > * {
    min-width: 0;
  }
"#;

const ROW_NO_WRAP_STYLE: &str = r#"
  row-l[layout~="nowrap"]  {
      flex-wrap:nowrap;
  }
"#;

const ROW_TWIN_WIDTH_STYLE: &str = r#"
  row-l[layout~="twin-width"] > * {
      flex-grow:1;
      flex-basis:0;
      min-width: 0;
  }
"#;

fn row_direction_style(value: &str) -> String {
    formatdoc!(
        r#"
        row-l[layout~="direction:{value}"]{{
            flex-direction: {value};
        }}
        "#,
    )
}

fn row_justify_style(value: &str) -> String {
    formatdoc!(
        r#"
        row-l[layout~="justify:{value}"]{{
            justify-content: {value};
        }}
        "#,
    )
}

fn row_align_style(value: &str) -> String {
    formatdoc!(
        r#"
        row-l[layout~="align:{value}"]{{
            align-items: {value};
        }}
        "#,
    )
}
fn row_gap_style(value: &str, harmonic: String) -> String {
    formatdoc!(
        r#"
        row-l[layout~="gap:{value}"]{{
            gap: {harmonic};
        }}
        "#,
    )
}

fn row_gap_x_style(value: &str, harmonic: String) -> String {
    formatdoc!(
        r#"
        row-l[layout~="gap-x:{value}"]{{
            column-gap: {harmonic};
        }}
        "#,
    )
}

fn row_gap_y_style(value: &str, harmonic: String) -> String {
    formatdoc!(
        r#"
        row-l[layout~="gap-y:{value}"]{{
            row-gap: {harmonic};
        }}
        "#,
    )
}

pub fn row_css(
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
    set.insert(ROW_STYLE.to_string());

    if nowrap {
        set.insert(ROW_NO_WRAP_STYLE.to_string());
    }
    if twin_width {
        set.insert(ROW_TWIN_WIDTH_STYLE.to_string());
    }
    if let Some(value) = direction {
        set.insert(row_direction_style(value));
    }
    if let Some(value) = justify {
        set.insert(row_justify_style(value));
    }
    if let Some(value) = align {
        set.insert(row_align_style(value));
    }
    if let Some(value) = gap {
        let harmonic_value = get_harmonic(value, harmonic_ratio);
        set.insert(row_gap_style(value, harmonic_value));
    }
    if let Some(ref value) = gap_x {
        let harmonic_value = get_harmonic(value, harmonic_ratio);
        set.insert(row_gap_x_style(value, harmonic_value));
    }
    if let Some(ref value) = gap_y {
        let harmonic_value = get_harmonic(value, harmonic_ratio);
        set.insert(row_gap_y_style(value, harmonic_value));
    }
}
