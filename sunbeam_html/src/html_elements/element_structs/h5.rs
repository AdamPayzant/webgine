use crate::html_elements::common_attributes;

#[derive(Debug, Clone, PartialEq)]
pub struct H5 {}

impl Default for H5 {
    fn default() -> Self {
        H5 {}
    }
}

impl common_attributes::Element for H5 {
    fn add_attribute(&mut self, name: String, value: String) {
        _ = name;
        _ = value;
    }
}
