use crate::html_elements::common_attributes;

#[derive(Clone)]
pub struct Blockquote {
    cite: Option<String>,
}

impl Default for Blockquote {
    fn default() -> Self {
        Blockquote { cite: None }
    }
}

impl common_attributes::Element for Blockquote {
    fn add_attribute(&mut self, name: String, value: String) {
        match name.as_str() {
            "cite" => self.cite = Some(value),
            _ => {}
        }
    }
}
