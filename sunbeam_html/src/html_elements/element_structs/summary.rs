use crate::html_elements::common_attributes;

#[derive(Clone)]
pub struct Summary {}

impl Default for Summary {
    fn default() -> Self {
        Summary {}
    }
}

impl common_attributes::Element for Summary {
    fn add_attribute(&mut self, name: String, value: String) {
        _ = name;
        _ = value;
    }
}
