use crate::html_elements::common_attributes;

// Name different due to namespaces, should probably be
// considered again
pub struct HtmlOption {
    disabled: bool,
    label: Option<String>,
    selected: bool,
    value: Option<String>,
}

impl Default for HtmlOption {
    fn default() -> Self {
        HtmlOption {
            disabled: false,
            label: None,
            selected: false,
            value: None,
        }
    }
}

impl common_attributes::Element for HtmlOption {
    fn add_attribute(&mut self, name: String, value: String) {
        match name.as_str() {
            "disabled" => self.disabled = true,
            "label" => self.label = Some(value),
            "selected" => self.selected = true,
            "value" => self.value = Some(value),
            _ => {}
        }
    }
}
