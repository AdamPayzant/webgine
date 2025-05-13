use crate::html_elements::common_attributes;

#[derive(Debug, Clone, PartialEq)]
pub struct Tr {}

impl Default for Tr {
    fn default() -> Self {
        Tr {}
    }
}

impl common_attributes::Element for Tr {
    fn add_attribute(&mut self, name: String, value: String) {
        _ = name;
        _ = value;
    }
}
