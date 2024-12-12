use indoc::formatdoc;
use std::collections::HashSet;
pub fn flex_basis_css(value: &str, set: &mut HashSet<String>) {
    set.insert(formatdoc!(
        r#"
        [layout~="flex-basis:{value}"]{{
            flex-basis: {value};
        }}
        "#
    ));
}

pub fn flex_grow_css(value: &str, set: &mut HashSet<String>) {
    set.insert(formatdoc!(
        r#"
        [layout~="flex-grow:{value}"]{{
            flex-grow: {value};
        }}
        "#
    ));
}

pub fn flex_shrink_css(value: &str, set: &mut HashSet<String>) {
    set.insert(formatdoc!(
        r#"
        [layout~="flex-shrink:{value}"]{{
            flex-shrink: {value};
        }}
        "#
    ));
}
