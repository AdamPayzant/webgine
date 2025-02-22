use crate::html_elements::common_attributes;

pub enum LinkAsOption {
    Audio,
    Document,
    Embed,
    Fetch,
    Font,
    Image,
    Object,
    Script,
    Style,
    Track,
    Video,
    Worker,
}

impl LinkAsOption {
    pub fn derive_link(value: &str) -> Option<LinkAsOption> {
        use LinkAsOption::*;
        match value {
            "audio" => Some(Audio),
            "document" => Some(Document),
            "embed" => Some(Embed),
            "fetch" => Some(Fetch),
            "font" => Some(Font),
            "image" => Some(Image),
            "object" => Some(Object),
            "script" => Some(Script),
            "style" => Some(Style),
            "track" => Some(Track),
            "video" => Some(Video),
            "worker" => Some(Worker),
            _ => None,
        }
    }
}

pub struct Link {
    link_as: Option<LinkAsOption>,
    blocking: Option<common_attributes::BlockingOption>,
    crossorigin: common_attributes::CrossOriginOption,
    disabled: bool,
    fetchpriority: common_attributes::FetchPriorityOption,
    href: Option<String>,       // URL
    hreflang: Option<String>,   // Lang tag
    imagesizes: Option<String>, // TODO: This will need to be parsed
    imagesrcset: Vec<String>, // TODO: Like imagesizes this should probably be it's own type when implemented
    integrity: Option<String>,
    media: Option<String>, // TODO: Eventually this needs to be a media type
    referrerpolicy: common_attributes::ReferrerPolicyOption,
    rel: common_attributes::Rel,
    sizes: Option<String>, // TODO: This will eventually need to be parsed
    title: Option<String>,
    mimetype: Option<String>, // TODO: Implement mime types
}

impl Default for Link {
    fn default() -> Self {
        Link {
            link_as: None,
            blocking: None,
            crossorigin: common_attributes::CrossOriginOption::default(),
            disabled: false,
            fetchpriority: common_attributes::FetchPriorityOption::default(),
            href: None,
            hreflang: None,
            imagesizes: None,
            imagesrcset: Vec::new(),
            integrity: None,
            media: None,
            referrerpolicy: common_attributes::ReferrerPolicyOption::default(),
            rel: common_attributes::Rel::default(),
            sizes: None,
            title: None,
            mimetype: None,
        }
    }
}

impl common_attributes::Element for Link {
    fn add_attribute(&mut self, name: String, value: String) {
        match name.as_str() {
            "as" => self.link_as = LinkAsOption::derive_link(value.as_str()),
            "blocking" => {
                self.blocking = common_attributes::BlockingOption::derive_blocking(value.as_str())
            }
            "crossorigin" => {
                self.crossorigin =
                    common_attributes::CrossOriginOption::derive_crossorigin(value.as_str());
            }
            "disabled" => self.disabled = true,
            "fetchpriority" => {
                self.fetchpriority =
                    common_attributes::FetchPriorityOption::derive_priority(value.as_str())
            }
            "href" => self.href = Some(value),
            "hreflang" => self.hreflang = Some(value),
            "imagesizes" => self.imagesizes = Some(value),
            "imagesrcset" => self.imagesrcset = value.split(",").map(|s| s.to_string()).collect(),
            "integrity" => self.integrity = Some(value),
            "media" => self.media = Some(value),
            "referrerpolicy" => {
                self.referrerpolicy =
                    common_attributes::ReferrerPolicyOption::derive_policy(value.as_str())
            }
            "rel" => self.rel = common_attributes::Rel::derive_rels(value.as_str()),
            "sizes" => self.sizes = Some(value),
            "title" => self.title = Some(value),
            "type" => self.mimetype = Some(value),
            _ => {}
        }
    }
}
