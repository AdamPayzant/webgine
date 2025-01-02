#[derive(Default)]
pub enum KindOptions {
    Subtitles,
    Captions,
    Chapters,
    #[default]
    Metadata,
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
