use crate::html_elements::common_attributes;

pub struct Map {
    name: Option<String>,
}

impl Default for Map {
    fn default() -> Self {
        Map { name: None }
    }
}

impl common_attributes::Element for Map {
    fn add_attribute(&mut self, name: String, value: String) {
        match name.as_str() {
            "name" => self.name = Some(value),
            _ => {}
        }
    }
}
