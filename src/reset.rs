use indoc::formatdoc;

use crate::config::LayoutStyleConfig;

pub const RESET_CSS: &str = r#"
* {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
}

img {
    display: block;
    width:100%;
}

a {
    all: unset;
    display: inline-block;
    cursor: pointer;
}


input {
    display: block;
    width:100%;
}

button{
    cursor: pointer;
    color:inherit;
    border:none;
    font:inherit;
}

span, strong, label{
    display: inline-block;
}

"#;

pub fn get_font_size(config: &LayoutStyleConfig) -> String {
    let first = "1200px";
    let second = "600px";

    let base_value = &config.base_value;
    let base_value_unit: f64 = base_value.trim_end_matches("px").parse().unwrap();
    let resizing_ratio = config.resizing_ratio;

    // Extract numeric parts by trimming the "px" suffix and parsing to integers
    let first_num: i32 = first.trim_end_matches("px").parse().unwrap();
    let second_num: i32 = second.trim_end_matches("px").parse().unwrap();

    let interval = first_num - second_num;
    let substracted_resizing_ratio = config.resizing_ratio - 1.0;
    let relative_screen_width = format!(r#"(100vw - {})"#, config.min_screen);
    let denominateur = format!(r#"({} / {})"#, relative_screen_width, interval);
    let final_denominateur = format!(
        r#"({:.2} * {} + 1px) / {}"#,
        substracted_resizing_ratio, denominateur, config.resizing_ratio,
    );

    let font_size = formatdoc!(
        r#"
        html{{
            font-size: clamp(
            calc({base_value} / {resizing_ratio}),
            calc({base_value_unit} * {final_denominateur}),
            {base_value});
        }}
      "#
    );

    font_size
}

pub fn reset_css(config: &LayoutStyleConfig) -> String {
    let font_size = get_font_size(config);
    format!("{RESET_CSS}\n{font_size}")
}
