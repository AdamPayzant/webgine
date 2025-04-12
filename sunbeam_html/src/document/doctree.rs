use crate::display_data::display_box::DisplayBox;
use crate::document::node;
use log;

use pollster;

// Uses an array backed tree where we pass out indexes instead of references.
// This allows us to have doubly-linked relations without getting into
// complicated lifetimes or arcs
//
// Eventually this should be redone. Anytime a node is removed,
// that array index is set to none for the remainder of the object.
// This happens because we don't know who holds a doctree node, and it
// would be better to "waste" memory than swap out a reference under
// someone's nose.
#[derive(Debug)]
pub struct Doctree {
    data: Vec<Option<node::Node>>,
    root_node: Vec<DoctreeNode>,
}

impl Doctree {
    pub fn new() -> Doctree {
        Doctree {
            data: Vec::new(),
            root_node: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: node::Node) -> DoctreeNode {
        log::trace!("Inserting node {:?}", node);
        self.data.push(Some(node));

        DoctreeNode {
            idx: self.data.len() - 1,
        }
    }

    pub fn add_root_node(&mut self, node: node::Node) -> DoctreeNode {
        let node = self.add_node(node);
        self.root_node.push(node.clone());

        node
    }

    pub fn get_node(&self, doctree_node: &DoctreeNode) -> Option<&node::Node> {
        match self.data.get(doctree_node.idx) {
            Some(v) => v.as_ref(),
            None => None,
        }
    }

    pub fn get_mut_node(&mut self, doctree_node: &DoctreeNode) -> Option<&mut node::Node> {
        match self.data.get_mut(doctree_node.idx) {
            Some(v) => v.as_mut(),
            None => None,
        }
    }

    pub fn get_last_node(&mut self) -> Option<&mut node::Node> {
        match self.data.last_mut() {
            Some(v) => v.as_mut(),
            None => None,
        }
    }

    pub fn remove_at(&mut self, idx: usize) {
        if self.data.len() <= idx {
            return;
        }

        self.data[idx] = None;
    }

    pub fn get_element_name(&self, node: &DoctreeNode) -> Option<String> {
        if let Some(node) = self.get_node(node) {
            if let node::NodeType::Element(element) = &node.node_type {
                Some(element.get_name().to_lowercase())
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn get_root_node_list(&self) -> Vec<DoctreeNode> {
        self.root_node.clone()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn get_display_data(&self) -> Vec<DisplayBox> {
        let mut futures = Vec::new();
        for n in &self.root_node {
            if let Some(node) = self.get_node(&n) {
                futures.push(node.get_node_displaybox(self));
            }
        }

        futures.into_iter().map(|f| pollster::block_on(f)).collect()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct DoctreeNode {
    pub idx: usize,
}

impl PartialEq for DoctreeNode {
    fn eq(&self, other: &Self) -> bool {
        self.idx == other.idx
    }

    fn ne(&self, other: &Self) -> bool {
        self.idx != other.idx
    }
}
