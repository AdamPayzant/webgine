use crate::html_elements::common_attributes;

#[derive(Clone)]
pub struct Embed {
    height: usize,
    width: usize,
    src: Option<String>,       // URL
    mime_type: Option<String>, // Mime type
}

impl Default for Embed {
    fn default() -> Self {
        Embed {
            height: 0,
            width: 0,
            src: None,
            mime_type: None,
        }
    }
}

impl common_attributes::Element for Embed {
    fn add_attribute(&mut self, name: String, value: String) {
        match name.as_str() {
            "height" => match value.parse() {
                Ok(h) => self.height = h,
                Err(_) => {}
            },
            "width" => match value.parse() {
                Ok(w) => self.width = w,
                Err(_) => {}
            },
            "src" => self.src = Some(value),
            "type" => self.mime_type = Some(value),
            _ => {}
        }
    }
}
