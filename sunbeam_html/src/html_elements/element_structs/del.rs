use crate::html_elements::common_attributes;

#[derive(Debug, Clone, PartialEq)]
pub struct Del {
    cite: Option<String>,
    datetime: Option<String>, // TODO: Probably should implement an explicit date-string type
}

impl Default for Del {
    fn default() -> Self {
        Del {
            cite: None,
            datetime: None,
        }
    }
}

impl common_attributes::Element for Del {
    fn add_attribute(&mut self, name: String, value: String) {
        match name.as_str() {
            "cite" => self.cite = Some(value),
            "datetime" => self.datetime = Some(value),
            _ => {}
        }
    }
}
