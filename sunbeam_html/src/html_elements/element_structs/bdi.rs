/* BDI - Bidirectional Isolate element
 *
 * Ignores parent's dir attribute and uses user agent or it's own
 */
pub struct BDI {}

impl Default for BDI {
    fn default() -> Self {
        BDI {}
    }
}
