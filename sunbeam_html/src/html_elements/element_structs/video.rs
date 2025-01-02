use crate::html_elements::common_attributes;

#[derive(Default)]
pub enum VideoPreloadOption {
    None,
    #[default]
    Metadata,
    Auto,
}

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
