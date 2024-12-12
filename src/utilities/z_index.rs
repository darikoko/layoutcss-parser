use indoc::formatdoc;
use std::collections::HashSet;
pub fn z_index_css(value: &str, set: &mut HashSet<String>) {
    set.insert(formatdoc!(
        r#"
        [layout~="z-index:{value}"]{{
            z-index: {value};
        }}
        "#
    ));
}
