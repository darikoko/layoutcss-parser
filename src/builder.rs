use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use crate::classes::LayoutClass;
use crate::components::Component;
use crate::media_query::MediaQuery;

#[derive(Hash, PartialEq, Eq, Debug)]
pub enum LayoutElement<'a> {
    LayoutComponent(Component<'a>, Option<MediaQuery>),
    LayoutUtility(LayoutClass<'a>, Option<MediaQuery>),
}

impl<'a> LayoutElement<'a> {
    /// Generate the css of the layout element
    /// and insert it inside the Hashset passed.
    /// This method consumes the LayoutElement, because we dont need it anymore
    /// after we get the css.
    pub fn insert_css(
        self,
        harmonic_ratio: f64,
        set: &mut HashSet<String>,
        media_queries_rules: &mut HashMap<MediaQuery, HashSet<String>>,
    ) {
        //we can handle here if we should pass the hashset from the css_mq or from css
        //to manage media queries
        match self {
            Self::LayoutComponent(component, None) => component.insert_css(harmonic_ratio, set),
            //here pass the set from the hashmap
            Self::LayoutComponent(component, Some(mq)) => {
                let val = media_queries_rules.entry(mq).or_insert_with(HashSet::new);
                component.insert_css(harmonic_ratio, val);
            }
            Self::LayoutUtility(class, None) => class.insert_css(harmonic_ratio, set),
            //here pass the set from the hashmap
            Self::LayoutUtility(class, Some(mq)) => {
                let val = media_queries_rules.entry(mq).or_insert_with(HashSet::new);
                class.insert_css(harmonic_ratio, val);
            }
        }
    }
}

pub fn generate<'a>(
    tag_name: &'a str,
    layout_attribute: Option<&'a str>,
    media_query: Option<MediaQuery>,
    set: &mut HashSet<LayoutElement<'a>>,
) {
    let mut component = Component::from_str(tag_name);
    if let Some(layout_attribute) = layout_attribute {
        for class in layout_attribute.split_whitespace() {
            let layout_class = LayoutClass::try_from(class);

            // If a LayoutClass has been created from the previous step
            if let Ok(current_class) = layout_class {
                // In the case where a component has been created from
                // the first line of this function
                // here we will handle all LayoutClass attached to a component
                // to then modify the component accordingly
                if let Ok(comp) = &mut component {
                    match current_class {
                        LayoutClass::MaxWidth(v) => {
                            match comp {
                                Component::Box { max_width, .. } => *max_width = Some(v),
                                Component::Center { max_width, .. } => *max_width = Some(v),
                                _ => {}
                            };
                        }
                        LayoutClass::MinCellWidth(v) => {
                            match comp {
                                Component::Grid { min_cell_width, .. } => *min_cell_width = Some(v),
                                _ => {}
                            };
                        }
                        LayoutClass::MinCols(v) => {
                            match comp {
                                Component::Grid { min_cols, .. } => *min_cols = Some(v),
                                _ => {}
                            };
                        }
                        LayoutClass::MaxCols(v) => {
                            match comp {
                                Component::Grid { max_cols, .. } => *max_cols = Some(v),
                                _ => {}
                            };
                        }
                        LayoutClass::Recursive => {
                            match comp {
                                Component::Stack { recursive, .. } => *recursive = true,
                                Component::Center { recursive, .. } => *recursive = true,
                                _ => {}
                            };
                        }
                        LayoutClass::Reverse => {
                            match comp {
                                Component::Switcher { reverse, .. } => *reverse = true,
                                Component::Sidebar { reverse, .. } => *reverse = true,
                                _ => {}
                            };
                        }
                        LayoutClass::Screen => {
                            match comp {
                                Component::Extender { screen, .. } => *screen = true,
                                _ => {}
                            };
                        }
                        LayoutClass::TwinWidth => {
                            match comp {
                                Component::Ledge { twin_width, .. } => *twin_width = true,
                                _ => {}
                            };
                        }
                        LayoutClass::NoWrap => {
                            match comp {
                                Component::Ledge { nowrap, .. } => *nowrap = true,
                                _ => {}
                            };
                        }
                        LayoutClass::HideBar => {
                            match comp {
                                Component::Slider { hide_bar, .. } => *hide_bar = true,
                                _ => {}
                            };
                        }
                        LayoutClass::Grow => {
                            match comp {
                                Component::Box { grow, .. } => *grow = true,
                                _ => {}
                            };
                        }
                        LayoutClass::Template(v) => {
                            match comp {
                                Component::Area { template, .. } => *template = Some(v),
                                _ => {}
                            };
                        }
                        LayoutClass::Row(v) => {
                            match comp {
                                Component::Area { rows, .. } => rows.push(v),
                                _ => {}
                            };
                        }
                        LayoutClass::Col(v) => {
                            match comp {
                                Component::Area { cols, .. } => cols.push(v),
                                _ => {}
                            };
                        }
                        LayoutClass::Gap(v) => {
                            match comp {
                                Component::Area { gap, .. } => *gap = Some(v),
                                Component::Grid { gap, .. } => *gap = Some(v),
                                Component::Icon { gap, .. } => *gap = Some(v),
                                Component::Ledge { gap, .. } => *gap = Some(v),
                                Component::Rack { gap, .. } => *gap = Some(v),
                                Component::Sidebar { gap, .. } => *gap = Some(v),
                                Component::Slider { gap, .. } => *gap = Some(v),
                                Component::Stack { gap, .. } => *gap = Some(v),
                                Component::Switcher { gap, .. } => *gap = Some(v),
                                _ => {}
                            };
                        }
                        LayoutClass::GapX(v) => {
                            match comp {
                                Component::Area { gap_x, .. } => *gap_x = Some(v),
                                Component::Grid { gap_x, .. } => *gap_x = Some(v),
                                Component::Ledge { gap_x, .. } => *gap_x = Some(v),
                                Component::Sidebar { gap_x, .. } => *gap_x = Some(v),
                                Component::Switcher { gap_x, .. } => *gap_x = Some(v),
                                _ => {}
                            };
                        }
                        LayoutClass::GapY(v) => {
                            match comp {
                                Component::Area { gap_y, .. } => *gap_y = Some(v),
                                Component::Grid { gap_y, .. } => *gap_y = Some(v),
                                Component::Ledge { gap_y, .. } => *gap_y = Some(v),
                                Component::Sidebar { gap_y, .. } => *gap_y = Some(v),
                                Component::Switcher { gap_y, .. } => *gap_y = Some(v),
                                _ => {}
                            };
                        }
                        LayoutClass::GapDir(v) => {
                            match comp {
                                Component::Icon { gap_dir, .. } => *gap_dir = Some(v),
                                _ => {}
                            };
                        }
                        LayoutClass::Scale(v) => {
                            match comp {
                                Component::Icon { scale, .. } => *scale = Some(v),
                                _ => {}
                            };
                        }
                        LayoutClass::Align(v) => {
                            match comp {
                                Component::Icon { align, .. } => *align = Some(v),
                                Component::Ledge { align, .. } => *align = Some(v),
                                _ => {}
                            };
                        }
                        LayoutClass::Justify(v) => {
                            match comp {
                                Component::Ledge { justify, .. } => *justify = Some(v),
                                _ => {}
                            };
                        }

                        LayoutClass::Position(v) => {
                            match comp {
                                Component::Outsider { position, .. } => *position = Some(v),
                                _ => {}
                            };
                        }
                        LayoutClass::Top(v) => {
                            match comp {
                                Component::Outsider { top, .. } => *top = Some(v),
                                _ => {}
                            };
                        }
                        LayoutClass::Bottom(v) => {
                            match comp {
                                Component::Outsider { bottom, .. } => *bottom = Some(v),
                                _ => {}
                            };
                        }
                        LayoutClass::Left(v) => {
                            match comp {
                                Component::Outsider { left, .. } => *left = Some(v),
                                _ => {}
                            };
                        }
                        LayoutClass::Right(v) => {
                            match comp {
                                Component::Outsider { right, .. } => *right = Some(v),
                                _ => {}
                            };
                        }

                        LayoutClass::Height(v) => {
                            match comp {
                                Component::Rack { height, .. } => *height = Some(v),
                                Component::Slider { height, .. } => *height = Some(v),
                                _ => {}
                            };
                        }
                        LayoutClass::ItemWidth(v) => {
                            match comp {
                                Component::Slider { item_width, .. } => *item_width = Some(v),
                                _ => {}
                            };
                        }
                        LayoutClass::MinHeight(v) => {
                            match comp {
                                Component::Rack { min_height, .. } => *min_height = Some(v),
                                _ => {}
                            };
                        }
                        LayoutClass::MaxHeight(v) => {
                            match comp {
                                Component::Rack { max_height, .. } => *max_height = Some(v),
                                _ => {}
                            };
                        }
                        LayoutClass::SideWidth(v) => {
                            match comp {
                                Component::Sidebar { side_width, .. } => *side_width = Some(v),
                                _ => {}
                            };
                        }
                        LayoutClass::Side(v) => {
                            match comp {
                                Component::Sidebar { side, .. } => *side = Some(v),
                                _ => {}
                            };
                        }
                        LayoutClass::ContentMin(v) => {
                            match comp {
                                Component::Sidebar { content_min, .. } => *content_min = Some(v),
                                _ => {}
                            };
                        }
                        LayoutClass::Threshold(v) => {
                            match comp {
                                Component::Switcher { threshold, .. } => *threshold = Some(v),
                                _ => {}
                            };
                        }
                        LayoutClass::Limit(v) => {
                            match comp {
                                Component::Switcher { limit, .. } => *limit = Some(v),
                                _ => {}
                            };
                        }
                        LayoutClass::KeepP => {
                            match comp {
                                Component::Extender { keep_p, .. } => *keep_p = true,
                                _ => {}
                            };
                        }
                        LayoutClass::KeepPL => {
                            match comp {
                                Component::Extender { keep_pl, .. } => *keep_pl = true,
                                _ => {}
                            };
                        }
                        LayoutClass::KeepPR => {
                            match comp {
                                Component::Extender { keep_pr, .. } => *keep_pr = true,
                                _ => {}
                            };
                        }
                        LayoutClass::KeepCenter => {
                            match comp {
                                Component::Extender { keep_center, .. } => *keep_center = true,
                                _ => {}
                            };
                        }
                        LayoutClass::Shrink => {
                            match comp {
                                Component::Sidebar { shrink, .. } => *shrink = true,
                                _ => {}
                            };
                        }
                        LayoutClass::AndText => {
                            match comp {
                                Component::Center { and_text, .. } => *and_text = true,
                                _ => {}
                            };
                        }
                        _ => {
                            // we dont want to scope utility rules of the component outside the mq
                            // into the mq rules so utilities in the component should be built without mq
                            let final_mq = match media_query {
                                Some(MediaQuery::SuperiorTo(_, _)) => None,
                                Some(MediaQuery::InferiorOrEqualTo(_)) => media_query.clone(),
                                _ => None,
                            };
                            set.insert(LayoutElement::LayoutUtility(current_class, final_mq));
                        }
                    };
                } else {
                    // we dont want to scope utility rules of the component outside the mq
                    // into the mq rules so utilities in the component should be built without mq
                    let final_mq = match media_query {
                        Some(MediaQuery::SuperiorTo(_, _)) => None,
                        Some(MediaQuery::InferiorOrEqualTo(_)) => media_query.clone(),
                        _ => None,
                    };
                    set.insert(LayoutElement::LayoutUtility(current_class, final_mq));
                }
            }
        }
    }
    if let Ok(cc) = component {
        //TODO check if we are
        set.insert(LayoutElement::LayoutComponent(cc, media_query));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn media_queries_are_generated() {
        let mut set: HashSet<String> = HashSet::new();
        let mut mq_set: HashMap<MediaQuery, HashSet<String>> = HashMap::new();
        let el = LayoutElement::LayoutComponent(
            Component::Box {
                max_width: Some("800px"),
                grow: false,
            },
            Some(MediaQuery::SuperiorTo(
                800,
                "p:2 max-width:400px".to_string(),
            )),
        );
        let el2 = LayoutElement::LayoutComponent(
            Component::Box {
                max_width: Some("1200px"),
                grow: false,
            },
            Some(MediaQuery::SuperiorTo(
                800,
                "p:2 max-width:400px".to_string(),
            )),
        );

        el.insert_css(1.618, &mut set, &mut mq_set);
        el2.insert_css(1.618, &mut set, &mut mq_set);
        println!("{:?}oooooooooooo", mq_set);
        assert_eq!(4, 4)
    }

    #[test]
    fn attribute_layout_with_component_and_utilities_append_hashset_correctly() {
        let mut set: HashSet<LayoutElement> = HashSet::new();
        generate(
            "box-l",
            Some("max-width:440px max-width:440px grow p:2 p:4 p:2"),
            None,
            &mut set,
        );
        println!("{:?}oooooooooooo", set);
        assert_eq!(4, 4)
    }
}
