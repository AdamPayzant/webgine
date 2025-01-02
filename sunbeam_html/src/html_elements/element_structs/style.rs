use crate::html_elements::common_attributes;

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
