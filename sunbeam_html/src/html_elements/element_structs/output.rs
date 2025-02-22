use crate::html_elements::common_attributes;

pub struct Output {
    output_for: Vec<String>,
    form: Option<String>, // ID
    name: Option<String>,
}

impl Default for Output {
    fn default() -> Self {
        Output {
            output_for: Vec::new(),
            form: None,
            name: None,
        }
    }
}

impl common_attributes::Element for Output {
    fn add_attribute(&mut self, name: String, value: String) {
        match name.as_str() {
            "for" => self.output_for = value.split(" ").map(|s| s.to_string()).collect(),
            "form" => self.form = Some(value),
            "name" => self.name = Some(value),
            _ => {}
        }
    }
}
