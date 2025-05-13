use crate::html_elements::common_attributes;

#[derive(Debug, Clone, PartialEq)]
pub struct Em {}

impl Default for Em {
    fn default() -> Self {
        Em {}
    }
}

impl common_attributes::Element for Em {
    fn add_attribute(&mut self, name: String, value: String) {
        _ = name;
        _ = value;
    }
}
