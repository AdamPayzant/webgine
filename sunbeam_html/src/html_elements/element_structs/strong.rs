use crate::html_elements::common_attributes;

#[derive(Debug, Clone, PartialEq)]
pub struct Strong {}

impl Default for Strong {
    fn default() -> Self {
        Strong {}
    }
}

impl common_attributes::Element for Strong {
    fn add_attribute(&mut self, name: String, value: String) {
        _ = name;
        _ = value;
    }
}
