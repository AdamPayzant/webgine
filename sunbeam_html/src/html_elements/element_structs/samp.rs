use crate::html_elements::common_attributes;

#[derive(Debug, Clone)]
pub struct Samp {}

impl Default for Samp {
    fn default() -> Self {
        Samp {}
    }
}

impl common_attributes::Element for Samp {
    fn add_attribute(&mut self, name: String, value: String) {
        _ = name;
        _ = value;
    }
}
