use crate::html_elements::common_attributes;

#[derive(Debug, Clone, Default)]
pub enum DecodingOption {
    Sync,
    Async,
    #[default]
    Auto,
}

#[derive(Debug, Clone, Default)]
pub enum LoadingOption {
    #[default]
    Eager,
    Lazy,
}

#[derive(Debug, Clone)]
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

impl common_attributes::Element for Img {
    fn add_attribute(&mut self, name: String, value: String) {
        match name.as_str() {
            "alt" => self.alt = Some(value),
            "crossorigin" => {
                self.crossorigin =
                    common_attributes::CrossOriginOption::derive_crossorigin(value.as_str())
            }
            "decoding" => {
                self.decoding = match value.as_str() {
                    "sync" => DecodingOption::Sync,
                    "async" => DecodingOption::Async,
                    "auto" | _ => DecodingOption::Auto,
                }
            }
            "elementtiming" => self.elementtiming = Some(value),
            "fetchpriority" => {
                self.fetchpriority =
                    common_attributes::FetchPriorityOption::derive_priority(value.as_str())
            }
            "height" => match value.parse() {
                Ok(h) => self.height = h,
                Err(_) => {}
            },
            "ismap" => self.ismap = true,
            "loading" => {
                self.loading = match value.as_str() {
                    "lazy" => LoadingOption::Lazy,
                    "eager" | _ => LoadingOption::Eager,
                }
            }
            "referrerpolicy" => {
                self.referrerpolicy =
                    common_attributes::ReferrerPolicyOption::derive_policy(value.as_str())
            }
            "sizes" => self.size = value.split(",").map(|s| s.trim().to_string()).collect(),
            "src" => self.src = Some(value),
            "srcset" => self.srcset = value.split(",").map(|s| s.to_string()).collect(),
            "width" => match value.parse() {
                Ok(w) => self.width = w,
                Err(_) => {}
            },
            "usemap" => self.usemap = Some(value),
            _ => {}
        }
    }
}
