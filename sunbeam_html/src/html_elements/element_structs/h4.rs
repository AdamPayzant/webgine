use crate::html_elements::common_attributes;

#[derive(Clone)]
pub struct H4 {}

impl Default for H4 {
    fn default() -> Self {
        H4 {}
    }
}

impl common_attributes::Element for H4 {
    fn add_attribute(&mut self, name: String, value: String) {
        _ = name;
        _ = value;
    }
}
