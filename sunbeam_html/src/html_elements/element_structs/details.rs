use crate::html_elements::common_attributes;

pub struct Details {
    open: bool,
    name: Option<String>,
}

impl Default for Details {
    fn default() -> Self {
        Details {
            open: false,
            name: None,
        }
    }
}

impl common_attributes::Element for Details {
    fn add_attribute(&mut self, name: String, value: String) {
        match name.as_str() {
            "open" => self.open = true,
            "name" => self.name = Some(value),
            _ => {}
        }
    }
}
