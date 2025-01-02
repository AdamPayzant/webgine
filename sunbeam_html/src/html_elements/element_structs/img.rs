use crate::html_elements::common_attributes;

#[derive(Default)]
pub enum DecodingOption {
    Sync,
    Async,
    #[default]
    Auto,
}

#[derive(Default)]
pub enum LoadingOption {
    #[default]
    Eager,
    Lazy,
}

pub struct Img {
    alt: Option<String>,
    crossorigin: common_attributes::CrossOriginOption,
    decoding: DecodingOption,
    elementtiming: Option<String>, // Label
    fetchpriority: common_attributes::FetchPriorityOption,
    height: usize,
    ismap: bool,
    loading: LoadingOption,
    referrerpolicy: common_attributes::ReferrerPolicyOption,
    size: Vec<String>, // TODO: Entries should probably be their own type, will do when implemented
    src: Option<String>, // URL
    srcset: Vec<String>, // TODO: Like size this should probably be it's own type when implemented
    width: usize,
    usemap: Option<String>, // Partial URL
}

impl Default for Img {
    fn default() -> Self {
        Img {
            alt: None,
            crossorigin: common_attributes::CrossOriginOption::default(),
            decoding: DecodingOption::default(),
            elementtiming: None,
            fetchpriority: common_attributes::FetchPriorityOption::default(),
            height: 0,
            ismap: false,
            loading: LoadingOption::default(),
            referrerpolicy: common_attributes::ReferrerPolicyOption::StrictOriginWhenCrossOrigin,
            size: Vec::new(),
            src: None,
            srcset: Vec::new(),
            width: 0,
            usemap: None,
        }
    }
}
