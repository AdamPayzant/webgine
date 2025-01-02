#[derive(Default)]
pub enum ListTypeOption {
    #[default]
    Number,
    UppercaseLetters,
    LowercaseLetters,
    UppercaseRomanNumerals,
    LowercaseRomanNumerals,
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
