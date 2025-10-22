use crate::dom::{AttrMap, Node};
use std::collections::HashMap;

/// A simple HTML parser
pub struct Parser {
    pos: usize,
    input: String,
}

impl Parser {
    /// Parse an HTML document
    pub fn parse(source: String) -> Node {
        let mut parser = Parser {
            pos: 0,
            input: source,
        };
        let nodes = parser.parse_nodes();

        // If there's a single root element, return it. Otherwise wrap in a div.
        if nodes.len() == 1 {
            nodes.into_iter().next().unwrap()
        } else {
            Node::element("html".to_string(), HashMap::new(), nodes)
        }
    }

    /// Parse a sequence of sibling nodes
    fn parse_nodes(&mut self) -> Vec<Node> {
        let mut nodes = Vec::new();
        loop {
            self.consume_whitespace();
            if self.eof() || self.starts_with("</") {
                break;
            }
            nodes.push(self.parse_node());
        }
        nodes
    }

    /// Parse a single node
    fn parse_node(&mut self) -> Node {
        if self.next_char() == '<' {
            if self.starts_with("<!--") {
                self.parse_comment();
                // After skipping comment, parse the next node
                if !self.eof() && !self.starts_with("</") {
                    self.parse_node()
                } else {
                    Node::text(String::new())
                }
            } else if self.starts_with("<!") {
                self.parse_doctype();
                // After skipping doctype, parse the next node
                if !self.eof() && !self.starts_with("</") {
                    self.parse_node()
                } else {
                    Node::text(String::new())
                }
            } else {
                self.parse_element()
            }
        } else {
            self.parse_text()
        }
    }

    /// Parse an element tag
    fn parse_element(&mut self) -> Node {
        // Opening tag
        assert_eq!(self.consume_char(), '<');
        let tag_name = self.parse_tag_name();
        let attrs = self.parse_attributes();
        
        // Check for self-closing tag
        let self_closing = if self.starts_with("/>") {
            self.consume_char(); // consume '/'
            true
        } else {
            false
        };
        
        assert_eq!(self.consume_char(), '>');

        // For self-closing tags, return immediately with no children
        if self_closing {
            return Node::element(tag_name, attrs, Vec::new());
        }

        // Contents
        let children = self.parse_nodes();

        // Closing tag (skip if EOF or we're at another opening tag for void elements)
        if !self.eof() && self.starts_with("</") {
            assert_eq!(self.consume_char(), '<');
            assert_eq!(self.consume_char(), '/');
            let close_tag = self.parse_tag_name();
            // Be lenient if closing tag doesn't match (malformed HTML)
            if close_tag != tag_name {
                eprintln!("Warning: Mismatched tags: expected </{tag_name}>, found </{close_tag}>");
            }
            assert_eq!(self.consume_char(), '>');
        }

        Node::element(tag_name, attrs, children)
    }
    
    /// Skip over a comment
    fn parse_comment(&mut self) {
        assert!(self.starts_with("<!--"));
        // Skip <!--
        self.pos += 4;
        // Find the end of comment
        while !self.eof() && !self.starts_with("-->") {
            self.consume_char();
        }
        // Skip -->
        if self.starts_with("-->") {
            self.pos += 3;
        }
    }
    
    /// Skip over a DOCTYPE declaration
    fn parse_doctype(&mut self) {
        assert!(self.starts_with("<!"));
        self.consume_char(); // <
        self.consume_char(); // !
        // Consume until we find >
        while !self.eof() && self.next_char() != '>' {
            self.consume_char();
        }
        if !self.eof() {
            self.consume_char(); // >
        }
    }

    /// Parse a tag or attribute name
    fn parse_tag_name(&mut self) -> String {
        self.consume_while(|c| c.is_alphanumeric())
    }

    /// Parse attributes inside an element tag
    fn parse_attributes(&mut self) -> AttrMap {
        let mut attributes = HashMap::new();
        loop {
            self.consume_whitespace();
            if self.next_char() == '>' || self.starts_with("/>") {
                break;
            }
            if let Some((name, value)) = self.parse_attribute() {
                attributes.insert(name, value);
            }
        }
        attributes
    }

    /// Parse a single attribute
    fn parse_attribute(&mut self) -> Option<(String, String)> {
        let name = self.parse_tag_name();
        
        // Handle boolean attributes (no value)
        self.consume_whitespace();
        if self.next_char() != '=' {
            return Some((name, String::new()));
        }
        
        self.consume_char(); // consume '='
        self.consume_whitespace();
        
        let value = if self.next_char() == '"' || self.next_char() == '\'' {
            self.parse_attr_value()
        } else {
            // Unquoted attribute value
            self.consume_while(|c| !c.is_whitespace() && c != '>' && c != '/')
        };
        
        Some((name, value))
    }

    /// Parse an attribute value
    fn parse_attr_value(&mut self) -> String {
        let open_quote = self.consume_char();
        assert!(open_quote == '"' || open_quote == '\'');
        let value = self.consume_while(|c| c != open_quote);
        assert_eq!(self.consume_char(), open_quote);
        value
    }

    /// Parse a text node
    fn parse_text(&mut self) -> Node {
        Node::text(self.consume_while(|c| c != '<'))
    }

    /// Consume and discard whitespace characters
    fn consume_whitespace(&mut self) {
        self.consume_while(|c| c.is_whitespace());
    }

    /// Consume characters while the test is true
    fn consume_while<F>(&mut self, test: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();
        while !self.eof() && test(self.next_char()) {
            result.push(self.consume_char());
        }
        result
    }

    /// Return the current character without consuming it
    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    /// Check if the remaining input starts with the given string
    fn starts_with(&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
    }

    /// Return true if all input is consumed
    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    /// Consume and return the next character
    fn consume_char(&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, cur_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.pos += next_pos;
        cur_char
    }
}
