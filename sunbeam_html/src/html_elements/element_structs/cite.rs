use crate::html_elements::common_attributes;

#[derive(Debug, Clone, PartialEq)]
pub struct Cite {}

impl Default for Cite {
    fn default() -> Self {
        Cite {}
    }
}

impl common_attributes::Element for Cite {
    fn add_attribute(&mut self, name: String, value: String) {
        _ = name;
        _ = value;
    }
}
