use crate::html_elements::common_attributes;

pub struct Rt {}

impl Default for Rt {
    fn default() -> Self {
        Rt {}
    }
}

impl common_attributes::Element for Rt {
    fn add_attribute(&mut self, name: String, value: String) {
        _ = name;
        _ = value;
    }
}
