use crate::html_elements::common_attributes;

pub struct Ul {}

impl Default for Ul {
    fn default() -> Self {
        Ul {}
    }
}

impl common_attributes::Element for Ul {
    fn add_attribute(&mut self, name: String, value: String) {
        _ = name;
        _ = value;
    }
}
