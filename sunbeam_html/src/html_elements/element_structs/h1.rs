use crate::html_elements::common_attributes;

pub struct H1 {}

impl Default for H1 {
    fn default() -> Self {
        H1 {}
    }
}

impl common_attributes::Element for H1 {
    fn add_attribute(&mut self, name: String, value: String) {
        _ = name;
        _ = value;
    }
}
