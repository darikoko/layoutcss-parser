use indoc::formatdoc;
use std::collections::HashSet;
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

fn outsider_top_style(value: &str) -> String {
    formatdoc!(
        r#"
        outsider-l[layout~="top:{value}"]{{
            top:{value};
        }}
        "#,
    )
}

fn outsider_bottom_style(value: &str) -> String {
    formatdoc!(
        r#"
        outsider-l[layout~="bottom:{value}"]{{
            bottom:{value};
        }}
        "#,
    )
}

fn outsider_left_style(value: &str) -> String {
    formatdoc!(
        r#"
        outsider-l[layout~="left:{value}"]{{
            left:{value};
        }}
        "#,
    )
}

fn outsider_right_style(value: &str) -> String {
    formatdoc!(
        r#"
        outsider-l[layout~="right:{value}"]{{
            right:{value};
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
    set: &mut HashSet<String>,
) {
    set.insert(OUTSIDER_STYLE.to_string());
    if let Some(value) = position {
        set.insert(outsider_position_style(value));
    }
    if let Some(value) = top {
        set.insert(outsider_top_style(value));
    }
    if let Some(value) = bottom {
        set.insert(outsider_bottom_style(value));
    }
    if let Some(value) = left {
        set.insert(outsider_left_style(value));
    }
    if let Some(value) = right {
        set.insert(outsider_right_style(value));
    }
}
