use crate::html_elements::common_attributes;

#[derive(Debug, Clone, PartialEq)]
pub struct Nav {}

impl Default for Nav {
    fn default() -> Self {
        Nav {}
    }
}

impl common_attributes::Element for Nav {
    fn add_attribute(&mut self, name: String, value: String) {
        _ = name;
        _ = value;
    }
}
