use crate::html_elements::common_attributes;

pub struct Meta {
    charset: Option<String>, // TODO: Add charset type
    content: Option<String>,
    http_equiv: common_attributes::HttpEquivalent,
    media: String, // TODO: Add media query type
    name: Option<String>,
}

impl Default for Meta {
    fn default() -> Self {
        Meta {
            charset: None,
            content: None,
            http_equiv: common_attributes::HttpEquivalent::default(),
            media: "all".to_string(),
            name: None,
        }
    }
}
