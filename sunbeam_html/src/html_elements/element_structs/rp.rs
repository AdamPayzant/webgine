use crate::html_elements::common_attributes;

pub struct Rp {}

impl Default for Rp {
    fn default() -> Self {
        Rp {}
    }
}

impl common_attributes::Element for Rp {
    fn add_attribute(&mut self, name: String, value: String) {
        _ = name;
        _ = value;
    }
}
