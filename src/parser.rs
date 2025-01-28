use std::collections::HashSet;
use State::*;

use crate::{
    builder::{generate, LayoutElement},
    media_query::{extract_breakpoint, MediaQuery},
};

#[derive(Debug, PartialEq)]
pub enum State {
    Resting,
    InsideTag,
    ReadingTagName,
    AfterTagName,
    ReadingAttributeName,
    WaitingAttributeValue,
    ReadingAttributeValue,
}

pub struct Parser<'a> {
    pub state: State,
    pub text: &'a str,
    pub tag_name_start: Option<usize>,
    pub tag_name_end: Option<usize>,
    pub attribute_name_start: Option<usize>,
    pub attribute_name_end: Option<usize>,
    pub layout_attribute_value_start: Option<usize>,
    pub layout_attribute_value_end: Option<usize>,
    pub layout_breakpoint_attribute_value_start: Option<usize>,
    pub layout_breakpoint_attribute_value_end: Option<usize>,
    pub biggest_breakpoint: Option<usize>,
    pub biggest_breakpoint_value: Option<&'a str>,
}

impl<'a> Parser<'a> {
    pub fn new(text: &'a str) -> Self {
        Parser {
            state: Resting,
            text,
            tag_name_start: None,
            tag_name_end: None,
            attribute_name_start: None,
            attribute_name_end: None,
            layout_attribute_value_start: None,
            layout_attribute_value_end: None,
            layout_breakpoint_attribute_value_start: None,
            layout_breakpoint_attribute_value_end: None,
            biggest_breakpoint: None,
            biggest_breakpoint_value: None,
        }
    }

    pub fn reset_indexes(&mut self) {
        self.tag_name_start = None;
        self.tag_name_end = None;
        self.attribute_name_start = None;
        self.attribute_name_end = None;
        self.layout_attribute_value_start = None;
        self.layout_attribute_value_end = None;
        self.layout_breakpoint_attribute_value_start = None;
        self.layout_breakpoint_attribute_value_end = None;
        self.biggest_breakpoint = None;
        self.biggest_breakpoint_value = None;
    }

    pub fn tag_name(&self) -> Option<&'a str> {
        match (self.tag_name_start, self.tag_name_end) {
            (Some(start), Some(end)) => Some(&self.text[start..=end]),
            _ => None,
        }
    }

    pub fn tag_name_new(&self) -> &'a str {
        &self.text[self.tag_name_start.unwrap()..=self.tag_name_end.unwrap()]
    }

    pub fn attribute_name(&self) -> Option<&'a str> {
        match (self.attribute_name_start, self.attribute_name_end) {
            (Some(start), Some(end)) => Some(&self.text[start..=end]),
            _ => None,
        }
    }

    pub fn layout_attribute_value(&self) -> Option<&'a str> {
        match (
            self.layout_attribute_value_start,
            self.layout_attribute_value_end,
        ) {
            // we have to check if end > start in the case of an empty attribute value like this `class=""`
            // because in this case end < start and so it will cause an error
            (Some(start), Some(end)) if end > start => Some(&self.text[start..=end]),
            _ => None,
        }
    }

    pub fn layout_breakpoint_attribute_value(&self) -> Option<&'a str> {
        match (
            self.layout_breakpoint_attribute_value_start,
            self.layout_breakpoint_attribute_value_end,
        ) {

            // TODO add a test to check if an empty layout breakpoint reset the component correctly
            // with a center with and-text in the normal layout for example
            (Some(start), Some(end)) if end > start => Some(&self.text[start..=end]),
            // this line allows to correctly reset the component when the breakpoint is empty 
            (Some(start), Some(end)) if end <= start => Some(""),
            _ => None,
        }
    }

    /// update the biggest_breakpoint of the parser only if the new breakpoint is superior
    /// to the one of the parser or if the parser has None as biggest_breakpoint
    pub fn update_biggest_breakpoint(&mut self, breakpoint: usize) -> bool {
        if let Some(parser_biggest_breakpoint) = self.biggest_breakpoint {
            if breakpoint > parser_biggest_breakpoint {
                self.biggest_breakpoint = Some(breakpoint);
                return true;
            }
        } else {
            self.biggest_breakpoint = Some(breakpoint);
            return true;
        }
        false
    }

    /// return the new state of the parser (without changing the parser's state) based on the current state and input character,
    /// if None is returned the state hasn't changed
    pub fn transition(&self, c: char) -> Option<State> {
        match (&self.state, c) {
            (Resting, '<') => Some(InsideTag),
            (InsideTag, c) if c.is_alphabetic() => Some(ReadingTagName),
            (ReadingTagName, c) if c.is_whitespace() => Some(AfterTagName),
            (AfterTagName, c) if c.is_alphabetic() => Some(ReadingAttributeName),
            (ReadingAttributeName, c) if c.is_whitespace() => Some(AfterTagName),
            (ReadingAttributeName, '=') => Some(WaitingAttributeValue),
            //TODO curly braces counter
            (WaitingAttributeValue, '"') => Some(ReadingAttributeValue),
            (ReadingAttributeValue, '"') => Some(AfterTagName),
            (AfterTagName | ReadingTagName | ReadingAttributeName, '>') => Some(Resting),
            _ => None,
        }
    }

    /// parse the given text to generate layout elements and
    /// add it to the set passed in parameter.
    pub fn parse(&mut self, elements: &mut HashSet<LayoutElement<'a>>) {
        for (i, c) in self.text.char_indices() {
            let new_state = self.transition(c);
            // if we enter here, the state has changed
            if let Some(state) = new_state {
                match (&self.state, &state) {
                    (_, ReadingTagName) => self.tag_name_start = Some(i),
                    (ReadingTagName, AfterTagName) => self.tag_name_end = Some(i - 1),
                    (_, ReadingAttributeName) => self.attribute_name_start = Some(i),
                    (ReadingAttributeName, AfterTagName | WaitingAttributeValue) => {
                        self.attribute_name_end = Some(i - 1)
                    }
                    (_, ReadingAttributeValue) => {
                        if let Some(attribute_name) = self.attribute_name() {
                            if attribute_name == "layout" {
                                self.layout_attribute_value_start = Some(i + 1);
                            } else if attribute_name.starts_with("layout@") {
                                self.layout_breakpoint_attribute_value_start = Some(i + 1);
                            }
                        }
                    }
                    (ReadingAttributeValue, AfterTagName) => {
                        if let Some(attribute_name) = self.attribute_name() {
                            if attribute_name == "layout" {
                                self.layout_attribute_value_end = Some(i - 1);
                                // when we are processing a media query layout attribute
                                // we should call generate too but with a MediaQuery
                                // as parameter
                            } else if attribute_name.starts_with("layout@") {
                                self.layout_breakpoint_attribute_value_end = Some(i - 1);
                                if let (Some(breakpoint), Some(attribute_value)) = (
                                    extract_breakpoint(attribute_name),
                                    self.layout_breakpoint_attribute_value(),
                                ) {
                                    let new_biggest_breakpoint_found =
                                        self.update_biggest_breakpoint(breakpoint);
                                    if new_biggest_breakpoint_found {
                                        self.biggest_breakpoint_value =
                                            self.layout_breakpoint_attribute_value();
                                    }
                                    // because it's a media-query layout attribute we know it will be InferioOrEqualTo
                                    let mq_new = MediaQuery::InferiorOrEqualTo(breakpoint);
                                    generate(
                                        self.tag_name_new(),
                                        Some(attribute_value),
                                        Some(mq_new),
                                        elements,
                                    );
                                }
                            }
                        }
                    }

                    // when we are leaving a tag, we generate layout elements
                    // and we reset the indexes of the parser
                    (current_state, Resting) => {
                        // if we were reading tag name at previous state that means
                        // that the tag contains only its name
                        // so we need to set tag_name_end
                        if current_state == &ReadingTagName {
                            self.tag_name_end = Some(i - 1);
                        }
                        if let (Some(tag_name), layout_value) =
                            (self.tag_name(), self.layout_attribute_value())
                        {
                            let mq = if let (Some(biggest_breakpoint), Some(breakpoint_value)) = (
                                self.biggest_breakpoint,
                                self.biggest_breakpoint_value,
                            ) {
                                Some(MediaQuery::SuperiorTo(biggest_breakpoint, breakpoint_value.to_string()))
                            } else {
                                None
                            };
                            generate(tag_name, layout_value, mq, elements);
                        }
                        self.reset_indexes();
                    }
                    _ => {}
                };
                // as the state has changed, whe have to update the state of the parser
                self.state = state;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::builder::LayoutElement;

    use super::*;

    // media query test
    #[test]
    fn media_query_update_biggest_breakpoint_value_of_parser_when_many_breakpoints() {
        let mut set: HashSet<LayoutElement> = HashSet::new();
        let mut parser =
            Parser::new("<div  layout@600px=\"p:3\"  layout@900px=\"p:7\"  layout@700px=\"p:1\"");
        parser.parse(&mut set);
        assert_eq!(parser.biggest_breakpoint_value, Some("p:7"));
    }
    #[test]
    fn media_query_update_biggest_breakpoint_value_of_parser() {
        let mut set: HashSet<LayoutElement> = HashSet::new();
        let mut parser = Parser::new("<div  layout@600px=\"p:3\"");
        parser.parse(&mut set);
        assert_eq!(parser.biggest_breakpoint_value, Some("p:3"));
    }

    #[test]
    fn media_query_update_biggest_breakpoint_of_parser() {
        let mut set: HashSet<LayoutElement> = HashSet::new();
        let mut parser = Parser::new("<div  layout@600px=\"p:3\"");
        parser.parse(&mut set);
        println!("{:?}", set);
        assert_eq!(parser.biggest_breakpoint, Some(600));
    }
    #[test]
    fn media_query_only_become_layout_element() {
        let mut set: HashSet<LayoutElement> = HashSet::new();
        let mut parser = Parser::new("<div  layout@600px=\"p:3\"");
        parser.parse(&mut set);
        println!("{:?}", set);
    }
    #[test]
    fn layout_bp_attribute_value_start_and_end_are_correct() {
        let mut set: HashSet<LayoutElement> = HashSet::new();
        let mut parser = Parser::new("<ledge-l layout=\"gap:1\" layout@600px=\"gap:2 p:3\"");
        parser.parse(&mut set);
        assert_eq!(parser.attribute_name(), Some("layout@600px"));
        assert_eq!(parser.layout_breakpoint_attribute_value_start, Some(38));
        assert_eq!(parser.layout_breakpoint_attribute_value_end, Some(46));
    }

    #[test]
    fn component_without_layout_attribute_generate_css() {
        let mut set: HashSet<LayoutElement> = HashSet::new();
        let mut parser = Parser::new("<ledge-l>");
        parser.parse(&mut set);
        assert!(set.len() == 1);
    }

    #[test]
    fn test_extract_breakpoint_attribute_is_added_to_breakpoints() {
        let mut set: HashSet<LayoutElement> = HashSet::new();
        let mut parser = Parser::new("<ledge-l layout@600px=\"p:2\"");
        parser.parse(&mut set);
        assert_eq!(parser.layout_breakpoint_attribute_value_start, Some(23));
    }

    #[test]
    fn test_extract_breakpoint_with_nothing_after_at() {
        let mq_attribute_value = "layout@";
        let result = extract_breakpoint(mq_attribute_value);

        assert_eq!(result, None);
    }

    #[test]
    fn test_extract_breakpoint_without_at() {
        let mq_attribute_value = "layout600px";
        let result = extract_breakpoint(mq_attribute_value);
        assert_eq!(result, None);
    }

    #[test]
    fn test_extract_breakpoint_with_correct_formating() {
        let mq_attribute_value = "layout@600px";
        let result = extract_breakpoint(mq_attribute_value);
        assert_eq!(result, Some(600));
    }
    //parse tests

    #[test]
    fn when_state_change_for_resting_the_parser_indexes_have_to_be_reset() {
        let mut set: HashSet<LayoutElement> = HashSet::new();
        let mut parser = Parser::new("<div layout=\"gap:2\" >");
        parser.parse(&mut set);
        assert_eq!(parser.tag_name_start, None);
        assert_eq!(parser.tag_name_end, None);
        assert_eq!(parser.attribute_name_start, None);
        assert_eq!(parser.attribute_name_end, None);
        assert_eq!(parser.layout_attribute_value_start, None);
        assert_eq!(parser.layout_attribute_value_end, None);
    }

    #[test]
    fn other_attribute_name_than_layout_doesnt_set_attribute_value_start_and_attribute_value_end() {
        let mut set: HashSet<LayoutElement> = HashSet::new();
        let mut parser = Parser::new("<div class=\"bonsoir\"");
        parser.parse(&mut set);
        assert_eq!(parser.layout_attribute_value_start, None);
        assert_eq!(parser.layout_attribute_value_end, None);
    }

    #[test]
    fn state_changing_from_reading_attribute_value_to_after_tag_name_set_attribute_value_end() {
        let mut set: HashSet<LayoutElement> = HashSet::new();
        let mut parser = Parser::new("<div layout=\"bonsoir\"");
        parser.parse(&mut set);
        assert_eq!(parser.layout_attribute_value_end, Some(19));
    }

    #[test]
    fn check_attribute_value_start_and_end_when_empty_attribute_value() {
        let mut set: HashSet<LayoutElement> = HashSet::new();
        let mut parser = Parser::new("<div layout=\"\" ");
        parser.parse(&mut set);
        assert_eq!(parser.layout_attribute_value(), None);
    }

    #[test]
    fn state_changing_to_reading_attribute_value_set_attribute_value_start() {
        let mut set: HashSet<LayoutElement> = HashSet::new();
        let mut parser = Parser::new("<div layout=\" ");
        parser.parse(&mut set);
        assert_eq!(parser.layout_attribute_value_start, Some(13));
    }

    #[test]
    fn state_changing_from_reading_attribute_name_to_after_tag_name_or_waiting_attribute_value_set_attribute_name_end(
    ) {
        let mut set: HashSet<LayoutElement> = HashSet::new();
        let mut parser = Parser::new("<div class ");
        parser.parse(&mut set);
        assert_eq!(parser.attribute_name_end, Some(9));
    }

    #[test]
    fn state_to_reading_attribute_name_set_attribute_name_start() {
        let mut set: HashSet<LayoutElement> = HashSet::new();
        let mut parser = Parser::new("<div c");
        parser.parse(&mut set);
        assert_eq!(parser.attribute_name_start, Some(5));
    }

    #[test]
    fn state_changing_from_reading_tag_name_to_after_tag_name_set_tag_name_end() {
        let mut set: HashSet<LayoutElement> = HashSet::new();
        let mut parser = Parser::new("<div ");
        parser.parse(&mut set);
        assert_eq!(parser.tag_name_end, Some(3));
    }

    #[test]
    fn state_changing_to_reading_tag_name_set_tag_name_start() {
        let mut set: HashSet<LayoutElement> = HashSet::new();
        let mut parser = Parser::new("<d");
        parser.parse(&mut set);
        assert_eq!(parser.tag_name_start, Some(1));
    }
    // transition tests

    #[test]
    fn reading_attribute_value_and_anything_except_double_quote_return_same_state() {
        let mut parser = Parser::new("");
        parser.state = ReadingAttributeValue;
        assert_eq!(parser.transition('i'), None);
    }

    #[test]
    fn reading_tag_name_or_attribute_name_and_alphabetic_return_same_state() {
        let mut parser = Parser::new("");

        parser.state = ReadingTagName;
        assert_eq!(parser.transition('i'), None);

        parser.state = ReadingAttributeName;
        assert_eq!(parser.transition('i'), None);
    }

    #[test]
    fn reading_attribute_value_and_double_quote_return_after_tag_name() {
        let mut parser = Parser::new("");
        parser.state = ReadingAttributeValue;
        assert_eq!(parser.transition('"'), Some(AfterTagName));
    }

    #[test]
    fn waiting_attribute_value_and_double_quote_return_reading_attribute_value() {
        let mut parser = Parser::new("");
        parser.state = WaitingAttributeValue;
        assert_eq!(parser.transition('"'), Some(ReadingAttributeValue));
    }

    #[test]
    fn reading_attribute_name_and_equal_return_waiting_attribute_value() {
        let mut parser = Parser::new("");
        parser.state = ReadingAttributeName;
        assert_eq!(parser.transition('='), Some(WaitingAttributeValue));
    }

    #[test]
    fn reading_attribute_name_and_right_chevron_return_resting() {
        let mut parser = Parser::new("");
        parser.state = ReadingAttributeName;
        assert_eq!(parser.transition('>'), Some(Resting));
    }

    #[test]
    fn reading_tag_name_and_right_chevron_return_resting() {
        let mut parser = Parser::new("");
        parser.state = ReadingTagName;
        assert_eq!(parser.transition('>'), Some(Resting));
    }

    #[test]
    fn resting_and_left_chevron_return_inside_tag() {
        let parser = Parser::new("");
        assert_eq!(parser.transition('<'), Some(InsideTag));
    }

    #[test]
    fn resting_and_a_return_resting() {
        let parser = Parser::new("");
        assert_eq!(parser.transition('a'), None);
    }

    #[test]
    fn inside_tag_and_alpha_return_reading_tag_name() {
        let mut parser = Parser::new("");
        parser.state = InsideTag;
        assert_eq!(parser.transition('d'), Some(ReadingTagName));
    }

    #[test]
    fn reading_tag_name_and_whitespace_return_after_tag_name() {
        let mut parser = Parser::new("");
        parser.state = ReadingTagName;
        assert_eq!(parser.transition(' '), Some(AfterTagName));

        let mut parser = Parser::new("");
        parser.state = ReadingTagName;
        assert_eq!(parser.transition('\n'), Some(AfterTagName));

        let mut parser = Parser::new("");
        parser.state = ReadingTagName;
        assert_eq!(parser.transition('\t'), Some(AfterTagName));
    }

    #[test]
    fn after_tag_name_and_alphabetic_return_reading_attribute_name() {
        let mut parser = Parser::new("");
        parser.state = AfterTagName;
        assert_eq!(parser.transition('c'), Some(ReadingAttributeName));
    }

    #[test]
    fn reading_attribute_name_and_whitespace_return_after_tag_name() {
        let mut parser = Parser::new("");
        parser.state = ReadingAttributeName;
        assert_eq!(parser.transition(' '), Some(AfterTagName));
    }

    #[test]
    fn after_tag_name_and_right_chevron_return_resting() {
        let mut parser = Parser::new("");
        parser.state = AfterTagName;
        assert_eq!(parser.transition('>'), Some(Resting));
    }
}
