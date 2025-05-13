use crate::html_elements::common_attributes;

#[derive(Debug, Clone, PartialEq)]
pub struct Base {
    href: Option<String>,
    target: common_attributes::Target,
}

impl Default for Base {
    fn default() -> Self {
        Base {
            href: None,
            target: common_attributes::Target::default(),
        }
    }
}

impl common_attributes::Element for Base {
    fn add_attribute(&mut self, name: String, value: String) {
        match name.as_str() {
            "href" => self.href = Some(value),
            "target" => self.target = common_attributes::Target::derive_target(value.as_str()),
            _ => {}
        };
    }
}
