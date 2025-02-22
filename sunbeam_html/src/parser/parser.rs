use crate::document::doctree;
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

pub enum DocumentParseError {
    UnparsableDocument,
}

pub fn parse_document(data: &str) -> Document {
    let mut parser = Parser::new(data);
    parser.parse_html()
}

#[derive(Default, Clone, Copy)]
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

    pub fn pop(&mut self, tokenizer: &mut tokenizer::Tokenizer) {
        self.stack.pop();
        tokenizer.pop_open_tag();
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
        }
    }

    fn create_element_from_token(&self, token: HtmlToken) -> node::Node {
        if self.speculative_parser.is_some() {}
        // TODO: Lookup custom element, for now we skip

        let mut element = HTMLElement::new(token.data);

        token.attributes.into_iter().for_each(|val| {
            element.add_attribute(val.name, val.value);
        });

        node::Node::new(node::NodeType::Unknown("".to_string()))
    }

    // Node Insertion functions
    fn insert_element_from_token(
        &mut self,
        doctree: &mut doctree::Doctree,
        is_root: bool,
        nostackpush: bool,
        token: HtmlToken,
    ) {
        let node_name = token.data.clone();
        let is_self_closing = token.flags.self_closing;

        let mut element = self.create_element_from_token(token);
        let node = if is_root {
            doctree.add_root_node(element)
        } else {
            element.parent = self.open_node_stack.last().cloned();

            let n = doctree.add_node(element);
            match self.open_node_stack.last() {
                Some(p) => match doctree.get_mut_node(p) {
                    Some(parent) => {
                        parent.add_child(n);
                    }
                    None => {}
                },
                None => {}
            };

            n
        };

        if !is_self_closing && !nostackpush {
            self.open_node_stack
                .push(&mut self.tokenizer, node, node_name);
        }
    }

    fn insert_element(&mut self, doctree: &mut doctree::Doctree, is_root: bool, e: node::Node) {
        let mut element = e;
        let node = if is_root {
            doctree.add_root_node(element)
        } else {
            element.parent = self.open_node_stack.last().cloned();

            let n = doctree.add_node(element);
            match self.open_node_stack.last() {
                Some(p) => match doctree.get_mut_node(p) {
                    Some(parent) => {
                        parent.add_child(n);
                    }
                    None => {}
                },
                None => {}
            };

            n
        };

        self.open_node_stack.push_self_only(node);
    }

    fn insert_comment_token(&mut self, doc: &mut Document, is_root: bool, str: &str) {
        if is_root {
            doc.doctree
                .add_root_node(node::Node::new(node::NodeType::Comment(str.to_string())));
            return;
        }
        let node = doc
            .doctree
            .add_node(node::Node::new(node::NodeType::Comment(str.to_string())));

        if let Some(idx) = self.open_node_stack.last() {
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

        let mut node = node::Node::new(node::NodeType::Text(token.data));
        node.parent = self.open_node_stack.last().cloned();
        let n = doc.doctree.add_node(node);

        match self.open_node_stack.last() {
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
            _ => {
                return;
            }
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
                    self.insert_element_from_token(&mut doc.doctree, false, false, token);

                    if true {
                        // TODO: Check if there is a template on the open stack
                        self.form_element = self.open_node_stack.last().cloned();
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
                "a" => {}
                _ => {}
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
                    _ => {}
                };
            }
        }
    }

    fn parse_text(&mut self, doc: &mut Document, token: HtmlToken) {}

    fn parse_in_table(&mut self, doc: &mut Document, token: HtmlToken) {}

    fn parse_in_table_text(&mut self, doc: &mut Document, token: HtmlToken) {}

    fn parse_in_caption(&mut self, doc: &mut Document, token: HtmlToken) {}

    fn parse_in_column_group(&mut self, doc: &mut Document, token: HtmlToken) {}

    fn parse_in_table_body(&mut self, doc: &mut Document, token: HtmlToken) {}

    fn parse_in_row(&mut self, doc: &mut Document, token: HtmlToken) {}

    fn parse_in_cell(&mut self, doc: &mut Document, token: HtmlToken) {}

    fn parse_in_select(&mut self, doc: &mut Document, token: HtmlToken) {}

    fn parse_in_select_table(&mut self, doc: &mut Document, token: HtmlToken) {}

    fn parse_in_template(&mut self, doc: &mut Document, token: HtmlToken) {}

    fn parse_after_body(&mut self, doc: &mut Document, token: HtmlToken) {}

    fn parse_in_frameset(&mut self, doc: &mut Document, token: HtmlToken) {}

    fn parse_after_frameset(&mut self, doc: &mut Document, token: HtmlToken) {}

    fn parse_after_after_body(&mut self, doc: &mut Document, token: HtmlToken) {}

    fn parse_after_after_frameset(&mut self, doc: &mut Document, token: HtmlToken) {}

    pub fn parse_html(&mut self) -> document::Document {
        // TODO: Determine encoding of document
        //       For now we just assume UTF-8
        // TODO: Normalize Newlines

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
