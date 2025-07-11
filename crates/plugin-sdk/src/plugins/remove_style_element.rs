// this_file: crates/plugin-sdk/src/plugins/remove_style_element.rs

//! Remove style element plugin implementation
//!
//! This plugin removes all <style> elements from SVG documents.
//! This is useful when you want to remove all embedded CSS styles.
//!
//! Reference: SVGOPROTECTED_58_static str {
        "removeStyleElement"
    }

    fn description(&self) -> &'static str {
        PROTECTED_5_
    }

    fn validate_params(&self, params: &Value) -> Result<()> {
        Self::parse_config(params)?;
        Ok(())
    }

    fn apply(&self, document: &mut Document) -> Result<()> {
        self.process_element(&mut document.root);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::borrow::Cow;
    use vexy_svgo_core::ast::{Document, Element, Node};

    fn create_element(name: &'static str) -> Element<'static> {
        let mut element = Element::new(name);
        element.name = Cow::Borrowed(name);
        element
    }

    #[test]
    fn test_plugin_creation() {
        let plugin = RemoveStyleElementPlugin::new();
        assert_eq!(plugin.name(), "removeStyleElement");
        assert_eq!(
            plugin.description(),
            "removes <style> element (disabled by default)"
        );
    }

    #[test]
    fn test_removes_style_elements() {
        let plugin = RemoveStyleElementPlugin::new();

        let mut doc = Document::new();
        doc.root = create_element("svg");

        // Add style element
        let mut style = create_element("style");
        style
            .children
            .push(Node::Text(".red { fill: red; }".to_string()));
        doc.root.children.push(Node::Element(style));

        // Add other elements
        let rect = create_element("rect");
        doc.root.children.push(Node::Element(rect));

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Check that style was removed
        assert_eq!(doc.root.children.len(), 1);
        if let Node::Element(elem) = &doc.root.children[0] {
            assert_eq!(elem.name, "rect");
        } else {
            panic!("Expected element node");
        }
    }

    #[test]
    fn test_removes_multiple_style_elements() {
        let plugin = RemoveStyleElementPlugin::new();

        let mut doc = Document::new();
        doc.root = create_element("svg");

        // Add multiple style elements
        let mut style1 = create_element("style");
        style1
            .children
            .push(Node::Text(".class1 { fill: blue; }".to_string()));
        doc.root.children.push(Node::Element(style1));

        let circle = create_element("circle");
        doc.root.children.push(Node::Element(circle));

        let mut style2 = create_element("style");
        style2
            .children
            .push(Node::Text(".class2 { stroke: green; }".to_string()));
        doc.root.children.push(Node::Element(style2));

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Check that all styles were removed
        assert_eq!(doc.root.children.len(), 1);
        if let Node::Element(elem) = &doc.root.children[0] {
            assert_eq!(elem.name, "circle");
        }
    }

    #[test]
    fn test_removes_nested_style_elements() {
        let plugin = RemoveStyleElementPlugin::new();

        let mut doc = Document::new();
        doc.root = create_element("svg");

        // Create nested structure
        let mut defs = create_element("defs");
        let mut style = create_element("style");
        style
            .children
            .push(Node::Text("#id { opacity: 0.5; }".to_string()));
        defs.children.push(Node::Element(style));

        let gradient = create_element("linearGradient");
        defs.children.push(Node::Element(gradient));

        doc.root.children.push(Node::Element(defs));

        // Add style in another group
        let mut group = create_element("g");
        let mut nested_style = create_element("style");
        nested_style
            .children
            .push(Node::Text(".nested { fill: yellow; }".to_string()));
        group.children.push(Node::Element(nested_style));

        let path = create_element("path");
        group.children.push(Node::Element(path));

        doc.root.children.push(Node::Element(group));

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Check nested removal
        if let Node::Element(defs) = &doc.root.children[0] {
            assert_eq!(defs.children.len(), 1);
            if let Node::Element(grad) = &defs.children[0] {
                assert_eq!(grad.name, "linearGradient");
            }
        }

        if let Node::Element(g) = &doc.root.children[1] {
            assert_eq!(g.children.len(), 1);
            if let Node::Element(p) = &g.children[0] {
                assert_eq!(p.name, "path");
            }
        }
    }

    #[test]
    fn test_preserves_style_attributes() {
        let plugin = RemoveStyleElementPlugin::new();

        let mut doc = Document::new();
        doc.root = create_element("svg");

        // Add element with style attribute
        let mut rect = create_element("rect");
        rect.attributes
            .insert("style".to_string(), "fill: red; stroke: blue;".to_string());
        doc.root.children.push(Node::Element(rect));

        // Add style element
        let mut style = create_element("style");
        style
            .children
            .push(Node::Text(".class { fill: green; }".to_string()));
        doc.root.children.push(Node::Element(style));

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Check that style element was removed but style attribute preserved
        assert_eq!(doc.root.children.len(), 1);
        if let Node::Element(elem) = &doc.root.children[0] {
            assert_eq!(elem.name, "rect");
            assert_eq!(
                elem.attributes.get("style"),
                Some(&"fill: red; stroke: blue;".to_string())
            );
        }
    }

    #[test]
    fn test_empty_document() {
        let plugin = RemoveStyleElementPlugin::new();

        let mut doc = Document::new();
        doc.root = create_element("svg");

        // Apply plugin to empty document
        let result = plugin.apply(&mut doc);
        assert!(result.is_ok());
    }

    #[test]
    fn test_document_without_styles() {
        let plugin = RemoveStyleElementPlugin::new();

        let mut doc = Document::new();
        doc.root = create_element("svg");

        // Add non-style elements
        doc.root
            .children
            .push(Node::Element(create_element("rect")));
        doc.root
            .children
            .push(Node::Element(create_element("circle")));
        doc.root
            .children
            .push(Node::Element(create_element("path")));

        let children_before = doc.root.children.len();

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Check that nothing was removed
        assert_eq!(doc.root.children.len(), children_before);
    }

    #[test]
    fn test_style_with_attributes() {
        let plugin = RemoveStyleElementPlugin::new();

        let mut doc = Document::new();
        doc.root = create_element("svg");

        // Add style element with attributes
        let mut style = create_element("style");
        style
            .attributes
            .insert("type".to_string(), "text/css".to_string());
        style
            .attributes
            .insert("media".to_string(), "screen".to_string());
        style.children.push(Node::Text(
            "@media print { .no-print { display: none; } }".to_string(),
        ));
        doc.root.children.push(Node::Element(style));

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Check that style was removed regardless of attributes
        assert_eq!(doc.root.children.len(), 0);
    }

    #[test]
    fn test_parameter_validation() {
        let plugin = RemoveStyleElementPlugin::new();

        // Empty object is valid
        assert!(plugin.validate_params(&json!({})).is_ok());

        // Null is valid
        assert!(plugin.validate_params(&Value::Null).is_ok());

        // Non-object is invalid
        assert!(plugin.validate_params(&json!("invalid")).is_err());
    }

    #[test]
    fn test_cdata_style_content() {
        let plugin = RemoveStyleElementPlugin::new();

        let mut doc = Document::new();
        doc.root = create_element("svg");

        // Add style with CDATA content
        let mut style = create_element("style");
        style.children.push(Node::Text(
            "<![CDATA[ .class { fill: red; } ]]>".to_string(),
        ));
        doc.root.children.push(Node::Element(style));

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Check that style was removed
        assert_eq!(doc.root.children.len(), 0);
    }
}

// Use parameterized testing framework for SVGO fixture tests
crate::plugin_fixture_tests!(RemoveStyleElementPlugin, "removeStyleElement");
        doc.root
            .children
            .push(Node::Element(create_element("path")));

        let children_before = doc.root.children.len();

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Check that nothing was removed
        assert_eq!(doc.root.children.len(), children_before);
    }

    #[test]
    fn test_style_with_attributes() {
        let plugin = RemoveStyleElementPlugin::new();

        let mut doc = Document::new();
        doc.root = create_element("svg");

        // Add style element with attributes
        let mut style = create_element("style");
        style
            .attributes
            .insert("type".to_string(), "text/css".to_string());
        style
            .attributes
            .insert("media".to_string(), "screen".to_string());
        style.children.push(Node::Text(
            "@media print { .no-print { display: none; } }".to_string(),
        ));
        doc.root.children.push(Node::Element(style));

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Check that style was removed regardless of attributes
        assert_eq!(doc.root.children.len(), 0);
    }

    #[test]
    fn test_parameter_validation() {
        let plugin = RemoveStyleElementPlugin::new();

        // Empty object is valid
        assert!(plugin.validate_params(&json!({})).is_ok());

        // Null is valid
        assert!(plugin.validate_params(&Value::Null).is_ok());

        // Non-object is invalid
        assert!(plugin.validate_params(&json!("invalid")).is_err());
    }

    #[test]
    fn test_cdata_style_content() {
        let plugin = RemoveStyleElementPlugin::new();

        let mut doc = Document::new();
        doc.root = create_element("svg");

        // Add style with CDATA content
        let mut style = create_element("style");
        style.children.push(Node::Text(
            "<![CDATA[ .class { fill: red; } ]]>".to_string(),
        ));
        doc.root.children.push(Node::Element(style));

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Check that style was removed
        assert_eq!(doc.root.children.len(), 0);
    }
}

// Use parameterized testing framework for SVGO fixture tests
crate::plugin_fixture_tests!(RemoveStyleElementPlugin, "removeStyleElement");
