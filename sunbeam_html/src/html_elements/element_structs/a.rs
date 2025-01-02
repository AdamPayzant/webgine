use crate::lang_system::LangTag;

use crate::html_elements::common_attributes;

/* A - Anchor element
 *
 */
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
