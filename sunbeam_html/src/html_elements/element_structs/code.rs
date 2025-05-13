use crate::html_elements::common_attributes;

#[derive(Debug, Clone, PartialEq)]
pub struct Code {}

impl Default for Code {
    fn default() -> Self {
        Code {}
    }
}

impl common_attributes::Element for Code {
    fn add_attribute(&mut self, name: String, value: String) {
        _ = name;
        _ = value;
    }
}
