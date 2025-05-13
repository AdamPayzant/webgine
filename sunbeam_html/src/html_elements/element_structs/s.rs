use crate::html_elements::common_attributes;

#[derive(Debug, Clone, PartialEq)]
pub struct S {}

impl Default for S {
    fn default() -> Self {
        S {}
    }
}

impl common_attributes::Element for S {
    fn add_attribute(&mut self, name: String, value: String) {
        _ = name;
        _ = value;
    }
}
