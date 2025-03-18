use crate::html_elements::common_attributes;

#[derive(Debug, Clone)]
pub struct Slot {
    name: Option<String>,
}

impl Default for Slot {
    fn default() -> Self {
        Slot { name: None }
    }
}

impl common_attributes::Element for Slot {
    fn add_attribute(&mut self, name: String, value: String) {
        match name.as_str() {
            "name" => self.name = Some(value),
            _ => {}
        }
    }
}
