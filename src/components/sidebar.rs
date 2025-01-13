use indoc::formatdoc;
use crate::harmonic::get_harmonic;

use std::collections::HashSet;
const SIDEBAR_STYLE: &str = r#"
  sidebar-l{
    display: flex;
    flex-wrap: wrap;
  }
"#;

const SIDEBAR_REVERSE_STYLE: &str = r#"
  sidebar-l[layout~=reverse]{
    flex-wrap: wrap-reverse;
  }
"#;

fn sidebar_shrink_style(reverse: bool) -> String {
    formatdoc!(
        r#"
        sidebar-l[layout~=shrink]{{
            align-items: flex-{};
        }}
        "#,
        if reverse { "end" } else { "start" }
    )
}

fn sidebar_gap_style(value: &str, harmonic: String) -> String {
    formatdoc!(
        r#"
        sidebar-l[layout~="gap:{value}"]{{
            gap: {harmonic};
        }}
        "#,
    )
}

fn sidebar_gap_x_style(value: &str, harmonic: String) -> String {
    formatdoc!(
        r#"
        sidebar-l[layout~="gap-x:{value}"]{{
            column-gap: {harmonic};
        }}
        "#,
    )
}

fn sidebar_gap_y_style(value: &str, harmonic: String) -> String {
    formatdoc!(
        r#"
        sidebar-l[layout~="gap-y:{value}"]{{
            row-gap: {harmonic};
        }}
        "#,
    )
}

fn sidebar_group_style(
    // they are String instead of &str
    // because they have already been replaced
    side_selector: String,
    side_width_selector: String,
    content_min_selector: String,
    selector_one: &str,
    selector_two: &str,
    side_width: &str,
    content_min: &str,
) -> String {
    formatdoc!(
        r#"
        sidebar-l{side_selector}{side_width_selector}{content_min_selector} > {selector_one}:not(outsider-l) {{
              flex-basis: {side_width};
              flex-grow: 1;
              min-inline-size: initial;
              min-width:0;
              min-height:0;
        }}

        sidebar-l{side_selector}{side_width_selector}{content_min_selector} > {selector_two}:not(outsider-l) {{
                flex-basis: 0;
                flex-grow: 999;
                min-inline-size: {content_min};
        }}
        "#,
    )
}

pub fn sidebar_css(
    reverse: bool,
    shrink: bool,
    side: Option<&str>,
    side_width: Option<&str>,
    content_min: Option<&str>,
    gap: Option<&str>,
    gap_x: Option<&str>,
    gap_y: Option<&str>,
    harmonic_ratio: f64,
    set: &mut HashSet<String>,
) {
    set.insert(SIDEBAR_STYLE.to_string());
    if reverse {
        set.insert(SIDEBAR_REVERSE_STYLE.to_string());
    }
    if shrink {
        set.insert(sidebar_shrink_style(reverse));
    }
    if let Some(value) = gap {
        let harmonic_value = get_harmonic(&value, harmonic_ratio);
        set.insert(sidebar_gap_style(value, harmonic_value));
    }
    if let Some(ref value) = gap_x {
        let harmonic_value = get_harmonic(&value, harmonic_ratio);
        set.insert(sidebar_gap_x_style(value, harmonic_value));
    }
    if let Some(ref value) = gap_y {
        let harmonic_value = get_harmonic(&value, harmonic_ratio);
        set.insert(sidebar_gap_y_style(value, harmonic_value));
    }
    if side.is_some() || side_width.is_some() || content_min.is_some() {
        let side_width_selector = if let Some(ref val) = side_width {
            formatdoc!(r#"[layout*="side-width:{}"]"#, val)
        } else {
            "".to_string()
        };
        let content_min_selector = if let Some(ref val) = content_min {
            formatdoc!(r#"[layout*="content-min:{}"]"#, val)
        } else {
            "".to_string()
        };
        let side_selector = if let Some(ref val) = side {
            formatdoc!(r#"[layout*="side:{}"]"#, val)
        } else {
            "".to_string()
        };

        let selector_one = match side {
            None => ":first-child",
            Some(val) if val == "left" => ":first-child",
            _ => ":last-child",
        };
        let selector_two = if selector_one == ":first-child" {
            ":last-child"
        } else {
            ":first-child"
        };
        let content_min = if let Some(val) = content_min {
            val
        } else {
            "50%"
        };
        let side_width = if let Some(val) = side_width {
            val
        } else {
            "auto"
        };

        set.insert(sidebar_group_style(
            side_selector,
            side_width_selector,
            content_min_selector,
            selector_one,
            selector_two,
            side_width,
            content_min,
        ));
    }
}
