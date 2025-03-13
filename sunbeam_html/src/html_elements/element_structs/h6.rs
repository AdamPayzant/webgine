use crate::html_elements::common_attributes;

#[derive(Clone)]
pub struct H6 {}

impl Default for H6 {
    fn default() -> Self {
        H6 {}
    }
}

impl common_attributes::Element for H6 {
    fn add_attribute(&mut self, name: String, value: String) {
        _ = name;
        _ = value;
    }
}
