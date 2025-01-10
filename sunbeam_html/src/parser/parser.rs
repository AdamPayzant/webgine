use crate::html_elements;
use crate::parser::tokenizer;

pub enum NodeType {
    Text(String),
    Element(html_elements::HTMLElement),
    Unknown(String),
}

pub struct Node {
    pub children: Vec<Node>,
    pub node_type: NodeType,
}

pub enum DocumentParseError {
    UnparsableDocument,
}

pub fn parse_document(data: &str) -> Result<Node, DocumentParseError> {
    let mut parsed = parse_html(data);

    if parsed.len() == 1 {
        Ok(parsed.pop().unwrap())
    } else {
        Err(DocumentParseError::UnparsableDocument)
    }
}

#[derive(Default)]
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

struct OpenElement {}

pub fn parse_html(data: &str) -> Vec<Node> {
    // TODO: Determine encoding of document
    //       For now we just assume UTF-8
    // TODO: Normalize Newlines

    let tokens = tokenizer::tokenize_string(data);

    let open_elements: Vec<OpenElement> = Vec::new();

    return Vec::new();
}
