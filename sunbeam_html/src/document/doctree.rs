use crate::document::node;

// Uses an array backed tree where we pass out indexes instead of references.
// This allows us to have doubly-linked relations without getting into
// complicated lifetimes or arcs
//
// Eventually this should be reassessed, but for now it should provide
// an acceptable interface if the implementation needs to be replaced
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
        // Walk to find an open spot
        let mut i = 0;
        while i < self.data.len() {
            if matches!(self.data[i], None) {
                self.data[i] = Some(node);
                return DoctreeNode { idx: i };
            }
            i += 1;
        }
        // Can't find an open slot, return
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
}

#[derive(Clone, Copy)]
pub struct DoctreeNode {
    idx: usize,
}

impl PartialEq for DoctreeNode {
    fn eq(&self, other: &Self) -> bool {
        self.idx == other.idx
    }

    fn ne(&self, other: &Self) -> bool {
        self.idx != other.idx
    }
}
