use crate::html_elements::common_attributes;

pub struct U {}

impl Default for U {
    fn default() -> Self {
        U {}
    }
}

impl common_attributes::Element for U {
    fn add_attribute(&mut self, name: String, value: String) {
        _ = name;
        _ = value;
    }
}
