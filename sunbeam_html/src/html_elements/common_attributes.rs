#[derive(Default)]
pub enum Rel {
    #[default]
    Void,

    Alternate,
    Author,
    Bookmark,
    Canonical,
    DnsPrefetch,
    External,
    Expect,
    Help,
    Icon,
    License,
    Manifest,
    Me,
    ModulePreload,
    Next,
    NoFollow,
    Noopener,
    NoReferrer,
    Opener,
    Pingback,
    Preconnect,
    Prefetch,
    Preload,
    Prerender,
    Prev,
    PrivacyPolicy,
    Search,
    StyleSheet,
    Tag,
    TermsOfService,
}

#[derive(Default)]
pub enum ReferrerPolicyOption {
    NoReferrer,
    NoReferrerWhenDowngrade,
    Origin,
    OriginWhenCrossOrigin,
    SameOrigin,
    StrictOrigin,
    #[default]
    StrictOriginWhenCrossOrigin,
    UnsafeURL,
}

#[derive(Default)]
pub enum Target {
    #[default]
    TargetSelf,

    Blank,
    Parent,
    Top,
    UnfencedTop,
}

pub struct MediaControlList {
    nodownloads: bool,
    nofullscreen: bool,
    noremoteplayback: bool,
}

impl Default for MediaControlList {
    fn default() -> Self {
        MediaControlList {
            nodownloads: false,
            nofullscreen: false,
            noremoteplayback: false,
        }
    }
}

#[derive(Default)]
pub enum FetchPriorityOption {
    High,
    Low,
    #[default]
    Auto,
}

#[derive(Default)]
pub enum CrossOriginOption {
    #[default]
    None,
    Anonymous,
    UseCredentials,
}

pub struct HttpEquivalent {
    content_security_policy: bool,
    content_type: bool,
    default_style: bool,
    x_ua_compatible: bool,
    refresh: bool,
}

impl Default for HttpEquivalent {
    fn default() -> Self {
        HttpEquivalent {
            content_security_policy: false,
            content_type: false,
            default_style: false,
            x_ua_compatible: false,
            refresh: false,
        }
    }
}

pub enum BlockingOption {
    Render,
}

#[derive(Default)]
pub enum AutoComplete {
    #[default]
    Off,
    On,
    List(Vec<String>),
}
