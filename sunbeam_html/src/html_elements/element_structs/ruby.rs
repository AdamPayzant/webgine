use crate::html_elements::common_attributes;

#[derive(Debug, Clone)]
pub struct Ruby {}

impl Default for Ruby {
    fn default() -> Self {
        Ruby {}
    }
}

impl common_attributes::Element for Ruby {
    fn add_attribute(&mut self, name: String, value: String) {
        _ = name;
        _ = value;
    }
}
