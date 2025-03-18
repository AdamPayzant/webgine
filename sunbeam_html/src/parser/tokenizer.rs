use std::collections::VecDeque;

#[derive(Clone, Debug)]
pub struct DoctypeData {
    pub public_identifier: Option<String>,
    pub system: Option<String>,
}

impl DoctypeData {
    fn new() -> DoctypeData {
        DoctypeData {
            public_identifier: None,
            system: None,
        }
    }
}

#[derive(Clone, Debug)]
pub enum TokenTag {
    Character,
    Comment,
    StartTag,
    EndTag,
    Doctype(DoctypeData),
    EoF,
}

#[derive(Clone)]
pub struct HtmlAttribute {
    pub name: String,
    pub value: String,
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
pub struct HtmlTokenFlags {
    pub self_closing: bool,
    pub force_quirks: bool,
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
    pub tag: TokenTag,
    pub data: String,
    pub attributes: Vec<HtmlAttribute>,
    pub flags: HtmlTokenFlags,
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

#[derive(Debug, Clone, Copy)]
pub enum FsmState {
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

macro_rules! emit_token {
    ($self:ident, $arg:expr) => {{
        $self.token_buffer.push_back($arg);
        $self.idx += 1;
        return;
    }};
}

macro_rules! emit_and_reconsume {
    ($self:ident, $arg:expr) => {{
        $self.token_buffer.push_back($arg);
        return;
    }};
}

macro_rules! emit_token_no_return {
    ($self:ident, $arg: expr) => {
        $self.token_buffer.push_back($arg);
    };
}

// Wraps continue to make the intent more clear
macro_rules! reconsume {
    () => {
        continue;
    };
}

pub struct Tokenizer {
    idx: usize,
    chars: Vec<char>,

    state: FsmState,
    return_state: FsmState,

    cur_token: HtmlToken,
    token_buffer: VecDeque<HtmlToken>,

    temp_buffer: String,
    pub open_tag_stack: Vec<String>,
    character_reference_code: u32,
}

// self.chars[self.idx].to_lowercase().next().unwrap()
// '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}'

impl Tokenizer {
    pub fn init(html_string: &str) -> Tokenizer {
        Tokenizer {
            idx: 0,
            chars: html_string.chars().collect(),
            state: FsmState::Data,
            return_state: FsmState::Data,
            cur_token: HtmlToken::new(TokenTag::EoF),
            token_buffer: VecDeque::new(),
            temp_buffer: String::new(),
            open_tag_stack: Vec::new(),
            character_reference_code: 0,
        }
    }

    pub fn get_next_token(&mut self) -> HtmlToken {
        match self.token_buffer.pop_front() {
            Some(val) => val,
            None => {
                self.next_token_iteration();
                match self.token_buffer.pop_front() {
                    Some(val) => val,
                    None => self.get_eof_token(),
                }
            }
        }
    }

    pub fn push_open_tag(&mut self, tag_name: String) {
        self.open_tag_stack.push(tag_name);
    }

    pub fn pop_open_tag(&mut self) {
        self.open_tag_stack.pop();
    }

    pub fn is_open_tag_stack_empty(&mut self) -> bool {
        self.open_tag_stack.is_empty()
    }

    pub fn set_state(&mut self, state: FsmState) {
        self.state = state;
    }

    fn next_token_iteration(&mut self) {
        use FsmState::*;

        let mut temp_buf = self.temp_buffer.clone();

        while self.idx < self.chars.len() {
            log::trace!(
                "tokenizer FSM state: {:?}, parsing character: {}",
                self.state,
                self.chars[self.idx]
            );
            match self.state {
                Data => match self.chars[self.idx] {
                    '&' => {
                        self.return_state = Data;
                        self.state = CharacterReference;
                    }
                    '<' => self.state = TagOpen,
                    _ => {
                        emit_token!(self, new_character_string(self.chars[self.idx]));
                    }
                },
                RCData => match self.chars[self.idx] {
                    '&' => {
                        self.return_state = RCData;
                        self.state = CharacterReference;
                    }
                    '<' => self.state = RCDataLessThan,
                    '\u{0000}' => {
                        emit_token!(self, new_character_string('\u{FFFD}'));
                    }
                    _ => {
                        emit_token!(self, new_character_string(self.chars[self.idx]));
                    }
                },
                RawText => match self.chars[self.idx] {
                    '<' => self.state = RawTextLessThan,
                    '\u{0000}' => {
                        emit_token!(self, new_character_string('\u{FFFD}'));
                    }
                    _ => {
                        emit_token!(self, new_character_string(self.chars[self.idx]));
                    }
                },
                ScriptData => match self.chars[self.idx] {
                    '<' => self.state = ScriptDataLessThan,
                    '\u{0000}' => {
                        emit_token!(self, new_character_string('\u{FFFD}'));
                    }
                    _ => {
                        emit_token!(self, new_character_string(self.chars[self.idx]));
                    }
                },
                Plaintext => match self.chars[self.idx] {
                    '\u{0000}' => {
                        emit_token!(self, new_character_string('\u{FFFD}'));
                    }
                    _ => {
                        emit_token!(self, new_character_string(self.chars[self.idx]));
                    }
                },
                TagOpen => match self.chars[self.idx] {
                    '!' => self.state = MarkupDeclarationOpen,
                    '/' => self.state = EndTagOpen,
                    '?' => {
                        self.cur_token = HtmlToken::new(TokenTag::Comment);
                        self.state = BogusComment;
                        reconsume!();
                    }
                    _ => {
                        if self.chars[self.idx].is_alphabetic() {
                            self.cur_token = HtmlToken::new(TokenTag::StartTag);
                            self.state = TagName;
                            reconsume!();
                        } else {
                            self.state = Data;
                            emit_and_reconsume!(self, new_character_string('<'));
                        }
                    }
                },
                EndTagOpen => match self.chars[self.idx] {
                    '>' => self.state = Data,
                    _ => {
                        if self.chars[self.idx].is_alphabetic() {
                            self.cur_token = HtmlToken::new(TokenTag::EndTag);
                            self.state = TagName;
                            reconsume!();
                        } else {
                            self.cur_token = HtmlToken::new(TokenTag::Comment);
                            self.state = BogusComment;
                            reconsume!();
                        }
                    }
                },
                TagName => match self.chars[self.idx] {
                    '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' => {
                        self.state = BeforeAttributeName
                    }
                    '/' => self.state = SelfClosingStartTag,
                    '>' => {
                        self.state = Data;
                        emit_token!(self, self.cur_token.clone());
                    }
                    '\u{0000}' => self.cur_token.data_append('\u{FFFD}'),
                    _ => self
                        .cur_token
                        .data_append(self.chars[self.idx].to_lowercase().next().unwrap()),
                },
                RCDataLessThan => match self.chars[self.idx] {
                    '/' => {
                        temp_buf = String::new();
                        self.temp_buffer = String::new();
                        self.state = RCDataEndTagOpen;
                    }
                    _ => {
                        self.state = RCData;
                        emit_and_reconsume!(self, new_character_string('<'));
                    }
                },
                RCDataEndTagOpen => {
                    if self.chars[self.idx].is_alphabetic() {
                        self.cur_token = HtmlToken::new(TokenTag::EndTag);
                        self.state = RCDataEndTagName;
                        reconsume!();
                    } else {
                        self.state = RCData;
                        emit_token_no_return!(self, new_character_string('<'));
                        emit_and_reconsume!(self, new_character_string('/'));
                    }
                }
                RCDataEndTagName => {
                    let mut acted = false;
                    match self.open_tag_stack.last() {
                        Some(val) => {
                            if *val == self.cur_token.data {
                                match self.chars[self.idx] {
                                    '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' => {
                                        self.state = BeforeAttributeName;
                                        acted = true;
                                    }
                                    '/' => {
                                        self.state = SelfClosingStartTag;
                                        acted = true;
                                    }
                                    '>' => {
                                        self.state = Data;
                                        acted = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        None => {}
                    }
                    if self.chars[self.idx].is_alphabetic() {
                        self.cur_token
                            .data_append(self.chars[self.idx].to_lowercase().next().unwrap());
                        temp_buf.push(self.chars[self.idx]);
                        self.temp_buffer.push(self.chars[self.idx]);
                        acted = true;
                    }

                    if !acted {
                        emit_token_no_return!(self, new_character_string('<'));
                        emit_token_no_return!(self, new_character_string('/'));
                        temp_buf.chars().for_each(|c| {
                            emit_token_no_return!(self, new_character_string(c));
                        });
                        self.state = RCData;
                        return;
                    }
                }
                RawTextLessThan => match self.chars[self.idx] {
                    '/' => {
                        temp_buf = String::new();
                        self.temp_buffer = String::new();
                        self.state = RawTextEndTagOpen;
                    }
                    _ => {
                        self.state = RawText;
                        emit_and_reconsume!(self, new_character_string('<'));
                    }
                },
                RawTextEndTagOpen => {
                    if self.chars[self.idx].is_alphabetic() {
                        self.cur_token = HtmlToken::new(TokenTag::EndTag);
                        self.state = RawTextEndTagName;
                        reconsume!();
                    } else {
                        self.state = RawText;
                        emit_token_no_return!(self, new_character_string('<'));
                        emit_and_reconsume!(self, new_character_string('/'));
                    }
                }
                RawTextEndTagName => {
                    let mut acted = false;
                    match self.open_tag_stack.last() {
                        Some(val) => {
                            if *val == self.cur_token.data {
                                match self.chars[self.idx] {
                                    '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' => {
                                        self.state = BeforeAttributeName;
                                        acted = true;
                                    }
                                    '/' => {
                                        self.state = SelfClosingStartTag;
                                        acted = true;
                                    }
                                    '>' => {
                                        self.state = Data;
                                        acted = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        None => {}
                    }
                    if self.chars[self.idx].is_alphabetic() {
                        self.cur_token
                            .data_append(self.chars[self.idx].to_lowercase().next().unwrap());
                        temp_buf.push(self.chars[self.idx]);
                        self.temp_buffer.push(self.chars[self.idx]);
                        acted = true;
                    }

                    if !acted {
                        emit_token_no_return!(self, new_character_string('<'));
                        emit_token_no_return!(self, new_character_string('/'));
                        temp_buf.chars().for_each(|c| {
                            emit_token_no_return!(self, new_character_string(c));
                        });
                        self.state = RawText;
                        return;
                    }
                }
                ScriptDataLessThan => match self.chars[self.idx] {
                    '/' => {
                        temp_buf = String::new();
                        self.temp_buffer = String::new();
                        self.state = ScriptDataEndTagOpen;
                    }
                    '!' => {
                        self.state = ScriptDataEscapeStart;
                        emit_token_no_return!(self, new_character_string('<'));
                        emit_and_reconsume!(self, new_character_string('!'));
                    }
                    _ => {
                        self.state = ScriptData;
                        emit_and_reconsume!(self, new_character_string('<'));
                    }
                },
                ScriptDataEndTagOpen => {
                    if self.chars[self.idx].is_alphabetic() {
                        self.cur_token = HtmlToken::new(TokenTag::EndTag);
                        self.state = ScriptDataEndTagName;
                        reconsume!();
                    } else {
                        self.state = ScriptData;
                        emit_token_no_return!(self, new_character_string('<'));
                        emit_and_reconsume!(self, new_character_string('/'));
                    }
                }
                ScriptDataEndTagName => {
                    let mut acted = false;
                    match self.open_tag_stack.last() {
                        Some(val) => {
                            if *val == self.cur_token.data {
                                match self.chars[self.idx] {
                                    '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' => {
                                        self.state = BeforeAttributeName;
                                        acted = true;
                                    }
                                    '/' => {
                                        self.state = SelfClosingStartTag;
                                        acted = true;
                                    }
                                    '>' => {
                                        self.state = Data;
                                        acted = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        None => {}
                    }
                    if self.chars[self.idx].is_alphabetic() {
                        self.cur_token
                            .data_append(self.chars[self.idx].to_lowercase().next().unwrap());
                        temp_buf.push(self.chars[self.idx]);
                        self.temp_buffer.push(self.chars[self.idx]);
                        acted = true;
                    }

                    if !acted {
                        emit_token_no_return!(self, new_character_string('<'));
                        emit_token_no_return!(self, new_character_string('/'));
                        temp_buf.chars().for_each(|c| {
                            emit_token_no_return!(self, new_character_string(c));
                        });
                        self.state = ScriptData;
                        return;
                    }
                }
                ScriptDataEscapeStart => match self.chars[self.idx] {
                    '-' => {
                        self.state = ScriptDataEscapedDash;
                        emit_token!(self, new_character_string('-'));
                    }
                    _ => {
                        self.state = ScriptData;
                        reconsume!();
                    }
                },
                ScriptDataEscaped => match self.chars[self.idx] {
                    '-' => {
                        self.state = ScriptDataEscapedDash;
                        emit_token!(self, new_character_string('-'));
                    }
                    '<' => self.state = ScriptDataEscapedLessThan,
                    '\u{0000}' => {
                        emit_token!(self, new_character_string('\u{FFFD}'));
                    }
                    _ => {
                        emit_token!(self, new_character_string(self.chars[self.idx]));
                    }
                },
                ScriptDataEscapedDash => match self.chars[self.idx] {
                    '-' => {
                        self.state = ScriptDataEscapedDashDash;
                        emit_token!(self, new_character_string('-'));
                    }
                    '<' => self.state = ScriptDataEscapedLessThan,
                    '\u{0000}' => {
                        self.state = ScriptDataEscaped;
                        emit_token!(self, new_character_string('\u{FFFD}'));
                    }
                    _ => {
                        self.state = ScriptDataEscaped;
                        emit_token!(self, new_character_string(self.chars[self.idx]));
                    }
                },
                ScriptDataEscapedDashDash => match self.chars[self.idx] {
                    '-' => {
                        emit_token!(self, new_character_string('-'));
                    }
                    '<' => self.state = ScriptDataEscapedLessThan,
                    '>' => {
                        self.state = ScriptData;
                        emit_token!(self, new_character_string('>'));
                    }
                    '\u{0000}' => {
                        self.state = ScriptDataEscaped;
                        emit_token!(self, new_character_string('\u{FFFD}'));
                    }
                    _ => {
                        self.state = ScriptDataEscaped;
                        emit_token!(self, new_character_string(self.chars[self.idx]));
                    }
                },
                ScriptDataEscapedLessThan => match self.chars[self.idx] {
                    '/' => {
                        temp_buf = String::new();
                        self.temp_buffer = String::new();
                        self.state = ScriptDataEscapedEndTagOpen;
                    }
                    _ => {
                        if self.chars[self.idx].is_alphabetic() {
                            temp_buf = String::new();
                            self.temp_buffer = String::new();
                            self.state = ScriptDataDoubleEscapeStart;
                            emit_and_reconsume!(self, new_character_string('<'));
                        } else {
                            self.state = ScriptDataEscaped;
                            emit_and_reconsume!(self, new_character_string('<'));
                        }
                    }
                },
                ScriptDataEscapedEndTagOpen => {
                    if self.chars[self.idx].is_alphabetic() {
                        self.cur_token = HtmlToken::new(TokenTag::EndTag);
                        self.state = ScriptDataEscapedEndTagName;
                        reconsume!();
                    } else {
                        self.state = ScriptDataEscaped;
                        emit_token_no_return!(self, new_character_string('<'));
                        emit_and_reconsume!(self, new_character_string('/'));
                    }
                }
                ScriptDataEscapedEndTagName => {
                    let mut acted = false;
                    match self.open_tag_stack.last() {
                        Some(val) => {
                            if *val == self.cur_token.data {
                                match self.chars[self.idx] {
                                    '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' => {
                                        self.state = BeforeAttributeName;
                                        acted = true;
                                    }
                                    '/' => {
                                        self.state = SelfClosingStartTag;
                                        acted = true;
                                    }
                                    '>' => {
                                        self.state = Data;
                                        acted = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        None => {}
                    }
                    if self.chars[self.idx].is_alphabetic() {
                        self.cur_token
                            .data_append(self.chars[self.idx].to_lowercase().next().unwrap());
                        temp_buf.push(self.chars[self.idx]);
                        self.temp_buffer.push(self.chars[self.idx]);
                        acted = true;
                    }

                    if !acted {
                        emit_token_no_return!(self, new_character_string('<'));
                        emit_token_no_return!(self, new_character_string('/'));
                        temp_buf.chars().for_each(|c| {
                            emit_token_no_return!(self, new_character_string(c));
                        });
                        self.state = ScriptDataEscaped;
                        return;
                    }
                }
                ScriptDataDoubleEscapeStart => match self.chars[self.idx] {
                    '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' | '/' | '>' => {
                        if temp_buf == "script" {
                            self.state = ScriptDataDoubleEscaped;
                        } else {
                            self.state = ScriptDataEscaped;
                        }
                        emit_token!(self, new_character_string(self.chars[self.idx]));
                    }
                    _ => {
                        if self.chars[self.idx].is_alphabetic() {
                            temp_buf.push(self.chars[self.idx].to_lowercase().next().unwrap());
                            self.temp_buffer
                                .push(self.chars[self.idx].to_lowercase().next().unwrap());
                            emit_token!(self, new_character_string(self.chars[self.idx]));
                        } else {
                            self.state = ScriptDataEscaped;
                            reconsume!();
                        }
                    }
                },
                ScriptDataDoubleEscaped => match self.chars[self.idx] {
                    '-' => {
                        self.state = ScriptDataDoubleEscapedDash;
                        emit_token!(self, new_character_string('-'));
                    }
                    '<' => {
                        self.state = ScriptDataDoubleEscapedLessThan;
                        emit_token!(self, new_character_string('<'));
                    }
                    '\u{0000}' => {
                        emit_token!(self, new_character_string('\u{FFFD}'));
                    }
                    _ => {
                        emit_token!(self, new_character_string(self.chars[self.idx]));
                    }
                },
                ScriptDataDoubleEscapedDash => match self.chars[self.idx] {
                    '-' => {
                        self.state = ScriptDataDoubleEscapedDashDash;
                        emit_token!(self, new_character_string('-'));
                    }
                    '<' => {
                        self.state = ScriptDataDoubleEscapedLessThan;
                        emit_token!(self, new_character_string('<'));
                    }
                    '\u{0000}' => {
                        self.state = ScriptDataDoubleEscaped;
                        emit_token!(self, new_character_string('\u{FFFD}'));
                    }
                    _ => {
                        self.state = ScriptDataDoubleEscaped;
                        emit_token!(self, new_character_string(self.chars[self.idx]));
                    }
                },
                ScriptDataDoubleEscapedDashDash => match self.chars[self.idx] {
                    '-' => {
                        emit_token!(self, new_character_string('-'));
                    }
                    '<' => {
                        self.state = ScriptDataDoubleEscapedLessThan;
                        emit_token!(self, new_character_string('<'));
                    }
                    '>' => {
                        self.state = ScriptData;
                        emit_token!(self, new_character_string('>'));
                    }
                    '\u{0000}' => {
                        self.state = ScriptDataDoubleEscaped;
                        emit_token!(self, new_character_string('\u{FFFD}'));
                    }
                    _ => {
                        self.state = ScriptDataDoubleEscaped;
                        emit_token!(self, new_character_string(self.chars[self.idx]));
                    }
                },
                ScriptDataDoubleEscapedLessThan => match self.chars[self.idx] {
                    '/' => {
                        temp_buf = String::new();
                        self.temp_buffer = String::new();
                        self.state = ScriptDataDoubleEscapeEnd;
                        emit_token!(self, new_character_string('/'));
                    }
                    _ => {
                        self.state = ScriptDataDoubleEscaped;
                        reconsume!();
                    }
                },
                ScriptDataDoubleEscapeEnd => match self.chars[self.idx] {
                    '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' | '/' | '>' => {
                        if temp_buf == "script" {
                            self.state = ScriptDataEscaped;
                        } else {
                            self.state = ScriptDataDoubleEscaped;
                        }
                        emit_token!(self, new_character_string(self.chars[self.idx]));
                    }
                    _ => {
                        if self.chars[self.idx].is_alphabetic() {
                            temp_buf.push(self.chars[self.idx].to_lowercase().next().unwrap());
                            self.temp_buffer
                                .push(self.chars[self.idx].to_lowercase().next().unwrap());
                            emit_token!(self, new_character_string(self.chars[self.idx]));
                        } else {
                            self.state = ScriptDataDoubleEscaped;
                            reconsume!();
                        }
                    }
                },
                BeforeAttributeName => match self.chars[self.idx] {
                    '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' => {}
                    '/' | '>' => {
                        self.state = AfterAttributeName;
                        reconsume!();
                    }
                    '=' => {
                        self.cur_token.attributes.push(HtmlAttribute::new());
                        self.cur_token
                            .attributes
                            .last_mut()
                            .unwrap()
                            .name
                            .push(self.chars[self.idx]);
                        self.state = AttributeName;
                    }
                    _ => {
                        self.cur_token.attributes.push(HtmlAttribute::new());
                        self.state = AttributeName;
                        reconsume!();
                    }
                },
                AttributeName => match self.chars[self.idx] {
                    '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' | '/' | '>' => {
                        self.state = AfterAttributeName;
                        reconsume!();
                    }
                    '=' => self.state = BeforeAttributeValue,
                    '\u{0000}' => {
                        self.cur_token
                            .attributes
                            .last_mut()
                            .unwrap()
                            .name
                            .push('\u{FFFD}');
                    }
                    // '"' | '\u{0027}' | '<' => {} // Error but treated as the anything else category
                    _ => {
                        self.cur_token
                            .attributes
                            .last_mut()
                            .unwrap()
                            .name
                            .push(self.chars[self.idx].to_lowercase().next().unwrap());
                    }
                },
                AfterAttributeName => match self.chars[self.idx] {
                    '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' => {} // Ignore
                    '/' => self.state = SelfClosingStartTag,
                    '=' => self.state = BeforeAttributeValue,
                    '>' => {
                        self.state = Data;
                        emit_token!(self, self.cur_token.clone());
                    }
                    _ => {
                        self.cur_token.attributes.push(HtmlAttribute::new());
                        self.state = AttributeName;
                        reconsume!();
                    }
                },
                BeforeAttributeValue => match self.chars[self.idx] {
                    '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' => {} // Ignore
                    '"' => self.state = AttributeValueDoubleQuoted,
                    '\u{0027}' => self.state = AttributeValueSingleQuoted,
                    '>' => {
                        self.state = Data;
                        emit_token!(self, self.cur_token.clone());
                    }
                    _ => {
                        self.state = AttributeValueUnquoted;
                        reconsume!();
                    }
                },
                AttributeValueDoubleQuoted => match self.chars[self.idx] {
                    '"' => self.state = AfterAttributeValueQuoted,
                    '&' => {
                        self.return_state = AttributeValueDoubleQuoted;
                        self.state = CharacterReference;
                    }
                    '\u{0000}' => {
                        self.cur_token
                            .attributes
                            .last_mut()
                            .unwrap()
                            .value
                            .push('\u{FFFD}');
                    }
                    _ => {
                        self.cur_token
                            .attributes
                            .last_mut()
                            .unwrap()
                            .value
                            .push(self.chars[self.idx].to_lowercase().next().unwrap());
                    }
                },
                AttributeValueSingleQuoted => match self.chars[self.idx] {
                    '\u{0027}' => self.state = AfterAttributeValueQuoted,
                    '&' => {
                        self.return_state = AttributeValueDoubleQuoted;
                        self.state = CharacterReference;
                    }
                    '\u{0000}' => {
                        self.cur_token
                            .attributes
                            .last_mut()
                            .unwrap()
                            .value
                            .push('\u{FFFD}');
                    }
                    _ => {
                        self.cur_token
                            .attributes
                            .last_mut()
                            .unwrap()
                            .value
                            .push(self.chars[self.idx].to_lowercase().next().unwrap());
                    }
                },
                AttributeValueUnquoted => match self.chars[self.idx] {
                    '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' => {
                        self.state = BeforeAttributeName
                    }
                    '&' => {
                        self.return_state = AttributeValueUnquoted;
                        self.state = CharacterReference;
                    }
                    '>' => {
                        self.state = Data;
                        emit_token!(self, self.cur_token.clone());
                    }
                    '\u{0000}' => {
                        self.cur_token
                            .attributes
                            .last_mut()
                            .unwrap()
                            .value
                            .push('\u{FFFD}');
                    }
                    // '"' | '\u{0027}' | '<' | '=' | '`' => {} // Error, treat as anything else
                    _ => {
                        self.cur_token
                            .attributes
                            .last_mut()
                            .unwrap()
                            .value
                            .push(self.chars[self.idx].to_lowercase().next().unwrap());
                    }
                },
                AfterAttributeValueQuoted => match self.chars[self.idx] {
                    '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' => {
                        self.state = BeforeAttributeName
                    }
                    '/' => self.state = SelfClosingStartTag,
                    '>' => {
                        self.state = Data;
                        emit_token!(self, self.cur_token.clone());
                    }
                    _ => {
                        self.state = BeforeAttributeName;
                        reconsume!();
                    }
                },
                SelfClosingStartTag => match self.chars[self.idx] {
                    '>' => {
                        self.cur_token.flags.self_closing = true;
                        self.state = Data;

                        emit_token!(self, self.cur_token.clone());
                    }
                    _ => {
                        self.state = BeforeAttributeName;
                        reconsume!();
                    }
                },
                BogusComment => match self.chars[self.idx] {
                    '>' => {
                        self.state = Data;
                        emit_token!(self, self.cur_token.clone());
                    }
                    '\u{0000}' => {
                        self.cur_token.data.push('\u{FFFD}');
                    }
                    _ => {
                        self.cur_token.data.push(self.chars[self.idx]);
                    }
                },
                MarkupDeclarationOpen => {
                    if self.idx + 1 < self.chars.len()
                        && compare_slices(&self.chars[self.idx..self.idx + 2], &['-', '-'], true)
                    {
                        self.idx += 2;
                        self.cur_token = HtmlToken::new(TokenTag::Comment);
                        self.state = CommentStart;
                    } else if self.idx + 7 < self.chars.len()
                        && compare_slices(
                            &self.chars[self.idx..self.idx + 7],
                            &['d', 'o', 'c', 't', 'y', 'p', 'e'],
                            false,
                        )
                    {
                        self.idx += 7;
                        self.state = Doctype;
                    } else if self.idx + 7 < self.chars.len()
                        && compare_slices(
                            &self.chars[self.idx..self.idx + 8],
                            &['[', 'C', 'D', 'A', 'T', 'a', '['],
                            true,
                        )
                    {
                        self.idx += 8;
                        // TODO: Check if there is an adjusted current node
                        //       and it's not an element in HTML namespace.
                        //       If not, create a comment instead
                        self.state = CDataSection;
                    } else {
                        self.cur_token = HtmlToken::new(TokenTag::Comment);
                        self.state = BogusComment;
                    }
                }
                CommentStart => match self.chars[self.idx] {
                    '-' => self.state = CommentStartDash,
                    '>' => {
                        self.state = Data;
                        emit_token!(self, self.cur_token.clone());
                    }
                    _ => {
                        self.state = Comment;
                        reconsume!();
                    }
                },
                CommentStartDash => match self.chars[self.idx] {
                    '-' => self.state = CommentEnd,
                    '>' => {
                        self.state = Data;
                        emit_token!(self, self.cur_token.clone());
                    }
                    _ => {
                        self.cur_token.data_append('-');
                        self.state = Comment;
                        reconsume!();
                    }
                },
                Comment => match self.chars[self.idx] {
                    '<' => {
                        self.cur_token.data_append('<');
                        self.state = CommentLessThan;
                    }
                    '-' => self.state = CommentEndDash,
                    '\u{0000}' => self.cur_token.data_append('\u{FFFD}'),
                    _ => self.cur_token.data_append(self.chars[self.idx]),
                },
                CommentLessThan => match self.chars[self.idx] {
                    '!' => {
                        self.cur_token.data_append('!');
                        self.state = CommentLessThanBang;
                    }
                    '<' => self.cur_token.data_append('<'),
                    _ => {
                        self.state = Comment;
                        reconsume!();
                    }
                },
                CommentLessThanBang => match self.chars[self.idx] {
                    '-' => self.state = CommentLessThanBangDash,
                    _ => {
                        self.state = Comment;
                        reconsume!();
                    }
                },
                CommentLessThanBangDash => match self.chars[self.idx] {
                    '-' => self.state = CommentLessThanBangDashDash,
                    _ => {
                        self.state = Comment;
                        reconsume!();
                    }
                },
                CommentLessThanBangDashDash => {
                    self.state = CommentEnd;
                    reconsume!();
                }
                CommentEndDash => match self.chars[self.idx] {
                    '-' => self.state = CommentEnd,
                    _ => {
                        self.cur_token.data_append('-');
                        self.state = Comment;
                        reconsume!();
                    }
                },
                CommentEnd => match self.chars[self.idx] {
                    '>' => {
                        self.state = Data;
                        emit_token!(self, self.cur_token.clone());
                    }
                    '!' => self.state = CommentEndBang,
                    '-' => self.cur_token.data_append('-'),
                    _ => {
                        self.cur_token.data_append('-');
                        self.state = Comment;
                        reconsume!();
                    }
                },
                CommentEndBang => match self.chars[self.idx] {
                    '-' => {
                        self.cur_token.data_append('-');
                        self.cur_token.data_append('!');
                        self.state = CommentEndDash;
                    }
                    '>' => {
                        self.state = Data;
                        emit_token!(self, self.cur_token.clone());
                    }
                    _ => {
                        self.cur_token.data_append('-');
                        self.cur_token.data_append('!');
                        self.state = Comment;
                        reconsume!();
                    }
                },
                Doctype => match self.chars[self.idx] {
                    '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' => {
                        self.state = BeforeDoctypeName
                    }
                    _ => {
                        self.state = BeforeDoctypeName;
                        reconsume!();
                    }
                },
                BeforeDoctypeName => match self.chars[self.idx] {
                    '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' => {}
                    '\u{0000}' => {
                        self.cur_token = HtmlToken::new(TokenTag::Doctype(DoctypeData::new()));
                        self.cur_token.data_append('\u{FFFD}');
                        self.state = DoctypeName;
                    }
                    '>' => {
                        self.cur_token = HtmlToken::new(TokenTag::Doctype(DoctypeData::new()));
                        self.cur_token.flags.force_quirks = true;
                        self.state = Data;
                        emit_token!(self, self.cur_token.clone());
                    }
                    _ => {
                        self.cur_token = HtmlToken::new(TokenTag::Doctype(DoctypeData::new()));
                        self.cur_token
                            .data_append(self.chars[self.idx].to_lowercase().next().unwrap());
                        self.state = DoctypeName;
                    }
                },
                DoctypeName => match self.chars[self.idx] {
                    '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' => self.state = AfterDoctype,
                    '>' => {
                        self.state = Data;
                        emit_token!(self, self.cur_token.clone());
                    }
                    '\u{0000}' => self.cur_token.data_append('\u{FFFD}'),
                    _ => self
                        .cur_token
                        .data_append(self.chars[self.idx].to_lowercase().next().unwrap()),
                },
                AfterDoctypeName => match self.chars[self.idx] {
                    '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' => {}
                    '>' => {
                        self.state = Data;
                        emit_token!(self, self.cur_token.clone());
                    }
                    _ => {
                        if self.idx + 6 < self.chars.len()
                            && compare_slices(
                                &self.chars[self.idx..self.idx + 7],
                                &['p', 'u', 'b', 'l', 'i', 'c'],
                                false,
                            )
                        {
                            self.idx += 7;
                            self.state = AfterDoctypePublicKeyword;
                            reconsume!();
                        }
                        if self.idx + 6 < self.chars.len()
                            && compare_slices(
                                &self.chars[self.idx..self.idx + 7],
                                &['s', 'y', 's', 't', 'e', 'm'],
                                false,
                            )
                        {
                            self.idx += 7;
                            reconsume!();
                        }

                        self.cur_token.flags.force_quirks = true;
                        self.state = BogusComment;
                        reconsume!();
                    }
                },
                AfterDoctypePublicKeyword => match self.chars[self.idx] {
                    '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' => {
                        self.state = BeforeDoctypePublicIndentifier;
                    }
                    '"' => match self.cur_token.tag {
                        TokenTag::Doctype(ref mut val) => {
                            val.public_identifier = Some(String::new());
                            self.state = DoctypePublicIdentifierDoubleQuote;
                        }
                        _ => {
                            // TODO: This is fine as a panic for now,
                            //       but we need better error handling
                            panic!("HTML Tokenizer ")
                        }
                    },
                    '\u{0027}' => match self.cur_token.tag {
                        TokenTag::Doctype(ref mut val) => {
                            val.public_identifier = Some(String::new());
                            self.state = DoctypePublicIdentifierSingleQuote;
                        }
                        _ => {
                            // TODO: This is fine as a panic for now,
                            //       but we need better error handling
                            panic!("HTML Tokenizer ")
                        }
                    },
                    '>' => {
                        self.cur_token.flags.force_quirks = true;
                        self.state = Data;
                        emit_token!(self, self.cur_token.clone());
                    }
                    _ => {
                        self.cur_token.flags.force_quirks = true;
                        self.state = BogusDoctype;
                        reconsume!();
                    }
                },
                BeforeDoctypePublicIndentifier => match self.chars[self.idx] {
                    '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' => {} // Ignore
                    '"' => match self.cur_token.tag {
                        TokenTag::Doctype(ref mut val) => {
                            val.public_identifier = Some(String::new());
                            self.state = DoctypePublicIdentifierDoubleQuote;
                        }
                        _ => {
                            // TODO: This is fine as a panic for now,
                            //       but we need better error handling
                            panic!("HTML Tokenizer ")
                        }
                    },
                    '\u{0027}' => match self.cur_token.tag {
                        TokenTag::Doctype(ref mut val) => {
                            val.public_identifier = Some(String::new());
                            self.state = DoctypePublicIdentifierSingleQuote;
                        }
                        _ => {
                            // TODO: This is fine as a panic for now,
                            //       but we need better error handling
                            panic!("HTML Tokenizer ")
                        }
                    },
                    '>' => {
                        self.cur_token.flags.force_quirks = true;
                        self.state = Data;
                        emit_token!(self, self.cur_token.clone());
                    }
                    _ => {
                        self.cur_token.flags.force_quirks = true;
                        self.state = BogusDoctype;
                        reconsume!();
                    }
                },
                DoctypePublicIdentifierDoubleQuote => match self.chars[self.idx] {
                    '"' => self.state = AfterDoctypePublicIdentifier,
                    '\u{0000}' => match self.cur_token.tag {
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
                            self.state = DoctypePublicIdentifierDoubleQuote;
                        }
                        _ => {
                            // TODO: This is fine as a panic for now,
                            //       but we need better error handling
                            panic!("HTML Tokenizer ")
                        }
                    },
                    '>' => {
                        self.cur_token.flags.force_quirks = true;
                        self.state = Data;
                        emit_token!(self, self.cur_token.clone());
                    }
                    _ => match self.cur_token.tag {
                        TokenTag::Doctype(ref mut val) => {
                            match val.public_identifier {
                                Some(ref mut s) => {
                                    s.push(self.chars[self.idx]);
                                }
                                None => {
                                    // TODO: Correct with error handling
                                    panic!("In DoctypePublicIdentifierDoubleQuote without public identifier created");
                                }
                            }
                            self.state = DoctypePublicIdentifierDoubleQuote;
                        }
                        _ => {
                            // TODO: This is fine as a panic for now,
                            //       but we need better error handling
                            panic!("HTML Tokenizer ")
                        }
                    },
                },
                DoctypePublicIdentifierSingleQuote => match self.chars[self.idx] {
                    '\u{0027}' => self.state = AfterDoctypePublicIdentifier,
                    '\u{0000}' => match self.cur_token.tag {
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
                            self.state = DoctypePublicIdentifierDoubleQuote;
                        }
                        _ => {
                            // TODO: This is fine as a panic for now,
                            //       but we need better error handling
                            panic!("HTML Tokenizer ")
                        }
                    },
                    '>' => {
                        self.cur_token.flags.force_quirks = true;
                        self.state = Data;
                        emit_token!(self, self.cur_token.clone());
                    }
                    _ => match self.cur_token.tag {
                        TokenTag::Doctype(ref mut val) => {
                            match val.public_identifier {
                                Some(ref mut s) => {
                                    s.push(self.chars[self.idx]);
                                }
                                None => {
                                    // TODO: Correct with error handling
                                    panic!("In DoctypePublicIdentifierSingleQuote without public identifier created");
                                }
                            }
                            self.state = DoctypePublicIdentifierDoubleQuote;
                        }
                        _ => {
                            // TODO: This is fine as a panic for now,
                            //       but we need better error handling
                            panic!("HTML Tokenizer ")
                        }
                    },
                },
                AfterDoctypePublicIdentifier => match self.chars[self.idx] {
                    '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' => {
                        self.state = BetweenDoctypePublicAndSystemIdentifiers;
                    }
                    '>' => {
                        self.state = Data;
                        emit_token!(self, self.cur_token.clone());
                    }
                    '"' => match self.cur_token.tag {
                        TokenTag::Doctype(ref mut val) => {
                            val.system = Some(String::new());
                            self.state = DoctypeSystemIdentifierDoubleQuote;
                        }
                        _ => {
                            // TODO: This is fine as a panic for now,
                            //       but we need better error handling
                            panic!("HTML Tokenizer ")
                        }
                    },
                    '\u{0027}' => match self.cur_token.tag {
                        TokenTag::Doctype(ref mut val) => {
                            val.system = Some(String::new());
                            self.state = DoctypeSystemIdentifierSingleQuote;
                        }
                        _ => {
                            // TODO: This is fine as a panic for now,
                            //       but we need better error handling
                            panic!("HTML Tokenizer ")
                        }
                    },
                    _ => {
                        self.cur_token.flags.force_quirks = true;
                        self.state = BogusDoctype;
                        reconsume!();
                    }
                },
                BetweenDoctypePublicAndSystemIdentifiers => match self.chars[self.idx] {
                    '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' => {}
                    '>' => {
                        self.state = Data;
                        emit_token!(self, self.cur_token.clone());
                    }
                    '"' => match self.cur_token.tag {
                        TokenTag::Doctype(ref mut val) => {
                            val.system = Some(String::new());
                            self.state = DoctypeSystemIdentifierDoubleQuote;
                        }
                        _ => {
                            // TODO: This is fine as a panic for now,
                            //       but we need better error handling
                            panic!("HTML Tokenizer ")
                        }
                    },
                    '\u{0027}' => match self.cur_token.tag {
                        TokenTag::Doctype(ref mut val) => {
                            val.system = Some(String::new());
                            self.state = DoctypeSystemIdentifierSingleQuote;
                        }
                        _ => {
                            // TODO: This is fine as a panic for now,
                            //       but we need better error handling
                            panic!("HTML Tokenizer ")
                        }
                    },
                    _ => {
                        self.cur_token.flags.force_quirks = true;
                        self.state = BogusDoctype;
                        reconsume!();
                    }
                },
                AfterDoctypeSystemKeyword => match self.chars[self.idx] {
                    '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' => {
                        self.state = BeforeDoctypeSystemIdentifier;
                    }
                    '"' => match self.cur_token.tag {
                        TokenTag::Doctype(ref mut val) => {
                            val.system = Some(String::new());
                            self.state = DoctypeSystemIdentifierDoubleQuote;
                        }
                        _ => {
                            // TODO: This is fine as a panic for now,
                            //       but we need better error handling
                            panic!("HTML Tokenizer ")
                        }
                    },
                    '\u{0027}' => match self.cur_token.tag {
                        TokenTag::Doctype(ref mut val) => {
                            val.system = Some(String::new());
                            self.state = DoctypeSystemIdentifierSingleQuote;
                        }
                        _ => {
                            // TODO: This is fine as a panic for now,
                            //       but we need better error handling
                            panic!("HTML Tokenizer ")
                        }
                    },
                    '>' => {
                        self.cur_token.flags.force_quirks = true;
                        self.state = Data;
                        emit_token!(self, self.cur_token.clone());
                    }
                    _ => {
                        self.cur_token.flags.force_quirks = true;
                        self.state = BogusDoctype;
                        reconsume!();
                    }
                },
                BeforeDoctypeSystemIdentifier => match self.chars[self.idx] {
                    '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' => {}
                    '"' => match self.cur_token.tag {
                        TokenTag::Doctype(ref mut val) => {
                            val.system = Some(String::new());
                            self.state = DoctypeSystemIdentifierDoubleQuote;
                        }
                        _ => {
                            // TODO: This is fine as a panic for now,
                            //       but we need better error handling
                            panic!("HTML Tokenizer ")
                        }
                    },
                    '\u{0027}' => match self.cur_token.tag {
                        TokenTag::Doctype(ref mut val) => {
                            val.system = Some(String::new());
                            self.state = DoctypeSystemIdentifierSingleQuote;
                        }
                        _ => {
                            // TODO: This is fine as a panic for now,
                            //       but we need better error handling
                            panic!("HTML Tokenizer ")
                        }
                    },
                    '>' => {
                        self.cur_token.flags.force_quirks = true;
                        self.state = Data;
                        emit_token!(self, self.cur_token.clone());
                    }
                    _ => {
                        self.cur_token.flags.force_quirks = true;
                        self.state = BogusDoctype;
                        reconsume!();
                    }
                },
                DoctypeSystemIdentifierDoubleQuote => match self.chars[self.idx] {
                    '"' => self.state = AfterDoctypeSystemIdentifier,
                    '\u{0000}' => match self.cur_token.tag {
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
                        self.cur_token.flags.force_quirks = true;
                        self.state = Data;
                        emit_token!(self, self.cur_token.clone());
                    }
                    _ => match self.cur_token.tag {
                        TokenTag::Doctype(ref mut val) => {
                            match val.system {
                                Some(ref mut s) => s.push(self.chars[self.idx]),
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
                DoctypeSystemIdentifierSingleQuote => match self.chars[self.idx] {
                    '\u{0027}' => self.state = AfterDoctypeSystemIdentifier,
                    '\u{0000}' => match self.cur_token.tag {
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
                        self.cur_token.flags.force_quirks = true;
                        self.state = Data;
                        emit_token!(self, self.cur_token.clone());
                    }
                    _ => match self.cur_token.tag {
                        TokenTag::Doctype(ref mut val) => {
                            match val.system {
                                Some(ref mut s) => s.push(self.chars[self.idx]),
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
                AfterDoctypeSystemIdentifier => match self.chars[self.idx] {
                    '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' => {}
                    '>' => {
                        self.state = Data;
                        emit_token!(self, self.cur_token.clone());
                    }
                    _ => {
                        self.state = BogusDoctype;
                        reconsume!();
                    }
                },
                BogusDoctype => match self.chars[self.idx] {
                    '>' => {
                        self.state = Data;
                        emit_token!(self, self.cur_token.clone());
                    }
                    _ => {}
                },
                CDataSection => match self.chars[self.idx] {
                    ']' => self.state = CDataSectionBracket,
                    _ => {
                        emit_token!(self, new_character_string(self.chars[self.idx]));
                    }
                },
                CDataSectionBracket => match self.chars[self.idx] {
                    ']' => self.state = CDataSectionEnd,
                    _ => {
                        self.state = CDataSection;
                        emit_and_reconsume!(self, new_character_string(']'));
                    }
                },
                CDataSectionEnd => match self.chars[self.idx] {
                    ']' => {
                        emit_token!(self, new_character_string(']'));
                    }
                    '>' => self.state = Data,
                    _ => {
                        self.state = CDataSection;
                        emit_token_no_return!(self, new_character_string(']'));
                        emit_and_reconsume!(self, new_character_string(']'));
                    }
                },
                CharacterReference => {
                    temp_buf = "&".to_string();
                    match self.chars[self.idx] {
                        '#' => {
                            temp_buf.push(self.chars[self.idx]);
                            self.temp_buffer.push(self.chars[self.idx]);
                            self.state = NumericCharacterReference;
                        }
                        _ => {
                            if self.chars[self.idx].is_alphanumeric() {
                                self.state = NamedCharacterReference;
                                reconsume!();
                            } else {
                                if is_part_of_attribute(self.return_state) {
                                    temp_buf
                                        .chars()
                                        .for_each(|c| self.cur_token.append_to_last_attribute(c));
                                } else {
                                    temp_buf.chars().for_each(|c| {
                                        emit_token_no_return!(self, new_character_string(c));
                                    });
                                }

                                self.state = self.return_state;
                                reconsume!();
                            }
                        }
                    }
                }
                // NamedCharacterReference => {}
                AmbiguousAmpersand => {
                    if self.chars[self.idx].is_alphanumeric() {
                        if is_part_of_attribute(self.return_state) {
                            self.cur_token
                                .append_to_last_attribute(self.chars[self.idx]);
                        } else {
                            emit_token!(self, new_character_string(self.chars[self.idx]));
                        }
                    } else {
                        self.state = self.return_state;
                        reconsume!();
                    }
                }
                NumericCharacterReference => {
                    self.character_reference_code = 0;
                    match self.chars[self.idx] {
                        'x' | 'X' => {
                            temp_buf.push(self.chars[self.idx]);
                            self.temp_buffer.push(self.chars[self.idx]);
                            self.state = HexadecimalCharacterReferenceStart;
                        }
                        _ => {
                            self.state = DecimalCharacterReferenceStart;
                            reconsume!();
                        }
                    }
                }
                HexadecimalCharacterReferenceStart => match self.chars[self.idx] {
                    '0'..='9' | 'A'..='F' | 'a'..='f' => {
                        self.state = HexadecimalCharacterReference;
                        reconsume!();
                    }
                    _ => {
                        if is_part_of_attribute(self.return_state) {
                            temp_buf
                                .chars()
                                .for_each(|c| self.cur_token.append_to_last_attribute(c));
                        } else {
                            temp_buf.chars().for_each(|c| {
                                emit_token_no_return!(self, new_character_string(c));
                            });
                        }
                        self.state = self.return_state;
                        reconsume!();
                    }
                },
                DecimalCharacterReferenceStart => match self.chars[self.idx] {
                    '0'..='9' => {
                        self.state = DecimalCharacterReference;
                        reconsume!();
                    }
                    _ => {
                        if is_part_of_attribute(self.return_state) {
                            temp_buf
                                .chars()
                                .for_each(|c| self.cur_token.append_to_last_attribute(c));
                        } else {
                            temp_buf.chars().for_each(|c| {
                                emit_token_no_return!(self, new_character_string(c));
                            });
                        }
                        self.state = self.return_state;
                        reconsume!();
                    }
                },
                HexadecimalCharacterReference => match self.chars[self.idx] {
                    '0'..='9' | 'A'..='F' | 'a'..='f' => {
                        self.character_reference_code *= 16;
                        self.character_reference_code += self.chars[self.idx].to_digit(16).unwrap();
                    }
                    ';' => self.state = NumericCharacterReferenceEnd,
                    _ => {
                        self.state = NumericCharacterReferenceEnd;
                        reconsume!();
                    }
                },
                DecimalCharacterReference => match self.chars[self.idx] {
                    '0'..='9' => {
                        self.character_reference_code *= 16;
                        self.character_reference_code += self.chars[self.idx].to_digit(10).unwrap();
                    }
                    ';' => self.state = NumericCharacterReferenceEnd,
                    _ => {
                        self.state = NumericCharacterReferenceEnd;
                        reconsume!();
                    }
                },
                NumericCharacterReferenceEnd => {
                    match self.character_reference_code {
                        // Null check
                        0x00 |
                        // Surrogate check
                        0xD800..=0xDBFF | 0xDC00..=0xDFFF | 0x10FFFF.. => {
                            self.character_reference_code = 0xFFFD;
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
                            self.character_reference_code =
                                lookup_character_reference(self.character_reference_code);
                        }
                    };
                    temp_buf = String::new();
                    self.temp_buffer = String::new();
                    // This cast should probably be error checked
                    temp_buf.push(self.character_reference_code as u8 as char);
                    self.temp_buffer
                        .push(self.character_reference_code as u8 as char);
                    if is_part_of_attribute(self.return_state) {
                        temp_buf
                            .chars()
                            .for_each(|c| self.cur_token.append_to_last_attribute(c));
                    } else {
                        temp_buf.chars().for_each(|c| {
                            emit_token!(self, new_character_string(c));
                        });
                    }

                    self.state = self.return_state;
                }
                _ => {
                    log::error!("Unsupported tag found");
                }
            };
            self.idx += 1;
        }
    }

    fn get_eof_token(&mut self) -> HtmlToken {
        // TODO: Some states have specific EoF behaviour
        //       This should eventually check the state
        //       and behave correctly
        match self.state {
            _ => HtmlToken::new(TokenTag::EoF),
        }
    }
}

#[cfg(test)]
mod test {}
