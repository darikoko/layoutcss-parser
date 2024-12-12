use indoc::formatdoc;
use crate::harmonic::get_harmonic;
use std::collections::HashSet;
const SLIDER_STYLE: &str = r#"
  slider-l{
    display: flex;
    block-size: auto;
    overflow-x: auto;
    overflow-y: hidden;
  }

  slider-l > *:not(outsider-l) {
      flex-shrink: 0;
      flex-grow: 0;
      height: auto;
      min-width: 0px;
  }

  slider-l > img{
      object-fit: cover;
  }
"#;

const SLIDER_HIDE_BAR_STYLE: &str = r#"
  slider-l[layout~="hide-bar"]{
    overflow: hidden;
  }
"#;

fn slider_item_width_style(value: &str) -> String {
    formatdoc!(
        r#"
        slider-l[layout~="item-width:{value}"] > *:not(outsider-l){{
            flex-basis:{value};
        }}
        "#,
    )
}

fn slider_height_style(value: &str) -> String {
    formatdoc!(
        r#"
        slider-l[layout~="height:{value}"] > *:not(outsider-l){{
            block-size:{value};
        }}
        "#,
    )
}

fn slider_gap_style(value: &str, harmonic: f64) -> String {
    formatdoc!(
        r#"
        slider-l[layout~="gap:{value}"] {{
            gap: {harmonic:.2}rem;
        }}
        "#,
    )
}

pub fn slider_css(
    hide_bar: bool,
    item_width: Option<&str>,
    height: Option<&str>,
    gap: Option<&str>,
    harmonic_ratio: f64,
    set: &mut HashSet<String>,
) {
    set.insert(SLIDER_STYLE.to_string());
    if hide_bar {
        set.insert(SLIDER_HIDE_BAR_STYLE.to_string());
    }
    if let Some(value) = item_width {
        set.insert(slider_item_width_style(value));
    }
    if let Some(value) = height {
        set.insert(slider_height_style(value));
    }

    if let Some(value) = gap {
        let harmonic_value = get_harmonic(value, harmonic_ratio);
        set.insert(slider_gap_style(value, harmonic_value));
    }
}
