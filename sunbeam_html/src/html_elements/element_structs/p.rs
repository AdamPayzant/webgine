use crate::html_elements::common_attributes;

#[derive(Debug, Clone, PartialEq)]
pub struct P {}

impl Default for P {
    fn default() -> Self {
        P {}
    }
}

impl common_attributes::Element for P {
    fn add_attribute(&mut self, name: String, value: String) {
        _ = name;
        _ = value;
    }
}
