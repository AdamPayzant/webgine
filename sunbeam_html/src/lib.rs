mod html_elements;
mod lang_system;

pub enum NodeType {
    Text(String),
    Element(html_elements::HTMLElement),
    Unknown(String),
}

pub struct Node {
    pub children: Vec<Node>,
    pub node_type: NodeType,
}

struct HtmlMetadata {}

struct Document {
    metadata: HtmlMetadata,
    head: Node,
    body: Node,
}

fn parse_html(data: String) -> Node {
    Node {
        children: Vec::new(),
        node_type: NodeType::Text("Placeholder".to_string()),
    }
}

// fn new_document() -> Document {}

#[cfg(test)]
mod tests {
    use super::*;
}
