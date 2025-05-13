use crate::html_elements::common_attributes;

/* Address - Contact address element
 *
 */
#[derive(Debug, Clone, PartialEq)]
pub struct Address {}

impl Default for Address {
    fn default() -> Self {
        Address {}
    }
}

impl common_attributes::Element for Address {
    fn add_attribute(&mut self, name: String, value: String) {
        _ = name;
        _ = value;
    }
}
