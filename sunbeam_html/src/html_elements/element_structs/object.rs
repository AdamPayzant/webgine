use crate::html_elements::common_attributes;

#[derive(Debug, Clone)]
pub struct Object {
    data: Option<String>, // URL
    form: Option<String>, // ID
    height: Option<usize>,
    name: Option<String>,
    content_type: Option<String>,
    width: Option<usize>,
}

impl Default for Object {
    fn default() -> Self {
        Object {
            data: None,
            form: None,
            height: None,
            name: None,
            content_type: None,
            width: None,
        }
    }
}

impl common_attributes::Element for Object {
    fn add_attribute(&mut self, name: String, value: String) {
        match name.as_str() {
            "data" => self.data = Some(value),
            "form" => self.form = Some(value),
            "height" => {
                self.height = match value.parse() {
                    Ok(v) => Some(v),
                    Err(_) => None,
                }
            }
            "name" => self.name = Some(value),
            "type" => self.content_type = Some(value),
            "width" => {
                self.width = match value.parse() {
                    Ok(v) => Some(v),
                    Err(_) => None,
                }
            }
            _ => {}
        }
    }
}
