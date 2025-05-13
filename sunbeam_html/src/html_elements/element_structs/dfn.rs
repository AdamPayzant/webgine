use crate::html_elements::common_attributes;

#[derive(Debug, Clone, PartialEq)]
pub struct Dfn {}

impl Default for Dfn {
    fn default() -> Self {
        Dfn {}
    }
}

impl common_attributes::Element for Dfn {
    fn add_attribute(&mut self, name: String, value: String) {
        _ = name;
        _ = value;
    }
}
