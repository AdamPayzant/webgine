use crate::html_elements::common_attributes;

#[derive(Debug, Clone, PartialEq)]
pub struct Kbd {}

impl Default for Kbd {
    fn default() -> Self {
        Kbd {}
    }
}

impl common_attributes::Element for Kbd {
    fn add_attribute(&mut self, name: String, value: String) {
        _ = name;
        _ = value;
    }
}
