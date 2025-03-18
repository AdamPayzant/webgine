use crate::html_elements::common_attributes;

#[derive(Debug, Clone)]
pub struct Var {}

impl Default for Var {
    fn default() -> Self {
        Var {}
    }
}

impl common_attributes::Element for Var {
    fn add_attribute(&mut self, name: String, value: String) {
        _ = name;
        _ = value;
    }
}
