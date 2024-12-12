use indoc::formatdoc;
use std::collections::HashSet;
const RATIO_STYLE: &str = r#"
    img[layout~="ratio"],video[layout~="ratio"] {
    inline-size: 100%;
    object-fit: cover;
    }
"#;

pub fn ratio_css(value: &str, set: &mut HashSet<String>) {
    set.insert(RATIO_STYLE.to_string());
    set.insert(formatdoc!(
        r#"
        [layout~="ratio:{value}"]{{
            aspect-ratio: {value};
        }}
        "#
    ));
}
