use crate::html_elements::common_attributes;

#[derive(Debug, Clone)]
pub struct Legend {}

impl Default for Legend {
    fn default() -> Self {
        Legend {}
    }
}

impl common_attributes::Element for Legend {
    fn add_attribute(&mut self, name: String, value: String) {
        _ = name;
        _ = value;
    }
}
