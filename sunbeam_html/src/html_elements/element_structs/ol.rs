use crate::html_elements::common_attributes;

#[derive(Default)]
pub enum ListTypeOption {
    #[default]
    Number,
    UppercaseLetters,
    LowercaseLetters,
    UppercaseRomanNumerals,
    LowercaseRomanNumerals,
}

impl ListTypeOption {
    pub fn derive_type(value: &str) -> ListTypeOption {
        match value {
            "a" => Self::LowercaseLetters,
            "A" => Self::UppercaseLetters,
            "i" => Self::LowercaseRomanNumerals,
            "I" => Self::LowercaseRomanNumerals,
            "1" | _ => Self::Number,
        }
    }
}

pub struct Ol {
    reversed: bool,
    start: usize,
    list_type: ListTypeOption,
}

impl Default for Ol {
    fn default() -> Self {
        Ol {
            reversed: false,
            start: 1,
            list_type: ListTypeOption::default(),
        }
    }
}

impl common_attributes::Element for Ol {
    fn add_attribute(&mut self, name: String, value: String) {
        match name.as_str() {
            "reversed" => self.reversed = true,
            "start" => match value.parse() {
                Ok(v) => self.start = v,
                Err(_) => {}
            },
            "type" => self.list_type = ListTypeOption::derive_type(value.as_str()),
            _ => {}
        }
    }
}
