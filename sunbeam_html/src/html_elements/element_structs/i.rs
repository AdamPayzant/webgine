use crate::html_elements::common_attributes;

#[derive(Debug, Clone)]
pub struct I {}

impl Default for I {
    fn default() -> Self {
        I {}
    }
}

impl common_attributes::Element for I {
    fn add_attribute(&mut self, name: String, value: String) {
        _ = name;
        _ = value;
    }
}
