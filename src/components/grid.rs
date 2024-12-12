use indoc::formatdoc;
use std::collections::HashSet;

use crate::harmonic::get_harmonic;

const GRID_STYLE: &str = r#"
  grid-l{
    display: grid;
  }
"#;

fn grid_gap_style(value: &str, harmonic: f64) -> String {
    formatdoc!(
        r#"
        grid-l[layout~="gap:{value}"]{{
            gap: {harmonic:.2}rem;
        }}
        "#,
    )
}

fn grid_gap_x_style(value: &str, harmonic: f64) -> String {
    formatdoc!(
        r#"
        grid-l[layout~="gap-x:{value}"]{{
            column-gap: {harmonic:.2}rem;
        }}
        "#,
    )
}

fn grid_gap_y_style(value: &str, harmonic: f64) -> String {
    formatdoc!(
        r#"
        grid-l[layout~="gap-y:{value}"]{{
            row-gap: {harmonic:.2}rem;
        }}
        "#,
    )
}

fn grid_group_empty(min_cell_width: &str) -> String {
    formatdoc!(
        r#"
        grid-l[layout*="min-cell-width:{min_cell_width}"] {{
            grid-template-columns: repeat(auto-fit, minmax(min({min_cell_width}, 100%),1fr));
        }}
        "#,
    )
}

fn grid_group_max_cols(min_cell_width: &str, max_cols: &str, gap_delta_max: f64) -> String {
    formatdoc!(
        r#"
        grid-l[layout*="min-cell-width:{min_cell_width}"][layout*="max-cols:{max_cols}"]{{
            grid-template-columns: repeat(auto-fit, minmax(min(100%, max({min_cell_width}, (100% / {max_cols} - {gap_delta_max}rem))),1fr));
        }}
        "#,
    )
}

fn grid_group_min_cols(min_cell_width: &str, min_cols: &str, gap_delta_min: f64) -> String {
    formatdoc!(
        r#"
        grid-l[layout*="min-cell-width:{min_cell_width}"][layout*="min-cols:{min_cols}"]:has(:nth-child({min_cols})){{
            grid-template-columns: repeat(auto-fit, minmax(min((100% / {min_cols} - {gap_delta_min}rem), {min_cell_width}), 1fr));
        }}
        grid-l[layout*="min-cell-width:{min_cell_width}"][layout*="min-cols:{min_cols}"]{{
            grid-template-columns: repeat({min_cols}, 1fr);
        }}
        "#,
    )
}

fn grid_group_min_cols_max_cols(
    min_cell_width: &str,
    min_cols: &str,
    max_cols: &str,
    gap_delta_min: f64,
    gap_delta_max: f64,
    fr: f64,
) -> String {
    formatdoc!(
        r#"
        grid-l[layout*="min-cell-width:{min_cell_width}"][layout*="min-cols:{min_cols}"][layout*="max-cols:{max_cols}"]:has(:nth-child({min_cols})){{
            grid-template-columns:
                repeat(auto-fit,
                    minmax(
                        min(
                            (100% / {min_cols} - {gap_delta_min}rem),
                                max({min_cell_width}, (100% / {max_cols} - {gap_delta_max}rem))
                            ),
                            {fr}fr
                            )
                        )
                    }}
        grid-l[layout*="min-cell-width:{min_cell_width}"][layout*="min-cols:{min_cols}"][layout*="max-cols:{max_cols}"]{{
            grid-template-columns: repeat({min_cols}, 1fr);
        }}
        "#,
    )
}

fn gap_delta(cols: &str, gap: Option<&str>, harmonic_ratio: f64) -> f64 {
    if let Some(value) = gap {
        match cols.parse::<f64>() {
            Ok(val) => get_harmonic(&value, harmonic_ratio) * (val - 0.98) / val,
            Err(_) => 0.0,
        }
    } else {
        0.0
    }
}

pub fn grid_css(
    min_cell_width: Option<&str>,
    min_cols: Option<&str>,
    max_cols: Option<&str>,
    gap: Option<&str>,
    gap_x: Option<&str>,
    gap_y: Option<&str>,
    harmonic_ratio: f64,
    set: &mut HashSet<String>,
) {
    set.insert(GRID_STYLE.to_string());
    if let Some(ref value) = gap {
        let harmonic_value = get_harmonic(value, harmonic_ratio);
        set.insert(grid_gap_style(value, harmonic_value));
    }
    if let Some(ref value) = gap_x {
        let harmonic_value = get_harmonic(value, harmonic_ratio);
        set.insert(grid_gap_x_style(value, harmonic_value));
    }
    if let Some(ref value) = gap_y {
        let harmonic_value = get_harmonic(value, harmonic_ratio);
        set.insert(grid_gap_y_style(value, harmonic_value));
    }
    if let Some(min_cell_width) = min_cell_width {
        match (min_cols, max_cols) {
            (Some(min_cols), Some(max_cols)) => {
                let gap_delta_min = gap_delta(min_cols, gap, harmonic_ratio);
                let gap_delta_max = gap_delta(max_cols, gap, harmonic_ratio);
                let fr = 1.0 / min_cols.parse::<f64>().unwrap_or(-1.0);
                set.insert(grid_group_min_cols_max_cols(
                    min_cell_width,
                    min_cols,
                    max_cols,
                    gap_delta_min,
                    gap_delta_max,
                    fr,
                ));
            }
            (Some(min_cols), None) => {
                let gap_delta_min = gap_delta(min_cols, gap, harmonic_ratio);
                set.insert(grid_group_min_cols(min_cell_width, min_cols, gap_delta_min));
            }
            (None, Some(max_cols)) => {
                let gap_delta_max = gap_delta(max_cols, gap, harmonic_ratio);
                set.insert(grid_group_max_cols(min_cell_width, max_cols, gap_delta_max));
            }
            _ => {
                set.insert(grid_group_empty(min_cell_width));
            }
        }
    }
}
