use crate::html_elements::common_attributes;

/* BDI - Bidirectional Isolate element
 *
 * Ignores parent's dir attribute and uses user agent or it's own
 */
#[derive(Clone)]
pub struct BDI {}

impl Default for BDI {
    fn default() -> Self {
        BDI {}
    }
}

impl common_attributes::Element for BDI {
    fn add_attribute(&mut self, name: String, value: String) {
        _ = name;
        _ = value;
    }
}
