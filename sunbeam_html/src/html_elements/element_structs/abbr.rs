use crate::html_elements::common_attributes;

/* Abbr - Abbreviation Element
 *
 * Abbreviation only contain global attributes and child nodes
 * so it's an empty struct
 */
pub struct Abbr {}

impl Default for Abbr {
    fn default() -> Self {
        Abbr {}
    }
}

impl common_attributes::Element for Abbr {
    fn add_attribute(&mut self, name: String, value: String) {
        _ = name;
        _ = value;
    }
}
