// Name different due to namespaces, should probably be
// considered again
pub struct HtmlOption {
    disabled: bool,
    label: Option<String>,
    selected: bool,
    value: Option<String>,
}

impl Default for HtmlOption {
    fn default() -> Self {
        HtmlOption {
            disabled: false,
            label: None,
            selected: false,
            value: None,
        }
    }
}
