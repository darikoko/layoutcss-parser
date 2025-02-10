pub mod builder;
pub mod dev;
pub mod classes;
pub mod reset;
pub mod components;
pub mod config;
pub mod harmonic;
pub mod media_query;
pub mod parser;
pub mod utilities;

use builder::LayoutElement;
use config::LayoutStyleConfig;
use indoc::formatdoc;
use media_query::MediaQuery;
use std::collections::{HashMap, HashSet};

pub fn generate_final_css(
    css_set: &HashSet<String>,
    mq_rules: &HashMap<MediaQuery, HashSet<String>>,
) -> String {
    // We join all the classic rules (non mq) into a single String
    let mut final_css = css_set.iter().cloned().collect::<Vec<String>>().join("");

    // Now we have to process the media queries,
    // we get the keys in the correct order to process them to get a
    // correct css specificty
    let mut ordered_mq_keys: Vec<MediaQuery> = mq_rules.keys().cloned().collect();

    // Sorts according to the 'cmp' method in MediaQuery
    ordered_mq_keys.sort();

    for key in ordered_mq_keys {
        if let Some(hash_set) = mq_rules.get(&key) {
            // replace selector in media query to increase specificity
            // to avoid conflict
            let rule_with_modified_selector: String = hash_set
                .iter()
                .map(|s| match &key {
                    MediaQuery::InferiorOrEqualTo(breakpoint) => {
                        s.replace("[layout", format!("[layout\\@{breakpoint}px").as_str())
                    }
                    MediaQuery::SuperiorTo(breakpoint, attribute_value) => s.replace(
                        "-l[layout",
                        format!("-l[layout\\@{breakpoint}px=\"{attribute_value}\"][layout")
                            .as_str(),
                    ),
                })
                .collect::<Vec<String>>()
                .join("");
            // wrap the previous modified rule
            // into a media query
            let wrapped_in_mq = match key {
                MediaQuery::SuperiorTo(breakpoint, _) => formatdoc!(
                    r#"
                            @media (width > {breakpoint}px) {{
                                {rule_with_modified_selector}
                            }}
                            "#
                ),
                MediaQuery::InferiorOrEqualTo(breakpoint) => formatdoc!(
                    r#"
                            @media (width <= {breakpoint}px) {{
                                {rule_with_modified_selector}
                            }}"#
                ),
            };
            // at this rules to the final css
            final_css.push_str(wrapped_in_mq.as_str());
        }
    }
    final_css
}

/// This function can take css_set and css_mq_rules if you already have
/// process a string before and you want to avoid to get duplicate values
/// this way the new css will contains the previous css plus new rules from the new string
pub fn get_css_from_string<'a>(
    text: &'a String,
    previous_css_rules: Option<&mut HashSet<String>>,
    previous_css_mq_rules: Option<&mut HashMap<MediaQuery, HashSet<String>>>,
    layout_style_config: &LayoutStyleConfig,
) -> String {
    let mut layout_elements: HashSet<LayoutElement> = HashSet::new();

    let mut local_css_rules: HashSet<String> = HashSet::new();
    let css_rules = match previous_css_rules {
        Some(x) => x,
        None => &mut local_css_rules,
    };

    css_rules.insert(reset::reset_css(&layout_style_config));
    if layout_style_config.dev {
        css_rules.insert(dev::DEV_CSS.to_string());
    }

    let mut local_css_mq_rules: HashMap<MediaQuery, HashSet<String>> = HashMap::new();
    let css_mq_rules = match previous_css_mq_rules {
        Some(x) => x,
        None => &mut local_css_mq_rules,
    };
    let mut parser = parser::Parser::new(text);
    parser.parse(&mut layout_elements);
    for element in layout_elements.drain() {
        element.insert_css(layout_style_config.harmonic_ratio, css_rules, css_mq_rules);
    }
    generate_final_css(&css_rules, &css_mq_rules)
}
