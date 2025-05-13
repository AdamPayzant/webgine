use crate::html_elements::common_attributes;

#[derive(Debug, Clone, Default, PartialEq)]
pub enum VideoPreloadOption {
    None,
    #[default]
    Metadata,
    Auto,
}

impl VideoPreloadOption {
    pub fn derive_preload(value: &str) -> VideoPreloadOption {
        match value {
            "none" => Self::None,
            "metadata" => Self::Metadata,
            "auto" | _ => Self::Auto,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Video {
    autoplay: bool,
    controls: bool,
    controlslist: common_attributes::MediaControlList,
    crossorigin: Option<common_attributes::CrossOriginOption>,
    disablepictureinpicture: bool,
    disableremoteplayback: bool,
    height: Option<usize>,
    loop_video: bool,
    muted: bool,
    playsinline: bool,
    poster: Option<String>, // URL
    preload: VideoPreloadOption,
    src: Option<String>, // URL
    width: Option<usize>,
}

impl Default for Video {
    fn default() -> Self {
        Video {
            autoplay: false,
            controls: false,
            controlslist: common_attributes::MediaControlList::default(),
            crossorigin: None,
            disablepictureinpicture: false,
            disableremoteplayback: false,
            height: None,
            loop_video: false,
            muted: false,
            playsinline: false,
            poster: None,
            preload: VideoPreloadOption::default(),
            src: None,
            width: None,
        }
    }
}

impl common_attributes::Element for Video {
    fn add_attribute(&mut self, name: String, value: String) {
        match name.as_str() {
            "autoplay" => self.autoplay = true,
            "controls" => self.controls = true,
            "controlslist" => {
                self.controlslist =
                    common_attributes::MediaControlList::derive_media_control(value.as_str())
            }
            "crossorigin" => {
                self.crossorigin = Some(common_attributes::CrossOriginOption::derive_crossorigin(
                    value.as_str(),
                ))
            }
            "disablepictureinpicture" => self.disablepictureinpicture = true,
            "disableremoteplayback" => self.disableremoteplayback = true,
            "height" => {
                self.height = match value.parse() {
                    Ok(v) => Some(v),
                    Err(_) => None,
                }
            }
            "loop" => self.loop_video = true,
            "muted" => self.muted = true,
            "playsinline" => self.playsinline = true,
            "poster" => self.poster = Some(value),
            "preload" => self.preload = VideoPreloadOption::derive_preload(value.as_str()),
            "src" => self.src = Some(value),
            "width" => {
                self.width = match value.parse() {
                    Ok(v) => Some(v),
                    Err(_) => None,
                }
            }
            _ => {}
        }
    }
}
