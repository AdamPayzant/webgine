use crate::html_elements::common_attributes;

pub struct Section {}

impl Default for Section {
    fn default() -> Self {
        Section {}
    }
}

impl common_attributes::Element for Section {
    fn add_attribute(&mut self, name: String, value: String) {
        _ = name;
        _ = value;
    }
}
