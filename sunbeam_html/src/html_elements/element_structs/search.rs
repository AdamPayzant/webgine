use crate::html_elements::common_attributes;

#[derive(Clone)]
pub struct Search {}

impl Default for Search {
    fn default() -> Self {
        Search {}
    }
}

impl common_attributes::Element for Search {
    fn add_attribute(&mut self, name: String, value: String) {
        _ = name;
        _ = value;
    }
}
