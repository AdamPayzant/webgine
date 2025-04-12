#[derive(Debug, Clone)]
pub struct Rel {
    alternate: bool,
    author: bool,
    bookmark: bool,
    canonical: bool,
    dns_prefetch: bool,
    external: bool,
    expect: bool,
    help: bool,
    icon: bool,
    license: bool,
    manifest: bool,
    me: bool,
    module_preload: bool,
    next: bool,
    no_follow: bool,
    noopener: bool,
    no_referrer: bool,
    opener: bool,
    pingback: bool,
    preconnect: bool,
    prefetch: bool,
    preload: bool,
    prerender: bool,
    prev: bool,
    privacy_policy: bool,
    search: bool,
    style_sheet: bool,
    tag: bool,
    terms_of_service: bool,
}

impl Default for Rel {
    fn default() -> Self {
        Rel {
            alternate: false,
            author: false,
            bookmark: false,
            canonical: false,
            dns_prefetch: false,
            external: false,
            expect: false,
            help: false,
            icon: false,
            license: false,
            manifest: false,
            me: false,
            module_preload: false,
            next: false,
            no_follow: false,
            noopener: false,
            no_referrer: false,
            opener: false,
            pingback: false,
            preconnect: false,
            prefetch: false,
            preload: false,
            prerender: false,
            prev: false,
            privacy_policy: false,
            search: false,
            style_sheet: false,
            tag: false,
            terms_of_service: false,
        }
    }
}

impl Rel {
    pub fn derive_rels(value: &str) -> Rel {
        let mut res = Rel::default();

        value.split(" ").for_each(|s| match s {
            "alternate" => res.alternate = true,
            "author" => res.author = true,
            "bookmark" => res.bookmark = true,
            "canonical" => res.canonical = true,
            "dns-prefetch" => res.dns_prefetch = true,
            "external" => res.external = true,
            "expect" => res.expect = true,
            "help" => res.help = true,
            "icon" => res.icon = true,
            "license" => res.license = true,
            "manifest" => res.manifest = true,
            "me" => res.me = true,
            "modulepreload" => res.module_preload = true,
            "next" => res.next = true,
            "nofollow" => res.no_follow = true,
            "noopener" => res.noopener = true,
            "noreferrer" => res.no_referrer = true,
            "opener" => res.opener = true,
            "pingback" => res.pingback = true,
            "preconnect" => res.preconnect = true,
            "prefetch" => res.prefetch = true,
            "preload" => res.preload = true,
            "prerender" => res.prerender = true,
            "prev" => res.prev = true,
            "privacy-policy" => res.privacy_policy = true,
            "search" => res.search = true,
            "stylesheet" => res.style_sheet = true,
            "tag" => res.tag = true,
            "terms-of-service" => res.terms_of_service = true,
            _ => {}
        });

        res
    }
}

#[derive(Debug, Clone, Default)]
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

impl ReferrerPolicyOption {
    pub fn derive_policy(value: &str) -> ReferrerPolicyOption {
        match value {
            "no-referrer" => ReferrerPolicyOption::NoReferrer,
            "no-referrer-when-downgrade" => ReferrerPolicyOption::NoReferrerWhenDowngrade,
            "origin" => ReferrerPolicyOption::Origin,
            "origin-when-cross-origin" => ReferrerPolicyOption::OriginWhenCrossOrigin,
            "same-origin" => ReferrerPolicyOption::SameOrigin,
            "strict-origin" => ReferrerPolicyOption::StrictOrigin,
            "strict-origin-when-cross-origin" => ReferrerPolicyOption::StrictOriginWhenCrossOrigin,
            "unsafe-url" => ReferrerPolicyOption::UnsafeURL,
            _ => ReferrerPolicyOption::default(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub enum Target {
    #[default]
    TargetSelf,

    Blank,
    Parent,
    Top,
    UnfencedTop,
}

impl Target {
    pub fn derive_target(value: &str) -> Target {
        match value {
            "_self" => Target::TargetSelf,
            "_blank" => Target::Blank,
            "_parent" => Target::Parent,
            "_top" => Target::Top,
            "_unfencedTop" => Target::UnfencedTop,
            _ => Target::default(),
        }
    }
}

#[derive(Debug, Clone)]
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

impl MediaControlList {
    pub fn derive_media_control(value: &str) -> MediaControlList {
        let mut res = MediaControlList::default();
        value.split(" ").for_each(|s| match s {
            "nodownload" => res.nodownloads = true,
            "nofullscreen" => res.nofullscreen = true,
            "noremoteplayback" => res.noremoteplayback = true,
            _ => {}
        });

        res
    }
}

#[derive(Debug, Clone, Default)]
pub enum FetchPriorityOption {
    High,
    Low,
    #[default]
    Auto,
}

impl FetchPriorityOption {
    pub fn derive_priority(value: &str) -> FetchPriorityOption {
        match value {
            "high" => FetchPriorityOption::High,
            "low" => FetchPriorityOption::Low,
            "auto" | _ => FetchPriorityOption::Auto,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub enum CrossOriginOption {
    #[default]
    None,
    Anonymous,
    UseCredentials,
}

impl CrossOriginOption {
    pub fn derive_crossorigin(value: &str) -> CrossOriginOption {
        match value {
            "use-credentials" => CrossOriginOption::UseCredentials,
            "anonymous" | _ => CrossOriginOption::Anonymous,
        }
    }
}

#[derive(Debug, Clone)]
pub enum HttpEquivalent {
    ContentSecurityPolicy,
    ContentType,
    DefaultStyle,
    XUaCompatible,
    Refresh,
}

impl HttpEquivalent {
    pub fn derive_equivalent(value: &str) -> Option<HttpEquivalent> {
        use HttpEquivalent::*;
        match value {
            "content-security-policy" => Some(ContentSecurityPolicy),
            "content-type" => Some(ContentType),
            "default-style" => Some(DefaultStyle),
            "x-ua-compatible" => Some(XUaCompatible),
            "refresh" => Some(Refresh),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum BlockingOption {
    Render,
}

impl BlockingOption {
    pub fn derive_blocking(value: &str) -> Option<BlockingOption> {
        match value {
            "render" => Some(BlockingOption::Render),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub enum AutoComplete {
    #[default]
    Off,
    On,
    List(Vec<String>),
}

impl AutoComplete {
    pub fn derive_type(value: &str) -> AutoComplete {
        match value {
            "on" => AutoComplete::On,
            "off" => AutoComplete::Off,
            _ => AutoComplete::List(value.split(" ").map(|s| s.to_string()).collect()),
        }
    }
}

pub trait Element {
    fn add_attribute(&mut self, name: String, value: String);
}
