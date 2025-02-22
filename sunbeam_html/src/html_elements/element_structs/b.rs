use crate::html_elements::common_attributes;

pub struct B {}

impl Default for B {
    fn default() -> Self {
        B {}
    }
}

impl common_attributes::Element for B {
    fn add_attribute(&mut self, name: String, value: String) {
        _ = name;
        _ = value;
    }
}
