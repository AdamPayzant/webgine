use crate::html_elements::common_attributes;

#[derive(Debug, Clone)]
pub struct Mark {}

impl Default for Mark {
    fn default() -> Self {
        Mark {}
    }
}

impl common_attributes::Element for Mark {
    fn add_attribute(&mut self, name: String, value: String) {
        _ = name;
        _ = value;
    }
}
