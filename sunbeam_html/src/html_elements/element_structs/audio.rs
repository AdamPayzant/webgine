use crate::html_elements::common_attributes;

pub enum CrossOrigin {
    Anonymous,
    UseCredentials,
}

#[derive(Default)]
pub enum PreloadOptions {
    #[default]
    Metadata,

    None,
    Auto,
}

pub struct Audio {
    autoplay: bool,
    controls: bool,
    controlslist: common_attributes::MediaControlList,
    crossorigin: Option<CrossOrigin>,
    disableremoteplayback: bool,
    loop_content: bool,
    muted: bool,
    preload: PreloadOptions,
    source: Option<String>,
}

impl Default for Audio {
    fn default() -> Self {
        Audio {
            autoplay: false,
            controls: false,
            controlslist: common_attributes::MediaControlList::default(),
            crossorigin: None,
            disableremoteplayback: false,
            loop_content: false,
            muted: false,
            preload: PreloadOptions::default(),
            source: None,
        }
    }
}
