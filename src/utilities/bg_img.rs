use indoc::formatdoc;
use std::collections::HashSet;
const BG_IMG_STYLE: &str = r#"
    [layout*="bg-img"]{
        background-origin: border-box;
        background-repeat: no-repeat;
        background-position: center;
        background-size: cover;
    }
"#;

pub fn bg_img_css(value: &str, set: &mut HashSet<String>) {
    set.insert(BG_IMG_STYLE.to_string());
    set.insert(formatdoc!(
        r#"
        [layout~="bg-img:{value}"]{{
            background-image: url({value});
        }}
        "#
    ));
}
