pub struct Fieldset {
    disabled: bool,
    form: Option<String>, // ID
    name: Option<String>,
}

impl Default for Fieldset {
    fn default() -> Self {
        Fieldset {
            disabled: false,
            form: None,
            name: None,
        }
    }
}
