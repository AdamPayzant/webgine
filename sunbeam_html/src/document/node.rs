use pollster;

use crate::document::doctree;
use crate::html_elements;
use crate::{
    display_data::{
        self,
        display_box::{self, DisplayBox},
    },
    html_elements::HTMLElementType,
};

use super::document;

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

    pub async fn get_node_displaybox(&self, doc: &document::Document) -> DisplayBox {
        let mut res = DisplayBox::new();

        match &self.node_type {
            NodeType::Text(s) => {
                res.data = display_box::DisplayBoxData::Text(display_data::text::Text {
                    data: s.clone(),
                    font: None,
                });
                // TODO: Eventually format based on styling
            }
            NodeType::Element(e) => {
                if matches!(e.element_type, HTMLElementType::Head(_)) {
                    // Don't render anything under head
                    return res;
                }
                res = e.get_display_box();
            }
            // TODO: Get display information for element
            _ => {}
        };

        let mut futures = Vec::new();
        for n in &self.children {
            if let Some(node) = doc.doctree.get_node(&n) {
                futures.push(node.get_node_displaybox(&doc));
            }
        }

        res.children = futures.into_iter().map(|f| pollster::block_on(f)).collect();
        // TODO: Go through the children and correct positioning

        res
    }
}
