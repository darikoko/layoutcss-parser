use indoc::formatdoc;
use std::collections::HashSet;

pub fn align_self_css(value: &str, set: &mut HashSet<String>) {
    set.insert(formatdoc!(
        r#"
        [layout~="align-self:{value}"]{{
            align-self: {value};
        }}
        "#
    ));
}
