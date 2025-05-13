use crate::html_elements::common_attributes;

#[derive(Debug, Clone, PartialEq)]
pub struct Dialog {
    open: bool,
}

impl Default for Dialog {
    fn default() -> Self {
        Dialog { open: false }
    }
}

impl common_attributes::Element for Dialog {
    fn add_attribute(&mut self, name: String, value: String) {
        _ = value;
        match name.as_str() {
            "open" => self.open = true,
            _ => {}
        }
    }
}
