use crate::document::doctree;
use crate::document::doctree::DoctreeNode;
use crate::document::document;
use crate::document::document::Document;
use crate::document::node;
use crate::document::node::NodeType;
use crate::html_elements;
use crate::html_elements::element_structs;
use crate::html_elements::HTMLElement;
use crate::html_elements::HTMLElementType;
use crate::parser::tokenizer;

use tokenizer::TokenTag;

use super::tokenizer::HtmlToken;

use log;

pub enum DocumentParseError {
    UnparsableDocument,
}

pub fn parse_document(data: &str) -> Document {
    let mut parser = Parser::new(data);
    parser.parse_html()
}

#[derive(Default, Clone, Copy, Debug)]
enum InsertionMode {
    #[default]
    Initial,
    BeforeHtml,
    BeforeHead,
    InHead,
    InHeadNoscript,
    AfterHead,
    InBody,
    Text,
    InTable,
    InTableText,
    InCaption,
    InColumnGroup,
    InTableBody,
    InRow,
    InCell,
    InSelect,
    InSelectTable,
    InTemplate,
    AfterBody,
    InFrameset,
    AfterFrameset,
    AfterAfterBody,
    AfterAfterFrameset,
}

struct OpenNodeStack {
    pub stack: Vec<doctree::DoctreeNode>,
}

impl OpenNodeStack {
    pub fn new() -> OpenNodeStack {
        OpenNodeStack { stack: Vec::new() }
    }

    pub fn push(
        &mut self,
        tokenizer: &mut tokenizer::Tokenizer,
        node: doctree::DoctreeNode,
        node_name: String,
    ) {
        self.stack.push(node);
        tokenizer.push_open_tag(node_name);
    }

    pub fn push_self_only(&mut self, node: doctree::DoctreeNode) -> usize {
        self.stack.push(node);

        self.stack.len() - 1
    }

    pub fn pop(&mut self, tokenizer: &mut tokenizer::Tokenizer) -> Option<DoctreeNode> {
        tokenizer.pop_open_tag();
        self.stack.pop()
    }

    pub fn remove_at(&mut self, idx: usize) {
        self.stack.remove(idx);
    }

    pub fn first(&mut self) -> Option<&doctree::DoctreeNode> {
        self.stack.first()
    }

    pub fn last(&mut self) -> Option<&doctree::DoctreeNode> {
        self.stack.last()
    }

    pub fn len(&self) -> usize {
        self.stack.len()
    }

    pub fn at(&self, index: usize) -> Option<&doctree::DoctreeNode> {
        self.stack.get(index)
    }
}

macro_rules! is_entry_element {
    ($doc:expr, $entry:expr, $pattern:pat) => {
        if let Some(entry) = $doc.doctree.get_node($entry) {
            match &entry.node_type {
                NodeType::Element(element) => match element.element_type {
                    $pattern => true,
                    _ => false,
                },
                _ => false,
            }
        } else {
            false
        }
    };
}

fn element_name_matches(doc: &Document, entry: &doctree::DoctreeNode, name: &str) -> bool {
    if let Some(node) = doc.doctree.get_node(entry) {
        if let NodeType::Element(element) = &node.node_type {
            return element.get_name() == name;
        }
    }
    false
}

fn is_element_special(
    doctree: &doctree::Doctree,
    entry: &doctree::DoctreeNode,
    restrict: bool,
) -> bool {
    if let Some(entry) = doctree.get_node(entry) {
        use html_elements::HTMLElementType::*;
        match &entry.node_type {
            NodeType::Element(element) => match element.element_type {
                // Applet(_) | Basefont(_) | Bgsound(_) | Center(_) | Dir(_) |
                // Frame(_) | Frameset(_) | Param(_) | Plaintext(_) | Xmp(_) |
                // MathMls | SVGs
                Area(_) | Article(_) | Aside(_) | Base(_) | Blockquote(_) | Body(_) | Br(_)
                | Button(_) | Caption(_) | Col(_) | Colgroup(_) | Dd(_) | Details(_) | Dl(_)
                | Dt(_) | Fieldset(_) | Figcaption(_) | Figure(_) | Footer(_) | Form(_)
                | Noscript(_) | Object(_) | Ol(_) | Pre(_) | Script(_) | Search(_) | Section(_)
                | Select(_) | Source(_) | Style(_) | Summary(_) | Table(_) | Tbody(_) | Td(_)
                | Template(_) | Textarea(_) | Tfoot(_) | Th(_) | THead(_) | Title(_) | Tr(_)
                | Track(_) | Ul(_) | Wbr(_) => true,
                Address(_) | Div(_) | P(_) => {
                    if restrict {
                        true
                    } else {
                        false
                    }
                }
                _ => false,
            },
            _ => false,
        }
    } else {
        false
    }
}

fn is_token_whitespace(token: &HtmlToken) -> bool {
    token
        .data
        .starts_with(&['\u{0009}', '\u{000A}', '\u{000C}', '\u{000D}', '\u{0020}'])
}

pub struct Parser {
    fragment_parser_ctx: Option<node::Node>,
    tokenizer: tokenizer::Tokenizer,
    insertion_mode: InsertionMode,
    original_insertion_mode: InsertionMode,
    cannot_change_mode: bool,
    scripting: bool,
    speculative_parser: Option<()>,
    open_node_stack: OpenNodeStack,

    head_element: Option<doctree::DoctreeNode>,
    form_element: Option<doctree::DoctreeNode>,
    reconsume_token: Option<HtmlToken>,
    should_exit: bool,

    parser_pause_flag: bool,
    script_nesting_level: u32,

    pending_character_tokens: Vec<HtmlToken>,
}

impl Parser {
    pub fn new(data: &str) -> Parser {
        Parser {
            fragment_parser_ctx: None,
            tokenizer: tokenizer::Tokenizer::init(data),
            insertion_mode: InsertionMode::Initial,
            original_insertion_mode: InsertionMode::Initial,
            cannot_change_mode: false,
            scripting: true,
            speculative_parser: None,
            open_node_stack: OpenNodeStack::new(),

            head_element: None,
            form_element: None,
            reconsume_token: None,
            should_exit: false,

            parser_pause_flag: false,
            script_nesting_level: 0,

            pending_character_tokens: Vec::new(),
        }
    }

    pub fn new_fragment_parser(data: &str, ctx: node::Node) -> Parser {
        Parser {
            fragment_parser_ctx: Some(ctx),
            tokenizer: tokenizer::Tokenizer::init(data),
            insertion_mode: InsertionMode::Initial,
            original_insertion_mode: InsertionMode::Initial,
            cannot_change_mode: false,
            scripting: true,
            speculative_parser: None,
            open_node_stack: OpenNodeStack::new(),

            head_element: None,
            form_element: None,
            reconsume_token: None,
            should_exit: false,

            parser_pause_flag: false,
            script_nesting_level: 0,

            pending_character_tokens: Vec::new(),
        }
    }

    // Utilities functions

    fn create_element_from_token(&self, token: HtmlToken) -> node::Node {
        if self.speculative_parser.is_some() {}
        // TODO: Lookup custom element, for now we skip

        let mut element = HTMLElement::new(token.data);

        token.attributes.into_iter().for_each(|val| {
            element.add_attribute(val.name, val.value);
        });

        node::Node::new(node::NodeType::Unknown("".to_string()))
    }

    fn clear_stack_back_to_table(&mut self, doc: &mut Document) {
        while match self.open_node_stack.last() {
            Some(n) => !is_entry_element!(
                doc,
                n,
                HTMLElementType::Table(_) | HTMLElementType::Template(_) | HTMLElementType::Html(_)
            ),
            None => false,
        } {
            self.open_node_stack.pop(&mut self.tokenizer);
        }
    }

    fn reset_insertion_mode(&mut self, doc: &Document) {
        let mut last = false;
        let mut node_idx = self.open_node_stack.len() - 1;

        loop {
            last = if node_idx == 0 { true } else { false };
            let node = match self.fragment_parser_ctx.as_ref() {
                Some(ctx_node) => ctx_node,
                None => match self.open_node_stack.at(node_idx) {
                    Some(n) => match doc.doctree.get_node(n) {
                        Some(node) => node,
                        None => {
                            // ERROR: This is a confusing, off spec error that we should never hit
                            self.insertion_mode = InsertionMode::InBody;
                            return;
                        }
                    },
                    None => {
                        // ERROR: This is a confusing, off spec error that we should never hit
                        self.insertion_mode = InsertionMode::InBody;
                        return;
                    }
                },
            };

            if let NodeType::Element(e) = &node.node_type {
                use HTMLElementType::*;
                match e.element_type {
                    Select(_) => {
                        let mut ancestor_idx = node_idx;
                        loop {
                            if last || ancestor_idx == 0 {
                                self.insertion_mode = InsertionMode::InTable;
                                return;
                            }
                            match self.open_node_stack.at(ancestor_idx) {
                                Some(n) => {
                                    if is_entry_element!(doc, n, Template(_)) {
                                        self.insertion_mode = InsertionMode::InTable;
                                        return;
                                    }
                                    if is_entry_element!(doc, n, Table(_)) {
                                        self.insertion_mode = InsertionMode::InSelectTable;
                                        return;
                                    }
                                }
                                None => {
                                    // This is an absolutely confusing error
                                }
                            };
                            ancestor_idx -= 1;
                        }
                    }
                    Td(_) | Th(_) => {
                        if !last {
                            self.insertion_mode = InsertionMode::InCell;
                            return;
                        }
                    }
                    Tr(_) => {
                        self.insertion_mode = InsertionMode::InRow;
                        return;
                    }
                    Tbody(_) | THead(_) | Tfoot(_) => {
                        self.insertion_mode = InsertionMode::InTableBody;
                        return;
                    }
                    Caption(_) => {
                        self.insertion_mode = InsertionMode::InCaption;
                        return;
                    }
                    Colgroup(_) => {
                        self.insertion_mode = InsertionMode::InColumnGroup;
                        return;
                    }
                    Table(_) => {
                        self.insertion_mode = InsertionMode::InTable;
                        return;
                    }
                    Template(_) => {
                        // TODO: Reassess when template support is added
                        self.insertion_mode = InsertionMode::InTemplate;
                        return;
                    }
                    Head(_) => {
                        if !last {
                            self.insertion_mode = InsertionMode::InHead;
                            return;
                        }
                    }
                    Body(_) => {
                        self.insertion_mode = InsertionMode::InBody;
                        return;
                    }
                    // TODO: Frameset
                    Html(_) => {
                        if self.head_element.is_none() {
                            self.insertion_mode = InsertionMode::BeforeHead;
                        } else {
                            self.insertion_mode = InsertionMode::AfterHead;
                        }
                        return;
                    }
                    _ => {}
                }
            }

            if last {
                self.insertion_mode = InsertionMode::InBody;
                return;
            }
            node_idx -= 1;
        }
    }

    fn clear_stack_back_to_table_body(&mut self, doc: &Document) {
        while let Some(n) = self.open_node_stack.last() {
            if is_entry_element!(
                doc,
                n,
                HTMLElementType::Tbody(_)
                    | HTMLElementType::Tfoot(_)
                    | HTMLElementType::THead(_)
                    | HTMLElementType::Template(_)
                    | HTMLElementType::Html(_)
            ) {
                return;
            }
            self.open_node_stack.pop(&mut self.tokenizer);
        }
    }

    fn clear_stack_back_to_table_row(&mut self, doc: &Document) {
        while let Some(n) = self.open_node_stack.last() {
            if is_entry_element!(
                doc,
                n,
                HTMLElementType::Tr(_) | HTMLElementType::Template(_) | HTMLElementType::Html(_)
            ) {
                return;
            }
            self.open_node_stack.pop(&mut self.tokenizer);
        }
    }

    fn close_cell(&mut self, doc: &mut Document) {
        // TODO: Generate implied end tags
        self.open_node_stack.pop(&mut self.tokenizer);
        if let Some(node) = self.open_node_stack.last() {
            if !is_entry_element!(doc, node, HTMLElementType::Td(_) | HTMLElementType::Th(_)) {
                // ERROR
            }
        }
        while let Some(node) = self.open_node_stack.pop(&mut self.tokenizer) {
            if !is_entry_element!(doc, &node, HTMLElementType::Td(_) | HTMLElementType::Th(_)) {
                break;
            }
        }
        // TODO: Clear list of active formatting elements back to marker
        self.insertion_mode = InsertionMode::InRow;
    }

    fn is_in_table_scope(&self, doc: &mut Document) -> bool {
        self.open_node_stack.stack.iter().any(|n| {
            is_entry_element!(
                doc,
                n,
                HTMLElementType::Html(_) | HTMLElementType::Table(_) | HTMLElementType::Template(_)
            )
        })
    }

    fn is_element_in_table_scope(&self, doc: &Document, element_name: &str) -> bool {
        let mut idx = self.open_node_stack.len() - 1;
        let mut found = false;

        while let Some(node) = self.open_node_stack.at(idx) {
            if !found {
                if let Some(n) = doc.doctree.get_node(node) {
                    if let NodeType::Element(element) = &n.node_type {
                        found = element.get_name() == element_name;
                    }
                }
            } else if is_entry_element!(
                doc,
                node,
                HTMLElementType::Html(_) | HTMLElementType::Table(_) | HTMLElementType::Template(_)
            ) {
                return true;
            }
        }
        false
    }

    // Node Insertion functions
    fn insert_element_from_token(
        &mut self,
        doctree: &mut doctree::Doctree,
        is_root: bool,
        nostackpush: bool,
        token: HtmlToken,
    ) -> DoctreeNode {
        let node_name = token.data.clone();
        let is_self_closing = token.flags.self_closing;

        let mut element = self.create_element_from_token(token);
        let node = if is_root || self.open_node_stack.len() == 0 {
            doctree.add_root_node(element)
        } else {
            let parent = self.open_node_stack.last();
            element.parent = parent.cloned();

            let n = doctree.add_node(element);
            if let Some(p) = parent {
                if let Some(parent_node) = doctree.get_mut_node(p) {
                    parent_node.add_child(n);
                }
            };

            n
        };

        if !is_self_closing && !nostackpush {
            self.open_node_stack
                .push(&mut self.tokenizer, node, node_name);
        }
        node
    }

    fn insert_element(&mut self, doctree: &mut doctree::Doctree, is_root: bool, e: node::Node) {
        let mut element = e;
        let node = if is_root || self.open_node_stack.len() == 0 {
            doctree.add_root_node(element)
        } else {
            let parent = self.open_node_stack.last();
            element.parent = parent.cloned();

            let n = doctree.add_node(element);
            if let Some(p) = parent {
                if let Some(parent_node) = doctree.get_mut_node(p) {
                    parent_node.add_child(n);
                }
            }

            n
        };

        self.open_node_stack.push_self_only(node);
    }

    fn insert_comment_token(&mut self, doc: &mut Document, is_root: bool, str: &str) {
        if is_root || self.open_node_stack.len() == 0 {
            doc.doctree
                .add_root_node(node::Node::new(node::NodeType::Comment(str.to_string())));
            return;
        }

        let parent = self.open_node_stack.last();
        let node = doc
            .doctree
            .add_node(node::Node::new(node::NodeType::Comment(str.to_string())));

        if let Some(idx) = parent {
            if let Some(parent) = doc.doctree.get_mut_node(idx) {
                parent.add_child(node);
                doc.doctree
                    .get_mut_node(&node)
                    .unwrap()
                    .add_parent(Some(idx.clone()));
            }
        }
    }

    fn insert_character_token(&mut self, doc: &mut Document, token: HtmlToken) {
        match doc.doctree.get_last_node() {
            Some(n) => match n.node_type {
                node::NodeType::Text(ref mut text) => {
                    text.push_str(token.data.as_str());
                    return;
                }
                _ => {}
            },
            None => {}
        }

        let parent = self.open_node_stack.last();
        let mut node = node::Node::new(node::NodeType::Text(token.data));
        node.parent = parent.cloned();
        let n = doc.doctree.add_node(node);

        match parent {
            Some(p) => match doc.doctree.get_mut_node(p) {
                Some(parent) => parent.add_child(n),
                None => {}
            },
            None => {}
        }
    }

    fn generic_raw_text_parsing(&mut self, doc: &mut Document, token: HtmlToken) {
        self.insert_element_from_token(&mut doc.doctree, false, false, token);
        self.tokenizer.set_state(tokenizer::FsmState::RawText);
        self.original_insertion_mode = self.insertion_mode;
        self.insertion_mode = InsertionMode::Text;
    }

    fn generic_rcdata_parsing(&mut self, doc: &mut Document, token: HtmlToken) {
        self.insert_element_from_token(&mut doc.doctree, false, false, token);
        self.tokenizer.set_state(tokenizer::FsmState::RCData);
        self.original_insertion_mode = self.insertion_mode;
        self.insertion_mode = InsertionMode::Text;
    }

    // Insertion mode functions
    fn parse_initial(&mut self, doc: &mut Document, token: HtmlToken) {
        let mut anything_else = true;
        match token.tag {
            TokenTag::Character => {
                match token.data.as_str() {
                    "\u{0009}" | "\u{000A}" | "\u{000C}" | "\u{000D}" | "\u{0020}" => {
                        anything_else = false;
                    }
                    _ => {}
                };
            }
            TokenTag::Comment => {
                self.insert_comment_token(doc, true, token.data.as_str());
            }
            TokenTag::Doctype(ref data) => {
                doc.doctree
                    .add_root_node(node::Node::new(node::NodeType::DocumentType(
                        node::DocumentType {
                            name: token.data.clone(),
                            public_id: data.public_identifier.clone().unwrap_or(String::new()),
                            system_id: data.system.clone().unwrap_or(String::new()),
                        },
                    )));
                // TODO: Check if the document should be set to quirks mode
                // TODO: Check if limited quirks mode
                self.insertion_mode = InsertionMode::BeforeHtml;
                anything_else = false;
            }
            _ => {}
        };

        if anything_else {
            if !self.cannot_change_mode {
                doc.set_quirks_mode(document::QuirksMode::Quirks);
            }
            self.insertion_mode = InsertionMode::BeforeHtml;
            self.reconsume_token = Some(token);
        }
    }

    fn parse_before_html(&mut self, doc: &mut Document, token: HtmlToken) {
        match token.tag {
            TokenTag::Doctype(_) => {
                return;
            }
            TokenTag::Comment => {
                self.insert_comment_token(doc, true, token.data.as_str());
                return;
            }
            TokenTag::Character => {
                match token.data.as_str() {
                    "\u{0009}" | "\u{000A}" | "\u{000C}" | "\u{000D}" | "\u{0020}" => {
                        return;
                    }
                    _ => {}
                };
            }
            TokenTag::StartTag => {
                if token.data == "html" {
                    // Create a new token and push it onto the stack of open nodes
                    self.insert_element_from_token(&mut doc.doctree, true, false, token);
                    return;
                }
            }
            TokenTag::EndTag => match token.data.as_str() {
                "head" | "body" | "html" | "br" => {}
                _ => return,
            },
            _ => {}
        }

        // Anything else case
        self.open_node_stack.push(
            &mut self.tokenizer,
            doc.doctree
                .add_root_node(node::Node::new(node::NodeType::Element(
                    HTMLElement::from_element_type(HTMLElementType::Html(
                        element_structs::html::Html::default(),
                    )),
                ))),
            token.data.clone(),
        );

        self.reconsume_token = Some(token);
        self.insertion_mode = InsertionMode::BeforeHead;
    }

    fn parse_before_head(&mut self, doc: &mut Document, token: HtmlToken) {
        match token.tag {
            TokenTag::Character => {
                match token.data.as_str() {
                    "\u{0009}" | "\u{000A}" | "\u{000C}" | "\u{000D}" | "\u{0020}" => return,
                    _ => {}
                };
            }
            TokenTag::Comment => {
                self.insert_comment_token(doc, false, token.data.as_str());
                return;
            }
            TokenTag::Doctype(_) => {
                return;
            }
            TokenTag::StartTag => match token.data.as_str() {
                "html" => {
                    self.parse_in_body(doc, token);
                    return;
                }
                "head" => {
                    self.insert_element_from_token(&mut doc.doctree, false, false, token);
                    self.insertion_mode = InsertionMode::InHead;
                    return;
                }
                _ => {}
            },
            TokenTag::EndTag => match token.data.as_str() {
                "head" | "body" | "html" | "br" => {}
                _ => {
                    return;
                }
            },
            _ => {}
        };
        // Anything else
        let mut n = node::Node::new(node::NodeType::Element(
            html_elements::HTMLElement::from_element_type(HTMLElementType::Head(
                html_elements::element_structs::head::Head::default(),
            )),
        ));
        n.add_parent(self.open_node_stack.last().cloned());
        let node = doc.doctree.add_node(n);
        self.head_element = Some(node);
        match self.open_node_stack.last() {
            Some(p) => match doc.doctree.get_mut_node(p) {
                Some(parent) => parent.add_child(node),
                None => {}
            },
            None => {}
        };

        self.reconsume_token = Some(token);
        self.insertion_mode = InsertionMode::InHead;
    }

    fn parse_in_head(&mut self, doc: &mut Document, token: HtmlToken) {
        match token.tag {
            TokenTag::Character => {
                match token.data.as_str() {
                    "\u{0009}" | "\u{000A}" | "\u{000C}" | "\u{000D}" | "\u{0020}" => {
                        self.insert_character_token(doc, token);
                        return;
                    }
                    _ => {}
                };
            }
            TokenTag::Comment => {
                self.insert_comment_token(doc, false, token.data.as_str());
                return;
            }
            TokenTag::Doctype(_) => {
                return;
            }
            TokenTag::StartTag => {
                match token.data.as_str() {
                    "html" => {
                        self.parse_in_body(doc, token);
                        return;
                    }
                    "base" | "basefont" | "bgsound" | "link" => {
                        self.insert_element_from_token(&mut doc.doctree, false, true, token);
                        return;
                    }
                    "meta" => {
                        self.insert_element_from_token(&mut doc.doctree, false, true, token);
                        return;
                    }
                    "title" => {
                        self.insert_element_from_token(&mut doc.doctree, false, false, token);
                        self.insertion_mode = InsertionMode::Text;
                        return;
                    }
                    "noscript" => {
                        if self.scripting {
                            self.generic_raw_text_parsing(doc, token);
                        } else {
                            self.insert_element_from_token(&mut doc.doctree, false, false, token);
                            self.insertion_mode = InsertionMode::InHeadNoscript;
                        }
                        return;
                    }
                    "noframes" | "style" => {
                        self.generic_raw_text_parsing(doc, token);
                        return;
                    }
                    "script" => {
                        // TODO: Implement script parsing
                        return;
                    }
                    "template" => {
                        // TODO: Implement templating
                        return;
                    }
                    "head" => {
                        // Error, ignore
                        return;
                    }
                    _ => {}
                }
            }
            TokenTag::EndTag => {
                match token.data.as_str() {
                    "head" => {
                        self.open_node_stack.pop(&mut self.tokenizer);
                        self.insertion_mode = InsertionMode::AfterHead;
                        return;
                    }
                    "body" | "html" | "br" => {
                        // Anything else
                    }
                    "template" => {
                        // TODO: Implement template
                        return;
                    }
                    _ => {
                        // Error: Ignore
                        return;
                    }
                }
            }
            _ => {}
        };

        // Anything else
        self.open_node_stack.pop(&mut self.tokenizer);
        self.insertion_mode = InsertionMode::AfterHead;
        self.reconsume_token = Some(token);
    }

    fn parse_in_head_noscript(&mut self, doc: &mut Document, token: HtmlToken) {
        match token.tag {
            TokenTag::Doctype(_) => {
                // ERROR: Ignore
                return;
            }
            TokenTag::Comment => {
                self.insert_comment_token(doc, false, token.data.as_str());
                return;
            }
            TokenTag::Character => match token.data.as_str() {
                "\u{0009}" | "\u{000A}" | "\u{000C}" | "\u{000D}" | "\u{0020}" => {
                    return;
                }
                _ => {}
            },
            TokenTag::StartTag => match token.data.as_str() {
                "html" => {
                    self.parse_in_body(doc, token);
                    return;
                }
                "basefont" | "bgsound" | "link" | "meta" | "noframes" | "style" => {
                    self.insert_element_from_token(&mut doc.doctree, false, true, token);
                    return;
                }
                "head" | "noscript" => {
                    // Error: Ignore
                    return;
                }
                _ => {}
            },
            TokenTag::EndTag => match token.data.as_str() {
                "noscript" => {
                    self.open_node_stack.pop(&mut self.tokenizer);
                    self.insertion_mode = InsertionMode::InHead;
                    return;
                }
                "br" => {}
                _ => {
                    // ERROR: Ignore
                    return;
                }
            },
            _ => {}
        };

        // ERROR
        self.open_node_stack.pop(&mut self.tokenizer);
        self.insertion_mode = InsertionMode::InHead;
        self.reconsume_token = Some(token);
    }

    fn parse_after_head(&mut self, doc: &mut Document, token: HtmlToken) {
        match token.tag {
            TokenTag::Doctype(_) => {
                // ERROR: Ignore
                return;
            }
            TokenTag::Comment => {
                self.insert_comment_token(doc, false, token.data.as_str());
                return;
            }
            TokenTag::Character => match token.data.as_str() {
                "\u{0009}" | "\u{000A}" | "\u{000C}" | "\u{000D}" | "\u{0020}" => {
                    self.insert_character_token(doc, token);
                    return;
                }
                _ => {}
            },
            TokenTag::StartTag => {
                match token.data.as_str() {
                    "html" => {
                        self.parse_in_body(doc, token);
                        return;
                    }
                    "body" => {
                        self.insert_element_from_token(&mut doc.doctree, false, false, token);
                        // TODO: Set frameset-ok flag to not ok
                        self.insertion_mode = InsertionMode::InBody;
                        return;
                    }
                    "frameset" => {
                        self.insert_element_from_token(&mut doc.doctree, false, false, token);
                        self.insertion_mode = InsertionMode::InFrameset;
                        return;
                    }
                    "base" | "basefont" | "bgsound" | "link" | "meta" | "noframes" | "script"
                    | "style" | "template" | "title" => {
                        // ERROR: Parse error
                        if self.head_element.is_none() {
                            return;
                        }

                        let head_idx = self
                            .open_node_stack
                            .push_self_only(self.head_element.unwrap());
                        self.parse_in_head(doc, token);
                        self.open_node_stack.remove_at(head_idx);
                        return;
                    }
                    "head" => {
                        // Error: Parse error
                        return;
                    }
                    _ => {}
                }
            }
            TokenTag::EndTag => {
                match token.data.as_str() {
                    "template" => {
                        // TODO: Implement templates
                        return;
                    }
                    "body" | "html" | "br" => {}
                    _ => {
                        // ERROR: Parse error
                        return;
                    }
                };
            }
            _ => {}
        };

        // Anything else
        let mut n = node::Node::new(node::NodeType::Element(
            html_elements::HTMLElement::from_element_type(HTMLElementType::Body(
                html_elements::element_structs::body::Body::default(),
            )),
        ));
        n.add_parent(self.open_node_stack.last().cloned());
        let node = doc.doctree.add_node(n);
        if let Some(p) = self.open_node_stack.last() {
            if let Some(parent) = doc.doctree.get_mut_node(p) {
                parent.add_child(node);
            }
        }

        self.insertion_mode = InsertionMode::InBody;
        self.reconsume_token = Some(token);
    }

    fn parse_in_body(&mut self, doc: &mut Document, token: HtmlToken) {
        match token.tag {
            TokenTag::Character => {
                match token.data.as_str() {
                    "\u{0000}" => {
                        // ERROR: Parse error
                        return;
                    }
                    "\u{0009}" | "\u{000A}" | "\u{000C}" | "\u{000D}" | "\u{0020}" => {
                        // TODO: Reconstruct the active formatting element
                        self.insert_character_token(doc, token);
                        return;
                    }
                    _ => {
                        // TODO: Reconstruct the active formatting element
                        self.insert_character_token(doc, token);
                        // TODO: Set the frameset ok flag to not okay
                        return;
                    }
                }
            }
            TokenTag::Comment => {
                self.insert_comment_token(doc, false, token.data.as_str());
                return;
            }
            TokenTag::Doctype(_) => {
                // ERROR: Parse error
                return;
            }
            TokenTag::StartTag => match token.data.as_str() {
                "html" => {
                    // ERROR: Parse error
                    // TODO: Check if there's a template in the stack of open nodes
                    if false {
                    } else if let Some(r) = self.open_node_stack.first() {
                        if let Some(html) = doc.doctree.get_mut_node(r) {
                            token.attributes.into_iter().for_each(|val| {
                                // TODO: This should only affect attributes that don't already exist
                                html.add_attribute(val.name, val.value);
                            });
                        }
                    }
                    return;
                }
                "base" | "basefont" | "bgsound" | "link" | "meta" | "noframes" | "script"
                | "style" | "template" | "title" => {
                    self.parse_in_head(doc, token);
                    return;
                }
                "body" => {
                    // ERROR: Parse error
                    // TODO: This also applies if template is on the stack
                    if self.open_node_stack.len() <= 1
                        || if let Some(e) = self.open_node_stack.at(1) {
                            !is_entry_element!(doc, e, html_elements::HTMLElementType::Body(_))
                        } else {
                            true
                        }
                    {
                        // Ignore
                    } else {
                        // TODO: Set frameset to not okay
                        if let Some(r) = self.open_node_stack.at(1) {
                            if let Some(body) = doc.doctree.get_mut_node(r) {
                                token.attributes.into_iter().for_each(|val| {
                                    // TODO: This should only affect attributes that don't already exist
                                    body.add_attribute(val.name, val.value);
                                });
                            }
                        }
                    }
                    return;
                }
                "frameset" => {
                    // ERROR: Parse error
                    // TODO: This also applies if template is on the stack
                    if self.open_node_stack.len() <= 1
                        || if let Some(e) = self.open_node_stack.at(1) {
                            !is_entry_element!(doc, e, html_elements::HTMLElementType::Body(_))
                        } else {
                            true
                        }
                    {
                        // Ignore
                    } else {
                        // TODO: Set frameset to not okay
                        if let Some(r) = self.open_node_stack.at(1) {
                            if !is_entry_element!(doc, r, html_elements::HTMLElementType::Body(_)) {
                                return;
                            }
                            if let Some(p) = self.open_node_stack.last() {
                                if let Some(parent) = doc.doctree.get_mut_node(p) {
                                    if let Some(c) = self.open_node_stack.at(1) {
                                        parent.remove_child(c);
                                    }
                                }
                            }

                            while self.open_node_stack.len() > 1 {
                                self.open_node_stack.pop(&mut self.tokenizer);
                            }
                        }
                        self.insertion_mode = InsertionMode::InFrameset;
                    }
                    return;
                }
                "address" | "article" | "aside" | "blockquote" | "center" | "details"
                | "dialog" | "dir" | "div" | "dl" | "fieldset" | "figcaption" | "figure"
                | "footer" | "header" | "hgroup" | "main" | "menu" | "nav" | "ol" | "p"
                | "search" | "section" | "summary" | "ul" => {
                    // TODO: If the stack has a p element in button scope, close it
                    self.insert_element_from_token(&mut doc.doctree, false, false, token);
                    return;
                }
                "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => {
                    if let Some(cur_node) = self.open_node_stack.last() {
                        if let Some(n) = doc.doctree.get_node(cur_node) {
                            if let NodeType::Element(e) = &n.node_type {
                                if e.get_name().to_lowercase() == token.data {
                                    // ERROR: Parse error
                                    self.open_node_stack.pop(&mut self.tokenizer);
                                }
                            }
                        }
                    }

                    self.insert_element_from_token(&mut doc.doctree, false, false, token);
                    return;
                }
                "pre" | "listing" => {
                    // TODO: If the stack has a p element in button scope, close it
                    self.insert_element_from_token(&mut doc.doctree, false, false, token);
                    // TODO: Implement system for handling the next token
                    // TODO: Set frameset ok flag to not okay
                    return;
                }
                "form" => {
                    if self.form_element.is_some() {
                        // ERROR: Parse error
                        return;
                    }
                    // TODO: Check if there is a template on the open stack
                    // TODO: If the stack has a p element in button scope, close it
                    let node =
                        self.insert_element_from_token(&mut doc.doctree, false, false, token);

                    if true {
                        // TODO: Check if there is a template on the open stack
                        self.form_element = Some(node);
                    }
                    return;
                }
                "li" => {
                    // TODO: Set frameset ok flag to not okay
                    let mut node_idx = self.open_node_stack.len() - 1;

                    while let Some(node) = self.open_node_stack.at(node_idx) {
                        if is_entry_element!(doc, node, html_elements::HTMLElementType::Li(_)) {
                            while self.open_node_stack.len() > node_idx {
                                self.open_node_stack.pop(&mut self.tokenizer);
                            }
                            break;
                        }
                        if is_element_special(&doc.doctree, node, true) {
                            break;
                        }

                        if node_idx == 0 {
                            break;
                        }
                        node_idx -= 1;
                    }

                    // TODO: If the stack has a p element in button scope, close it
                    self.insert_element_from_token(&mut doc.doctree, false, false, token);
                    return;
                }
                "dd" | "dt" => {
                    // TODO: Set the frameset ok flag to not okay
                    let mut node_idx = self.open_node_stack.len() - 1;

                    while let Some(node) = self.open_node_stack.at(node_idx) {
                        if is_entry_element!(
                            doc,
                            node,
                            html_elements::HTMLElementType::Dd(_)
                                | html_elements::HTMLElementType::Dt(_)
                        ) {
                            while self.open_node_stack.len() > node_idx {
                                self.open_node_stack.pop(&mut self.tokenizer);
                            }
                            break;
                        }

                        if is_element_special(&doc.doctree, node, true) {
                            break;
                        }

                        if node_idx == 0 {
                            break;
                        }
                        node_idx -= 1;
                    }

                    // TODO: If the stack has a p element in button scope, close it
                    self.insert_element_from_token(&mut doc.doctree, false, false, token);
                    return;
                }
                "plaintext" => {
                    // TODO: If the stack has a p element in button scope, close it
                    self.insert_element_from_token(&mut doc.doctree, false, false, token);
                    self.tokenizer.set_state(tokenizer::FsmState::Plaintext);
                    return;
                }
                "button" => {
                    if self.open_node_stack.stack.clone().into_iter().any(|i| {
                        is_entry_element!(doc, &i, html_elements::HTMLElementType::Button(_))
                    }) {
                        // Parse Error
                        loop {
                            // NOTE: This would be much nicer as a while let ... && is_entry
                            //       That's currently unstable so we do this instead
                            let entry = match self.open_node_stack.last() {
                                Some(val) => val,
                                None => break,
                            };

                            if is_entry_element!(
                                doc,
                                entry,
                                html_elements::HTMLElementType::Button(_)
                            ) {
                                self.open_node_stack.pop(&mut self.tokenizer);
                                break;
                            }
                            self.open_node_stack.pop(&mut self.tokenizer);
                        }
                    }

                    // TODO: Reconstruct active formatting elements
                    self.insert_element_from_token(&mut doc.doctree, false, false, token);
                    // TODO: Set frameset ok flag to not okay
                    return;
                }
                "a" => {
                    // TODO: Add list of active formatting elements and check for a elements
                    // TODO: Reconstruct list of active formatting
                    self.insert_element_from_token(&mut doc.doctree, false, false, token);
                    // TODO: push onto the list of active formatting elements
                    return;
                }
                "b" | "big" | "code" | "em" | "font" | "i" | "s" | "small" | "strike"
                | "strong" | "tt" | "u" => {
                    // TODO: Reconstruct the active formatting elements
                    self.insert_element_from_token(&mut doc.doctree, false, false, token);
                    // TODO: Push onto the list of active formatting
                    return;
                }
                // "nobr" => {}
                // "applet" | "marquee" | "object" => {}
                "table" => {
                    // TODO: check if the document is set to quirks mode
                    self.insert_element_from_token(&mut doc.doctree, false, false, token);
                    // TODO: Set frameset-ok flag to not okay
                    self.insertion_mode = InsertionMode::InTable;
                    return;
                }
                "area" | "br" | "embed" | "img" | "image" | "keygen" | "wbr" => {
                    // TODO: Reconstruct the active formatting element
                    self.insert_element_from_token(&mut doc.doctree, false, true, token);
                    // TODO: Set the frameset-ok flag to not ok
                    return;
                }
                "input" => {
                    // TODO: Reconstruct the active formatting element
                    self.insert_element_from_token(&mut doc.doctree, false, true, token.clone());
                    if token
                        .attributes
                        .into_iter()
                        .any(|attr| attr.name == "type" && attr.value.to_lowercase() == "hidden")
                    {
                        // TODO: Set frameset not ok
                    }
                    return;
                }
                "param" | "source" | "track" => {
                    self.insert_element_from_token(&mut doc.doctree, false, true, token);
                    return;
                }
                "hr" => {
                    // TODO: If the stack has a p element in button scope, close it
                    self.insert_element_from_token(&mut doc.doctree, false, true, token);
                    // TODO: Set frameset not okay
                    return;
                }
                "textarea" => {
                    self.insert_element_from_token(&mut doc.doctree, false, false, token);
                    // TODO: Check if the next token is a linefeed
                    self.tokenizer.set_state(tokenizer::FsmState::RCData);
                    self.original_insertion_mode = self.insertion_mode;
                    // TODO: Set frameset-ok not ok
                    self.insertion_mode = InsertionMode::Text;
                    return;
                }
                // "xmp" => {}
                "iframe" => {
                    // TODO: Set frameset-ok to not ok
                    self.generic_raw_text_parsing(doc, token);
                    return;
                }
                "noembed" => {
                    self.generic_raw_text_parsing(doc, token);
                    return;
                }
                "noscript" => {
                    if self.scripting {
                        self.generic_raw_text_parsing(doc, token);
                        return;
                    }
                }
                "select" => {
                    // TODO: Reconstruct active formatting
                    self.insert_element_from_token(&mut doc.doctree, false, false, token);
                    // TODO: Set frameset no  ok
                    self.insertion_mode = if matches!(
                        self.insertion_mode,
                        InsertionMode::InTable
                            | InsertionMode::InCaption
                            | InsertionMode::InTableBody
                            | InsertionMode::InRow
                            | InsertionMode::InCell
                    ) {
                        InsertionMode::InSelectTable
                    } else {
                        InsertionMode::InSelect
                    };
                    return;
                }
                "optgroup" | "option" => {
                    if let Some(item) = self.open_node_stack.last() {
                        if is_entry_element!(doc, item, html_elements::HTMLElementType::Option(_)) {
                            self.open_node_stack.pop(&mut self.tokenizer);
                        }
                    }
                    // TODO: Reconstruct active formatting
                    self.insert_element_from_token(&mut doc.doctree, false, false, token);
                    return;
                }
                "rb" | "rtc" => {
                    // TODO
                }
                "rp" | "rt" => {
                    // TODO
                }
                "math" => {
                    // TODO: Implement MathML
                }
                "svg" => {
                    // TODO: Reconstruct active formatting
                    // TODO: Adjust svg attributes
                    // TODO: Adjust foreign attributes
                    // TODO: Insert a foreign element for the token
                }
                "caption" | "col" | "colgroup" | "frame" | "head" | "tbody" | "td" | "tfoot"
                | "th" | "thead" | "tr" => {
                    // ERROR: Parse error
                    return;
                }
                _ => {
                    // TODO: Reconstruct the active formatting elements
                    self.insert_element_from_token(&mut doc.doctree, false, false, token);
                }
            },
            TokenTag::EoF => {
                // TODO: Check if the stack of template insertion modes is not empty
                // TODO: Check the stack of open elements non-exempt nodes for parse errors
                self.should_exit = true;
                return;
            }
            TokenTag::EndTag => {
                match token.data.as_str() {
                    "template" => {
                        self.parse_in_head(doc, token);
                        return;
                    }
                    "body" => {
                        // TODO: Check if the stack of template insertion modes is not empty
                        // TODO: Check the stack of open elements non-exempt nodes for parse errors
                        self.insertion_mode = InsertionMode::AfterBody;
                        return;
                    }
                    "html" => {
                        // Check if there's a body element in the open stack
                        let mut found = false;
                        for item in self.open_node_stack.stack.iter() {
                            if is_entry_element!(doc, item, html_elements::HTMLElementType::Body(_))
                            {
                                found = true;
                                break;
                            }
                        }
                        if !found {
                            // ERROR: Parse error
                            return;
                        }

                        // TODO: Check if there's invalid nodes in the open stack to determine error status
                        self.insertion_mode = InsertionMode::AfterBody;
                        self.reconsume_token = Some(token);
                    }
                    "address" | "article" | "aside" | "blockquote" | "button" | "center"
                    | "details" | "dialog" | "dir" | "div" | "dl" | "fieldset" | "figcaption"
                    | "figure" | "footer" | "header" | "hgroup" | "listing" | "main" | "menu"
                    | "nav" | "ol" | "pre" | "search" | "section" | "summary" | "ul" => {
                        if let Some(entry) = self.open_node_stack.last() {
                            if let Some(node) = doc.doctree.get_node(entry) {
                                if let NodeType::Element(e) = &node.node_type {
                                    if e.get_name().to_lowercase() != token.data {
                                        // Parse error
                                    }
                                }
                            }
                        }
                        while let Some(entry) = self.open_node_stack.last() {
                            if let Some(node) = doc.doctree.get_node(entry) {
                                if let NodeType::Element(e) = &node.node_type {
                                    if e.get_name().to_lowercase() == token.data {
                                        self.open_node_stack.pop(&mut self.tokenizer);
                                        return;
                                    }
                                }
                            }
                            self.open_node_stack.pop(&mut self.tokenizer);
                        }
                        // If we get here, something's very wrong
                        return;
                    }
                    "form" => {
                        // TODO: Check if there's a template
                        let n = self.form_element;
                        if n != self.open_node_stack.last().cloned() {
                            // Error
                        }

                        self.form_element = None;
                        if let Some(_) = n.clone() {
                            if self.open_node_stack.stack.clone().into_iter().any(|i| {
                                is_entry_element!(
                                    doc,
                                    &i,
                                    html_elements::HTMLElementType::Button(_)
                                )
                            }) {
                                self.open_node_stack.pop(&mut self.tokenizer);
                            } else {
                                // Parse error
                            }
                        } else {
                            // Parse error
                        }
                        return;
                    }
                    "p" => {
                        if !self.open_node_stack.stack.clone().into_iter().any(|i| {
                            is_entry_element!(doc, &i, html_elements::HTMLElementType::P(_))
                        }) {
                            // Parse Error
                            let node = node::Node::new(NodeType::Element(
                                html_elements::HTMLElement::from_element_type(HTMLElementType::P(
                                    html_elements::element_structs::p::P::default(),
                                )),
                            ));
                            self.insert_element(&mut doc.doctree, false, node);
                        }
                        self.open_node_stack.pop(&mut self.tokenizer);
                        return;
                    }
                    "li" => {
                        if !self.open_node_stack.stack.clone().into_iter().any(|i| {
                            is_entry_element!(doc, &i, html_elements::HTMLElementType::Li(_))
                        }) {
                            // ERROR: Parse Error
                            return;
                        }
                        loop {
                            // NOTE: This would be much nicer as a while let ... && is_entry
                            //       That's currently unstable so we do this instead
                            let entry = match self.open_node_stack.last() {
                                Some(val) => val,
                                None => break,
                            };

                            if is_entry_element!(doc, entry, html_elements::HTMLElementType::Li(_))
                            {
                                self.open_node_stack.pop(&mut self.tokenizer);
                                break;
                            }
                            self.open_node_stack.pop(&mut self.tokenizer);
                        }
                    }
                    "dd" | "dt" => {
                        if !self.open_node_stack.stack.clone().into_iter().any(|i| {
                            is_entry_element!(
                                doc,
                                &i,
                                html_elements::HTMLElementType::Dd(_)
                                    | html_elements::HTMLElementType::Dt(_)
                            )
                        }) {
                            // ERROR: Parse error
                            return;
                        }

                        loop {
                            // NOTE: This would be much nicer as a while let ... && is_entry
                            //       That's currently unstable so we do this instead
                            let entry = match self.open_node_stack.last() {
                                Some(val) => val,
                                None => break,
                            };

                            if is_entry_element!(
                                doc,
                                entry,
                                html_elements::HTMLElementType::Dd(_)
                                    | html_elements::HTMLElementType::Dt(_)
                            ) {
                                self.open_node_stack.pop(&mut self.tokenizer);
                                break;
                            }
                            self.open_node_stack.pop(&mut self.tokenizer);
                        }
                    }
                    "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => {
                        if !self.open_node_stack.stack.clone().into_iter().any(|node| {
                            if let Some(n) = doc.doctree.get_node(&node) {
                                if let NodeType::Element(e) = &n.node_type {
                                    if e.get_name().to_lowercase() == token.data {
                                        true
                                    } else {
                                        false
                                    }
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        }) {
                            // ERROR: Parse error
                            return;
                        }

                        while let Some(node) = self.open_node_stack.last() {
                            if let Some(n) = doc.doctree.get_node(&node) {
                                if let NodeType::Element(e) = &n.node_type {
                                    if e.get_name().to_lowercase().starts_with("h") {
                                        self.open_node_stack.pop(&mut self.tokenizer);
                                        return;
                                    }
                                }
                            }

                            self.open_node_stack.pop(&mut self.tokenizer);
                        }
                        return;
                    }
                    "a" | "b" | "big" | "code" | "em" | "font" | "i" | "s" | "small" | "strike"
                    | "strong" | "tt" | "u" => {
                        // TODO: Run the adoption agency algorithm
                        return;
                    }
                    // "applet" | "marquee" | "object" => {}
                    "br" => {
                        // Error: Parse error
                        // TODO: Reconstruct the active formatting element
                        self.insert_element_from_token(&mut doc.doctree, false, true, token);
                        // TODO: Set the frameset-ok flag to not ok
                        return;
                    }
                    _ => {
                        if token.data == "" {
                            return;
                        }

                        if !self.open_node_stack.stack.clone().into_iter().any(|i| {
                            doc.doctree.get_element_name(&i).unwrap_or("".to_string()) == token.data
                        }) {
                            // Error
                            return;
                        }
                        while let Some(n) = self.open_node_stack.last() {
                            if doc.doctree.get_element_name(n).unwrap_or("".to_string())
                                == token.data
                            {
                                self.open_node_stack.pop(&mut self.tokenizer);
                                return;
                            }
                            if is_element_special(&doc.doctree, n, false) {
                                // Error
                                return;
                            }
                            self.open_node_stack.pop(&mut self.tokenizer);
                        }
                    }
                };
            }
        }
    }

    fn parse_text(&mut self, doc: &mut Document, token: HtmlToken) {
        match token.tag {
            TokenTag::Character => {
                self.insert_character_token(doc, token);
            }
            TokenTag::EoF => {
                // ERROR
                if let Some(n) = self.open_node_stack.last() {
                    if let Some(node) = doc.doctree.get_mut_node(n) {
                        if let NodeType::Element(element) = &mut node.node_type {
                            if let HTMLElementType::Script(s) = &mut element.element_type {
                                s.enable_already_started();
                            }
                        }
                    }
                }
                self.open_node_stack.pop(&mut self.tokenizer);
                self.insertion_mode = self.original_insertion_mode;
                self.reconsume_token = Some(token);
                return;
            }
            TokenTag::EndTag => {
                match token.data.as_str() {
                    "script" => {
                        // TODO: Add javascript execution context stack
                        // if self.speculative_parser.is_none() && self.js_execution_ctx.is_empty() {
                        //     Perform microtask checkpoint
                        // }

                        let script = self.open_node_stack.pop(&mut self.tokenizer);
                        self.insertion_mode = self.original_insertion_mode;
                        // TODO: let the old insertion point have the same value as the current insertion point
                        //       let the insertion point be just before the next input character
                        self.script_nesting_level += 1;
                        if self.speculative_parser.is_none() {
                            // TODO: Prepare the script element
                        }

                        self.script_nesting_level -= 1;
                        if self.script_nesting_level == 0 {
                            self.parser_pause_flag = false;
                        }
                        // TODO: Set the insertion point to the old insertion point
                        if doc.get_pending_parse_blocking().is_none() {
                            if self.script_nesting_level != 0 {
                                self.parser_pause_flag = true;
                                // TODO: Abort any nested invokations of tokenizer
                            } else {
                                // TODO: Finish this as I add JS support
                            }
                        }
                        return;
                    }
                    _ => {
                        self.open_node_stack.pop(&mut self.tokenizer);
                        self.insertion_mode = self.original_insertion_mode;
                        return;
                    }
                }
            }
            _ => {}
        }
    }

    fn parse_in_table(&mut self, doc: &mut Document, token: HtmlToken) {
        match token.tag {
            TokenTag::Character => {
                if let Some(n) = self.open_node_stack.last() {
                    if is_entry_element!(
                        doc,
                        n,
                        HTMLElementType::Table(_)
                            | HTMLElementType::Tbody(_)
                            | HTMLElementType::Template(_)
                            | HTMLElementType::Tfoot(_)
                            | HTMLElementType::THead(_)
                            | HTMLElementType::Tr(_)
                    ) {
                        self.pending_character_tokens = Vec::new();
                        self.original_insertion_mode = self.insertion_mode;
                        self.insertion_mode = InsertionMode::InTableText;
                        return;
                    }
                }
            }
            TokenTag::Comment => {
                self.insert_comment_token(doc, false, &token.data);
                return;
            }
            TokenTag::Doctype(_) => {
                // ERROR: Parse error
                return;
            }
            TokenTag::StartTag => match token.data.as_str() {
                "caption" => {
                    self.clear_stack_back_to_table(doc);
                    // TODO: Insert a marker at the end of the list of active formatting elements
                    self.insert_character_token(doc, token);
                    self.insertion_mode = InsertionMode::InCaption;
                    return;
                }
                "colgroup" => {
                    self.clear_stack_back_to_table(doc);
                    self.insert_element_from_token(&mut doc.doctree, false, false, token);
                    self.insertion_mode = InsertionMode::InColumnGroup;
                    return;
                }
                "col" => {
                    self.clear_stack_back_to_table(doc);
                    let node = node::Node::new(NodeType::Element(
                        html_elements::HTMLElement::from_element_type(HTMLElementType::Colgroup(
                            html_elements::element_structs::colgroup::ColGroup::default(),
                        )),
                    ));
                    self.insert_element(&mut doc.doctree, false, node);
                    self.insertion_mode = InsertionMode::InColumnGroup;
                    self.reconsume_token = Some(token);
                    return;
                }
                "tbody" | "tfoot" | "thead" => {
                    self.clear_stack_back_to_table(doc);
                    self.insert_element_from_token(&mut doc.doctree, false, false, token);
                    self.insertion_mode = InsertionMode::InTableBody;
                    return;
                }
                "td" | "th" | "tr" => {
                    self.clear_stack_back_to_table(doc);
                    let node = node::Node::new(NodeType::Element(
                        html_elements::HTMLElement::from_element_type(HTMLElementType::Tbody(
                            html_elements::element_structs::tbody::TBody::default(),
                        )),
                    ));
                    self.insert_element(&mut doc.doctree, false, node);
                    self.insertion_mode = InsertionMode::InTableBody;
                    self.reconsume_token = Some(token);
                    return;
                }
                "table" => {
                    if !self
                        .open_node_stack
                        .stack
                        .clone()
                        .into_iter()
                        .any(|n| is_entry_element!(doc, &n, HTMLElementType::Table(_)))
                    {
                        // Parse error
                        return;
                    }

                    while let Some(n) = self.open_node_stack.pop(&mut self.tokenizer) {
                        if is_entry_element!(doc, &n, HTMLElementType::Table(_)) {
                            break;
                        }
                    }
                    self.reset_insertion_mode(doc);
                    self.reconsume_token = Some(token);
                    return;
                }
                "style" | "script" | "template" => {
                    self.parse_in_head(doc, token);
                    return;
                }
                "input" => {
                    if !token
                        .attributes
                        .iter()
                        .any(|val| val.name == "type" && val.value == "hidden")
                    {
                        // ERROR
                        self.insert_element_from_token(&mut doc.doctree, false, true, token);
                        return;
                    }
                }
                "form" => {
                    // ERROR
                    if self.form_element.is_none()
                        || self
                            .open_node_stack
                            .stack
                            .iter()
                            .any(|n| is_entry_element!(doc, n, HTMLElementType::Template(_)))
                    {
                        return;
                    }
                    self.form_element =
                        Some(self.insert_element_from_token(&mut doc.doctree, false, true, token));
                    return;
                }
                _ => {}
            },
            TokenTag::EndTag => match token.data.as_str() {
                "table" => {
                    if !self
                        .open_node_stack
                        .stack
                        .clone()
                        .into_iter()
                        .any(|n| is_entry_element!(doc, &n, HTMLElementType::Table(_)))
                    {
                        // Parse error
                        return;
                    }

                    while let Some(n) = self.open_node_stack.pop(&mut self.tokenizer) {
                        if is_entry_element!(doc, &n, HTMLElementType::Table(_)) {
                            break;
                        }
                    }
                    self.reset_insertion_mode(doc);
                    return;
                }
                "body" | "caption" | "col" | "colgroup" | "html" | "tbody" | "td" | "tfoot"
                | "th" | "thead" | "tr" => {
                    return;
                }
                "template" => {
                    self.parse_in_head(doc, token);
                    return;
                }
                _ => {}
            },
            TokenTag::EoF => {
                self.parse_in_body(doc, token);
                return;
            }
        };
        // ERROR
        // TODO: Enable foster parenting
        self.parse_in_body(doc, token);
        // TODO: Disable foster parenting
    }

    fn parse_in_table_text(&mut self, doc: &mut Document, token: HtmlToken) {
        match token.tag {
            TokenTag::Character => {
                if token.data == "\u{0000}" {
                    // ERROR
                    return;
                }
                self.pending_character_tokens.push(token);
                return;
            }
            _ => {
                if self
                    .pending_character_tokens
                    .iter()
                    .any(|t| !is_token_whitespace(t))
                {
                    // ERROR
                    for t in self.pending_character_tokens.clone() {
                        self.parse_in_table(doc, t);
                    }
                } else {
                    for t in self.pending_character_tokens.clone() {
                        self.insert_character_token(doc, t);
                    }
                }

                self.insertion_mode = self.original_insertion_mode;
                self.reconsume_token = Some(token);
                return;
            }
        }
    }

    fn parse_in_caption(&mut self, doc: &mut Document, token: HtmlToken) {
        match token.tag {
            TokenTag::EndTag => {
                match token.data.as_str() {
                    "caption" => {
                        if !self
                            .open_node_stack
                            .stack
                            .iter()
                            .any(|n| is_entry_element!(doc, n, HTMLElementType::Caption(_)))
                        {
                            // ERROR
                            return;
                        }

                        // TODO: Generate implied end tags
                        while let Some(n) = self.open_node_stack.last().cloned() {
                            self.open_node_stack.pop(&mut self.tokenizer);

                            if is_entry_element!(doc, &n, HTMLElementType::Caption(_)) {
                                break;
                            }
                        }
                        // TODO: Clear the list of active formatting elements up to the last marker
                        self.insertion_mode = InsertionMode::InTable;
                        return;
                    }
                    "table" => {
                        if !self
                            .open_node_stack
                            .stack
                            .iter()
                            .any(|n| is_entry_element!(doc, n, HTMLElementType::Caption(_)))
                        {
                            // ERROR
                            return;
                        }

                        // TODO: Generate implied end tags
                        while let Some(n) = self.open_node_stack.last().cloned() {
                            self.open_node_stack.pop(&mut self.tokenizer);

                            if is_entry_element!(doc, &n, HTMLElementType::Caption(_)) {
                                break;
                            }
                        }
                        // TODO: Clear the list of active formatting elements up to the last marker
                        self.insertion_mode = InsertionMode::InTable;
                        self.reconsume_token = Some(token);
                        return;
                    }
                    "body" | "col" | "colgroup" | "html" | "tbody" | "td" | "tfoot" | "th"
                    | "thead" | "tr" => {
                        // ERROR
                        return;
                    }
                    _ => {}
                }
            }
            TokenTag::StartTag => {
                match token.data.as_str() {
                    "caption" | "col" | "colgroup" | "tbody" | "td" | "tfoot" | "th" | "thead"
                    | "tr" => {
                        if !self
                            .open_node_stack
                            .stack
                            .iter()
                            .any(|n| is_entry_element!(doc, n, HTMLElementType::Caption(_)))
                        {
                            // ERROR
                            return;
                        }

                        // TODO: Generate implied end tags
                        while let Some(n) = self.open_node_stack.last().cloned() {
                            self.open_node_stack.pop(&mut self.tokenizer);

                            if is_entry_element!(doc, &n, HTMLElementType::Caption(_)) {
                                break;
                            }
                        }
                        // TODO: Clear the list of active formatting elements up to the last marker
                        self.insertion_mode = InsertionMode::InTable;
                        self.reconsume_token = Some(token);
                        return;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        self.parse_in_body(doc, token);
        return;
    }

    fn parse_in_column_group(&mut self, doc: &mut Document, token: HtmlToken) {
        match token.tag {
            TokenTag::Character => match token.data.as_str() {
                "\u{0009}" | "\u{000a}" | "\u{000c}" | "\u{000d}" | "\u{0020}" => {
                    self.insert_character_token(doc, token);
                    return;
                }
                _ => {}
            },
            TokenTag::Comment => {
                self.insert_comment_token(doc, false, token.data.as_str());
                return;
            }
            TokenTag::Doctype(_) => {
                // ERROR
                return;
            }
            TokenTag::StartTag => match token.data.as_str() {
                "html" => {
                    self.parse_in_body(doc, token);
                    return;
                }
                "col" => {
                    self.insert_element_from_token(&mut doc.doctree, false, true, token);
                    return;
                }
                "template" => {
                    self.parse_in_head(doc, token);
                    return;
                }
                _ => {}
            },
            TokenTag::EndTag => match token.data.as_str() {
                "colgroup" => {
                    if let Some(n) = self.open_node_stack.last() {
                        if !is_entry_element!(doc, n, HTMLElementType::Colgroup(_)) {
                            // ERROR
                            return;
                        } else {
                            self.open_node_stack.pop(&mut self.tokenizer);
                            self.insertion_mode = InsertionMode::InTable;
                            return;
                        }
                    }
                    // ERROR: Internal error
                    return;
                }
                "col" => {
                    // ERROR
                    return;
                }
                "template" => {
                    self.parse_in_head(doc, token);
                    return;
                }
                _ => {}
            },
            TokenTag::EoF => {
                self.parse_in_body(doc, token);
                return;
            }
        };
        if let Some(n) = self.open_node_stack.last() {
            if !is_entry_element!(doc, n, HTMLElementType::Colgroup(_)) {
                // ERROR: Parse error
                return;
            }
        } else {
            // ERROR: Internal
            return;
        }
        self.open_node_stack.pop(&mut self.tokenizer);
        self.insertion_mode = InsertionMode::InTable;
        self.reconsume_token = Some(token);
        return;
    }

    fn parse_in_table_body(&mut self, doc: &mut Document, token: HtmlToken) {
        match token.tag {
            TokenTag::StartTag => match token.data.as_str() {
                "tr" => {
                    self.clear_stack_back_to_table_body(doc);
                    self.insert_element_from_token(&mut doc.doctree, false, false, token);
                    self.insertion_mode = InsertionMode::InRow;
                    return;
                }
                "th" | "td" => {
                    // ERROR
                    self.clear_stack_back_to_table_body(doc);
                    self.insert_element(
                        &mut doc.doctree,
                        false,
                        node::Node::new(NodeType::Element(HTMLElement::from_element_type(
                            HTMLElementType::Tr(element_structs::tr::Tr::default()),
                        ))),
                    );
                    self.insertion_mode = InsertionMode::InRow;
                    self.reconsume_token = Some(token);
                    return;
                }
                "caption" | "col" | "colgroup" | "tbody" | "tfoot" | "thead" => {
                    // TODO: Check if stack has tbody, thead, or tfoot in table scope, if not, error and ignore

                    self.clear_stack_back_to_table_body(doc);
                    self.open_node_stack.pop(&mut self.tokenizer);
                    self.insertion_mode = InsertionMode::InTable;
                    self.reconsume_token = Some(token);
                    return;
                }
                _ => {}
            },
            TokenTag::EndTag => match token.data.as_str() {
                "tbody" | "tfoot" | "thead" => {
                    // TODO: Check if the stack of open elements has element in table scope

                    self.clear_stack_back_to_table_body(doc);
                    self.open_node_stack.pop(&mut self.tokenizer);
                    self.insertion_mode = InsertionMode::InTable;
                    return;
                }
                "table" => {
                    // TODO: Check if stack has tbody, thead, or tfoot in table scope, if not, error and ignore

                    self.clear_stack_back_to_table_body(doc);
                    self.open_node_stack.pop(&mut self.tokenizer);
                    self.insertion_mode = InsertionMode::InTable;
                    self.reconsume_token = Some(token);
                    return;
                }
                "body" | "caption" | "col" | "colgroup" | "html" | "td" | "th" | "tr" => {
                    // ERROR
                    return;
                }
                _ => {}
            },
            _ => {}
        };
        self.parse_in_table(doc, token);
        return;
    }

    fn parse_in_row(&mut self, doc: &mut Document, token: HtmlToken) {
        match token.tag {
            TokenTag::StartTag => match token.data.as_str() {
                "th" | "td" => {
                    self.clear_stack_back_to_table_row(doc);
                    self.insert_element_from_token(&mut doc.doctree, false, false, token);
                    self.insertion_mode = InsertionMode::InCell;
                    // TODO: insert marker to list of active formatting elements
                    return;
                }
                "caption" | "col" | "colgroup" | "tbody" | "tfoot" | "thead" | "tr" => {
                    if !self
                        .open_node_stack
                        .stack
                        .iter()
                        .any(|n| is_entry_element!(doc, n, HTMLElementType::Tr(_)))
                    {
                        // ERROR
                        return;
                    }

                    self.clear_stack_back_to_table_row(doc);
                    self.open_node_stack.pop(&mut self.tokenizer);
                    self.insertion_mode = InsertionMode::InTableBody;
                    self.reconsume_token = Some(token);
                    return;
                }
                _ => {}
            },
            TokenTag::EndTag => match token.data.as_str() {
                "tr" => {
                    if !self
                        .open_node_stack
                        .stack
                        .iter()
                        .any(|n| is_entry_element!(doc, n, HTMLElementType::Tr(_)))
                    {
                        // ERROR
                        return;
                    }

                    self.clear_stack_back_to_table_row(doc);
                    self.open_node_stack.pop(&mut self.tokenizer);
                    self.insertion_mode = InsertionMode::InTableBody;
                    return;
                }
                "table" => {
                    if !self
                        .open_node_stack
                        .stack
                        .iter()
                        .any(|n| is_entry_element!(doc, n, HTMLElementType::Tr(_)))
                    {
                        // ERROR
                        return;
                    }

                    self.clear_stack_back_to_table_row(doc);
                    self.open_node_stack.pop(&mut self.tokenizer);
                    self.insertion_mode = InsertionMode::InTableBody;
                    self.reconsume_token = Some(token);
                    return;
                }
                "tbody" | "tfoot" | "thead" => {
                    if !self.is_element_in_table_scope(doc, token.data.as_str()) {
                        // ERROR
                        return;
                    }
                    if !self
                        .open_node_stack
                        .stack
                        .iter()
                        .any(|n| is_entry_element!(doc, n, HTMLElementType::Tr(_)))
                    {
                        return;
                    }

                    self.clear_stack_back_to_table_row(doc);
                    self.open_node_stack.pop(&mut self.tokenizer);
                    self.insertion_mode = InsertionMode::InTable;
                    self.reconsume_token = Some(token);
                    return;
                }
                "body" | "caption" | "col" | "colgroup" | "html" | "td" | "th" => {
                    // ERROR
                    return;
                }
                _ => {}
            },
            _ => {}
        };
        self.parse_in_table(doc, token);
        return;
    }

    fn parse_in_cell(&mut self, doc: &mut Document, token: HtmlToken) {
        match token.tag {
            TokenTag::EndTag => match token.data.as_str() {
                "td" | "th" => {
                    if !self.is_element_in_table_scope(doc, token.data.as_str()) {
                        // ERROR
                        return;
                    }

                    // TODO: Generate end tags
                    if let Some(n) = self.open_node_stack.last() {
                        if let Some(node) = doc.doctree.get_node(n) {
                            if let NodeType::Element(e) = &node.node_type {
                                if e.get_name() != token.data {
                                    // ERROR
                                }
                            }
                        }
                    }
                    while let Some(n) = self.open_node_stack.pop(&mut self.tokenizer) {
                        if let Some(node) = doc.doctree.get_node(&n) {
                            if let NodeType::Element(e) = &node.node_type {
                                if e.get_name() == token.data {
                                    break;
                                }
                            }
                        }
                    }

                    // TODO: Clear list of active formatting elements up to last marker
                    self.insertion_mode = InsertionMode::InRow;
                    return;
                }
                "body" | "caption" | "col" | "colgroup" | "html" => {
                    // ERROR
                    return;
                }
                "table" | "tbody" | "tfoot" | "thead" | "tr" => {
                    if !self.is_element_in_table_scope(doc, token.data.as_str()) {
                        // ERROR
                        return;
                    }
                    self.close_cell(doc);
                    self.reconsume_token = Some(token);
                    return;
                }
                _ => {}
            },
            TokenTag::StartTag => match token.data.as_str() {
                "caption" | "col" | "colgroup" | "tbody" | "td" | "tfoot" | "th" | "thead"
                | "tr" => {
                    // TODO: Assert open stack has td or th
                    self.close_cell(doc);
                    return;
                }
                _ => {}
            },
            _ => {}
        }
        self.parse_in_body(doc, token);
        return;
    }

    fn parse_in_select(&mut self, doc: &mut Document, token: HtmlToken) {
        match token.tag {
            TokenTag::Character => match token.data.as_str() {
                "\u{0000}" => {
                    // ERROR
                    return;
                }
                _ => {
                    self.insert_character_token(doc, token);
                    return;
                }
            },
            TokenTag::Comment => {
                self.insert_comment_token(doc, false, token.data.as_str());
                return;
            }
            TokenTag::Doctype(_) => {
                // ERROR
                return;
            }
            TokenTag::StartTag => match token.data.as_str() {
                "html" => {
                    self.parse_in_body(doc, token);
                    return;
                }
                "option" => {
                    if let Some(n) = self.open_node_stack.last() {
                        if is_entry_element!(doc, n, HTMLElementType::Option(_)) {
                            self.open_node_stack.pop(&mut self.tokenizer);
                        }
                    }
                    self.insert_element_from_token(&mut doc.doctree, false, false, token);
                    return;
                }
                "optgroup" => {
                    if let Some(n) = self.open_node_stack.last() {
                        if is_entry_element!(
                            doc,
                            n,
                            HTMLElementType::Option(_) | HTMLElementType::Optgroup(_)
                        ) {
                            self.open_node_stack.pop(&mut self.tokenizer);
                        }
                    }
                    self.insert_element_from_token(&mut doc.doctree, false, false, token);
                    return;
                }
                "hr" => {
                    if let Some(n) = self.open_node_stack.last() {
                        if is_entry_element!(
                            doc,
                            n,
                            HTMLElementType::Option(_) | HTMLElementType::Optgroup(_)
                        ) {
                            self.open_node_stack.pop(&mut self.tokenizer);
                        }
                    }
                    self.insert_element_from_token(&mut doc.doctree, false, false, token);
                    // TODO: Acknowledge self closing
                    return;
                }
                "select" => {
                    // ERROR
                    if !self
                        .open_node_stack
                        .stack
                        .iter()
                        .any(|n| is_entry_element!(doc, n, HTMLElementType::Select(_)))
                    {
                        return;
                    }
                    while let Some(n) = self.open_node_stack.pop(&mut self.tokenizer) {
                        if is_entry_element!(doc, &n, HTMLElementType::Select(_)) {
                            break;
                        }
                    }
                    self.reset_insertion_mode(doc);
                    return;
                }
                "input" | "keygen" | "textarea" => {
                    // ERROR
                    if !self
                        .open_node_stack
                        .stack
                        .iter()
                        .any(|n| is_entry_element!(doc, n, HTMLElementType::Select(_)))
                    {
                        return;
                    }
                    while let Some(n) = self.open_node_stack.pop(&mut self.tokenizer) {
                        if is_entry_element!(doc, &n, HTMLElementType::Select(_)) {
                            break;
                        }
                    }
                    self.reset_insertion_mode(doc);
                    self.reconsume_token = Some(token);
                    return;
                }
                "script" | "template" => {
                    self.parse_in_head(doc, token);
                    return;
                }
                _ => {}
            },
            TokenTag::EndTag => match token.data.as_str() {
                "optgroup" => {
                    if let Some(n) = self.open_node_stack.last() {
                        if is_entry_element!(doc, n, HTMLElementType::Option(_))
                            && self.open_node_stack.len() > 1
                        {
                            if let Some(n) = self.open_node_stack.at(self.open_node_stack.len() - 2)
                            {
                                if is_entry_element!(doc, n, HTMLElementType::Optgroup(_)) {
                                    self.open_node_stack.pop(&mut self.tokenizer);
                                    self.open_node_stack.pop(&mut self.tokenizer);
                                    return;
                                }
                            }
                        }
                    }

                    if let Some(n) = self.open_node_stack.last() {
                        if is_entry_element!(doc, n, HTMLElementType::Optgroup(_)) {
                            self.open_node_stack.pop(&mut self.tokenizer);
                            return;
                        }
                    }
                    // ERROR
                    return;
                }
                "option" => {
                    if let Some(n) = self.open_node_stack.last() {
                        if is_entry_element!(doc, n, HTMLElementType::Option(_)) {
                            self.open_node_stack.pop(&mut self.tokenizer);
                            return;
                        }
                    }
                    // ERROR
                    return;
                }
                "select" => {
                    if !self
                        .open_node_stack
                        .stack
                        .iter()
                        .any(|n| is_entry_element!(doc, n, HTMLElementType::Select(_)))
                    {
                        // ERROR
                        return;
                    }

                    while let Some(n) = self.open_node_stack.pop(&mut self.tokenizer) {
                        if is_entry_element!(doc, &n, HTMLElementType::Select(_)) {
                            break;
                        }
                    }
                    self.reset_insertion_mode(doc);
                    return;
                }
                "template" => {
                    self.parse_in_head(doc, token);
                    return;
                }
                _ => {}
            },
            TokenTag::EoF => {
                self.parse_in_body(doc, token);
                return;
            }
        }
        // ERROR
        return;
    }

    fn parse_in_select_table(&mut self, doc: &mut Document, token: HtmlToken) {
        match token.tag {
            TokenTag::StartTag => match token.data.as_str() {
                "caption" | "table" | "tbody" | "tfoot" | "thead" | "tr" | "td" | "th" => {
                    // ERROR
                    while let Some(n) = self.open_node_stack.pop(&mut self.tokenizer) {
                        if is_entry_element!(doc, &n, HTMLElementType::Select(_)) {
                            break;
                        }
                    }
                    self.reset_insertion_mode(doc);
                    self.reconsume_token = Some(token);
                    return;
                }
                _ => {}
            },
            TokenTag::EndTag => match token.data.as_str() {
                "caption" | "table" | "tbody" | "tfoot" | "thead" | "tr" | "td" | "th" => {
                    // ERROR
                    if !self.is_element_in_table_scope(doc, &token.data) {
                        return;
                    }
                    while let Some(n) = self.open_node_stack.pop(&mut self.tokenizer) {
                        if is_entry_element!(doc, &n, HTMLElementType::Select(_)) {
                            break;
                        }
                    }
                    self.reset_insertion_mode(doc);
                    self.reconsume_token = Some(token);
                    return;
                }
                _ => {}
            },
            _ => {}
        }
        self.parse_in_select(doc, token);
    }

    fn parse_in_template(&mut self, doc: &mut Document, token: HtmlToken) {
        match token.tag {
            TokenTag::Character => {
                self.parse_in_body(doc, token);
                return;
            }
            TokenTag::Comment => {
                self.parse_in_body(doc, token);
                return;
            }
            TokenTag::Doctype(_) => {
                self.parse_in_body(doc, token);
                return;
            }
            TokenTag::StartTag => match token.data.as_str() {
                "base" | "basefont" | "bgsound" | "link" | "meta" | "noframes" | "script"
                | "style" | "template" | "title" => {
                    self.parse_in_head(doc, token);
                    return;
                }
                "caption" | "colgroup" | "tbody" | "tfoot" | "thead" => {
                    // TODO: pop current template insertion mode
                    // TODO: Push in table onto stack of template insertion modes
                    self.insertion_mode = InsertionMode::InTable;
                    self.reconsume_token = Some(token);
                    return;
                }
                "col" => {
                    // TODO: pop current template insertion mode
                    // TODO: Push in column group onto the template insertion stack
                    self.insertion_mode = InsertionMode::InColumnGroup;
                    self.reconsume_token = Some(token);
                    return;
                }
                "tr" => {
                    // TODO: pop current template insertion mode
                    // TODO: Push in table body onto the template insertion stack
                    self.insertion_mode = InsertionMode::InTableBody;
                    self.reconsume_token = Some(token);
                    return;
                }
                "td" | "th" => {
                    // TODO: pop current template insertion mode
                    // TODO: Push in row onto the template insertion stack
                    self.insertion_mode = InsertionMode::InRow;
                    self.reconsume_token = Some(token);
                    return;
                }
                _ => {
                    // TODO: pop current template insertion mode
                    // TODO: Push in body the template insertion stack
                    self.insertion_mode = InsertionMode::InBody;
                    self.reconsume_token = Some(token);
                    return;
                }
            },
            TokenTag::EndTag => match token.data.as_str() {
                "template" => {
                    self.parse_in_head(doc, token);
                }
                _ => {
                    // ERROR
                    return;
                }
            },
            TokenTag::EoF => {
                if !self
                    .open_node_stack
                    .stack
                    .iter()
                    .any(|n| is_entry_element!(doc, n, HTMLElementType::Template(_)))
                {
                    self.should_exit = true;
                }
                // ERROR
                while let Some(n) = self.open_node_stack.pop(&mut self.tokenizer) {
                    if is_entry_element!(doc, &n, HTMLElementType::Template(_)) {
                        break;
                    }
                }
                // TODO: Clear list of active formatting elements to marker
                // TODO: Pop last entry in template insertion mode stack
                self.reconsume_token = Some(token);
                return;
            }
        }
    }

    fn parse_after_body(&mut self, doc: &mut Document, token: HtmlToken) {
        match token.tag {
            TokenTag::Character => match token.data.as_str() {
                "\u{0009}" | "\u{000A}" | "\u{000C}" | "\u{000D}" | "\u{0020}" => {
                    self.parse_in_body(doc, token);
                    return;
                }
                _ => {}
            },
            TokenTag::Comment => {
                self.insert_comment_token(doc, false, &token.data);
                return;
            }
            TokenTag::Doctype(_) => {
                // ERROR
                return;
            }
            TokenTag::StartTag => match token.data.as_str() {
                "html" => {
                    self.parse_in_body(doc, token);
                    return;
                }
                _ => {}
            },
            TokenTag::EndTag => match token.data.as_str() {
                "html" => {
                    if self.fragment_parser_ctx.is_some() {
                        // ERROR
                        return;
                    }
                    self.insertion_mode = InsertionMode::AfterAfterBody;
                    return;
                }
                _ => {}
            },
            TokenTag::EoF => {
                self.should_exit = true;
                return;
            }
        }
        // ERROR
        self.insertion_mode = InsertionMode::InBody;
        self.reconsume_token = Some(token);
        return;
    }

    fn parse_in_frameset(&mut self, doc: &mut Document, token: HtmlToken) {
        match token.tag {
            TokenTag::Character => match token.data.as_str() {
                "\u{0009}" | "\u{000A}" | "\u{000C}" | "\u{000D}" | "\u{0020}" => {
                    self.insert_character_token(doc, token);
                    return;
                }
                _ => {}
            },
            TokenTag::Comment => {
                self.insert_comment_token(doc, false, &token.data);
                return;
            }
            TokenTag::Doctype(_) => {
                // ERROR
                return;
            }
            TokenTag::StartTag => match token.data.as_str() {
                "html" => {
                    self.parse_in_body(doc, token);
                    return;
                }
                "frameset" => {
                    self.insert_element_from_token(&mut doc.doctree, false, false, token);
                    return;
                }
                "frame" => {
                    self.insert_element_from_token(&mut doc.doctree, false, true, token);
                    // TODO: Acknowledge self closing tags
                    return;
                }
                "noframes" => {
                    self.parse_in_head(doc, token);
                    return;
                }
                _ => {}
            },
            TokenTag::EndTag => match token.data.as_str() {
                "frameset" => {
                    if self.open_node_stack.len() == 1 {
                        if let Some(n) = self.open_node_stack.last() {
                            if is_entry_element!(doc, n, HTMLElementType::Html(_)) {
                                // ERROR
                                return;
                            }
                        }
                    }
                    self.open_node_stack.pop(&mut self.tokenizer);
                    // TODO: Add frameset
                    // if self.fragment_parser_ctx.is_none() {
                    //     if let Some(n) = self.open_node_stack.last() {
                    //         // if is_entry_element!(doc, n, HTMLElementType::Frame)
                    //     }
                    // }
                    self.insertion_mode = InsertionMode::AfterFrameset;
                    return;
                }
                _ => {}
            },
            TokenTag::EoF => {
                if self.open_node_stack.len() != 1 {
                    // ERROR
                } else if let Some(n) = self.open_node_stack.last() {
                    if !is_entry_element!(doc, n, HTMLElementType::Html(_)) {
                        // ERROR
                    }
                }
                self.should_exit = true;
                return;
            }
            _ => {}
        }
        // ERROR
        return;
    }

    fn parse_after_frameset(&mut self, doc: &mut Document, token: HtmlToken) {
        match token.tag {
            TokenTag::Character => match token.data.as_str() {
                "\u{0009}" | "\u{000A}" | "\u{000C}" | "\u{000D}" | "\u{0020}" => {
                    self.insert_character_token(doc, token);
                    return;
                }
                _ => {}
            },
            TokenTag::Comment => {
                self.insert_comment_token(doc, false, &token.data);
                return;
            }
            TokenTag::Doctype(_) => {
                // ERROR
                return;
            }
            TokenTag::StartTag => match token.data.as_str() {
                "html" => {
                    self.parse_in_body(doc, token);
                    return;
                }
                "noframes" => {
                    self.parse_in_head(doc, token);
                    return;
                }
                _ => {}
            },
            TokenTag::EndTag => match token.data.as_str() {
                "html" => {
                    self.insertion_mode = InsertionMode::AfterAfterFrameset;
                    return;
                }
                _ => {}
            },
            TokenTag::EoF => {
                self.should_exit = true;
                return;
            }
        };
        // ERROR
        return;
    }

    fn parse_after_after_body(&mut self, doc: &mut Document, token: HtmlToken) {
        match token.tag {
            TokenTag::Comment => {
                self.insert_comment_token(doc, true, &token.data);
                return;
            }
            TokenTag::Doctype(_) => {
                self.parse_in_body(doc, token);
                return;
            }
            TokenTag::Character => match token.data.as_str() {
                "\u{0009}" | "\u{000A}" | "\u{000C}" | "\u{000D}" | "\u{0020}" => {
                    self.parse_in_body(doc, token);
                    return;
                }
                _ => {}
            },
            TokenTag::StartTag => match token.data.as_str() {
                "html" => {
                    self.parse_in_body(doc, token);
                    return;
                }
                _ => {}
            },
            TokenTag::EoF => {
                self.should_exit = true;
                return;
            }
            _ => {}
        };
        // ERROR
        self.insertion_mode = InsertionMode::InBody;
        self.reconsume_token = Some(token);
        return;
    }

    fn parse_after_after_frameset(&mut self, doc: &mut Document, token: HtmlToken) {
        match token.tag {
            TokenTag::Comment => {
                self.insert_comment_token(doc, true, &token.data);
                return;
            }
            TokenTag::Doctype(_) => {
                self.parse_in_body(doc, token);
                return;
            }
            TokenTag::Character => match token.data.as_str() {
                "\u{0009}" | "\u{000A}" | "\u{000C}" | "\u{000D}" | "\u{0020}" => {
                    self.parse_in_body(doc, token);
                    return;
                }
                _ => {}
            },
            TokenTag::StartTag => match token.data.as_str() {
                "html" => {
                    self.parse_in_body(doc, token);
                    return;
                }
                "noframes" => {
                    self.parse_in_head(doc, token);
                    return;
                }
                _ => {}
            },
            TokenTag::EoF => {
                self.should_exit = true;
                return;
            }
            _ => {}
        };
        // ERROR
        return;
    }

    pub fn parse_html(&mut self) -> document::Document {
        // TODO: Determine encoding of document
        //       For now we just assume UTF-8
        // TODO: Normalize Newlines
        log::debug!("Starting parser");
        let mut doc = document::Document::new();

        self.reconsume_token = None;
        self.should_exit = false;
        loop {
            if self.should_exit {
                break;
            }

            let token = match self.reconsume_token.clone() {
                Some(t) => {
                    self.reconsume_token = None;
                    t
                }
                None => self.tokenizer.get_next_token(),
            };
            // TODO: Check if we're in a foreign context or not
            //       For now follow normal insertion rules

            match self.insertion_mode {
                InsertionMode::Initial => self.parse_initial(&mut doc, token),
                InsertionMode::BeforeHtml => self.parse_before_html(&mut doc, token),
                InsertionMode::BeforeHead => self.parse_before_head(&mut doc, token),
                InsertionMode::InHead => self.parse_in_head(&mut doc, token),
                InsertionMode::InHeadNoscript => self.parse_in_head_noscript(&mut doc, token),
                InsertionMode::AfterHead => self.parse_after_head(&mut doc, token),
                InsertionMode::InBody => self.parse_in_body(&mut doc, token),
                InsertionMode::Text => self.parse_text(&mut doc, token),
                InsertionMode::InTable => self.parse_in_table(&mut doc, token),
                InsertionMode::InTableText => self.parse_in_table_text(&mut doc, token),
                InsertionMode::InCaption => self.parse_in_caption(&mut doc, token),
                InsertionMode::InColumnGroup => self.parse_in_column_group(&mut doc, token),
                InsertionMode::InTableBody => self.parse_in_table_body(&mut doc, token),
                InsertionMode::InRow => self.parse_in_row(&mut doc, token),
                InsertionMode::InCell => self.parse_in_cell(&mut doc, token),
                InsertionMode::InSelect => self.parse_in_select(&mut doc, token),
                InsertionMode::InSelectTable => self.parse_in_select_table(&mut doc, token),
                InsertionMode::InTemplate => self.parse_in_template(&mut doc, token),
                InsertionMode::AfterBody => self.parse_after_body(&mut doc, token),
                InsertionMode::InFrameset => self.parse_in_frameset(&mut doc, token),
                InsertionMode::AfterFrameset => self.parse_after_frameset(&mut doc, token),
                InsertionMode::AfterAfterBody => self.parse_after_after_body(&mut doc, token),
                InsertionMode::AfterAfterFrameset => {
                    self.parse_after_after_frameset(&mut doc, token)
                }
            };
        }

        return doc;
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test_empty_parse_no_panic() {
        parse_document("");
    }

    #[test]
    fn test_basic_html() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("test_resources/basic.html");

        let html = fs::read_to_string(d).unwrap();
        // println!("{}", html);

        let doc = parse_document(&html);
        let root_nodes = doc.doctree.get_root_node_list();

        let mut root_html_found = false;
        for r in root_nodes {
            let node = match doc.doctree.get_node(&r) {
                Some(n) => n,
                None => continue,
            };
            let node_element = match &node.node_type {
                NodeType::Element(e) => e,
                _ => continue,
            };
            if !matches!(node_element.element_type, HTMLElementType::Html(_)) {
                continue;
            }
            root_html_found = true;
            println!("{}", node.children.len());
            assert!(node.children.len() >= 2)
        }
        assert!(root_html_found);
    }
}
