use crate::lang_system::LangTag;

use crate::html_elements::common_attributes;

/* A - Anchor element
 *
 */
#[derive(Debug, Clone)]
pub struct A {
    download: Option<String>,
    href: Option<String>,
    hreflang: Option<LangTag>,
    ping: Vec<String>,
    referrerpolicy: common_attributes::ReferrerPolicyOption,
    rel: common_attributes::Rel,
    target: common_attributes::Target,
    mimetype: Option<String>,
}

impl Default for A {
    fn default() -> Self {
        A {
            download: None,
            href: None,
            hreflang: None,
            ping: Vec::new(),
            referrerpolicy: common_attributes::ReferrerPolicyOption::default(),
            rel: common_attributes::Rel::default(),
            target: common_attributes::Target::default(),
            mimetype: None,
        }
    }
}

impl common_attributes::Element for A {
    fn add_attribute(&mut self, name: String, value: String) {
        match name.as_str() {
            "download" => self.download = Some(value),
            "href" => self.href = Some(value),
            "hreflang" => {
                // TODO: Figure out lang system
            }
            "ping" => self.ping = value.split(",").map(|s| s.trim().to_string()).collect(),
            "referrerpolicy" => {
                self.referrerpolicy =
                    common_attributes::ReferrerPolicyOption::derive_policy(value.as_str())
            }
            "rel" => self.rel = common_attributes::Rel::derive_rels(value.as_str()),
            "target" => self.target = common_attributes::Target::derive_target(value.as_str()),
            "mimetype" => self.mimetype = Some(value),
            _ => {}
        }
    }
}
