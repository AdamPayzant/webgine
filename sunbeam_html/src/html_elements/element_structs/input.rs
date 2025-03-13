use crate::html_elements::common_attributes;

#[derive(Clone, Default)]
pub enum PopoverTargetActionOption {
    Hide,
    Show,
    #[default]
    Toggle,
}

#[derive(Clone, Default)]
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

impl InputTypeOption {
    pub fn derive_input(value: &str) -> InputTypeOption {
        use InputTypeOption::*;
        match value {
            "button" => Button,
            "checkbox" => Checkbox,
            "color" => Color,
            "date" => Date,
            "datetime-local" => DatetimeLocal,
            "email" => Email,
            "file" => File,
            "hidden" => Hidden,
            "image" => Image,
            "month" => Month,
            "number" => Number,
            "password" => Password,
            "radio" => Radio,
            "range" => Range,
            "reset" => Reset,
            "search" => Search,
            "submit" => Submit,
            "tel" => Tel,
            "text" => Text,
            "time" => Time,
            "url" => Url,
            "week" => Week,
            _ => Button,
        }
    }
}

#[derive(Clone)]
pub struct Input {
    accept: Option<String>, // TODO: Should probably make this it's own type
    alt: Option<String>,
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

impl common_attributes::Element for Input {
    fn add_attribute(&mut self, name: String, value: String) {
        match name.as_str() {
            "accept" => self.accept = Some(value),
            "alt" => self.alt = Some(value),
            "capture" => self.capture = Some(value),
            "checked" => self.checked = true,
            "dirname" => self.dirname = Some(value),
            "disabled" => self.disabled = true,
            "form" => self.form = Some(value),
            "formaction" => self.formaction = Some(value),
            "formenctype" => self.formenctype = Some(value),
            "formmethod" => self.formmethod = Some(value),
            "formnovalidate" => self.formnovalidate = Some(value),
            "formtarget" => self.formtarget = Some(value),
            "height" => match value.parse() {
                Ok(h) => self.height = Some(h),
                Err(_) => {}
            },
            "list" => self.list = Some(value),
            "max" => match value.parse() {
                Ok(m) => self.max = Some(m),
                Err(_) => {}
            },
            "maxlength" => match value.parse() {
                Ok(ml) => self.maxlength = Some(ml),
                Err(_) => {}
            },
            "min" => match value.parse() {
                Ok(m) => self.min = Some(m),
                Err(_) => {}
            },
            "minlength" => match value.parse() {
                Ok(ml) => self.minlength = Some(ml),
                Err(_) => {}
            },
            "multiple" => self.multiple = true,
            "name" => self.name = Some(value),
            "pattern" => self.pattern = Some(value),
            "placeholder" => self.placeholder = Some(value),
            "popovertarget" => self.popovertarget = Some(value),
            "popovertargetaction" => {
                self.popovertargetaction = match value.as_str() {
                    "hide" => PopoverTargetActionOption::Hide,
                    "show" => PopoverTargetActionOption::Show,
                    "toggle" | _ => PopoverTargetActionOption::Toggle,
                }
            }
            "readonly" => self.readonly = true,
            "required" => self.required = true,
            "size" => self.size = Some(value),
            "src" => self.src = Some(value),
            "step" => match value.parse() {
                Ok(s) => self.step = s,
                Err(_) => {}
            },
            "type" => self.input_type = InputTypeOption::derive_input(value.as_str()),
            "value" => self.value = Some(value),
            "width" => match value.parse() {
                Ok(w) => self.width = Some(w),
                Err(_) => {}
            },
            _ => {}
        }
    }
}
