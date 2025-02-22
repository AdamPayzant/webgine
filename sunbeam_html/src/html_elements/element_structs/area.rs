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

impl common_attributes::Element for Area {
    fn add_attribute(&mut self, name: String, value: String) {
        match name.as_str() {
            "alt" => self.alt = Some(value),
            "coords" => {
                // TODO: This should probably ignore the whole coords
                //       if there's an invalid entry
                self.coords = value
                    .split(",")
                    .filter_map(|s| match s.trim().parse::<u32>() {
                        Ok(v) => Some(v),
                        Err(_) => None,
                    })
                    .collect()
            }
            "download" => self.download = Some(value),
            "href" => self.href = Some(value),
            "ping" => self.ping = value.split(",").map(|s| s.trim().to_string()).collect(),
            "referrerpolicy" => {
                self.referrerpolicy =
                    common_attributes::ReferrerPolicyOption::derive_policy(value.as_str())
            }
            "rel" => self.rel = common_attributes::Rel::derive_rels(value.as_str()),
            "shape" => {
                self.shape = match value.as_str() {
                    "circle" => ShapeType::Circle,
                    "rect" => ShapeType::Rect,
                    "poly" => ShapeType::Poly,
                    _ => ShapeType::Default,
                }
            }
            "target" => self.target = common_attributes::Target::derive_target(value.as_str()),
            _ => {}
        };
    }
}
