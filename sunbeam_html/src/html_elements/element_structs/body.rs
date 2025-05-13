use crate::html_elements::common_attributes;

#[derive(Debug, Clone, PartialEq)]
pub struct Body {}

impl Default for Body {
    fn default() -> Self {
        Body {}
    }
}

impl common_attributes::Element for Body {
    fn add_attribute(&mut self, name: String, value: String) {
        _ = name;
        _ = value;
    }
}
