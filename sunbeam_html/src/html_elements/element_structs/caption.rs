use crate::html_elements::common_attributes;

#[derive(Clone)]
pub struct Caption {}

impl Default for Caption {
    fn default() -> Self {
        Caption {}
    }
}

impl common_attributes::Element for Caption {
    fn add_attribute(&mut self, name: String, value: String) {
        _ = name;
        _ = value;
    }
}
