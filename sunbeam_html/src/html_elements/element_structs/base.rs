use crate::html_elements::common_attributes;

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
