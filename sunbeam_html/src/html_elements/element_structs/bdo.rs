/* Dir Options
 *
 * Options for the Direction attribute to determine if the
 * Text is read based on the user agent, left to right, or right to left
 */
pub enum DirOptions {
    Ltr,
    Rtl,
}

/* bdo - Bidirectional text override
 *
 *
 */
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
