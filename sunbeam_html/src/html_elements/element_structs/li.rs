use crate::html_elements::common_attributes;

pub struct Li {
    value: Option<usize>,
}

impl Default for Li {
    fn default() -> Self {
        Li { value: None }
    }
}

impl common_attributes::Element for Li {
    fn add_attribute(&mut self, name: String, value: String) {
        match name.as_str() {
            "value" => match value.parse() {
                Ok(v) => self.value = Some(v),
                Err(_) => {}
            },
            _ => {}
        }
    }
}
