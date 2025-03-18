use crate::document::doctree;
use crate::html_elements;

#[derive(Clone, Debug)]
pub struct DocumentType {
    pub name: String,
    pub public_id: String,
    pub system_id: String,
}

#[derive(Clone, Debug)]
pub enum NodeType {
    Text(String),
    Element(html_elements::HTMLElement),
    Comment(String),
    DocumentType(DocumentType),
    Unknown(String),
}

impl NodeType {
    pub fn add_attribute(&mut self, name: String, value: String) {
        if let NodeType::Element(element) = self {
            element.add_attribute(name, value);
        }
    }
}

#[derive(Clone, Debug)]
pub struct Node {
    pub children: Vec<doctree::DoctreeNode>,
    pub parent: Option<doctree::DoctreeNode>,
    pub node_type: NodeType,
}

impl Node {
    pub fn new(node_type: NodeType) -> Node {
        Node {
            children: Vec::new(),
            parent: None,
            node_type,
        }
    }

    pub fn add_child(&mut self, child: doctree::DoctreeNode) {
        self.children.push(child);
    }

    pub fn remove_child(&mut self, child: &doctree::DoctreeNode) {
        let mut i = 0;
        while i < self.children.len() {
            if self.children[i] == *child {
                self.children.remove(i);
                return;
            }
            i += 1;
        }
    }

    pub fn add_parent(&mut self, parent: Option<doctree::DoctreeNode>) {
        self.parent = parent;
    }

    pub fn add_attribute(&mut self, name: String, value: String) {
        self.node_type.add_attribute(name, value);
    }
}
