use std::{collections::HashSet, hash::Hash, str::FromStr};

use area::area_css;
use center::center_css;
use extender::extender_css;
use grid::grid_css;
use icon::icon_css;
use ledge::ledge_css;
use outsider::outsider_css;
use r#box::box_css;
use rack::rack_css;
use sidebar::sidebar_css;
use slider::slider_css;
use stack::stack_css;
use switcher::switcher_css;

pub mod area;
pub mod r#box;
pub mod center;
pub mod extender;
pub mod grid;
pub mod icon;
pub mod ledge;
pub mod outsider;
pub mod rack;
pub mod sidebar;
pub mod slider;
pub mod stack;
pub mod switcher;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Component<'a> {
    Area {
        template: Option<&'a str>,
        rows: Vec<&'a str>,
        cols: Vec<&'a str>,
        gap: Option<&'a str>,
        gap_x: Option<&'a str>,
        gap_y: Option<&'a str>,
    },
    Box {
        max_width: Option<&'a str>,
        grow: bool,
    },
    Center {
        max_width: Option<&'a str>,
        and_text: bool,
        recursive: bool,
    },
    Extender {
        screen: bool,
        keep_center: bool,
        keep_p: bool,
        keep_pl: bool,
        keep_pr: bool,
    },
    Grid {
        min_cell_width: Option<&'a str>,
        min_cols: Option<&'a str>,
        max_cols: Option<&'a str>,
        gap: Option<&'a str>,
        gap_x: Option<&'a str>,
        gap_y: Option<&'a str>,
    },
    Icon {
        scale: Option<&'a str>,
        align: Option<&'a str>,
        gap_dir: Option<&'a str>,
        gap: Option<&'a str>,
    },
    Ledge {
        nowrap: bool,
        twin_width: bool,
        direction: Option<&'a str>,
        justify: Option<&'a str>,
        align: Option<&'a str>,
        gap: Option<&'a str>,
        gap_x: Option<&'a str>,
        gap_y: Option<&'a str>,
    },
    Outsider {
        position: Option<&'a str>,
        top: Option<&'a str>,
        bottom: Option<&'a str>,
        left: Option<&'a str>,
        right: Option<&'a str>,
    },
    Rack {
        height: Option<&'a str>,
        min_height: Option<&'a str>,
        max_height: Option<&'a str>,
        gap: Option<&'a str>,
    },
    Sidebar {
        reverse: bool,
        shrink: bool,
        side: Option<&'a str>,
        side_width: Option<&'a str>,
        content_min: Option<&'a str>,
        gap: Option<&'a str>,
        gap_x: Option<&'a str>,
        gap_y: Option<&'a str>,
    },
    Slider {
        hide_bar: bool,
        item_width: Option<&'a str>,
        height: Option<&'a str>,
        gap: Option<&'a str>,
    },
    Stack {
        gap: Option<&'a str>,
        recursive: bool,
    },
    Switcher {
        threshold: Option<&'a str>,
        limit: Option<&'a str>,
        reverse: bool,
        gap: Option<&'a str>,
        gap_x: Option<&'a str>,
        gap_y: Option<&'a str>,
    },
}

impl<'a> FromStr for Component<'a> {
    type Err = (); // Use a static string for the error type.

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "area-l" => Ok(Self::Area {
                template: None,
                rows: vec![],
                cols: vec![],
                gap: None,
                gap_x: None,
                gap_y: None,
            }),
            "box-l" => Ok(Self::Box {
                max_width: None,
                grow: false,
            }),
            "center-l" => Ok(Self::Center {
                max_width: None,
                and_text: false,
                recursive: false,
            }),
            "extender-l" => Ok(Self::Extender {
                screen: false,
                keep_center: false,
                keep_p: false,
                keep_pl: false,
                keep_pr: false,
            }),
            "grid-l" => Ok(Self::Grid {
                min_cell_width: None,
                min_cols: None,
                max_cols: None,
                gap: None,
                gap_x: None,
                gap_y: None,
            }),
            "icon-l" => Ok(Self::Icon {
                scale: None,
                align: None,
                gap_dir: None,
                gap: None,
            }),
            "ledge-l" | "row-l" => Ok(Self::Ledge {
                nowrap: false,
                twin_width: false,
                direction: None,
                justify: None,
                align: None,
                gap: None,
                gap_x: None,
                gap_y: None,
            }),
            "outsider-l" => Ok(Self::Outsider {
                position: None,
                top: None,
                bottom: None,
                left: None,
                right: None,
            }),
            "rack-l" => Ok(Self::Rack {
                height: None,
                min_height: None,
                max_height: None,
                gap: None,
            }),
            "sidebar-l" => Ok(Self::Sidebar {
                reverse: false,
                shrink: false,
                side: None,
                side_width: None,
                content_min: None,
                gap: None,
                gap_x: None,
                gap_y: None,
            }),
            "slider-l" => Ok(Self::Slider {
                hide_bar: false,
                item_width: None,
                height: None,
                gap: None,
            }),
            "stack-l" => Ok(Self::Stack {
                gap: None,
                recursive: false,
            }),
            "switcher-l" => Ok(Self::Switcher {
                threshold: None,
                limit: None,
                reverse: false,
                gap: None,
                gap_x: None,
                gap_y: None,
            }),
            _ => Err(()),
        }
    }
}

impl<'a> Component<'a> {
    /// Insert the css of the component
    /// inside the HashSet passed.
    /// This method consumes the component, because we dont need it anymore
    /// after we have inserted the css.
    pub fn insert_css(self, harmonic_ratio: f64, set: &mut HashSet<String>) {
        match self {
            Component::Area {
                template,
                rows,
                cols,
                gap,
                gap_x,
                gap_y,
            } => area_css(template, rows, cols, gap, gap_x, gap_y, harmonic_ratio, set),
            Component::Box { max_width, grow } => box_css(max_width, grow, set),
            Component::Center {
                max_width,
                and_text,
                recursive,
            } => center_css(max_width, and_text, recursive, set),
            Component::Extender {
                screen,
                keep_center,
                keep_p,
                keep_pl,
                keep_pr,
            } => extender_css(screen, keep_center,keep_p, keep_pl, keep_pr,  set),
            Component::Grid {
                min_cell_width,
                min_cols,
                max_cols,
                gap,
                gap_x,
                gap_y,
            } => grid_css(
                min_cell_width,
                min_cols,
                max_cols,
                gap,
                gap_x,
                gap_y,
                harmonic_ratio,
                set,
            ),
            Component::Icon {
                scale,
                align,
                gap_dir,
                gap,
            } => icon_css(scale, align, gap_dir, gap, harmonic_ratio, set),
            Component::Ledge {
                nowrap,
                twin_width,
                direction,
                justify,
                align,
                gap,
                gap_x,
                gap_y,
            } => ledge_css(
                nowrap,
                twin_width,
                direction,
                justify,
                align,
                gap,
                gap_x,
                gap_y,
                harmonic_ratio,
                set,
            ),
            Component::Outsider {
                position,
                top,
                bottom,
                left,
                right,
            } => outsider_css(position, top, bottom, left, right, harmonic_ratio, set),
            Component::Rack {
                height,
                min_height,
                max_height,
                gap,
            } => rack_css(height, min_height, max_height, gap, harmonic_ratio, set),
            Component::Sidebar {
                reverse,
                shrink,
                side,
                side_width,
                content_min,
                gap,
                gap_x,
                gap_y,
            } => sidebar_css(
                reverse,
                shrink,
                side,
                side_width,
                content_min,
                gap,
                gap_x,
                gap_y,
                harmonic_ratio,
                set,
            ),
            Component::Slider {
                hide_bar,
                item_width,
                height,
                gap,
            } => slider_css(hide_bar, item_width, height, gap, harmonic_ratio, set),
            Component::Stack { gap, recursive } => stack_css(gap, recursive, harmonic_ratio, set),
            Component::Switcher {
                threshold,
                limit,
                reverse,
                gap,
                gap_x,
                gap_y,
            } => switcher_css(
                threshold,
                limit,
                reverse,
                gap,
                gap_x,
                gap_y,
                harmonic_ratio,
                set,
            ),
        };
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_css_on_box() {
        let mut set: HashSet<String> = HashSet::new();
        let box_component = Component::Box {
            max_width: Some("440px"),
            grow: false,
        };
        box_component.insert_css(1.618, &mut set);
        println!("{:?}", set);
    }

    #[test]
    fn test_create_box() {
        let box_component = Component::from_str("box-l");

        assert_eq!(
            box_component,
            Ok(Component::Box {
                max_width: None,
                grow: false
            })
        );
    }
}
