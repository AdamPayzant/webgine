use crate::html_elements::common_attributes;

#[derive(Clone)]
pub struct Noscript {}

impl Default for Noscript {
    fn default() -> Self {
        Noscript {}
    }
}

impl common_attributes::Element for Noscript {
    fn add_attribute(&mut self, name: String, value: String) {
        _ = name;
        _ = value;
    }
}
