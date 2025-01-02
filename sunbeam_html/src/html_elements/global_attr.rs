use std::collections::HashMap;

use crate::lang_system::LangTag;

/* Auto Capitalization Options
 *
 * Setting for the auto capitalization option
 */
#[derive(Default)]
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
#[derive(Default)]
pub enum DirOptions {
    #[default]
    Auto,

    Ltr,
    Rtl,
}

#[derive(Default)]
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

#[derive(Default)]
pub enum ContentEditableOptions {
    #[default]
    False,
    True,
    PlainTextOnly,
}

// TODO: Determine if this is too bloated to attach to all HTML values.
//       Maybe instead make them all optional boxes? Most items are smaller
//       than a pointer though, so optional boxes would probably increase the
//       size
// TODO: Relevant strings should be migrated to ids and a lookup engine
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
    hidden: bool,
    id: String,
    inert: bool,
    inputmode: Option<InputModeOptions>,
    is: String, // TODO: Reassess when HTML engine supports custom components
    itemid: String,
    itemprop: String,
    itemref: Vec<String>,
    itemscope: bool,
    itemtype: String,
    lang: Option<LangTag>, // Ideally reference some lookup to our language system
    nonce: Option<String>,
    part: Vec<String>,
    popover: Option<String>,
    // roles: , Figure this one out
    slot: Option<String>,
    spellcheck: bool,
    // style: , Figure out how we're doing styling
    tabindex: Option<u32>,
    title: Option<String>,
    translate: bool,
    writingsuggestions: bool,
    unknown_attributes: Vec<String>,
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
            hidden: false,
            id: "".to_string(),
            inert: false,
            inputmode: None,
            is: "".to_string(),
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
            tabindex: None,
            title: None,
            translate: false,
            writingsuggestions: false,
            unknown_attributes: Vec::new(),
        }
    }
}
