use crate::html_elements::common_attributes;

#[derive(Clone)]
pub struct Figure {}

impl Default for Figure {
    fn default() -> Self {
        Figure {}
    }
}

impl common_attributes::Element for Figure {
    fn add_attribute(&mut self, name: String, value: String) {
        _ = name;
        _ = value;
    }
}
