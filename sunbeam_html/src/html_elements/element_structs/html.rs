use crate::html_elements::common_attributes;

#[derive(Debug, Clone)]
pub struct Html {
    xmlns: Option<String>, // URL
}

impl Default for Html {
    fn default() -> Self {
        Html { xmlns: None }
    }
}

impl common_attributes::Element for Html {
    fn add_attribute(&mut self, name: String, value: String) {
        match name.as_str() {
            "xmlns" => self.xmlns = Some(value),
            _ => {}
        }
    }
}
