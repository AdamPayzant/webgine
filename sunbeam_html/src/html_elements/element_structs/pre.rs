use crate::html_elements::common_attributes;

#[derive(Clone)]
pub struct Pre {}

impl Default for Pre {
    fn default() -> Self {
        Pre {}
    }
}

impl common_attributes::Element for Pre {
    fn add_attribute(&mut self, name: String, value: String) {
        _ = name;
        _ = value;
    }
}
