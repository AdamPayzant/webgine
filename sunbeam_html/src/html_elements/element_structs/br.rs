use crate::html_elements::common_attributes;

#[derive(Debug, Clone)]
pub struct Br {}

impl Default for Br {
    fn default() -> Self {
        Br {}
    }
}

impl common_attributes::Element for Br {
    fn add_attribute(&mut self, name: String, value: String) {
        _ = name;
        _ = value;
    }
}
