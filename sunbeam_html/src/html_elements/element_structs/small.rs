use crate::html_elements::common_attributes;

#[derive(Debug, Clone, PartialEq)]
pub struct Small {}

impl Default for Small {
    fn default() -> Self {
        Small {}
    }
}

impl common_attributes::Element for Small {
    fn add_attribute(&mut self, name: String, value: String) {
        _ = name;
        _ = value;
    }
}
