use crate::html_elements::common_attributes;

pub struct Select {
    autocomplete: common_attributes::AutoComplete,
    autofocus: bool,
    disabled: bool,
    form: Option<String>, // ID
    multiple: bool,
    name: Option<String>,
    required: bool,
    size: usize,
}

impl Default for Select {
    fn default() -> Self {
        Select {
            autocomplete: common_attributes::AutoComplete::default(),
            autofocus: false,
            disabled: false,
            form: None,
            multiple: false,
            name: None,
            required: false,
            size: 0,
        }
    }
}
