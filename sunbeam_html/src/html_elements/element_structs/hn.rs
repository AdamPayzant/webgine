/* Hn - Header levels
 *
 * Aggregates all header levels into a single struct
 *
 */
pub struct Hn {
    level: u8,
}

impl Default for Hn {
    fn default() -> Self {
        Hn { level: 1 }
    }
}
