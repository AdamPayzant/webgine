use crate::html_elements::common_attributes;

#[derive(Debug, Clone, PartialEq)]
pub struct Sup {}

impl Default for Sup {
    fn default() -> Self {
        Sup {}
    }
}

impl common_attributes::Element for Sup {
    fn add_attribute(&mut self, name: String, value: String) {
        _ = name;
        _ = value;
    }
}
