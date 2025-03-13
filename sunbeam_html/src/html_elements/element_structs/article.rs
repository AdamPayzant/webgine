use crate::html_elements::common_attributes;

#[derive(Clone)]
pub struct Article {}

impl Default for Article {
    fn default() -> Self {
        Article {}
    }
}

impl common_attributes::Element for Article {
    fn add_attribute(&mut self, name: String, value: String) {
        _ = name;
        _ = value;
    }
}
