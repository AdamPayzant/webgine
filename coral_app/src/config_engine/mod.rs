use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

mod fonts;

#[derive(Default, Deserialize)]
struct ParsedConfig {
    font_config: Option<fonts::FontConfig>,
}

impl ParsedConfig {
    pub fn resolve_parsed(&self) -> Config {
        let mut res = Config::default();

        if let Some(font_cfg) = self.font_config.clone() {
            res.font_config = font_cfg;
        }

        res
    }
}

#[derive(Default)]
pub struct Config {
    font_config: fonts::FontConfig,
}

impl Config {
    pub fn new() -> Config {
        let file = if cfg!(target_os = "linux") || cfg!(target_os = "macos") {
            let d = PathBuf::from("~/.config/webgine/coralconf.toml");

            match fs::read_to_string(d) {
                Ok(s) => s,
                Err(_) => return Config::default(),
            }
        } else {
            return Config::default();
        };

        let parsed_config: ParsedConfig = match toml::from_str(&file) {
            Ok(c) => c,
            Err(_) => return Config::default(),
        };

        parsed_config.resolve_parsed()
    }
}
