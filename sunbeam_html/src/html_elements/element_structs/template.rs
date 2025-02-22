use crate::html_elements::common_attributes;

#[derive(Default)]
pub enum ShadowrootModeOption {
    #[default]
    Open,
    Closed,
}

impl ShadowrootModeOption {
    pub fn derive_mode(value: &str) -> ShadowrootModeOption {
        match value {
            "closed" => Self::Closed,
            "open" | _ => Self::Open,
        }
    }
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

impl common_attributes::Element for Template {
    fn add_attribute(&mut self, name: String, value: String) {
        match name.as_str() {
            "shadowrootmode" => {
                self.shadowrootmode = ShadowrootModeOption::derive_mode(value.as_str())
            }
            "shadowrootclonable" => self.shadowrootclonable = true,
            "shadowrootdelegatedfocus" => self.shadowrootdelegatesfocus = true,
            _ => {}
        }
    }
}
