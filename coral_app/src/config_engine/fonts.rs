use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct FontConfig {
    pub default_font: String,
    pub size: u16,
}

impl Default for FontConfig {
    fn default() -> Self {
        FontConfig {
            default_font: "georgia".to_string(),
            size: 16,
        }
    }
}
