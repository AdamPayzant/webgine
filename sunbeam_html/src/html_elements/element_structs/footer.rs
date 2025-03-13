use crate::html_elements::common_attributes;

#[derive(Clone)]
pub struct Footer {}

impl Default for Footer {
    fn default() -> Self {
        Footer {}
    }
}

impl common_attributes::Element for Footer {
    fn add_attribute(&mut self, name: String, value: String) {
        _ = name;
        _ = value;
    }
}
