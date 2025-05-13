use crate::html_elements::common_attributes;

#[derive(Debug, Clone, PartialEq)]
pub struct ColGroup {
    span: u32,
}

impl Default for ColGroup {
    fn default() -> Self {
        ColGroup { span: 1 }
    }
}

impl common_attributes::Element for ColGroup {
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
