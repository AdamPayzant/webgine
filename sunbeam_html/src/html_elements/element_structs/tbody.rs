use crate::html_elements::common_attributes;

pub struct TBody {}

impl Default for TBody {
    fn default() -> Self {
        TBody {}
    }
}

impl common_attributes::Element for TBody {
    fn add_attribute(&mut self, name: String, value: String) {
        _ = name;
        _ = value;
    }
}
