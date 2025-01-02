use crate::html_elements::global_attr;

#[derive(Default)]
pub enum PopoverTargetActionOption {
    Hide,
    Show,
    #[default]
    Toggle,
}

#[derive(Default)]
pub enum InputTypeOption {
    #[default]
    Button,
    Checkbox,
    Color,
    Date,
    DatetimeLocal,
    Email,
    File,
    Hidden,
    Image,
    Month,
    Number,
    Password,
    Radio,
    Range,
    Reset,
    Search,
    Submit,
    Tel,
    Text,
    Time,
    Url,
    Week,
}

pub struct Input {
    accept: Option<String>, // TODO: Should probably make this it's own type
    alt: Option<String>,
    autocapitalize: global_attr::AutoCapitalizeOptions,
    autofocus: bool,
    capture: Option<String>,
    checked: bool,
    dirname: Option<String>,
    disabled: bool,
    form: Option<String>,       // ID
    formaction: Option<String>, // TODO: Add action once actions are implemented
    formenctype: Option<String>,
    formmethod: Option<String>,
    formnovalidate: Option<String>,
    formtarget: Option<String>,
    height: Option<usize>,
    id: Option<String>,   // ID
    list: Option<String>, // ID of relevant datalist element
    max: Option<isize>,
    maxlength: Option<usize>,
    min: Option<isize>,
    minlength: Option<usize>,
    multiple: bool,
    name: Option<String>,
    pattern: Option<String>, // Regex pattern
    placeholder: Option<String>,
    popovertarget: Option<String>, // ID
    popovertargetaction: PopoverTargetActionOption,
    readonly: bool,
    required: bool,
    size: Option<String>, // TODO: This will eventually need to be parsed and applied
    src: Option<String>,  // URL
    step: usize,          // TODO: This should consider what the input type is
    input_type: InputTypeOption,
    value: Option<String>,
    width: Option<usize>,
}

impl Default for Input {
    fn default() -> Self {
        Input {
            accept: None,
            alt: None,
            autocapitalize: global_attr::AutoCapitalizeOptions::default(),
            autofocus: false,
            capture: None,
            checked: false,
            dirname: None,
            disabled: false,
            form: None,
            formaction: None,
            formenctype: None,
            formmethod: None,
            formnovalidate: None,
            formtarget: None,
            height: None,
            id: None,
            list: None,
            max: None,
            maxlength: None,
            min: None,
            minlength: None,
            multiple: false,
            name: None,
            pattern: None,
            placeholder: None,
            popovertarget: None,
            popovertargetaction: PopoverTargetActionOption::default(),
            readonly: false,
            required: false,
            size: None,
            src: None,
            step: 1,
            input_type: InputTypeOption::default(),
            value: None,
            width: None,
        }
    }
}
