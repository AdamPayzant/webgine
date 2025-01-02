#[derive(Default)]
pub enum FormMethodOption {
    #[default]
    None,
    Post,
    Get,
    Diaglog,
}

#[derive(Default)]
pub enum FormTargetOption {
    #[default]
    SelfTarget,
    Blank,
    Parent,
    Top,
}

pub enum PopovertargetActionType {
    Hide,
    Show,
    Toggle,
}

#[derive(Default)]
pub enum ButtonType {
    #[default]
    Submit,
    Reset,
    Button,
}

pub struct Button {
    autofocus: bool,
    disabled: bool,
    form: Option<String>,       // ID
    formaction: Option<String>, // Action function override
    formenctype: Option<String>,
    formmethod: FormMethodOption,
    formvalidate: bool,
    formtarget: FormTargetOption,
    name: Option<String>,
    popovertarget: Option<String>, // ID
    popovertargetaction: Option<PopovertargetActionType>,
    button_type: ButtonType,
    value: Option<String>,
}

impl Default for Button {
    fn default() -> Self {
        Button {
            autofocus: false,
            disabled: false,
            form: None,
            formaction: None,
            formenctype: None,
            formmethod: FormMethodOption::default(),
            formvalidate: false,
            formtarget: FormTargetOption::default(),
            name: None,
            popovertarget: None,
            popovertargetaction: None,
            button_type: ButtonType::default(),
            value: None,
        }
    }
}
