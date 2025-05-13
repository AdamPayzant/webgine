use crate::html_elements::common_attributes;

#[derive(Debug, Clone, PartialEq)]
pub struct Dt {}

impl Default for Dt {
    fn default() -> Self {
        Dt {}
    }
}

impl common_attributes::Element for Dt {
    fn add_attribute(&mut self, name: String, value: String) {
        _ = name;
        _ = value;
    }
}
