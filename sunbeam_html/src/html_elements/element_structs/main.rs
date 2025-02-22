use crate::html_elements::common_attributes;

pub struct Main {}

impl Default for Main {
    fn default() -> Self {
        Main {}
    }
}

impl common_attributes::Element for Main {
    fn add_attribute(&mut self, name: String, value: String) {
        _ = name;
        _ = value;
    }
}
