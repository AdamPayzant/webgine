use crate::html_elements::common_attributes;

#[derive(Default)]
pub enum WrapOptions {
    Hard,
    #[default]
    Soft,
}

pub struct TextArea {
    autocomplete: common_attributes::AutoComplete,
    autocorrect: bool,
    autofocus: bool,
    cols: usize,
    disabled: bool,
    form: Option<String>, // ID
    maxlength: Option<usize>,
    minlength: Option<usize>,
    name: Option<String>,
    placeholder: Option<String>,
    readonly: bool,
    required: bool,
    rows: usize,
    spellcheck: bool,
    wrap: WrapOptions,
}

impl Default for TextArea {
    fn default() -> Self {
        TextArea {
            autocomplete: common_attributes::AutoComplete::default(),
            autocorrect: false,
            autofocus: false,
            cols: 20,
            disabled: false,
            form: None,
            maxlength: None,
            minlength: None,
            name: None,
            placeholder: None,
            readonly: false,
            required: false,
            rows: 2,
            spellcheck: false,
            wrap: WrapOptions::default(),
        }
    }
}

impl common_attributes::Element for TextArea {
    fn add_attribute(&mut self, name: String, value: String) {
        match name.as_str() {
            "autocomplete" => {
                self.autocomplete = common_attributes::AutoComplete::derive_type(value.as_str())
            }
            "autocorrect" => {
                self.autocorrect = match value.as_str() {
                    "on" => true,
                    "off" | _ => false,
                }
            }
            "autofocus" => self.autofocus = true,
            "cols" => match value.parse() {
                Ok(v) => self.cols = v,
                Err(_) => {}
            },
            "disabled" => self.disabled = true,
            "form" => self.form = Some(value),
            "maxlength" => {
                self.maxlength = match value.parse() {
                    Ok(v) => Some(v),
                    Err(_) => None,
                }
            }
            "minlength" => {
                self.minlength = match value.parse() {
                    Ok(v) => Some(v),
                    Err(_) => None,
                }
            }
            "name" => self.name = Some(value),
            "placeholder" => self.placeholder = Some(value),
            "readonly" => self.readonly = true,
            "required" => self.required = true,
            "rows" => match value.parse() {
                Ok(v) => self.rows = v,
                Err(_) => {}
            },
            "spellcheck" => {
                self.spellcheck = match value.as_str() {
                    // TODO: Handle default case
                    "false" => false,
                    "true" | _ => true,
                }
            }
            "wrap" => {
                self.wrap = match value.as_str() {
                    "hard" => WrapOptions::Hard,
                    "soft" | _ => WrapOptions::Soft,
                }
            }
            _ => {}
        }
    }
}
