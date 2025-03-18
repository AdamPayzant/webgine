use crate::html_elements::common_attributes;

#[derive(Debug, Clone, Default)]
pub enum FormMethodOption {
    #[default]
    None,
    Post,
    Get,
    Diaglog,
}

#[derive(Debug, Clone, Default)]
pub enum FormTargetOption {
    #[default]
    SelfTarget,
    Blank,
    Parent,
    Top,
}

#[derive(Debug, Clone)]
pub enum PopovertargetActionType {
    Hide,
    Show,
    Toggle,
}

#[derive(Debug, Clone, Default)]
pub enum ButtonType {
    #[default]
    Submit,
    Reset,
    Button,
}

#[derive(Debug, Clone)]
pub struct Button {
    autofocus: bool,
    disabled: bool,
    form: Option<String>,       // ID
    formaction: Option<String>, // Action function override
    formenctype: Option<String>,
    formmethod: FormMethodOption,
    formnovalidate: bool,
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
            formnovalidate: false,
            formtarget: FormTargetOption::default(),
            name: None,
            popovertarget: None,
            popovertargetaction: None,
            button_type: ButtonType::default(),
            value: None,
        }
    }
}

impl common_attributes::Element for Button {
    fn add_attribute(&mut self, name: String, value: String) {
        match name.as_str() {
            "autofocus" => self.autofocus = true,
            "disabled" => self.disabled = true,
            "form" => self.form = Some(value),
            "formaction" => self.formaction = Some(value),
            "formenctype" => self.formenctype = Some(value),
            "formmethod" => {
                self.formmethod = match value.as_str() {
                    "post" => FormMethodOption::Post,
                    "get" => FormMethodOption::Get,
                    "dialog" => FormMethodOption::Diaglog,
                    _ => FormMethodOption::None,
                }
            }
            "formnovalidate" => self.formnovalidate = true,
            "formtarget" => {
                self.formtarget = match value.as_str() {
                    "_self" => FormTargetOption::SelfTarget,
                    "_blank" => FormTargetOption::Blank,
                    "_parent" => FormTargetOption::Parent,
                    "_top" => FormTargetOption::Top,
                    _ => FormTargetOption::default(),
                }
            }
            "name" => self.name = Some(value),
            "popovertarget" => self.popovertarget = Some(value),
            "popovertargetaction" => {
                self.popovertargetaction = match value.as_str() {
                    "hide" => Some(PopovertargetActionType::Hide),
                    "show" => Some(PopovertargetActionType::Show),
                    "toggle" => Some(PopovertargetActionType::Toggle),
                    _ => None,
                }
            }
            "type" => {
                self.button_type = match value.as_str() {
                    "submit" => ButtonType::Submit,
                    "reset" => ButtonType::Reset,
                    "button" => ButtonType::Button,
                    _ => ButtonType::default(),
                }
            }
            "value" => self.value = Some(value),
            _ => {}
        }
    }
}
