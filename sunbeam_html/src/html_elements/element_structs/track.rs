use crate::html_elements::common_attributes;

#[derive(Default)]
pub enum KindOptions {
    Subtitles,
    Captions,
    Chapters,
    #[default]
    Metadata,
}

impl KindOptions {
    pub fn derive_kind(value: &str) -> Option<KindOptions> {
        match value {
            "subtitles" => Some(Self::Subtitles),
            "captions" => Some(Self::Captions),
            "chapters" => Some(Self::Chapters),
            "metadata" => Some(Self::Metadata),
            _ => None,
        }
    }
}

pub struct Track {
    default_track: bool,
    kind: Option<KindOptions>,
    label: Option<String>,
    src: Option<String>,     // URL
    srclang: Option<String>, // TODO: Needs language tag type
}

impl Default for Track {
    fn default() -> Self {
        Track {
            default_track: false,
            kind: None,
            label: None,
            src: None,
            srclang: None,
        }
    }
}

impl common_attributes::Element for Track {
    fn add_attribute(&mut self, name: String, value: String) {
        match name.as_str() {
            "default" => self.default_track = true,
            "kind" => self.kind = KindOptions::derive_kind(value.as_str()),
            "label" => self.label = Some(value),
            "src" => self.src = Some(value),
            "srclang" => self.srclang = Some(value),
            _ => {}
        }
    }
}
