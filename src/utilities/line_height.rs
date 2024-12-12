use indoc::formatdoc;
use std::collections::HashSet;
pub fn line_height_css(value: &str, set: &mut HashSet<String>) {
    set.insert(formatdoc!(
        r#"
        [layout~="line-height:{value}"]{{
            line-height: {value};
        }}
        "#
    ));
}
