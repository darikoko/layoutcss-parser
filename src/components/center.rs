use indoc::formatdoc;
use std::collections::HashSet;

const CENTER_STYLE: &str = r#"
center-l{
    box-sizing: content-box;
    max-inline-size: fit-content;
    margin-inline: auto;
    display: block;
    text-align: initial;
  }
"#;

const CENTER_AND_TEXT_STYLE: &str = r#"
  center-l[layout~="and-text"]{
    text-align:center;
  }
"#;

const CENTER_RECURSIVE_STYLE: &str = r#"
  center-l[layout~="recursive"]{
    display:flex;
    flex-direction:column;
    align-items:center;
  }
"#;

fn center_max_width_style(value: &str) -> String {
    formatdoc!(
        r#"
        center-l[layout~="max-width:{value}"]{{
            max-inline-size: {value};
            --center-max-width: {value};
        }}
        "#,
    )
}

pub fn center_css(
    max_width: Option<&str>,
    and_text: bool,
    recursive: bool,
    set: &mut HashSet<String>,
) {
    set.insert(CENTER_STYLE.to_string());
    if let Some(value) = max_width {
        set.insert(center_max_width_style(value));
    }
    if and_text {
        set.insert(CENTER_AND_TEXT_STYLE.to_string());
    }
    if recursive {
        set.insert(CENTER_RECURSIVE_STYLE.to_string());
    }
}
