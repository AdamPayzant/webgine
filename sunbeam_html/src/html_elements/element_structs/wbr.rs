use crate::html_elements::common_attributes;

pub struct Wbr {}

impl Default for Wbr {
    fn default() -> Self {
        Wbr {}
    }
}

impl common_attributes::Element for Wbr {
    fn add_attribute(&mut self, name: String, value: String) {
        _ = name;
        _ = value;
    }
}
