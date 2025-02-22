use crate::html_elements::common_attributes;

pub struct OptGroup {
    disabled: bool,
    label: String,
}

impl Default for OptGroup {
    fn default() -> Self {
        OptGroup {
            disabled: false,
            label: "".to_string(),
        }
    }
}

impl common_attributes::Element for OptGroup {
    fn add_attribute(&mut self, name: String, value: String) {
        match name.as_str() {
            "disabled" => self.disabled = true,
            "label" => self.label = value,
            _ => {}
        }
    }
}
