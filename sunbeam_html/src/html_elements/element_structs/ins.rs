use crate::html_elements::common_attributes;

pub struct Ins {
    cite: Option<String>,
    datetime: Option<String>, // Datetime string
}

impl Default for Ins {
    fn default() -> Self {
        Ins {
            cite: None,
            datetime: None,
        }
    }
}

impl common_attributes::Element for Ins {
    fn add_attribute(&mut self, name: String, value: String) {
        match name.as_str() {
            "cite" => self.cite = Some(value),
            "datetime" => self.datetime = Some(value),
            _ => {}
        }
    }
}
