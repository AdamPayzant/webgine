use crate::html_elements::common_attributes;

#[derive(Clone)]
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

impl common_attributes::Element for Select {
    fn add_attribute(&mut self, name: String, value: String) {
        match name.as_str() {
            "autocomplete" => {
                self.autocomplete = common_attributes::AutoComplete::derive_type(value.as_str())
            }
            "autofocus" => self.autofocus = true,
            "disabled" => self.disabled = true,
            "form" => self.form = Some(value),
            "multiple" => self.multiple = true,
            "name" => self.name = Some(value),
            "required" => self.required = true,
            "size" => match value.parse() {
                Ok(v) => self.size = v,
                Err(_) => {}
            },
            _ => {}
        }
    }
}
