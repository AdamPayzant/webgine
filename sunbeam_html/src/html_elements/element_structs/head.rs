use crate::html_elements::common_attributes;

#[derive(Debug, Clone)]
pub struct Head {}

impl Default for Head {
    fn default() -> Self {
        Head {}
    }
}

impl common_attributes::Element for Head {
    fn add_attribute(&mut self, name: String, value: String) {
        _ = name;
        _ = value;
    }
}
