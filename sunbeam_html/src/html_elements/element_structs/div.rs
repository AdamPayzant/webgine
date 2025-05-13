use crate::html_elements::common_attributes;

#[derive(Debug, Clone, PartialEq)]
pub struct Div {}

impl Default for Div {
    fn default() -> Self {
        Div {}
    }
}

impl common_attributes::Element for Div {
    fn add_attribute(&mut self, name: String, value: String) {
        _ = name;
        _ = value;
    }
}
