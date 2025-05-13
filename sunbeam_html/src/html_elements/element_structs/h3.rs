use crate::html_elements::common_attributes;

#[derive(Debug, Clone, PartialEq)]
pub struct H3 {}

impl Default for H3 {
    fn default() -> Self {
        H3 {}
    }
}

impl common_attributes::Element for H3 {
    fn add_attribute(&mut self, name: String, value: String) {
        _ = name;
        _ = value;
    }
}
