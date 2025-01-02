use crate::html_elements::common_attributes;
use crate::html_elements::global_attr;

#[derive(Default)]
pub enum WrapOptions {
    Hard,
    #[default]
    Soft,
}

pub struct TextArea {
    autocapitalize: global_attr::AutoCapitalizeOptions,
    autocomplete: common_attributes::AutoComplete,
    autocorrect: bool,
    autofocus: bool,
    cols: usize,
    dirname: global_attr::DirOptions,
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
            autocapitalize: global_attr::AutoCapitalizeOptions::default(),
            autocomplete: common_attributes::AutoComplete::default(),
            autocorrect: false,
            autofocus: false,
            cols: 20,
            dirname: global_attr::DirOptions::default(),
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
