use crate::html_elements::common_attributes;

#[derive(Debug, Clone)]
pub enum CrossOrigin {
    Anonymous,
    UseCredentials,
}

#[derive(Debug, Clone, Default)]
pub enum PreloadOptions {
    #[default]
    Metadata,

    None,
    Auto,
}

#[derive(Debug, Clone)]
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

impl common_attributes::Element for Audio {
    fn add_attribute(&mut self, name: String, value: String) {
        match name.as_str() {
            "autoplay" => self.autoplay = true,
            "controls" => self.controls = true,
            "controlslist" => {
                self.controlslist =
                    common_attributes::MediaControlList::derive_media_control(value.as_str())
            }
            "crossorigin" => {
                self.crossorigin = match value.as_str() {
                    "use-credentials" => Some(CrossOrigin::UseCredentials),
                    "anonymous" | _ => Some(CrossOrigin::Anonymous),
                }
            }
            "disableremoteplayback" => self.disableremoteplayback = true,
            "loop" => self.loop_content = true,
            "muted" => self.muted = true,
            "preload" => {
                self.preload = match value.as_str() {
                    "none" => PreloadOptions::None,
                    "metadata" => PreloadOptions::Metadata,
                    "auto" | _ => PreloadOptions::Auto,
                }
            }
            "src" => self.source = Some(value),
            _ => {}
        };
    }
}
