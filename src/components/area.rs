use indoc::formatdoc;
use crate::harmonic::get_harmonic;
use std::collections::HashSet;

pub const AREA_STYLE: &str = r#"
area-l{
        display: grid;
    }
"#;

fn area_gap_style(value: &str, harmonic: f64) -> String {
    formatdoc!(
        r#"
        area-l[layout~="gap:{value}"]{{
            gap: {harmonic:.2}rem;
        }}
        "#,
    )
}

fn area_gap_x_style(value: &str, harmonic: f64) -> String {
    formatdoc!(
        r#"
        area-l[layout~="gap-x:{value}"]{{
            column-gap: {harmonic:.2}rem;
        }}
        "#,
    )
}

fn area_gap_y_style(value: &str, harmonic: f64) -> String {
    formatdoc!(
        r#"
        area-l[layout~="gap-y:{value}"]{{
            row-gap: {harmonic:.2}rem;
        }}
        "#,
    )
}

fn area_grid_template_areas_style(value: &str, template: String) -> String {
    formatdoc!(
        r#"
        area-l[layout~="template:{value}"] {{
                grid-template-areas: {template};
            }}
        "#,
    )
}

fn area_grid_area_unit_style(value: &str, unit: char) -> String {
    formatdoc!(
        r#"
        area-l[layout~="template:{value}"] > :nth-child({unit}) {{
            grid-area: {unit};
        }}
        "#,
    )
}

fn area_rows_style(selector: &str, value: &str) -> String {
    formatdoc!(
        r#"
        area-l{selector}{{
            grid-template-rows: {value};
        }}
        "#,
    )
}

fn area_cols_style(selector: &str, value: &str) -> String {
    formatdoc!(
        r#"
        area-l{selector}{{
            grid-template-columns: {value};
        }}
        "#,
    )
}

/// return the number of rows and cols from a template layout class in a tuple like this (rows,cols)
/// for example "(a-a-b|a-a-b)" will returns (2,3) because their is 2 rows and 3 columns
fn count_rows_and_cols(text: &str) -> (usize, usize) {
    let rows = text.chars().filter(|c| *c == '|').count() + 1;
    let cols = text
        .chars()
        .take_while(|c| *c != '|')
        .filter(|c| *c == '-')
        .count()
        + 1;
    (rows, cols)
}

/// return the grid-template-areas value for
/// a specific template value, so "(a-a-b|a-a-b)" return "\"a a b\" \"a a b\""
fn grid_template_areas_value(text: &str) -> String {
    let mut areas = Vec::new();
    let text_without_parentheses = text.replace("(", "").replace(")", "");
    for part in text_without_parentheses.split('|') {
        let area = part
            .chars()
            .filter(|c| *c != '-')
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        areas.push(formatdoc!("\"{}\"", area));
    }
    areas.join(" ")
}

/// return the grid-template-columns value (if pattern is "col-") or grid-template-rows value (if pattern is "row-")
/// items is the list of col or row classes, pattern is "col-" or "row-" and number is the number of row or col.
fn grid_template_rows_or_cols_rule(items: &Vec<&str>, pattern: &str, number: usize) -> String {
    let mut rules: Vec<String> = vec![];
    // we iterate on 1 to number and for each value
    // we will check if there is col-x:... or row-x:... associated
    // if it's the case it will take the value after the ':'
    // else it will use 1fr
    for i in 1..=number {
        let pattern = formatdoc!("{}{}", pattern, i.to_string());
        if let Some(item) = items.iter().find(|&s| s.starts_with(&pattern)) {
            let parts: Vec<&str> = item.split(':').collect();
            if let Some(value) = parts.get(1) {
                rules.push(value.to_string());
            }
        } else {
            rules.push("1fr".to_string());
        }
    }
    rules.join(" ")
}

fn grid_template_rows_or_cols_selector(items: &Vec<&str>) -> String {
    let formatted_items: Vec<String> = items
        .iter()
        .map(|item| formatdoc!(r#"[layout~="{}"]"#, item))
        .collect();
    formatted_items.join("")
}

fn unique_letters(input: &str) -> Vec<char> {
    let mut unique_chars = Vec::new();
    for c in input.chars() {
        // Exclude '.' and check for uniqueness
        if c.is_alphabetic() && !unique_chars.contains(&c) {
            unique_chars.push(c);
        }
    }
    unique_chars.sort();
    unique_chars
}

pub fn area_css(
    template: Option<&str>,
    rows: Vec<&str>,
    cols: Vec<&str>,
    gap: Option<&str>,
    gap_x: Option<&str>,
    gap_y: Option<&str>,
    harmonic_ratio: f64,
    set: &mut HashSet<String>,
) {
    set.insert(AREA_STYLE.to_string());
    if let Some(template) = template {
        let template_areas = grid_template_areas_value(template);
        set.insert(area_grid_template_areas_style(template, template_areas));
        for letter in unique_letters(template) {
            set.insert(area_grid_area_unit_style(template, letter));
        }

        let (rows_nb, cols_nb) = count_rows_and_cols(template);
        if !rows.is_empty() {
            let selector = grid_template_rows_or_cols_selector(&rows);
            let value = grid_template_rows_or_cols_rule(&rows, "row-", rows_nb);
            set.insert(area_rows_style(&selector, &value));
        }
        if !cols.is_empty() {
            let selector = grid_template_rows_or_cols_selector(&cols);
            let value = grid_template_rows_or_cols_rule(&cols, "col-", cols_nb);
            set.insert(area_cols_style(&selector, &value));
        }
    }

    if let Some(value) = gap {
        let harmonic_value = get_harmonic(value, harmonic_ratio);
        set.insert(area_gap_style(value, harmonic_value));
    }
    if let Some(value) = gap_x {
        let harmonic_value = get_harmonic(value, harmonic_ratio);
        set.insert(area_gap_x_style(value, harmonic_value));
    }
    if let Some(value) = gap_y {
        let harmonic_value = get_harmonic(value, harmonic_ratio);
        set.insert(area_gap_y_style(value, harmonic_value));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo_css_area() {
        let mut css_set: HashSet<String> = HashSet::new();
        area_css(
            Some("(a-a-b-b-b|a-a-b-b-b)"),
            vec!["row-1:800px"],
            vec!["col-2:300px", "col-4:80px"],
            Some("1"),
            None,
            None,
            1.618,
            &mut css_set,
        );
        println!("{:?}", css_set);
    }
}
/*
struct AreaTemplate {
    rows: usize,
    columns: usize,
}
pub const AREA_STYLE: &str = r#"
area-l{
        display: grid;
    }
"#;

fn extract_chars(text: &str) -> Vec<char> {
    let char_list: Vec<char> = text
        .chars()
        .filter(|&c| c.is_alphabetic()) // Keep only letters
        .collect();

    char_list
}

fn grid_template_area_value(chars: Vec<char>, rows_number: usize) -> String {
    let chunks: Vec<String> = chars
        .chunks(rows_number) // Create groups of n characters
        .map(|chunk| chunk.iter().collect::<String>()) // Collect each group into a string
        .map(|group| formatdoc!("\"{}\"", group.replace("", " ").trim())) // Add quotes and spaces
        .collect();

    chunks.join(" ")
}

fn area_group_style(value: &str, template: String) -> String {
    formatdoc!(
        r#"
        area-l[layout~="template:{value}"] {{
                grid-template-areas: {template};
            }}
        "#,
    )
}

fn area_group_unit_style(value: &str, child_number: usize, letter: &char) -> String {
    formatdoc!(
        r#"
        area-l[layout~="template:{value}"] > :nth-child({child_number}) {{
            grid-area: {letter};
        }}
        "#,
    )
}

fn area_gap_style(value: &str, harmonic: f64) -> String {
    formatdoc!(
        r#"
        area-l[layout~="gap:{value}"]{{
            gap: {harmonic:.2}rem;
        }}
        "#,
    )
}

fn area_gap_x_style(value: &str, harmonic: f64) -> String {
    formatdoc!(
        r#"
        area-l[layout~="gap-x:{value}"]{{
            column-gap: {harmonic:.2}rem;
        }}
        "#,
    )
}

fn area_gap_y_style(value: &str, harmonic: f64) -> String {
    formatdoc!(
        r#"
        area-l[layout~="gap-y:{value}"]{{
            row-gap: {harmonic:.2}rem;
        }}
        "#,
    )
}

fn get_unique_letters(chars: &Vec<char>) -> Vec<char> {
    let mut unique_chars = Vec::new(); // Create an empty Vec to track unique characters

    for &ch in chars {
        if ch != '.' && !unique_chars.contains(&ch) {
            // Exclude '.' and check for uniqueness
            unique_chars.push(ch); // Add unique characters to the Vec
        }
    }
    unique_chars
}

fn get_template_rule(items: &Vec<&str>, pattern: &str, number: usize) -> String {
    let mut rules: Vec<String> = vec![];
    for i in 1..=number {
        let pattern = formatdoc!("{}{}", pattern, i.to_string());
        if let Some(item) = items.iter().find(|&s| s.starts_with(&pattern)) {
            let parts: Vec<&str> = item.split(':').collect();
            if let Some(value) = parts.get(1) {
                rules.push(value.to_string());
            }
        } else {
            rules.push("1fr".to_string());
        }
    }
    rules.join(" ")
}

fn parse_grid_string(input: &str) -> String {
    // Split the input into the grid definition and the size
    if let Some((grid_def, size)) = input.split_once(':') {
        // Handle both "col-" and "row-" prefixes
        if let Some(n_items) = grid_def
            .strip_prefix("col-")
            .or_else(|| grid_def.strip_prefix("row-"))
        {
            if let Ok(n) = n_items.parse::<usize>() {
                // Generate the resulting string
                let mut result = vec!["1fr"; n];
                if n > 0 {
                    result[n - 1] = size; // Replace the nth element with the size
                }
                return result.join(" ");
            }
        }
    }
    String::new() // Return an empty string in case of an error
}

fn get_template_selector(items: &Vec<&str>) -> String {
    let formatted_items: Vec<String> = items
        .iter()
        .map(|item| formatdoc!(r#"[layout~="{}"]"#, item))
        .collect();
    formatted_items.join("")
}

fn count_rows_and_cols(text: &str) -> (usize, usize) {
    let rows = text.chars().filter(|c| *c == '|').count() + 1;
    let cols = text.chars().filter(|c| *c == '-').count() / rows + 1;
    (rows, cols)
}

pub fn area_css(
    template: Option<&str>,
    gap: Option<&str>,
    gap_x: Option<&str>,
    gap_y: Option<&str>,
    rows: Vec<&str>,
    cols: Vec<&str>,
    harmonic_ratio: f64,
    set: &mut HashSet<String>,
) {
    if let Some(t) = template {
        // we have to generate the grid-template-area
        // and each grid area according the number of unique letter
        let chars = extract_chars(t);
        let unique_letters = get_unique_letters(&chars);
        let template_value = grid_template_area_value(chars, unique_letters.len());
        set.insert(area_group_style(t, template_value));
        for (i, c) in unique_letters.iter().enumerate() {
            set.insert(area_group_unit_style(t, i, c));
        }
    }
    if !rows.is_empty() {
        let selector = get_template_selector(&rows);
        let value = get_template_rule(&rows, "row-", rows_nb);
        set.insert(area_rows_style(&selector, &value))
    }
    if !cols.is_empty() {
        let selector = get_template_selector(&cols);
        let value = parse_grid_string(&cols, "col-", cols_nb);
        set.insert(area_cols_style(&selector, &value))
    }
    if let Some(ref value) = gap {
        let harmonic_value = get_harmonic(value, harmonic_ratio);
        set.insert(area_gap_style(value, harmonic_value));
    }
    if let Some(ref value) = gap_x {
        let harmonic_value = get_harmonic(value, harmonic_ratio);
        set.insert(area_gap_x_style(value, harmonic_value));
    }
    if let Some(ref value) = gap_y {
        let harmonic_value = get_harmonic(value, harmonic_ratio);
        set.insert(area_gap_y_style(value, harmonic_value));
    }
}


*/
