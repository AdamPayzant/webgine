use crate::html_elements::common_attributes;

#[derive(Debug, Clone, PartialEq)]
pub struct H2 {}

impl Default for H2 {
    fn default() -> Self {
        H2 {}
    }
}

impl common_attributes::Element for H2 {
    fn add_attribute(&mut self, name: String, value: String) {
        _ = name;
        _ = value;
    }
}
