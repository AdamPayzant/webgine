use crate::html_elements::common_attributes;

#[derive(Debug, Clone, PartialEq)]
pub struct Data {
    value: String,
}

impl Default for Data {
    fn default() -> Self {
        Data {
            value: "".to_string(),
        }
    }
}

impl common_attributes::Element for Data {
    fn add_attribute(&mut self, name: String, value: String) {
        match name.as_str() {
            "value" => self.value = value,
            _ => {}
        }
    }
}
