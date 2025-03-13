use crate::html_elements::common_attributes;

#[derive(Clone)]
pub struct Header {}

impl Default for Header {
    fn default() -> Self {
        Header {}
    }
}

impl common_attributes::Element for Header {
    fn add_attribute(&mut self, name: String, value: String) {
        _ = name;
        _ = value;
    }
}
