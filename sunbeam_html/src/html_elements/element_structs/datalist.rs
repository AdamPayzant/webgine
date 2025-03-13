use crate::html_elements::common_attributes;

#[derive(Clone)]
pub struct DataList {}

impl Default for DataList {
    fn default() -> Self {
        DataList {}
    }
}

impl common_attributes::Element for DataList {
    fn add_attribute(&mut self, name: String, value: String) {
        _ = name;
        _ = value;
    }
}
