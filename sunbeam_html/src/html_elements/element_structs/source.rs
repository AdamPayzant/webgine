use crate::html_elements::common_attributes;

#[derive(Clone)]
pub struct Source {
    mime_type: Option<String>, // TODO: Change when I have mime type system
    src: Option<String>,       // URL
    srcset: Vec<String>,       // List of URLs
    sizes: Vec<String>,        // TODO: Need a type for sizes
    media: Option<String>,     // Need a type for media queries
    height: Option<usize>,
    width: Option<usize>,
}

impl Default for Source {
    fn default() -> Self {
        Source {
            mime_type: None,
            src: None,
            srcset: Vec::new(),
            sizes: Vec::new(),
            media: None,
            height: None,
            width: None,
        }
    }
}

impl common_attributes::Element for Source {
    fn add_attribute(&mut self, name: String, value: String) {
        match name.as_str() {
            "type" => self.mime_type = Some(value),
            "src" => self.src = Some(value),
            "srcset" => self.srcset = value.split(",").map(|s| s.trim().to_string()).collect(),
            "sizes" => self.sizes = value.split(",").map(|s| s.trim().to_string()).collect(),
            "media" => self.media = Some(value),
            "height" => {
                self.height = match value.parse() {
                    Ok(v) => Some(v),
                    Err(_) => None,
                }
            }
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
