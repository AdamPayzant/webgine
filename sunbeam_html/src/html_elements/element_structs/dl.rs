use crate::html_elements::common_attributes;

#[derive(Debug, Clone, PartialEq)]
pub struct Dl {}

impl Default for Dl {
    fn default() -> Self {
        Dl {}
    }
}

impl common_attributes::Element for Dl {
    fn add_attribute(&mut self, name: String, value: String) {
        _ = name;
        _ = value;
    }
}
