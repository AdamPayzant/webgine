use crate::html_elements::common_attributes;

pub struct Q {}

impl Default for Q {
    fn default() -> Self {
        Q {}
    }
}

impl common_attributes::Element for Q {
    fn add_attribute(&mut self, name: String, value: String) {
        _ = name;
        _ = value;
    }
}
