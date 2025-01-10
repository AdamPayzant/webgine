#[derive(Clone)]
struct DoctypeData {
    public_identifier: Option<String>,
    system: Option<String>,
}

impl DoctypeData {
    fn new() -> DoctypeData {
        DoctypeData {
            public_identifier: None,
            system: None,
        }
    }
}

#[derive(Clone)]
enum TokenTag {
    Character,
    Comment,
    StartTag,
    EndTag,
    Doctype(DoctypeData),
    EoF,
}

#[derive(Clone)]
struct HtmlAttribute {
    name: String,
    value: String,
}

impl HtmlAttribute {
    fn new() -> HtmlAttribute {
        HtmlAttribute {
            name: String::new(),
            value: String::new(),
        }
    }
}

#[derive(Clone)]
struct HtmlTokenFlags {
    self_closing: bool,
    force_quirks: bool,
}

impl HtmlTokenFlags {
    fn new() -> HtmlTokenFlags {
        HtmlTokenFlags {
            self_closing: false,
            force_quirks: false,
        }
    }
}

#[derive(Clone)]
pub struct HtmlToken {
    tag: TokenTag,
    data: String,
    attributes: Vec<HtmlAttribute>,
    flags: HtmlTokenFlags,
}

impl HtmlToken {
    fn new(tag: TokenTag) -> HtmlToken {
        HtmlToken {
            tag,
            data: String::new(),
            attributes: Vec::new(),
            flags: HtmlTokenFlags::new(),
        }
    }
    fn data_append(&mut self, c: char) {
        self.data.push(c);
    }

    fn append_to_last_attribute(&mut self, c: char) {
        match self.attributes.last_mut() {
            Some(v) => v.value.push(c),
            None => {}
        };
    }
}

fn new_character_string(c: char) -> HtmlToken {
    HtmlToken {
        tag: TokenTag::Character,
        data: String::from(c),
        attributes: Vec::new(),
        flags: HtmlTokenFlags::new(),
    }
}

#[derive(Clone, Copy)]
enum FsmState {
    Data,
    RCData,
    RawText,
    ScriptData,
    Plaintext,
    TagOpen,
    EndTagOpen,
    TagName,
    RCDataLessThan,
    RCDataEndTagOpen,
    RCDataEndTagName,
    RawTextLessThan,
    RawTextEndTagOpen,
    RawTextEndTagName,
    ScriptDataLessThan,
    ScriptDataEndTagOpen,
    ScriptDataEndTagName,
    ScriptDataEscapeStart,
    ScriptDataEscapeStartDash,
    ScriptDataEscaped,
    ScriptDataEscapedDash,
    ScriptDataEscapedDashDash,
    ScriptDataEscapedLessThan,
    ScriptDataEscapedEndTagOpen,
    ScriptDataEscapedEndTagName,
    ScriptDataDoubleEscapeStart,
    ScriptDataDoubleEscaped,
    ScriptDataDoubleEscapedDash,
    ScriptDataDoubleEscapedDashDash,
    ScriptDataDoubleEscapedLessThan,
    ScriptDataDoubleEscapeEnd,
    BeforeAttributeName,
    AttributeName,
    AfterAttributeName,
    BeforeAttributeValue,
    AttributeValueDoubleQuoted,
    AttributeValueSingleQuoted,
    AttributeValueUnquoted,
    AfterAttributeValueQuoted,
    SelfClosingStartTag,
    BogusComment,
    MarkupDeclarationOpen,
    CommentStart,
    CommentStartDash,
    Comment,
    CommentLessThan,
    CommentLessThanBang,
    CommentLessThanBangDash,
    CommentLessThanBangDashDash,
    CommentEndDash,
    CommentEnd,
    CommentEndBang,
    Doctype,
    BeforeDoctypeName,
    DoctypeName,
    AfterDoctype,
    AfterDoctypeName,
    AfterDoctypePublicKeyword,
    BeforeDoctypePublicIndentifier,
    DoctypePublicIdentifier,
    DoctypePublicIdentifierDoubleQuote,
    DoctypePublicIdentifierSingleQuote,
    AfterDoctypePublicIdentifier,
    BetweenDoctypePublicAndSystemIdentifiers,
    AfterDoctypeSystemKeyword,
    BeforeDoctypeSystemIdentifier,
    DoctypeSystemIdentifierDoubleQuote,
    DoctypeSystemIdentifierSingleQuote,
    AfterDoctypeSystemIdentifier,
    BogusDoctype,
    CDataSection,
    CDataSectionBracket,
    CDataSectionEnd,
    CharacterReference,
    NamedCharacterReference,
    AmbiguousAmpersand,
    NumericCharacterReference,
    HexadecimalCharacterReferenceStart,
    DecimalCharacterReferenceStart,
    HexadecimalCharacterReference,
    DecimalCharacterReference,
    NumericCharacterReferenceEnd,
}

// TODO: There's probably a better way of comparing case insensitive arrays of characters,
//       but this will do for now
fn compare_slices(a1: &[char], a2: &[char], case_sensitive: bool) -> bool {
    if a1.len() != a2.len() {
        return false;
    }

    let mut idx = 0;
    while idx < a1.len() {
        if case_sensitive {
            if a1[idx] != a2[idx] {
                return false;
            }
        } else {
            if a1[idx].to_lowercase().next().unwrap() != a2[idx].to_lowercase().next().unwrap() {
                return false;
            }
        }

        idx += 1;
    }

    true
}

fn is_part_of_attribute(state: FsmState) -> bool {
    use FsmState::*;
    match state {
        AttributeValueDoubleQuoted | AttributeValueSingleQuoted | AttributeValueUnquoted => true,
        _ => false,
    }
}

fn lookup_character_reference(code: u32) -> u32 {
    match code {
        0x80 => 0x20AC,
        0x82 => 0x201A,
        0x83 => 0x0192,
        0x84 => 0x201E,
        0x85 => 0x2026,
        0x86 => 0x2020,
        0x87 => 0x2021,
        0x88 => 0x02C6,
        0x89 => 0x2030,
        0x8A => 0x0160,
        0x8B => 0x2039,
        0x8C => 0x0152,
        0x8E => 0x017D,
        0x91 => 0x2018,
        0x92 => 0x2019,
        0x93 => 0x201C,
        0x94 => 0x201D,
        0x95 => 0x2022,
        0x96 => 0x2013,
        0x97 => 0x2014,
        0x98 => 0x02DC,
        0x99 => 0x2122,
        0x9A => 0x0161,
        0x9B => 0x203A,
        0x9C => 0x0153,
        0x9E => 0x017E,
        0x9F => 0x0178,
        _ => code,
    }
}

// chars[idx].to_lowercase().next().unwrap()
// '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}'

// Ways HTML tokenizer spec is knowingly broken:
//  - In the case of duplicate attributes, does not occur and instead dups will be ignored by the parser
//  - We currently do not have a parser pause flag and thus cannot check it between steps
//  - NamedCharacterReference is unimplimented
//
// Any other breaks in the HTML tokenizer must be fixed as found
pub fn tokenize_string(html_string: &str) -> Vec<HtmlToken> {
    let mut tokens = Vec::new();

    let mut idx = 0;
    let chars: Vec<char> = html_string.chars().collect();
    let mut state = FsmState::Data;
    let mut open_tag_stack: Vec<String> = Vec::new();
    let mut return_state = state;

    let mut cur_token = HtmlToken::new(TokenTag::EoF);
    let mut temp_buffer = String::new();

    let mut character_reference_code = 0;

    while idx < chars.len() {
        // continue is used to reconsume or not consume
        use FsmState::*;

        match state {
            Data => match chars[idx] {
                '&' => {
                    return_state = Data;
                    state = CharacterReference;
                }
                '<' => state = TagOpen,
                _ => tokens.push(new_character_string(chars[idx])),
            },
            RCData => match chars[idx] {
                '&' => {
                    return_state = RCData;
                    state = CharacterReference;
                }
                '<' => state = RCDataLessThan,
                '\u{0000}' => tokens.push(new_character_string('\u{FFFD}')),
                _ => tokens.push(new_character_string(chars[idx])),
            },
            RawText => match chars[idx] {
                '<' => state = RawTextLessThan,
                '\u{0000}' => tokens.push(new_character_string('\u{FFFD}')),
                _ => tokens.push(new_character_string(chars[idx])),
            },
            ScriptData => match chars[idx] {
                '<' => state = ScriptDataLessThan,
                '\u{0000}' => tokens.push(new_character_string('\u{FFFD}')),
                _ => tokens.push(new_character_string(chars[idx])),
            },
            Plaintext => match chars[idx] {
                '\u{0000}' => tokens.push(new_character_string('\u{FFFD}')),
                _ => tokens.push(new_character_string(chars[idx])),
            },
            TagOpen => match chars[idx] {
                '!' => state = MarkupDeclarationOpen,
                '/' => state = EndTagOpen,
                '?' => {
                    cur_token = HtmlToken::new(TokenTag::Comment);
                    state = BogusComment;
                    continue;
                }
                _ => {
                    if chars[idx].is_alphabetic() {
                        cur_token = HtmlToken::new(TokenTag::StartTag);
                        state = TagName;
                        continue;
                    } else {
                        tokens.push(new_character_string('<'));
                        state = Data;
                        continue;
                    }
                }
            },
            EndTagOpen => match chars[idx] {
                '>' => state = Data,
                _ => {
                    if chars[idx].is_alphabetic() {
                        cur_token = HtmlToken::new(TokenTag::EndTag);
                        state = TagName;
                        continue;
                    } else {
                        cur_token = HtmlToken::new(TokenTag::Comment);
                        state = BogusComment;
                        continue;
                    }
                }
            },
            TagName => match chars[idx] {
                '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' => state = BeforeAttributeName,
                '/' => state = SelfClosingStartTag,
                '>' => {
                    state = Data;
                    tokens.push(cur_token.clone());

                    if !matches!(cur_token.tag, TokenTag::StartTag) {
                        open_tag_stack.push(cur_token.data.clone());
                    }
                }
                '\u{0000}' => cur_token.data_append('\u{FFFD}'),
                _ => cur_token.data_append(chars[idx].to_lowercase().next().unwrap()),
            },
            RCDataLessThan => match chars[idx] {
                '/' => {
                    temp_buffer = String::new();
                    state = RCDataEndTagOpen;
                }
                _ => {
                    tokens.push(new_character_string('<'));
                    state = RCData;
                    continue;
                }
            },
            RCDataEndTagOpen => {
                if chars[idx].is_alphabetic() {
                    cur_token = HtmlToken::new(TokenTag::EndTag);
                    state = RCDataEndTagName;
                    continue;
                } else {
                    tokens.push(new_character_string('<'));
                    tokens.push(new_character_string('/'));
                    state = RCData;
                    continue;
                }
            }
            RCDataEndTagName => {
                let mut acted = false;
                match open_tag_stack.last() {
                    Some(val) => {
                        if *val == cur_token.data {
                            match chars[idx] {
                                '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' => {
                                    state = BeforeAttributeName;
                                    acted = true;
                                }
                                '/' => {
                                    state = SelfClosingStartTag;
                                    acted = true;
                                }
                                '>' => {
                                    state = Data;
                                    acted = true;
                                }
                                _ => {}
                            }
                            if acted {
                                open_tag_stack.pop();
                            }
                        }
                    }
                    None => {}
                }
                if chars[idx].is_alphabetic() {
                    cur_token.data_append(chars[idx].to_lowercase().next().unwrap());
                    temp_buffer.push(chars[idx]);
                    acted = true;
                }

                if !acted {
                    tokens.push(new_character_string('<'));
                    tokens.push(new_character_string('/'));
                    temp_buffer
                        .chars()
                        .for_each(|c| tokens.push(new_character_string(c)));
                    state = RCData;
                    continue;
                }
            }
            RawTextLessThan => match chars[idx] {
                '/' => {
                    temp_buffer = String::new();
                    state = RawTextEndTagOpen;
                }
                _ => {
                    tokens.push(new_character_string('<'));
                    state = RawText;
                    continue;
                }
            },
            RawTextEndTagOpen => {
                if chars[idx].is_alphabetic() {
                    cur_token = HtmlToken::new(TokenTag::EndTag);
                    state = RawTextEndTagName;
                    continue;
                } else {
                    tokens.push(new_character_string('<'));
                    tokens.push(new_character_string('/'));
                    state = RawText;
                    continue;
                }
            }
            RawTextEndTagName => {
                let mut acted = false;
                match open_tag_stack.last() {
                    Some(val) => {
                        if *val == cur_token.data {
                            match chars[idx] {
                                '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' => {
                                    state = BeforeAttributeName;
                                    acted = true;
                                }
                                '/' => {
                                    state = SelfClosingStartTag;
                                    acted = true;
                                }
                                '>' => {
                                    state = Data;
                                    acted = true;
                                }
                                _ => {}
                            }
                            if acted {
                                open_tag_stack.pop();
                            }
                        }
                    }
                    None => {}
                }
                if chars[idx].is_alphabetic() {
                    cur_token.data_append(chars[idx].to_lowercase().next().unwrap());
                    temp_buffer.push(chars[idx]);
                    acted = true;
                }

                if !acted {
                    tokens.push(new_character_string('<'));
                    tokens.push(new_character_string('/'));
                    temp_buffer
                        .chars()
                        .for_each(|c| tokens.push(new_character_string(c)));
                    state = RawText;
                    continue;
                }
            }
            ScriptDataLessThan => match chars[idx] {
                '/' => {
                    temp_buffer = String::new();
                    state = ScriptDataEndTagOpen;
                }
                '!' => {
                    state = ScriptDataEscapeStart;
                    tokens.push(new_character_string('<'));
                    tokens.push(new_character_string('!'));
                }
                _ => {
                    tokens.push(new_character_string('<'));
                    state = ScriptData;
                    continue;
                }
            },
            ScriptDataEndTagOpen => {
                if chars[idx].is_alphabetic() {
                    cur_token = HtmlToken::new(TokenTag::EndTag);
                    state = ScriptDataEndTagName;
                    continue;
                } else {
                    tokens.push(new_character_string('<'));
                    tokens.push(new_character_string('/'));
                    state = ScriptData;
                    continue;
                }
            }
            ScriptDataEndTagName => {
                let mut acted = false;
                match open_tag_stack.last() {
                    Some(val) => {
                        if *val == cur_token.data {
                            match chars[idx] {
                                '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' => {
                                    state = BeforeAttributeName;
                                    acted = true;
                                }
                                '/' => {
                                    state = SelfClosingStartTag;
                                    acted = true;
                                }
                                '>' => {
                                    state = Data;
                                    acted = true;
                                }
                                _ => {}
                            }
                            if acted {
                                open_tag_stack.pop();
                            }
                        }
                    }
                    None => {}
                }
                if chars[idx].is_alphabetic() {
                    cur_token.data_append(chars[idx].to_lowercase().next().unwrap());
                    temp_buffer.push(chars[idx]);
                    acted = true;
                }

                if !acted {
                    tokens.push(new_character_string('<'));
                    tokens.push(new_character_string('/'));
                    temp_buffer
                        .chars()
                        .for_each(|c| tokens.push(new_character_string(c)));
                    state = ScriptData;
                    continue;
                }
            }
            ScriptDataEscapeStart => match chars[idx] {
                '-' => {
                    state = ScriptDataEscapedDash;
                    tokens.push(new_character_string('-'));
                }
                _ => {
                    state = ScriptData;
                    continue;
                }
            },
            ScriptDataEscaped => match chars[idx] {
                '-' => {
                    state = ScriptDataEscapedDash;
                    tokens.push(new_character_string('-'));
                }
                '<' => state = ScriptDataEscapedLessThan,
                '\u{0000}' => tokens.push(new_character_string('\u{FFFD}')),
                _ => tokens.push(new_character_string(chars[idx])),
            },
            ScriptDataEscapedDash => match chars[idx] {
                '-' => {
                    state = ScriptDataEscapedDashDash;
                    tokens.push(new_character_string('-'));
                }
                '<' => state = ScriptDataEscapedLessThan,
                '\u{0000}' => {
                    state = ScriptDataEscaped;
                    tokens.push(new_character_string('\u{FFFD}'));
                }
                _ => {
                    state = ScriptDataEscaped;
                    tokens.push(new_character_string(chars[idx]));
                }
            },
            ScriptDataEscapedDashDash => match chars[idx] {
                '-' => tokens.push(new_character_string('-')),
                '<' => state = ScriptDataEscapedLessThan,
                '>' => {
                    state = ScriptData;
                    tokens.push(new_character_string('>'));
                }
                '\u{0000}' => {
                    state = ScriptDataEscaped;
                    tokens.push(new_character_string('\u{FFFD}'));
                }
                _ => {
                    state = ScriptDataEscaped;
                    tokens.push(new_character_string(chars[idx]));
                }
            },
            ScriptDataEscapedLessThan => match chars[idx] {
                '/' => {
                    temp_buffer = String::new();
                    state = ScriptDataEscapedEndTagOpen;
                }
                _ => {
                    if chars[idx].is_alphabetic() {
                        temp_buffer = String::new();
                        tokens.push(new_character_string('<'));
                        state = ScriptDataDoubleEscapeStart;
                        continue;
                    } else {
                        tokens.push(new_character_string('<'));
                        state = ScriptDataEscaped;
                        continue;
                    }
                }
            },
            ScriptDataEscapedEndTagOpen => {
                if chars[idx].is_alphabetic() {
                    cur_token = HtmlToken::new(TokenTag::EndTag);
                    state = ScriptDataEscapedEndTagName;
                    continue;
                } else {
                    tokens.push(new_character_string('<'));
                    tokens.push(new_character_string('/'));
                    state = ScriptDataEscaped;
                    continue;
                }
            }
            ScriptDataEscapedEndTagName => {
                let mut acted = false;
                match open_tag_stack.last() {
                    Some(val) => {
                        if *val == cur_token.data {
                            match chars[idx] {
                                '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' => {
                                    state = BeforeAttributeName;
                                    acted = true;
                                }
                                '/' => {
                                    state = SelfClosingStartTag;
                                    acted = true;
                                }
                                '>' => {
                                    state = Data;
                                    acted = true;
                                }
                                _ => {}
                            }
                            if acted {
                                open_tag_stack.pop();
                            }
                        }
                    }
                    None => {}
                }
                if chars[idx].is_alphabetic() {
                    cur_token.data_append(chars[idx].to_lowercase().next().unwrap());
                    temp_buffer.push(chars[idx]);
                    acted = true;
                }

                if !acted {
                    tokens.push(new_character_string('<'));
                    tokens.push(new_character_string('/'));
                    temp_buffer
                        .chars()
                        .for_each(|c| tokens.push(new_character_string(c)));
                    state = ScriptDataEscaped;
                    continue;
                }
            }
            ScriptDataDoubleEscapeStart => match chars[idx] {
                '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' | '/' | '>' => {
                    if temp_buffer == "script" {
                        state = ScriptDataDoubleEscaped;
                    } else {
                        state = ScriptDataEscaped;
                    }
                    tokens.push(new_character_string(chars[idx]));
                }
                _ => {
                    if chars[idx].is_alphabetic() {
                        temp_buffer.push(chars[idx].to_lowercase().next().unwrap());
                        tokens.push(new_character_string(chars[idx]));
                    } else {
                        state = ScriptDataEscaped;
                        continue;
                    }
                }
            },
            ScriptDataDoubleEscaped => match chars[idx] {
                '-' => {
                    state = ScriptDataDoubleEscapedDash;
                    tokens.push(new_character_string('-'));
                }
                '<' => {
                    state = ScriptDataDoubleEscapedLessThan;
                    tokens.push(new_character_string('<'));
                }
                '\u{0000}' => tokens.push(new_character_string('\u{FFFD}')),
                _ => tokens.push(new_character_string(chars[idx])),
            },
            ScriptDataDoubleEscapedDash => match chars[idx] {
                '-' => {
                    state = ScriptDataDoubleEscapedDashDash;
                    tokens.push(new_character_string('-'));
                }
                '<' => {
                    state = ScriptDataDoubleEscapedLessThan;
                    tokens.push(new_character_string('<'));
                }
                '\u{0000}' => {
                    state = ScriptDataDoubleEscaped;
                    tokens.push(new_character_string('\u{FFFD}'));
                }
                _ => {
                    state = ScriptDataDoubleEscaped;
                    tokens.push(new_character_string(chars[idx]));
                }
            },
            ScriptDataDoubleEscapedDashDash => match chars[idx] {
                '-' => tokens.push(new_character_string('-')),
                '<' => {
                    state = ScriptDataDoubleEscapedLessThan;
                    tokens.push(new_character_string('<'));
                }
                '>' => {
                    state = ScriptData;
                    tokens.push(new_character_string('>'));
                }
                '\u{0000}' => {
                    state = ScriptDataDoubleEscaped;
                    tokens.push(new_character_string('\u{FFFD}'));
                }
                _ => {
                    state = ScriptDataDoubleEscaped;
                    tokens.push(new_character_string(chars[idx]));
                }
            },
            ScriptDataDoubleEscapedLessThan => match chars[idx] {
                '/' => {
                    temp_buffer = String::new();
                    state = ScriptDataDoubleEscapeEnd;
                    tokens.push(new_character_string('/'));
                }
                _ => {
                    state = ScriptDataDoubleEscaped;
                    continue;
                }
            },
            ScriptDataDoubleEscapeEnd => match chars[idx] {
                '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' | '/' | '>' => {
                    if temp_buffer == "script" {
                        state = ScriptDataEscaped;
                    } else {
                        state = ScriptDataDoubleEscaped;
                    }
                    tokens.push(new_character_string(chars[idx]));
                }
                _ => {
                    if chars[idx].is_alphabetic() {
                        temp_buffer.push(chars[idx].to_lowercase().next().unwrap());
                        tokens.push(new_character_string(chars[idx]));
                    } else {
                        state = ScriptDataDoubleEscaped;
                        continue;
                    }
                }
            },
            BeforeAttributeName => match chars[idx] {
                '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' => {}
                '/' | '>' => {
                    state = AfterAttributeName;
                    continue;
                }
                '=' => {
                    cur_token.attributes.push(HtmlAttribute::new());
                    cur_token
                        .attributes
                        .last_mut()
                        .unwrap()
                        .name
                        .push(chars[idx]);
                    state = AttributeName;
                }
                _ => {
                    cur_token.attributes.push(HtmlAttribute::new());
                    state = AttributeName;
                    continue;
                }
            },
            AttributeName => match chars[idx] {
                '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' | '/' | '>' => {
                    state = AfterAttributeName;
                    continue;
                }
                '=' => state = BeforeAttributeValue,
                '\u{0000}' => {
                    cur_token
                        .attributes
                        .last_mut()
                        .unwrap()
                        .name
                        .push('\u{FFFD}');
                }
                // '"' | '\u{0027}' | '<' => {} // Error but treated as the anything else category
                _ => {
                    cur_token
                        .attributes
                        .last_mut()
                        .unwrap()
                        .name
                        .push(chars[idx].to_lowercase().next().unwrap());
                }
            },
            AfterAttributeName => match chars[idx] {
                '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' => {} // Ignore
                '/' => state = SelfClosingStartTag,
                '=' => state = BeforeAttributeValue,
                '>' => {
                    state = Data;
                    tokens.push(cur_token.clone());

                    if matches!(cur_token.tag, TokenTag::StartTag) {
                        open_tag_stack.push(cur_token.data.clone());
                    }
                }
                _ => {
                    cur_token.attributes.push(HtmlAttribute::new());
                    state = AttributeName;
                    continue;
                }
            },
            BeforeAttributeValue => match chars[idx] {
                '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' => {} // Ignore
                '"' => state = AttributeValueDoubleQuoted,
                '\u{0027}' => state = AttributeValueSingleQuoted,
                '>' => {
                    state = Data;
                    tokens.push(cur_token.clone());

                    if matches!(cur_token.tag, TokenTag::StartTag) {
                        open_tag_stack.push(cur_token.data.clone());
                    }
                }
                _ => {
                    state = AttributeValueUnquoted;
                    continue;
                }
            },
            AttributeValueDoubleQuoted => match chars[idx] {
                '"' => state = AfterAttributeValueQuoted,
                '&' => {
                    return_state = AttributeValueDoubleQuoted;
                    state = CharacterReference;
                }
                '\u{0000}' => {
                    cur_token
                        .attributes
                        .last_mut()
                        .unwrap()
                        .value
                        .push('\u{FFFD}');
                }
                _ => {
                    cur_token
                        .attributes
                        .last_mut()
                        .unwrap()
                        .value
                        .push(chars[idx].to_lowercase().next().unwrap());
                }
            },
            AttributeValueSingleQuoted => match chars[idx] {
                '\u{0027}' => state = AfterAttributeValueQuoted,
                '&' => {
                    return_state = AttributeValueDoubleQuoted;
                    state = CharacterReference;
                }
                '\u{0000}' => {
                    cur_token
                        .attributes
                        .last_mut()
                        .unwrap()
                        .value
                        .push('\u{FFFD}');
                }
                _ => {
                    cur_token
                        .attributes
                        .last_mut()
                        .unwrap()
                        .value
                        .push(chars[idx].to_lowercase().next().unwrap());
                }
            },
            AttributeValueUnquoted => match chars[idx] {
                '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' => state = BeforeAttributeName,
                '&' => {
                    return_state = AttributeValueUnquoted;
                    state = CharacterReference;
                }
                '>' => {
                    state = Data;
                    tokens.push(cur_token.clone());

                    if matches!(cur_token.tag, TokenTag::StartTag) {
                        open_tag_stack.push(cur_token.data.clone());
                    }
                }
                '\u{0000}' => {
                    cur_token
                        .attributes
                        .last_mut()
                        .unwrap()
                        .value
                        .push('\u{FFFD}');
                }
                // '"' | '\u{0027}' | '<' | '=' | '`' => {} // Error, treat as anything else
                _ => {
                    cur_token
                        .attributes
                        .last_mut()
                        .unwrap()
                        .value
                        .push(chars[idx].to_lowercase().next().unwrap());
                }
            },
            AfterAttributeValueQuoted => match chars[idx] {
                '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' => state = BeforeAttributeName,
                '/' => state = SelfClosingStartTag,
                '>' => {
                    state = Data;
                    tokens.push(cur_token.clone());

                    if matches!(cur_token.tag, TokenTag::StartTag) {
                        open_tag_stack.push(cur_token.data.clone());
                    }
                }
                _ => {
                    state = BeforeAttributeName;
                    continue;
                }
            },
            SelfClosingStartTag => match chars[idx] {
                '>' => {
                    cur_token.flags.self_closing = true;
                    state = Data;

                    tokens.push(cur_token.clone());
                }
                _ => {
                    state = BeforeAttributeName;
                    continue;
                }
            },
            BogusComment => match chars[idx] {
                '>' => {
                    state = Data;
                    tokens.push(cur_token.clone());
                }
                '\u{0000}' => {
                    cur_token.data.push('\u{FFFD}');
                }
                _ => {
                    cur_token.data.push(chars[idx]);
                }
            },
            MarkupDeclarationOpen => {
                if idx + 1 < chars.len() && compare_slices(&chars[idx..idx + 2], &['-', '-'], true)
                {
                    idx += 2;
                    cur_token = HtmlToken::new(TokenTag::Comment);
                    state = CommentStart;

                    continue;
                }
                if idx + 7 < chars.len()
                    && compare_slices(
                        &chars[idx..idx + 8],
                        &['d', 'o', 'c', 't', 'y', 'p', 'e'],
                        false,
                    )
                {
                    idx += 8;
                    state = Doctype;
                    continue;
                }
                if idx + 7 < chars.len()
                    && compare_slices(
                        &chars[idx..idx + 8],
                        &['[', 'C', 'D', 'A', 'T', 'a', '['],
                        true,
                    )
                {
                    idx += 8;
                    // TODO: Check if there is an adjusted current node
                    //       and it's not an element in HTML namespace.
                    //       If not, create a comment instead
                    state = CDataSection;
                    continue;
                }

                cur_token = HtmlToken::new(TokenTag::Comment);
                state = BogusComment;
                continue;
            }
            CommentStart => match chars[idx] {
                '-' => state = CommentStartDash,
                '>' => {
                    state = Data;
                    tokens.push(cur_token.clone());
                }
                _ => {
                    state = Comment;
                    continue;
                }
            },
            CommentStartDash => match chars[idx] {
                '-' => state = CommentEnd,
                '>' => {
                    state = Data;
                    tokens.push(cur_token.clone());
                }
                _ => {
                    cur_token.data_append('-');
                    state = Comment;
                    continue;
                }
            },
            Comment => match chars[idx] {
                '<' => {
                    cur_token.data_append('<');
                    state = CommentLessThan;
                }
                '-' => state = CommentEndDash,
                '\u{0000}' => cur_token.data_append('\u{FFFD}'),
                _ => cur_token.data_append(chars[idx]),
            },
            CommentLessThan => match chars[idx] {
                '!' => {
                    cur_token.data_append('!');
                    state = CommentLessThanBang;
                }
                '<' => cur_token.data_append('<'),
                _ => {
                    state = Comment;
                    continue;
                }
            },
            CommentLessThanBang => match chars[idx] {
                '-' => state = CommentLessThanBangDash,
                _ => {
                    state = Comment;
                    continue;
                }
            },
            CommentLessThanBangDash => match chars[idx] {
                '-' => state = CommentLessThanBangDashDash,
                _ => {
                    state = Comment;
                    continue;
                }
            },
            CommentLessThanBangDashDash => {
                state = CommentEnd;
                continue;
            }
            CommentEndDash => match chars[idx] {
                '-' => state = CommentEnd,
                _ => {
                    cur_token.data_append('-');
                    state = Comment;
                    continue;
                }
            },
            CommentEnd => match chars[idx] {
                '>' => {
                    state = Data;
                    tokens.push(cur_token.clone());
                }
                '!' => state = CommentEndBang,
                '-' => cur_token.data_append('-'),
                _ => {
                    cur_token.data_append('-');
                    state = Comment;
                    continue;
                }
            },
            CommentEndBang => match chars[idx] {
                '-' => {
                    cur_token.data_append('-');
                    cur_token.data_append('!');
                    state = CommentEndDash;
                }
                '>' => {
                    state = Data;
                    tokens.push(cur_token.clone())
                }
                _ => {
                    cur_token.data_append('-');
                    cur_token.data_append('!');
                    state = Comment;
                    continue;
                }
            },
            Doctype => match chars[idx] {
                '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' => state = BeforeDoctypeName,
                _ => {
                    state = BeforeDoctypeName;
                    continue;
                }
            },
            BeforeDoctypeName => match chars[idx] {
                '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' => {}
                '\u{0000}' => {
                    cur_token = HtmlToken::new(TokenTag::Doctype(DoctypeData::new()));
                    cur_token.data_append('\u{FFFD}');
                    state = DoctypeName;
                }
                '>' => {
                    cur_token = HtmlToken::new(TokenTag::Doctype(DoctypeData::new()));
                    cur_token.flags.force_quirks = true;
                    state = Data;
                    tokens.push(cur_token.clone());
                }
                _ => {
                    cur_token = HtmlToken::new(TokenTag::Doctype(DoctypeData::new()));
                    cur_token.data_append(chars[idx].to_lowercase().next().unwrap());
                    state = DoctypeName;
                }
            },
            DoctypeName => match chars[idx] {
                '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' => state = AfterDoctype,
                '>' => {
                    state = Data;
                    tokens.push(cur_token.clone());
                }
                '\u{0000}' => cur_token.data_append('\u{FFFD}'),
                _ => cur_token.data_append(chars[idx].to_lowercase().next().unwrap()),
            },
            AfterDoctypeName => match chars[idx] {
                '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' => {}
                '>' => {
                    state = Data;
                    tokens.push(cur_token.clone());
                }
                _ => {
                    if idx + 6 < chars.len()
                        && compare_slices(
                            &chars[idx..idx + 7],
                            &['p', 'u', 'b', 'l', 'i', 'c'],
                            false,
                        )
                    {
                        idx += 7;
                        state = AfterDoctypePublicKeyword;
                        continue;
                    }
                    if idx + 6 < chars.len()
                        && compare_slices(
                            &chars[idx..idx + 7],
                            &['s', 'y', 's', 't', 'e', 'm'],
                            false,
                        )
                    {
                        idx += 7;
                        continue;
                    }

                    cur_token.flags.force_quirks = true;
                    state = BogusComment;
                    continue;
                }
            },
            AfterDoctypePublicKeyword => match chars[idx] {
                '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' => {
                    state = BeforeDoctypePublicIndentifier;
                }
                '"' => match cur_token.tag {
                    TokenTag::Doctype(ref mut val) => {
                        val.public_identifier = Some(String::new());
                        state = DoctypePublicIdentifierDoubleQuote;
                    }
                    _ => {
                        // TODO: This is fine as a panic for now,
                        //       but we need better error handling
                        panic!("HTML Tokenizer ")
                    }
                },
                '\u{0027}' => match cur_token.tag {
                    TokenTag::Doctype(ref mut val) => {
                        val.public_identifier = Some(String::new());
                        state = DoctypePublicIdentifierSingleQuote;
                    }
                    _ => {
                        // TODO: This is fine as a panic for now,
                        //       but we need better error handling
                        panic!("HTML Tokenizer ")
                    }
                },
                '>' => {
                    cur_token.flags.force_quirks = true;
                    state = Data;
                    tokens.push(cur_token.clone());
                }
                _ => {
                    cur_token.flags.force_quirks = true;
                    state = BogusDoctype;
                    continue;
                }
            },
            BeforeDoctypePublicIndentifier => match chars[idx] {
                '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' => {} // Ignore
                '"' => match cur_token.tag {
                    TokenTag::Doctype(ref mut val) => {
                        val.public_identifier = Some(String::new());
                        state = DoctypePublicIdentifierDoubleQuote;
                    }
                    _ => {
                        // TODO: This is fine as a panic for now,
                        //       but we need better error handling
                        panic!("HTML Tokenizer ")
                    }
                },
                '\u{0027}' => match cur_token.tag {
                    TokenTag::Doctype(ref mut val) => {
                        val.public_identifier = Some(String::new());
                        state = DoctypePublicIdentifierSingleQuote;
                    }
                    _ => {
                        // TODO: This is fine as a panic for now,
                        //       but we need better error handling
                        panic!("HTML Tokenizer ")
                    }
                },
                '>' => {
                    cur_token.flags.force_quirks = true;
                    state = Data;
                    tokens.push(cur_token.clone())
                }
                _ => {
                    cur_token.flags.force_quirks = true;
                    state = BogusDoctype;
                    continue;
                }
            },
            DoctypePublicIdentifierDoubleQuote => match chars[idx] {
                '"' => state = AfterDoctypePublicIdentifier,
                '\u{0000}' => match cur_token.tag {
                    TokenTag::Doctype(ref mut val) => {
                        match val.public_identifier {
                            Some(ref mut s) => {
                                s.push('\u{FFFD}');
                            }
                            None => {
                                // TODO: Correct with error handling
                                panic!("In DoctypePublicIdentifierDoubleQuote without public identifier created");
                            }
                        }
                        state = DoctypePublicIdentifierDoubleQuote;
                    }
                    _ => {
                        // TODO: This is fine as a panic for now,
                        //       but we need better error handling
                        panic!("HTML Tokenizer ")
                    }
                },
                '>' => {
                    cur_token.flags.force_quirks = true;
                    state = Data;
                    tokens.push(cur_token.clone());
                }
                _ => match cur_token.tag {
                    TokenTag::Doctype(ref mut val) => {
                        match val.public_identifier {
                            Some(ref mut s) => {
                                s.push(chars[idx]);
                            }
                            None => {
                                // TODO: Correct with error handling
                                panic!("In DoctypePublicIdentifierDoubleQuote without public identifier created");
                            }
                        }
                        state = DoctypePublicIdentifierDoubleQuote;
                    }
                    _ => {
                        // TODO: This is fine as a panic for now,
                        //       but we need better error handling
                        panic!("HTML Tokenizer ")
                    }
                },
            },
            DoctypePublicIdentifierSingleQuote => match chars[idx] {
                '\u{0027}' => state = AfterDoctypePublicIdentifier,
                '\u{0000}' => match cur_token.tag {
                    TokenTag::Doctype(ref mut val) => {
                        match val.public_identifier {
                            Some(ref mut s) => {
                                s.push('\u{FFFD}');
                            }
                            None => {
                                // TODO: Correct with error handling
                                panic!("In DoctypePublicIdentifierSingleQuote without public identifier created");
                            }
                        }
                        state = DoctypePublicIdentifierDoubleQuote;
                    }
                    _ => {
                        // TODO: This is fine as a panic for now,
                        //       but we need better error handling
                        panic!("HTML Tokenizer ")
                    }
                },
                '>' => {
                    cur_token.flags.force_quirks = true;
                    state = Data;
                    tokens.push(cur_token.clone());
                }
                _ => match cur_token.tag {
                    TokenTag::Doctype(ref mut val) => {
                        match val.public_identifier {
                            Some(ref mut s) => {
                                s.push(chars[idx]);
                            }
                            None => {
                                // TODO: Correct with error handling
                                panic!("In DoctypePublicIdentifierSingleQuote without public identifier created");
                            }
                        }
                        state = DoctypePublicIdentifierDoubleQuote;
                    }
                    _ => {
                        // TODO: This is fine as a panic for now,
                        //       but we need better error handling
                        panic!("HTML Tokenizer ")
                    }
                },
            },
            AfterDoctypePublicIdentifier => match chars[idx] {
                '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' => {
                    state = BetweenDoctypePublicAndSystemIdentifiers;
                }
                '>' => {
                    state = Data;
                    tokens.push(cur_token.clone());
                }
                '"' => match cur_token.tag {
                    TokenTag::Doctype(ref mut val) => {
                        val.system = Some(String::new());
                        state = DoctypeSystemIdentifierDoubleQuote;
                    }
                    _ => {
                        // TODO: This is fine as a panic for now,
                        //       but we need better error handling
                        panic!("HTML Tokenizer ")
                    }
                },
                '\u{0027}' => match cur_token.tag {
                    TokenTag::Doctype(ref mut val) => {
                        val.system = Some(String::new());
                        state = DoctypeSystemIdentifierSingleQuote;
                    }
                    _ => {
                        // TODO: This is fine as a panic for now,
                        //       but we need better error handling
                        panic!("HTML Tokenizer ")
                    }
                },
                _ => {
                    cur_token.flags.force_quirks = true;
                    state = BogusDoctype;
                    continue;
                }
            },
            BetweenDoctypePublicAndSystemIdentifiers => match chars[idx] {
                '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' => {}
                '>' => {
                    state = Data;
                    tokens.push(cur_token.clone())
                }
                '"' => match cur_token.tag {
                    TokenTag::Doctype(ref mut val) => {
                        val.system = Some(String::new());
                        state = DoctypeSystemIdentifierDoubleQuote;
                    }
                    _ => {
                        // TODO: This is fine as a panic for now,
                        //       but we need better error handling
                        panic!("HTML Tokenizer ")
                    }
                },
                '\u{0027}' => match cur_token.tag {
                    TokenTag::Doctype(ref mut val) => {
                        val.system = Some(String::new());
                        state = DoctypeSystemIdentifierSingleQuote;
                    }
                    _ => {
                        // TODO: This is fine as a panic for now,
                        //       but we need better error handling
                        panic!("HTML Tokenizer ")
                    }
                },
                _ => {
                    cur_token.flags.force_quirks = true;
                    state = BogusDoctype;
                    continue;
                }
            },
            AfterDoctypeSystemKeyword => match chars[idx] {
                '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' => {
                    state = BeforeDoctypeSystemIdentifier;
                }
                '"' => match cur_token.tag {
                    TokenTag::Doctype(ref mut val) => {
                        val.system = Some(String::new());
                        state = DoctypeSystemIdentifierDoubleQuote;
                    }
                    _ => {
                        // TODO: This is fine as a panic for now,
                        //       but we need better error handling
                        panic!("HTML Tokenizer ")
                    }
                },
                '\u{0027}' => match cur_token.tag {
                    TokenTag::Doctype(ref mut val) => {
                        val.system = Some(String::new());
                        state = DoctypeSystemIdentifierSingleQuote;
                    }
                    _ => {
                        // TODO: This is fine as a panic for now,
                        //       but we need better error handling
                        panic!("HTML Tokenizer ")
                    }
                },
                '>' => {
                    cur_token.flags.force_quirks = true;
                    state = Data;
                    tokens.push(cur_token.clone());
                }
                _ => {
                    cur_token.flags.force_quirks = true;
                    state = BogusDoctype;
                    continue;
                }
            },
            BeforeDoctypeSystemIdentifier => match chars[idx] {
                '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' => {}
                '"' => match cur_token.tag {
                    TokenTag::Doctype(ref mut val) => {
                        val.system = Some(String::new());
                        state = DoctypeSystemIdentifierDoubleQuote;
                    }
                    _ => {
                        // TODO: This is fine as a panic for now,
                        //       but we need better error handling
                        panic!("HTML Tokenizer ")
                    }
                },
                '\u{0027}' => match cur_token.tag {
                    TokenTag::Doctype(ref mut val) => {
                        val.system = Some(String::new());
                        state = DoctypeSystemIdentifierSingleQuote;
                    }
                    _ => {
                        // TODO: This is fine as a panic for now,
                        //       but we need better error handling
                        panic!("HTML Tokenizer ")
                    }
                },
                '>' => {
                    cur_token.flags.force_quirks = true;
                    state = Data;
                    tokens.push(cur_token.clone());
                }
                _ => {
                    cur_token.flags.force_quirks = true;
                    state = BogusDoctype;
                    continue;
                }
            },
            DoctypeSystemIdentifierDoubleQuote => match chars[idx] {
                '"' => state = AfterDoctypeSystemIdentifier,
                '\u{0000}' => match cur_token.tag {
                    TokenTag::Doctype(ref mut val) => {
                        match val.system {
                            Some(ref mut s) => s.push('\u{FFFD}'),
                            None => panic!("In DoctypeSystemIdentifier without system set"),
                        };
                    }
                    _ => {
                        // TODO: This is fine as a panic for now,
                        //       but we need better error handling
                        panic!("HTML Tokenizer ")
                    }
                },
                '>' => {
                    cur_token.flags.force_quirks = true;
                    state = Data;
                    tokens.push(cur_token.clone());
                }
                _ => match cur_token.tag {
                    TokenTag::Doctype(ref mut val) => {
                        match val.system {
                            Some(ref mut s) => s.push(chars[idx]),
                            None => panic!("In DoctypeSystemIdentifier without system set"),
                        };
                    }
                    _ => {
                        // TODO: This is fine as a panic for now,
                        //       but we need better error handling
                        panic!("HTML Tokenizer ")
                    }
                },
            },
            DoctypeSystemIdentifierSingleQuote => match chars[idx] {
                '\u{0027}' => state = AfterDoctypeSystemIdentifier,
                '\u{0000}' => match cur_token.tag {
                    TokenTag::Doctype(ref mut val) => {
                        match val.system {
                            Some(ref mut s) => s.push('\u{FFFD}'),
                            None => panic!("In DoctypeSystemIdentifier without system set"),
                        };
                    }
                    _ => {
                        // TODO: This is fine as a panic for now,
                        //       but we need better error handling
                        panic!("HTML Tokenizer ")
                    }
                },
                '>' => {
                    cur_token.flags.force_quirks = true;
                    state = Data;
                    tokens.push(cur_token.clone());
                }
                _ => match cur_token.tag {
                    TokenTag::Doctype(ref mut val) => {
                        match val.system {
                            Some(ref mut s) => s.push(chars[idx]),
                            None => panic!("In DoctypeSystemIdentifier without system set"),
                        };
                    }
                    _ => {
                        // TODO: This is fine as a panic for now,
                        //       but we need better error handling
                        panic!("HTML Tokenizer ")
                    }
                },
            },
            AfterDoctypeSystemIdentifier => match chars[idx] {
                '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' => {}
                '>' => {
                    state = Data;
                    tokens.push(cur_token.clone());
                }
                _ => {
                    state = BogusDoctype;
                    continue;
                }
            },
            BogusDoctype => match chars[idx] {
                '>' => {
                    state = Data;
                    tokens.push(cur_token.clone());
                }
                _ => {}
            },
            CDataSection => match chars[idx] {
                ']' => state = CDataSectionBracket,
                _ => tokens.push(new_character_string(chars[idx])),
            },
            CDataSectionBracket => match chars[idx] {
                ']' => state = CDataSectionEnd,
                _ => {
                    tokens.push(new_character_string(']'));
                    state = CDataSection;
                    continue;
                }
            },
            CDataSectionEnd => match chars[idx] {
                ']' => tokens.push(new_character_string(']')),
                '>' => state = Data,
                _ => {
                    tokens.push(new_character_string(']'));
                    tokens.push(new_character_string(']'));
                    state = CDataSection;
                    continue;
                }
            },
            CharacterReference => {
                temp_buffer = "&".to_string();
                match chars[idx] {
                    '#' => {
                        temp_buffer.push(chars[idx]);
                        state = NumericCharacterReference;
                    }
                    _ => {
                        if chars[idx].is_alphanumeric() {
                            state = NamedCharacterReference;
                            continue;
                        } else {
                            if is_part_of_attribute(return_state) {
                                temp_buffer
                                    .chars()
                                    .for_each(|c| cur_token.append_to_last_attribute(c));
                            } else {
                                temp_buffer
                                    .chars()
                                    .for_each(|c| tokens.push(new_character_string(c)));
                            }

                            state = return_state;
                            continue;
                        }
                    }
                }
            }
            // NamedCharacterReference => {}
            AmbiguousAmpersand => {
                if chars[idx].is_alphanumeric() {
                    if is_part_of_attribute(return_state) {
                        cur_token.append_to_last_attribute(chars[idx]);
                    } else {
                        tokens.push(new_character_string(chars[idx]));
                    }
                } else {
                    state = return_state;
                    continue;
                }
            }
            NumericCharacterReference => {
                character_reference_code = 0;
                match chars[idx] {
                    'x' | 'X' => {
                        temp_buffer.push(chars[idx]);
                        state = HexadecimalCharacterReferenceStart;
                    }
                    _ => {
                        state = DecimalCharacterReferenceStart;
                        continue;
                    }
                }
            }
            HexadecimalCharacterReferenceStart => match chars[idx] {
                '0'..='9' | 'A'..='F' | 'a'..='f' => {
                    state = HexadecimalCharacterReference;
                    continue;
                }
                _ => {
                    if is_part_of_attribute(return_state) {
                        temp_buffer
                            .chars()
                            .for_each(|c| cur_token.append_to_last_attribute(c));
                    } else {
                        temp_buffer
                            .chars()
                            .for_each(|c| tokens.push(new_character_string(c)));
                    }
                    state = return_state;
                    continue;
                }
            },
            DecimalCharacterReferenceStart => match chars[idx] {
                '0'..='9' => {
                    state = DecimalCharacterReference;
                    continue;
                }
                _ => {
                    if is_part_of_attribute(return_state) {
                        temp_buffer
                            .chars()
                            .for_each(|c| cur_token.append_to_last_attribute(c));
                    } else {
                        temp_buffer
                            .chars()
                            .for_each(|c| tokens.push(new_character_string(c)));
                    }
                    state = return_state;
                    continue;
                }
            },
            HexadecimalCharacterReference => match chars[idx] {
                '0'..='9' | 'A'..='F' | 'a'..='f' => {
                    character_reference_code *= 16;
                    character_reference_code += chars[idx].to_digit(16).unwrap();
                }
                ';' => state = NumericCharacterReferenceEnd,
                _ => {
                    state = NumericCharacterReferenceEnd;
                    continue;
                }
            },
            DecimalCharacterReference => match chars[idx] {
                '0'..='9' => {
                    character_reference_code *= 16;
                    character_reference_code += chars[idx].to_digit(10).unwrap();
                }
                ';' => state = NumericCharacterReferenceEnd,
                _ => {
                    state = NumericCharacterReferenceEnd;
                    continue;
                }
            },
            NumericCharacterReferenceEnd => {
                match character_reference_code {
                    // Null check
                    0x00 |
                    // Surrogate check
                    0xD800..=0xDBFF | 0xDC00..=0xDFFF | 0x10FFFF.. => {
                        character_reference_code = 0xFFFD;
                    }
                    // // NonCharacter Check
                    // 0xFDD0..=0xFDEF
                    // | 0xFFFE
                    // | 0xFFFF
                    // | 0x1FFFE
                    // | 0x1FFFF
                    // | 0x2FFFF
                    // | 0x3FFFE
                    // | 0x3FFFF
                    // | 0x4FFFE
                    // | 0x4FFFF
                    // | 0x5FFFE
                    // | 0x5FFFF
                    // | 0x6FFFE
                    // | 0x6FFFF
                    // | 0x7FFFE
                    // | 0x7FFFF
                    // | 0x8FFFE
                    // | 0x8FFFF
                    // | 0x9FFFE
                    // | 0x9FFFF
                    // | 0xAFFFE
                    // | 0xAFFFF
                    // | 0xBFFFE
                    // | 0xBFFFF
                    // | 0xCFFFE
                    // | 0xCFFFF
                    // | 0xDFFFE
                    // | 0xDFFFF
                    // | 0xEFFFE
                    // | 0xEFFFF
                    // | 0xFFFFE
                    // | 0xFFFFF
                    // | 0x10FFFE => {}
                    // // Control Character Check
                    // 0x0001..0x0020 | 0x007f..=0x009f => {}
                    _ => {
                        character_reference_code =
                            lookup_character_reference(character_reference_code);
                    }
                };
                temp_buffer = String::new();
                // This cast should probably be error checked
                temp_buffer.push(character_reference_code as u8 as char);
                if is_part_of_attribute(return_state) {
                    temp_buffer
                        .chars()
                        .for_each(|c| cur_token.append_to_last_attribute(c));
                } else {
                    temp_buffer
                        .chars()
                        .for_each(|c| tokens.push(new_character_string(c)));
                }

                state = return_state;
            }
            _ => {
                println!("Unsupported tag found");
            }
        };
        idx += 1;
    }

    tokens
}
