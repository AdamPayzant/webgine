use crate::html_elements::common_attributes;

#[derive(Clone)]
pub struct Dd {}

impl Default for Dd {
    fn default() -> Self {
        Dd {}
    }
}

impl common_attributes::Element for Dd {
    fn add_attribute(&mut self, name: String, value: String) {
        _ = name;
        _ = value;
    }
}
