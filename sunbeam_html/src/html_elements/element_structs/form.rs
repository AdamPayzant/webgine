use crate::html_elements::common_attributes;

pub struct Form {
    accept_charset: Option<String>, // TODO: Implement charset lookup
    autocapitalize: bool,
    autocomplete: bool,
    name: Option<String>,
    rel: Vec<common_attributes::Rel>,
}

impl Default for Form {
    fn default() -> Self {
        Form {
            accept_charset: None,
            autocapitalize: false,
            autocomplete: false,
            name: None,
            rel: Vec::new(),
        }
    }
}
