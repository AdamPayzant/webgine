use crate::html_elements::common_attributes;

pub struct HGroup {}

impl Default for HGroup {
    fn default() -> Self {
        HGroup {}
    }
}

impl common_attributes::Element for HGroup {
    fn add_attribute(&mut self, name: String, value: String) {
        _ = name;
        _ = value;
    }
}
