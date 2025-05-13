use crate::html_elements::common_attributes;

#[derive(Debug, Clone, PartialEq)]
pub struct Hr {}

impl Default for Hr {
    fn default() -> Self {
        Hr {}
    }
}

impl common_attributes::Element for Hr {
    fn add_attribute(&mut self, name: String, value: String) {
        _ = name;
        _ = value;
    }
}
