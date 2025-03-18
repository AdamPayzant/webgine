use crate::html_elements::common_attributes;

#[derive(Debug, Clone)]
pub struct Fieldset {
    disabled: bool,
    form: Option<String>, // ID
    name: Option<String>,
}

impl Default for Fieldset {
    fn default() -> Self {
        Fieldset {
            disabled: false,
            form: None,
            name: None,
        }
    }
}

impl common_attributes::Element for Fieldset {
    fn add_attribute(&mut self, name: String, value: String) {
        match name.as_str() {
            "disabled" => self.disabled = true,
            "form" => self.form = Some(value),
            "name" => self.name = Some(value),
            _ => {}
        }
    }
}
