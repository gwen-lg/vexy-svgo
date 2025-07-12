//! Tests for the SVG parser module

#[cfg(test)]
mod tests {
    use crate::parser::main::Parser;
    use crate::ast::Node;

    #[test]
    fn test_parse_simple_svg() {
        let svg = r#"<svg width="100" height="100"></svg>"#;
        let parser = Parser::new();
        let result = parser.parse(svg);
        
        assert!(result.is_ok());
        let doc = result.unwrap();
        assert_eq!(doc.root.name, "svg");
        assert_eq!(doc.root.attr("width"), Some("100"));
        assert_eq!(doc.root.attr("height"), Some("100"));
    }

    #[test]
    fn test_parse_self_closing_element() {
        let svg = r#"<svg><rect x="10" y="10" width="50" height="50"/></svg>"#;
        let parser = Parser::new();
        let result = parser.parse(svg);
        
        assert!(result.is_ok());
        let doc = result.unwrap();
        assert_eq!(doc.root.children.len(), 1);
        
        if let Node::Element(rect) = &doc.root.children[0] {
            assert_eq!(rect.name, "rect");
            assert_eq!(rect.attr("x"), Some("10"));
            assert_eq!(rect.attr("y"), Some("10"));
            assert_eq!(rect.attr("width"), Some("50"));
            assert_eq!(rect.attr("height"), Some("50"));
        } else {
            panic!("Expected element node");
        }
    }

    #[test]
    fn test_parse_with_text_content() {
        let svg = r#"<svg><text>Hello World</text></svg>"#;
        let parser = Parser::new();
        let result = parser.parse(svg);
        
        assert!(result.is_ok());
        let doc = result.unwrap();
        assert_eq!(doc.root.children.len(), 1);
        
        if let Node::Element(text_elem) = &doc.root.children[0] {
            assert_eq!(text_elem.name, "text");
            assert_eq!(text_elem.children.len(), 1);
            
            if let Node::Text(text) = &text_elem.children[0] {
                assert_eq!(text.trim(), "Hello World");
            } else {
                panic!("Expected text node");
            }
        } else {
            panic!("Expected element node");
        }
    }

    #[test]
    fn test_parse_with_comments() {
        let svg = r#"<svg><!-- This is a comment --><rect/></svg>"#;
        let parser = Parser::new();
        let result = parser.parse(svg);
        
        assert!(result.is_ok());
        let doc = result.unwrap();
        // Comments should be preserved in the AST
        assert!(doc.root.children.len() >= 1);
    }

    #[test]
    fn test_parse_with_namespaces() {
        let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"><rect/></svg>"#;
        let parser = Parser::new();
        let result = parser.parse(svg);
        
        assert!(result.is_ok());
        let doc = result.unwrap();
        assert_eq!(doc.root.name, "svg");
        
        // Check that namespaces are properly handled
        assert!(doc.root.namespaces.contains_key(""));
        assert!(doc.root.namespaces.contains_key("xlink"));
    }

    #[test]
    fn test_parse_invalid_xml() {
        let svg = r#"<svg><rect></svg>"#; // Missing closing tag for rect
        let parser = Parser::new();
        let result = parser.parse(svg);
        
        // This should either succeed (if quick-xml auto-closes) or fail gracefully
        // The important thing is that it doesn't panic
        match result {
            Ok(_) => {}, // Auto-closed by parser
            Err(_) => {}, // Expected error
        }
    }

    #[test]
    fn test_parse_empty_svg() {
        let svg = r#"<svg></svg>"#;
        let parser = Parser::new();
        let result = parser.parse(svg);
        
        assert!(result.is_ok());
        let doc = result.unwrap();
        assert_eq!(doc.root.name, "svg");
        assert_eq!(doc.root.children.len(), 0);
    }

    #[test]
    fn test_parse_nested_elements() {
        let svg = r#"<svg><g><rect/><circle/></g></svg>"#;
        let parser = Parser::new();
        let result = parser.parse(svg);
        
        assert!(result.is_ok());
        let doc = result.unwrap();
        assert_eq!(doc.root.children.len(), 1);
        
        if let Node::Element(group) = &doc.root.children[0] {
            assert_eq!(group.name, "g");
            assert_eq!(group.children.len(), 2);
            
            if let Node::Element(rect) = &group.children[0] {
                assert_eq!(rect.name, "rect");
            } else {
                panic!("Expected rect element");
            }
            
            if let Node::Element(circle) = &group.children[1] {
                assert_eq!(circle.name, "circle");
            } else {
                panic!("Expected circle element");
            }
        } else {
            panic!("Expected group element");
        }
    }
}