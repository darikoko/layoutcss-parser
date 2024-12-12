use indoc::formatdoc;
use std::collections::HashSet;
pub fn hide_over_css(value: &str, set: &mut HashSet<String>) {
    set.insert(formatdoc!(
        r#"
        @media screen and (min-width: {value}) {{
            [layout~="hide-over:{value}"]{{
                display:none;
            }}
        }}
        "#
    ));
}

pub fn hide_under_css(value: &str, set: &mut HashSet<String>) {
    set.insert(formatdoc!(
        r#"
        @media screen and (max-width: {value}) {{
            [layout~="hide-under:{value}"]{{
                display:none;
            }}
        }}
        "#
    ));
}
