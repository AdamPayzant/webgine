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

impl SandboxItems {
    pub fn derive_sandbox(value: &str) -> SandboxItems {
        let mut res = SandboxItems::default();
        value.split(" ").for_each(|s| match s {
            "allow-downloads" => res.allow_downloads = true,
            "allow-forms" => res.allow_forms = true,
            "allow-modals" => res.allow_modals = true,
            "allow-orientation-lock" => res.allow_orientation_lock = true,
            "allow-pointer-lock" => res.allow_pointer_lock = true,
            "allow-popups" => res.allow_popups = true,
            "allow-popups-to-escape-sandbox" => res.allow_popups_to_escape_sandbox = true,
            "allow-presentation" => res.allow_presentation = true,
            "allow-same-origin" => res.allow_same_origin = true,
            "allow-scripts" => res.allow_scripts = true,
            "allow-top-navigation" => res.allow_top_navigation = true,
            "allow-top-navigation-by-user-activation" => {
                res.allow_top_navigation_by_user_activation = true
            }
            "allow-top-navigation-to-custom-protocols" => {
                res.allow_top_navigation_to_custom_protocols = true
            }
            _ => {}
        });

        res
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

impl common_attributes::Element for IFrame {
    fn add_attribute(&mut self, name: String, value: String) {
        match name.as_str() {
            "allow" => self.allow = value.split(" ").map(|s| s.to_string()).collect(),
            "height" => match value.parse() {
                Ok(h) => self.height = h,
                Err(_) => {}
            },
            "loading" => {
                self.loading = match value.as_str() {
                    "lazy" => IFrameLoadingOption::Lazy,
                    "eager" | _ => IFrameLoadingOption::Eager,
                }
            }
            "name" => self.name = Some(value),
            "referrerpolicy" => {
                self.referrerpolicy =
                    common_attributes::ReferrerPolicyOption::derive_policy(value.as_str())
            }
            "sandbox" => self.sandbox = Some(SandboxItems::derive_sandbox(value.as_str())),
            "src" => self.src = Some(value),
            "srcdoc" => self.srcdoc = Some(value),
            "width" => match value.parse() {
                Ok(w) => self.width = w,
                Err(_) => {}
            },
            _ => {}
        }
    }
}
