#[derive(Default)]
pub enum ShadowrootModeOption {
    #[default]
    Open,
    Closed,
}

pub struct Template {
    shadowrootmode: ShadowrootModeOption,
    shadowrootclonable: bool,
    shadowrootdelegatesfocus: bool,
}

impl Default for Template {
    fn default() -> Self {
        Template {
            shadowrootmode: ShadowrootModeOption::default(),
            shadowrootclonable: false,
            shadowrootdelegatesfocus: false,
        }
    }
}
