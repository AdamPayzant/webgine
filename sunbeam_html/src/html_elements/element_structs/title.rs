use crate::html_elements::common_attributes;

#[derive(Debug, Clone, PartialEq)]
pub struct Title {}

impl Default for Title {
    fn default() -> Self {
        Title {}
    }
}

impl common_attributes::Element for Title {
    fn add_attribute(&mut self, name: String, value: String) {
        _ = name;
        _ = value;
    }
}
