use crate::html_elements::common_attributes;

#[derive(Clone)]
pub struct FigCaption {}

impl Default for FigCaption {
    fn default() -> Self {
        FigCaption {}
    }
}

impl common_attributes::Element for FigCaption {
    fn add_attribute(&mut self, name: String, value: String) {
        _ = name;
        _ = value;
    }
}
