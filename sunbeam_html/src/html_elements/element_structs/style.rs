use crate::html_elements::common_attributes;

#[derive(Debug, Clone, PartialEq)]
pub struct Style {
    blocking: Option<common_attributes::BlockingOption>,
    media: Option<String>, // TODO: Need media query type
    nonce: Option<String>,
    title: Option<String>,
}

impl Default for Style {
    fn default() -> Self {
        Style {
            blocking: None,
            media: None,
            nonce: None,
            title: None,
        }
    }
}

impl common_attributes::Element for Style {
    fn add_attribute(&mut self, name: String, value: String) {
        match name.as_str() {
            "blocking" => {
                self.blocking = common_attributes::BlockingOption::derive_blocking(value.as_str())
            }
            "media" => self.media = Some(value),
            "nonce" => self.nonce = Some(value),
            "title" => self.title = Some(value),
            _ => {}
        }
    }
}
