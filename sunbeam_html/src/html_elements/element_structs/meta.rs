use crate::html_elements::common_attributes;

#[derive(Debug, Clone, PartialEq)]
pub struct Meta {
    charset: Option<String>, // TODO: Add charset type
    content: Option<String>,
    http_equiv: Option<common_attributes::HttpEquivalent>,
    media: String, // TODO: Add media query type
    name: Option<String>,
}

impl Default for Meta {
    fn default() -> Self {
        Meta {
            charset: None,
            content: None,
            http_equiv: None,
            media: "all".to_string(),
            name: None,
        }
    }
}

impl common_attributes::Element for Meta {
    fn add_attribute(&mut self, name: String, value: String) {
        match name.as_str() {
            "charset" => self.charset = Some(value),
            "content" => self.content = Some(value),
            "http-equiv" => {
                self.http_equiv =
                    common_attributes::HttpEquivalent::derive_equivalent(value.as_str())
            }
            "media" => self.media = value,
            "name" => self.name = Some(value),
            _ => {}
        }
    }
}
