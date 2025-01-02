use crate::html_elements::common_attributes;

#[derive(Default)]
pub enum ShapeType {
    #[default]
    Default,

    Rect,
    Circle,
    Poly,
}

/* Area - Image map area element
 *
 */
pub struct Area {
    alt: Option<String>,
    coords: Vec<u32>,
    download: Option<String>,
    href: Option<String>,
    ping: Vec<String>,
    referrerpolicy: common_attributes::ReferrerPolicyOption,
    rel: common_attributes::Rel,
    shape: ShapeType,
    target: common_attributes::Target,
}

impl Default for Area {
    fn default() -> Self {
        Area {
            alt: None,
            coords: Vec::new(),
            download: None,
            href: None,
            ping: Vec::new(),
            referrerpolicy: common_attributes::ReferrerPolicyOption::default(),
            rel: common_attributes::Rel::default(),
            shape: ShapeType::default(),
            target: common_attributes::Target::default(),
        }
    }
}
