use crate::html_elements::common_attributes;

#[derive(Debug, Clone, PartialEq)]
pub struct THead {}

impl Default for THead {
    fn default() -> Self {
        THead {}
    }
}

impl common_attributes::Element for THead {
    fn add_attribute(&mut self, name: String, value: String) {
        _ = name;
        _ = value;
    }
}
