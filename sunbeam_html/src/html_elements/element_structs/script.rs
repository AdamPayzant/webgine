use crate::html_elements::common_attributes;

#[derive(Default)]
pub enum ScriptTypeOption {
    #[default]
    Javascript,
    ImportMap,
    Module,

    Other,
}

pub struct Script {
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
        }
    }
}
