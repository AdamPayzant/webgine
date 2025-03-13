use crate::html_elements::common_attributes;

#[derive(Clone)]
pub struct Label {
    label_for: Option<String>, // ID
}

impl Default for Label {
    fn default() -> Self {
        Label { label_for: None }
    }
}

impl common_attributes::Element for Label {
    fn add_attribute(&mut self, name: String, value: String) {
        match name.as_str() {
            "for" => self.label_for = Some(value),
            _ => {}
        }
    }
}
