use std::collections::HashMap;

/// A DOM node
#[derive(Debug, Clone)]
pub struct Node {
    pub node_type: NodeType,
    pub children: Vec<Node>,
}

/// Types of DOM nodes
#[derive(Debug, Clone, PartialEq)]
pub enum NodeType {
    Element(ElementData),
    Text(String),
}

/// An HTML element with tag name and attributes
#[derive(Debug, Clone, PartialEq)]
pub struct ElementData {
    pub tag_name: String,
    pub attributes: AttrMap,
}

pub type AttrMap = HashMap<String, String>;

impl Node {
    /// Create a text node
    pub fn text(data: String) -> Node {
        Node {
            node_type: NodeType::Text(data),
            children: Vec::new(),
        }
    }

    /// Create an element node
    pub fn element(name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
        Node {
            node_type: NodeType::Element(ElementData {
                tag_name: name,
                attributes: attrs,
            }),
            children,
        }
    }
}

impl ElementData {
    /// Get the value of an attribute by name
    pub fn get_attribute(&self, name: &str) -> Option<&String> {
        self.attributes.get(name)
    }

    /// Get the id attribute if it exists
    pub fn id(&self) -> Option<&String> {
        self.get_attribute("id")
    }

    /// Get all class names (space-separated)
    pub fn classes(&self) -> Vec<&str> {
        self.get_attribute("class")
            .map(|s| s.split_whitespace().collect())
            .unwrap_or_default()
    }
}
