use std::collections::HashSet;
const RELATIVE_STYLE: &str = r#"
    [layout~="relative"] {
        position: relative;
        min-height: 1px;
    }
"#;

pub fn relative_css(set: &mut HashSet<String>) {
    set.insert(RELATIVE_STYLE.to_string());
}
