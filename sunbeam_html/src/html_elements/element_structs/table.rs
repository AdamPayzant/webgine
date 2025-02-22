use crate::html_elements::common_attributes;

pub struct Table {}

impl Default for Table {
    fn default() -> Self {
        Table {}
    }
}

impl common_attributes::Element for Table {
    fn add_attribute(&mut self, name: String, value: String) {
        _ = name;
        _ = value;
    }
}
