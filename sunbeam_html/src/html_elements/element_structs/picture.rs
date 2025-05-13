use crate::html_elements::common_attributes;

#[derive(Debug, Clone, PartialEq)]
pub struct Picture {}

impl Default for Picture {
    fn default() -> Self {
        Picture {}
    }
}

impl common_attributes::Element for Picture {
    fn add_attribute(&mut self, name: String, value: String) {
        _ = name;
        _ = value;
    }
}
