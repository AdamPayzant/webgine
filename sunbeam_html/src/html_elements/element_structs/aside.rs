use crate::html_elements::common_attributes;

#[derive(Debug, Clone, PartialEq)]
pub struct Aside {}

impl Default for Aside {
    fn default() -> Self {
        Aside {}
    }
}

impl common_attributes::Element for Aside {
    fn add_attribute(&mut self, name: String, value: String) {
        _ = name;
        _ = value;
    }
}
