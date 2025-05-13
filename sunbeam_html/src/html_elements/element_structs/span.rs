use crate::html_elements::common_attributes;

#[derive(Debug, Clone, PartialEq)]
pub struct Span {}

impl Default for Span {
    fn default() -> Self {
        Span {}
    }
}

impl common_attributes::Element for Span {
    fn add_attribute(&mut self, name: String, value: String) {
        _ = name;
        _ = value;
    }
}
