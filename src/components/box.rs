use indoc::formatdoc;
use std::collections::HashSet;

const BOX_STYLE: &str = r#"
  box-l{
    box-sizing: border-box;
    display: block;
    max-inline-size:fit-content;
  }
"#;

const BOX_GROW_STYLE: &str = r#"
  box-l[layout~="grow"] > *{
    width: 100%;
  }
"#;

fn box_max_width_style(value: &str) -> String {
    formatdoc!(
        r#"
        box-l[layout~="max-width:{value}"]{{
            max-inline-size:{value};
        }}
        "#,
    )
}

pub fn box_css(max_width: Option<&str>, grow: bool, set: &mut HashSet<String>) {
    set.insert(BOX_STYLE.to_string());
    if let Some(value) = max_width {
        set.insert(box_max_width_style(value));
    }
    if grow {
        set.insert(BOX_GROW_STYLE.to_string());
    }
}
