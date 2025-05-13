use crate::html_elements::common_attributes;

#[derive(Debug, Clone, PartialEq)]
pub struct Menu {}

impl Default for Menu {
    fn default() -> Self {
        Menu {}
    }
}

impl common_attributes::Element for Menu {
    fn add_attribute(&mut self, name: String, value: String) {
        _ = name;
        _ = value;
    }
}
