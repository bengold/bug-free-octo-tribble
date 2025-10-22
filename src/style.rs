use crate::css::{Selector, SimpleSelector, Specificity, Stylesheet, Value};
use crate::dom::{ElementData, Node, NodeType};
use std::collections::HashMap;

/// Map from CSS property names to values
pub type PropertyMap = HashMap<String, Value>;

/// A node with associated style data
#[derive(Debug)]
pub struct StyledNode<'a> {
    #[allow(dead_code)]
    pub node: &'a Node,
    pub specified_values: PropertyMap,
    pub children: Vec<StyledNode<'a>>,
}

#[derive(PartialEq)]
pub enum Display {
    Inline,
    Block,
    None,
}

impl<'a> StyledNode<'a> {
    /// Get a property value by name
    pub fn value(&self, name: &str) -> Option<Value> {
        self.specified_values.get(name).cloned()
    }

    /// Get the display property value
    pub fn display(&self) -> Display {
        match self.value("display") {
            Some(Value::Keyword(s)) => match s.as_str() {
                "block" => Display::Block,
                "none" => Display::None,
                _ => Display::Inline,
            },
            _ => {
                // Default block-level elements to display: block
                match self.node.node_type {
                    NodeType::Element(ref elem) => match elem.tag_name.as_str() {
                        "html" | "body" | "div" | "p" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6" 
                        | "ul" | "ol" | "li" | "header" | "footer" | "section" | "article" 
                        | "nav" | "aside" | "main" => Display::Block,
                        _ => Display::Inline,
                    },
                    NodeType::Text(_) => Display::Inline,
                }
            }
        }
    }

    /// Look up a value or return a default
    pub fn lookup(&self, name: &str, fallback_name: &str, default: &Value) -> Value {
        self.value(name)
            .or_else(|| self.value(fallback_name))
            .unwrap_or_else(|| default.clone())
    }
}

/// Apply a stylesheet to a DOM tree, creating a styled tree
pub fn style_tree<'a>(root: &'a Node, stylesheet: &'a Stylesheet) -> StyledNode<'a> {
    style_tree_with_parent(root, stylesheet, None)
}

fn style_tree_with_parent<'a>(
    node: &'a Node,
    stylesheet: &'a Stylesheet,
    parent_values: Option<&PropertyMap>,
) -> StyledNode<'a> {
    let mut specified_values = match node.node_type {
        NodeType::Element(ref elem) => specified_values(elem, stylesheet),
        NodeType::Text(_) => HashMap::new(),
    };

    // Inherit color from parent
    if let Some(parent) = parent_values {
        if let Some(parent_color) = parent.get("color") {
            specified_values.entry("color".to_string()).or_insert(parent_color.clone());
        }
    }

    let children = node
        .children
        .iter()
        .map(|child| style_tree_with_parent(child, stylesheet, Some(&specified_values)))
        .collect();

    StyledNode {
        node,
        specified_values,
        children,
    }
}

/// Get the specified values for a single element
fn specified_values(elem: &ElementData, stylesheet: &Stylesheet) -> PropertyMap {
    let mut values = HashMap::new();
    
    // Apply default styles for certain elements
    apply_default_styles(elem, &mut values);
    
    let mut rules = matching_rules(elem, stylesheet);

    // Sort by specificity (higher specificity last)
    rules.sort_by(|&(a, _), &(b, _)| a.cmp(&b));

    for (_, rule) in rules {
        for declaration in &rule.declarations {
            values.insert(declaration.name.clone(), declaration.value.clone());
        }
    }
    values
}

/// Apply default browser styles for certain elements
fn apply_default_styles(elem: &ElementData, values: &mut PropertyMap) {
    use crate::css::{Color, Value};
    
    match elem.tag_name.as_str() {
        "a" => {
            // Links default to blue color
            if !values.contains_key("color") {
                values.insert(
                    "color".to_string(),
                    Value::Color(Color {
                        r: 0,
                        g: 0,
                        b: 238,  // Blue
                        a: 255,
                    }),
                );
            }
        }
        "h1" => {
            if !values.contains_key("color") {
                values.insert(
                    "color".to_string(),
                    Value::Color(Color {
                        r: 0,
                        g: 0,
                        b: 0,
                        a: 255,
                    }),
                );
            }
        }
        _ => {}
    }
}

/// Find all rules that match an element
fn matching_rules<'a>(
    elem: &ElementData,
    stylesheet: &'a Stylesheet,
) -> Vec<(Specificity, &'a crate::css::Rule)> {
    stylesheet
        .rules
        .iter()
        .filter_map(|rule| match_rule(elem, rule))
        .collect()
}

/// Check if a rule matches an element, return specificity if it does
fn match_rule<'a>(
    elem: &ElementData,
    rule: &'a crate::css::Rule,
) -> Option<(Specificity, &'a crate::css::Rule)> {
    rule.selectors
        .iter()
        .find(|selector| matches(elem, *selector))
        .map(|selector| (selector.specificity(), rule))
}

/// Check if a selector matches an element
fn matches(elem: &ElementData, selector: &Selector) -> bool {
    match *selector {
        Selector::Simple(ref simple_selector) => matches_simple_selector(elem, simple_selector),
    }
}

fn matches_simple_selector(elem: &ElementData, selector: &SimpleSelector) -> bool {
    // Check tag name
    if selector.tag_name.iter().any(|name| elem.tag_name != *name) {
        return false;
    }

    // Check ID
    if selector.id.iter().any(|id| elem.id() != Some(id)) {
        return false;
    }

    // Check classes
    let elem_classes = elem.classes();
    if selector
        .classes
        .iter()
        .any(|class| !elem_classes.contains(&class.as_str()))
    {
        return false;
    }

    true
}
