use indoc::formatdoc;
use crate::harmonic::get_harmonic;

use std::collections::HashSet;
const STACK_STYLE: &str = r#"
  stack-l{
    display: block;
  }

  stack-l > *{
    margin-block: 0;
  }
"#;

fn stack_gap_style(value: &str, harmonic: f64) -> String {
    formatdoc!(
        r#"
        stack-l[layout~="gap:{value}"] > * + *:not(outsider-l){{
            margin-block-start: {harmonic:.2}rem;
        }}
        "#,
    )
}

fn stack_recursive_style(harmonic: f64) -> String {
    formatdoc!(
        r#"
        stack-l[layout~="recursive"] * + *:not(outsider-l){{
            margin-block-start: {harmonic:.2}rem;
        }}
        "#,
    )
}

pub fn stack_css(
    gap: Option<&str>,
    recursive: bool,
    harmonic_ratio: f64,
    set: &mut HashSet<String>,
) {
    set.insert(STACK_STYLE.to_string());
    if let Some(value) = gap {
        let harmonic_value = get_harmonic(value, harmonic_ratio);
        set.insert(stack_gap_style(value, harmonic_value));
        if recursive {
            set.insert(stack_recursive_style(harmonic_value));
        }
    }
}