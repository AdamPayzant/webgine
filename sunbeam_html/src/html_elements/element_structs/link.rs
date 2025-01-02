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
