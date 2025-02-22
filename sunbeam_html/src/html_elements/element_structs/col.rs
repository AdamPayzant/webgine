use crate::html_elements::common_attributes;

pub struct Col {
    span: u32,
}

impl Default for Col {
    fn default() -> Self {
        Col { span: 1 }
    }
}

impl common_attributes::Element for Col {
    fn add_attribute(&mut self, name: String, value: String) {
        match name.as_str() {
            "span" => {
                self.span = match value.parse() {
                    Ok(s) => s,
                    Err(_) => return,
                }
            }
            _ => {}
        }
    }
}
