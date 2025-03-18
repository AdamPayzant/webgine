use crate::html_elements::common_attributes;

#[derive(Debug, Clone)]
pub struct TFoot {}

impl Default for TFoot {
    fn default() -> Self {
        TFoot {}
    }
}

impl common_attributes::Element for TFoot {
    fn add_attribute(&mut self, name: String, value: String) {
        _ = name;
        _ = value;
    }
}
