use std::collections::HashMap;

use crate::lang_system::LangTag;

/* Auto Capitalization Options
 *
 * Setting for the auto capitalization option
 */
#[derive(Debug, Clone, Default)]
pub enum AutoCapitalizeOptions {
    #[default]
    None,

    Sentences,
    Words,
    Characters,
}

/* Dir Options
 *
 * Options for the Direction global attribute to determine if the
 * Text is read based on the user agent, left to right, or right to left
 */
#[derive(Debug, Clone, Default)]
pub enum DirOptions {
    #[default]
    Auto,

    Ltr,
    Rtl,
}

#[derive(Debug, Clone, Default)]
pub enum InputModeOptions {
    #[default]
    Text,

    None,
    Decimal,
    Numeric,
    Tel,
    Search,
    Email,
    Url,
}

#[derive(Debug, Clone, Default)]
pub enum ContentEditableOptions {
    #[default]
    False,
    True,
    PlainTextOnly,
}

#[derive(Debug, Clone, Default)]
pub enum HiddenState {
    #[default]
    NotHidden,
    Hidden,
    UntilFound,
}

#[derive(Debug, Clone, Default)]
pub enum PopOverState {
    #[default]
    Auto,
    Manual,
}

// TODO: Determine if this is too bloated to attach to all HTML values.
//       Maybe instead make them all optional boxes? Most items are smaller
//       than a pointer though, so optional boxes would probably increase the
//       size
// TODO: Relevant strings should be migrated to ids and a lookup engine
#[derive(Debug, Clone)]
pub struct GlobalAttributes {
    access_key: Option<char>, // This maybe shouldn't be a char
    auto_capitalize: AutoCapitalizeOptions,
    auto_focus: bool,
    class: String,
    contenteditable: ContentEditableOptions,
    // TODO: Validate performance to see if it would ever reasonably exceed a Vector lookup
    data: HashMap<String, String>,
    dir: DirOptions,
    draggable: bool,
    enterkeyhint: String,
    exportparts: Vec<String>, // Need to read up on this one
    hidden: HiddenState,
    id: String,
    inert: bool,
    inputmode: Option<InputModeOptions>,
    is: Option<String>, // TODO: Reassess when HTML engine supports custom components
    itemid: String,
    itemprop: String,
    itemref: Vec<String>,
    itemscope: bool,
    itemtype: String,
    lang: Option<LangTag>, // Ideally reference some lookup to our language system
    nonce: Option<String>,
    part: Vec<String>,
    popover: Option<PopOverState>,
    slot: Option<String>,
    spellcheck: bool,
    style: Vec<String>, // Figure out how we're doing styling
    tabindex: Option<u16>,
    title: Option<String>,
    translate: bool,
    writingsuggestions: bool,
}

impl Default for GlobalAttributes {
    fn default() -> GlobalAttributes {
        GlobalAttributes {
            access_key: None,
            auto_capitalize: AutoCapitalizeOptions::default(),
            auto_focus: false,
            class: "".to_string(),
            contenteditable: ContentEditableOptions::default(),
            data: HashMap::new(),
            dir: DirOptions::default(),
            draggable: false,
            enterkeyhint: "".to_string(),
            exportparts: Vec::new(),
            hidden: HiddenState::NotHidden,
            id: "".to_string(),
            inert: false,
            inputmode: None,
            is: None,
            itemid: "".to_string(),
            itemprop: "".to_string(),
            itemref: Vec::new(),
            itemscope: false,
            itemtype: "".to_string(),
            lang: None,
            nonce: None,
            part: Vec::new(),
            popover: None,
            slot: None,
            spellcheck: false,
            style: Vec::new(),
            tabindex: None,
            title: None,
            translate: false,
            writingsuggestions: false,
        }
    }
}

impl GlobalAttributes {
    // TODO: This needs to be benchmarked against a map data structure.
    pub fn add_attribute(&mut self, name: String, value: String) -> bool {
        match name.as_str() {
            "accesskey" => {
                self.access_key = value.chars().into_iter().next();
                return true;
            }
            "autocapitalize" => {
                self.auto_capitalize = match value.as_str() {
                    "sentences" | "on" => AutoCapitalizeOptions::Sentences,
                    "words" => AutoCapitalizeOptions::Words,
                    "characters" => AutoCapitalizeOptions::Characters,
                    "none" | "off" | _ => AutoCapitalizeOptions::None,
                };
                return true;
            }
            "autofocus" => {
                self.auto_focus = true;
                return true;
            }
            "class" => {
                self.class = value;
                return true;
            }
            "contenteditable" => {
                self.contenteditable = match value.as_str() {
                    "false" => ContentEditableOptions::False,
                    "plaintext-only" => ContentEditableOptions::PlainTextOnly,
                    _ => ContentEditableOptions::True,
                };
                return true;
            }
            "dir" => {
                self.dir = match value.as_str() {
                    "ltr" => DirOptions::Ltr,
                    "rtl" => DirOptions::Rtl,
                    "auto" => DirOptions::Auto,
                    _ => DirOptions::Auto, // TODO: Figure out the best way to reference the parent
                };
                return true;
            }
            "draggable" => {
                self.draggable = match value.as_str() {
                    "true" => true,
                    _ => false,
                };
                return true;
            }
            "enterkeyhint" => {
                self.enterkeyhint = value;
                return true;
            }
            "exportparts" => {
                self.exportparts = value.split(",").map(|s| s.trim().to_string()).collect();
                return true;
            }
            "hidden" => {
                self.hidden = match value.as_str() {
                    "until-found" => HiddenState::UntilFound,
                    _ => HiddenState::Hidden,
                };
                return true;
            }
            "id" => {
                self.id = value;
                return true;
            }
            "inert" => {
                self.inert = true;
                return true;
            }
            "inputmode" => {
                self.inputmode = match value.as_str() {
                    "none" => Some(InputModeOptions::None),
                    "decimal" => Some(InputModeOptions::Decimal),
                    "numeric" => Some(InputModeOptions::Numeric),
                    "tel" => Some(InputModeOptions::Tel),
                    "search" => Some(InputModeOptions::Search),
                    "email" => Some(InputModeOptions::Email),
                    "url" => Some(InputModeOptions::Url),
                    "text" | _ => Some(InputModeOptions::Url),
                };
                return true;
            }
            "is" => {
                self.is = Some(value);
                return true;
            }
            "itemid" => {
                self.itemid = value;
                return true;
            }
            "itemprop" => {
                self.itemprop = value;
                return true;
            }
            "itemref" => {
                self.itemref = value.split(" ").map(|s| s.to_string()).collect();
                return true;
            }
            "itemscope" => {
                self.itemscope = true;
                return true;
            }
            "itemtype" => {
                self.itemtype = value;
                return true;
            }
            "lang" => {
                // TODO: Implement language system
                return true;
            }
            "nonce" => {
                self.nonce = Some(value);
                return true;
            }
            "part" => {
                self.part = value.split("").map(|s| s.to_string()).collect();
                return true;
            }
            "popover" => {
                self.popover = Some(match value.as_str() {
                    "manual" => PopOverState::Manual,
                    _ => PopOverState::Auto,
                });
                return true;
            }
            "slot" => {
                self.slot = Some(value);
                return true;
            }
            "spellcheck" => {
                self.spellcheck = match value.as_str() {
                    "false" => false,
                    _ => true,
                };
                return true;
            }
            "style" => {
                self.style.push(value.clone());
            }
            "tabindex" => {
                // parse as a u32 so we cap to the maximum size
                self.tabindex = Some(match value.parse::<u32>() {
                    Ok(v) => {
                        if v > 32767 {
                            32767
                        } else {
                            v as u16
                        }
                    }
                    Err(_) => return false,
                });
            }
            "title" => {
                self.title = Some(value);
                return true;
            }
            "translate" => {
                self.translate = match value.as_str() {
                    "no" => false,
                    _ => true,
                };
                return true;
            }
            "writingsuggestions" => {
                self.writingsuggestions = match value.as_str() {
                    "false" => false,
                    _ => true,
                };
                return true;
            }
            _ => {}
        };

        if name.starts_with("data-") {
            self.data.insert(name.as_str()[5..].to_string(), value);
            return true;
        }

        return false;
    }

    pub fn get_inline_styling(&self) -> &Vec<String> {
        &self.style
    }

    pub fn get_classes(&self) -> &str {
        &self.class
    }
}
