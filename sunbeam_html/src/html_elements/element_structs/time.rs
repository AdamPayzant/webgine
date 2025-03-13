use crate::html_elements::common_attributes;

#[derive(Clone)]
pub struct Time {
    datetime: String, // TODO: Implement date string type
}

impl Default for Time {
    fn default() -> Self {
        Time {
            datetime: "".to_string(),
        }
    }
}

impl common_attributes::Element for Time {
    fn add_attribute(&mut self, name: String, value: String) {
        match name.as_str() {
            "datetime" => self.datetime = value,
            _ => {}
        }
    }
}
