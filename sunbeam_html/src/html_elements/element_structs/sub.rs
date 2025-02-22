use crate::html_elements::common_attributes;

pub struct Sub {}

impl Default for Sub {
    fn default() -> Self {
        Sub {}
    }
}

impl common_attributes::Element for Sub {
    fn add_attribute(&mut self, name: String, value: String) {
        _ = name;
        _ = value;
    }
}
