use crate::html_elements::common_attributes;

#[derive(Clone, Default)]
pub enum ScriptTypeOption {
    #[default]
    Javascript,
    ImportMap,
    Module,

    Other,
}

impl ScriptTypeOption {
    pub fn derive_type(value: &str) -> ScriptTypeOption {
        use ScriptTypeOption::*;
        match value {
            "importmap" => ImportMap,
            "module" => Module,
            "javascript" | "" => Javascript,
            _ => Other,
        }
    }
}

#[derive(Clone)]
pub struct Script {
    // Attributes
    async_script: bool,
    blocking: Option<common_attributes::BlockingOption>,
    crossorigin: common_attributes::CrossOriginOption,
    defer: bool,
    fetchpriority: common_attributes::FetchPriorityOption,
    integrity: Option<String>,
    nomodule: bool,
    nonce: Option<String>,
    referrerpolicy: common_attributes::ReferrerPolicyOption,
    src: Option<String>, // URL
    script_type: ScriptTypeOption,

    // Internal data
    already_started: bool,
}

impl Script {
    pub fn enable_already_started(&mut self) {
        self.already_started = true;
    }
}

impl Default for Script {
    fn default() -> Self {
        Script {
            async_script: false,
            blocking: None,
            crossorigin: common_attributes::CrossOriginOption::default(),
            defer: false,
            fetchpriority: common_attributes::FetchPriorityOption::default(),
            integrity: None,
            nomodule: false,
            nonce: None,
            referrerpolicy: common_attributes::ReferrerPolicyOption::default(),
            src: None,
            script_type: ScriptTypeOption::default(),
            already_started: false,
        }
    }
}

impl common_attributes::Element for Script {
    fn add_attribute(&mut self, name: String, value: String) {
        match name.as_str() {
            "async" => self.async_script = true,
            "blocking" => {
                self.blocking = common_attributes::BlockingOption::derive_blocking(value.as_str())
            }
            "crossorigin" => {
                self.crossorigin =
                    common_attributes::CrossOriginOption::derive_crossorigin(value.as_str())
            }
            "defer" => self.defer = true,
            "fetchpriority" => {
                self.fetchpriority =
                    common_attributes::FetchPriorityOption::derive_priority(value.as_str())
            }
            "integrity" => self.integrity = Some(value),
            "nomodule" => self.nonce = Some(value),
            "nonce" => self.nonce = Some(value),
            "referrerpolicy" => {
                self.referrerpolicy =
                    common_attributes::ReferrerPolicyOption::derive_policy(value.as_str())
            }
            "src" => self.src = Some(value),
            "type" => self.script_type = ScriptTypeOption::derive_type(value.as_str()),
            _ => {}
        }
    }
}
