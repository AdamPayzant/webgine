use crate::html_elements::common_attributes;

#[derive(Debug, Clone, PartialEq)]
pub struct Form {
    accept_charset: Option<String>, // TODO: Implement charset lookup
    autocomplete: bool,
    name: Option<String>,
    rel: common_attributes::Rel,
}

impl Default for Form {
    fn default() -> Self {
        Form {
            accept_charset: None,
            autocomplete: false,
            name: None,
            rel: common_attributes::Rel::default(),
        }
    }
}

impl common_attributes::Element for Form {
    fn add_attribute(&mut self, name: String, value: String) {
        match name.as_str() {
            "accept-charset" => self.accept_charset = Some(value),
            "autocomplete" => {
                self.autocomplete = match value.as_str() {
                    "on" => true,
                    "off" | _ => false,
                }
            }
            "name" => self.name = Some(value),
            "rel" => self.rel = common_attributes::Rel::derive_rels(value.as_str()),
            _ => {}
        }
    }
}
