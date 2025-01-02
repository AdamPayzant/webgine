use crate::html_elements::common_attributes;

#[derive(Default)]
pub enum IFrameLoadingOption {
    #[default]
    Eager,
    Lazy,
}

pub struct SandboxItems {
    allow_downloads: bool,
    allow_forms: bool,
    allow_modals: bool,
    allow_orientation_lock: bool,
    allow_pointer_lock: bool,
    allow_popups: bool,
    allow_popups_to_escape_sandbox: bool,
    allow_presentation: bool,
    allow_same_origin: bool,
    allow_scripts: bool,
    allow_top_navigation: bool,
    allow_top_navigation_by_user_activation: bool,
    allow_top_navigation_to_custom_protocols: bool,
}

impl Default for SandboxItems {
    fn default() -> Self {
        SandboxItems {
            allow_downloads: false,
            allow_forms: false,
            allow_modals: false,
            allow_orientation_lock: false,
            allow_pointer_lock: false,
            allow_popups: false,
            allow_popups_to_escape_sandbox: false,
            allow_presentation: false,
            allow_same_origin: false,
            allow_scripts: false,
            allow_top_navigation: false,
            allow_top_navigation_by_user_activation: false,
            allow_top_navigation_to_custom_protocols: false,
        }
    }
}

pub struct IFrame {
    allow: Vec<String>, // TODO: This be a enum instead of a string
    height: usize,
    loading: IFrameLoadingOption,
    name: Option<String>,
    referrerpolicy: common_attributes::ReferrerPolicyOption,
    sandbox: Option<SandboxItems>,
    src: Option<String>, // URL or about:blank
    srcdoc: Option<String>,
    width: usize,
}

impl Default for IFrame {
    fn default() -> Self {
        IFrame {
            allow: Vec::new(),
            height: 150,
            loading: IFrameLoadingOption::default(),
            name: None,
            referrerpolicy: common_attributes::ReferrerPolicyOption::StrictOriginWhenCrossOrigin,
            sandbox: None,
            src: None,
            srcdoc: None,
            width: 300,
        }
    }
}
