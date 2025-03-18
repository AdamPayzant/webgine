use crate::html_elements::common_attributes;

/* Dir Options
 *
 * Options for the Direction attribute to determine if the
 * Text is read based on the user agent, left to right, or right to left
 */
#[derive(Debug, Clone)]
pub enum DirOptions {
    Ltr,
    Rtl,
}

/* bdo - Bidirectional text override
 *
 *
 */
#[derive(Debug, Clone)]
pub struct Bdo {
    dir: DirOptions,
}

impl Default for Bdo {
    fn default() -> Self {
        Bdo {
            dir: DirOptions::Ltr,
        }
    }
}

impl common_attributes::Element for Bdo {
    fn add_attribute(&mut self, name: String, value: String) {
        match name.as_str() {
            "dir" => {
                self.dir = match value.as_str() {
                    "rtl" => DirOptions::Rtl,
                    "ltr" | _ => DirOptions::Ltr,
                }
            }
            _ => {}
        }
    }
}
