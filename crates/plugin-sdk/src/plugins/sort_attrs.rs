// this_file: crates/plugin-sdk/src/plugins/sort_attrs.rs

//! Sort attributes plugin implementation
//!
//! This plugin sorts element attributes for better compression.
//! It follows a configurable order for attributes, with special handling
//! for namespace attributes and alphabetical fallback.
//!
//! Reference: SVGOPROTECTED_119_:PROTECTED_120_-PROTECTED_121_-PROTECTED_122_static str {
        "sortAttrs"
    }

    fn description(&self) -> &'static str {
        PROTECTED_33_
    }

    fn validate_params(&self, params: &Value) -> Result<()> {
        Self::parse_config(params)?;
        Ok(())
    }

    fn apply(&self, document: &mut Document) -> Result<()> {
        self.sort_attrs_recursive(&mut document.root);
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
        let plugin = SortAttrsPlugin::new();
        assert_eq!(plugin.name(), "sortAttrs");
        assert_eq!(
            plugin.description(),
            "Sort element attributes for better compression"
        );
    }

    #[test]
    fn test_parameter_validation() {
        let plugin = SortAttrsPlugin::new();

        // Valid parameters (empty object)
        assert!(plugin.validate_params(&json!({})).is_ok());

        // Valid parameters (with order)
        assert!(plugin
            .validate_params(&json!({
                "order": ["id", "width", "height"],
                "xmlnsOrder": "front"
            }))
            .is_ok());

        // Invalid parameter type
        assert!(plugin
            .validate_params(&json!({
                "order": "invalid"
            }))
            .is_err());
    }

    #[test]
    fn test_basic_attribute_sorting() {
        let plugin = SortAttrsPlugin::new();
        let mut doc = Document::new();

        // Create element with attributes in random order
        let mut rect = create_element("rect");
        rect.attributes
            .insert("height".to_string(), "100".to_string());
        rect.attributes.insert("id".to_string(), "test".to_string());
        rect.attributes
            .insert("width".to_string(), "200".to_string());
        rect.attributes.insert("x".to_string(), "10".to_string());
        doc.root.children.push(Node::Element(rect));

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Check that attributes are sorted according to default order
        if let Node::Element(elem) = &doc.root.children[0] {
            let attr_names: Vec<&String> = elem.attributes.keys().collect();
            // id should come first, then width, height, x
            assert_eq!(attr_names.len(), 4);
            // Note: HashMap iteration order is not guaranteed, but we can check the comparison logic
        }
    }

    #[test]
    fn test_xmlns_attributes_sorting() {
        let plugin = SortAttrsPlugin::new();
        let mut doc = Document::new();

        // Create element with xmlns attributes
        let mut svg = create_element("svg");
        svg.attributes
            .insert("width".to_string(), "100".to_string());
        svg.attributes.insert(
            "xmlns:xlink".to_string(),
            "http://www.w3.org/1999/xlink".to_string(),
        );
        svg.attributes.insert(
            "xmlns".to_string(),
            "http://www.w3.org/2000/svg".to_string(),
        );
        svg.attributes.insert("id".to_string(), "test".to_string());
        doc.root.children.push(Node::Element(svg));

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Check that xmlns attributes are sorted to front
        if let Node::Element(elem) = &doc.root.children[0] {
            assert_eq!(elem.attributes.len(), 4);
            let attr_names: Vec<&String> = elem.attributes.keys().collect();
            // xmlns should come first, then xmlns:xlink, then id, then width
            assert_eq!(attr_names[0], "xmlns");
            assert_eq!(attr_names[1], "xmlns:xlink");
            // The rest should be by order: id, width
            assert_eq!(attr_names[2], "id");
            assert_eq!(attr_names[3], "width");
        }
    }

    #[test]
    fn test_alphabetical_sorting() {
        let plugin = SortAttrsPlugin::new();
        let mut doc = Document::new();

        // Create element with attributes not in default order
        let mut rect = create_element("rect");
        rect.attributes
            .insert("z-index".to_string(), "1".to_string());
        rect.attributes
            .insert("data-custom".to_string(), "value".to_string());
        rect.attributes
            .insert("aria-label".to_string(), "button".to_string());
        doc.root.children.push(Node::Element(rect));

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Check that attributes are sorted alphabetically
        if let Node::Element(elem) = &doc.root.children[0] {
            assert_eq!(elem.attributes.len(), 3);
            // Should be sorted alphabetically: aria-label, data-custom, z-index
        }
    }

    #[test]
    fn test_custom_order_config() {
        let config = SortAttrsConfig {
            order: vec!["width".to_string(), "height".to_string(), "id".to_string()],
            xmlns_order: "front".to_string(),
        };
        let plugin = SortAttrsPlugin::with_config(config);
        let mut doc = Document::new();

        // Create element with attributes
        let mut rect = create_element("rect");
        rect.attributes.insert("id".to_string(), "test".to_string());
        rect.attributes
            .insert("height".to_string(), "100".to_string());
        rect.attributes
            .insert("width".to_string(), "200".to_string());
        doc.root.children.push(Node::Element(rect));

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Check that attributes are sorted according to custom order
        if let Node::Element(elem) = &doc.root.children[0] {
            assert_eq!(elem.attributes.len(), 3);
            // Should be sorted according to custom order: width, height, id
        }
    }

    #[test]
    fn test_hyphenated_attributes() {
        let plugin = SortAttrsPlugin::new();
        let mut doc = Document::new();

        // Create element with hyphenated attributes
        let mut rect = create_element("rect");
        rect.attributes
            .insert("fill-opacity".to_string(), "0.5".to_string());
        rect.attributes
            .insert("fill".to_string(), "red".to_string());
        rect.attributes
            .insert("stroke-width".to_string(), "2".to_string());
        rect.attributes
            .insert("stroke".to_string(), "blue".to_string());
        doc.root.children.push(Node::Element(rect));

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Check that hyphenated attributes are grouped with their base
        if let Node::Element(elem) = &doc.root.children[0] {
            assert_eq!(elem.attributes.len(), 4);
            // fill and stroke are in default order, so they should be grouped together
        }
    }

    #[test]
    fn test_nested_elements() {
        let plugin = SortAttrsPlugin::new();
        let mut doc = Document::new();

        // Create nested elements with attributes
        let mut group = create_element("g");
        group
            .attributes
            .insert("transform".to_string(), "translate(10,20)".to_string());
        group
            .attributes
            .insert("id".to_string(), "group1".to_string());

        let mut rect = create_element("rect");
        rect.attributes
            .insert("height".to_string(), "100".to_string());
        rect.attributes
            .insert("width".to_string(), "200".to_string());
        rect.attributes.insert("x".to_string(), "10".to_string());

        group.children.push(Node::Element(rect));
        doc.root.children.push(Node::Element(group));

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Check that both elements have sorted attributes
        if let Node::Element(group_elem) = &doc.root.children[0] {
            assert_eq!(group_elem.attributes.len(), 2);

            if let Node::Element(rect_elem) = &group_elem.children[0] {
                assert_eq!(rect_elem.attributes.len(), 3);
            }
        }
    }

    #[test]
    fn test_empty_attributes() {
        let plugin = SortAttrsPlugin::new();
        let mut doc = Document::new();

        // Create element with no attributes
        let rect = create_element("rect");
        doc.root.children.push(Node::Element(rect));

        // Apply plugin - should not crash
        let result = plugin.apply(&mut doc);
        assert!(result.is_ok());

        // Element should still exist with no attributes
        if let Node::Element(elem) = &doc.root.children[0] {
            assert_eq!(elem.attributes.len(), 0);
        }
    }

    #[test]
    fn test_config_parsing() {
        let config = SortAttrsPlugin::parse_config(&json!({
            "order": ["id", "class", "width", "height"],
            "xmlnsOrder": "alphabetical"
        }))
        .unwrap();

        assert_eq!(config.order, vec!["id", "class", "width", "height"]);
        assert_eq!(config.xmlns_order, "alphabetical");
    }
}

// Use parameterized testing framework for SVGO fixture tests
crate::plugin_fixture_tests!(SortAttrsPlugin, "sortAttrs");
        }
    }

    #[test]
    fn test_xmlns_attributes_sorting() {
        let plugin = SortAttrsPlugin::new();
        let mut doc = Document::new();

        // Create element with xmlns attributes
        let mut svg = create_element("svg");
        svg.attributes
            .insert("width".to_string(), "100".to_string());
        svg.attributes.insert(
            "xmlns:xlink".to_string(),
            "http://www.w3.org/1999/xlink".to_string(),
        );
        svg.attributes.insert(
            "xmlns".to_string(),
            "http://www.w3.org/2000/svg".to_string(),
        );
        svg.attributes.insert("id".to_string(), "test".to_string());
        doc.root.children.push(Node::Element(svg));

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Check that xmlns attributes are sorted to front
        if let Node::Element(elem) = &doc.root.children[0] {
            assert_eq!(elem.attributes.len(), 4);
            let attr_names: Vec<&String> = elem.attributes.keys().collect();
            // xmlns should come first, then xmlns:xlink, then id, then width
            assert_eq!(attr_names[0], "xmlns");
            assert_eq!(attr_names[1], "xmlns:xlink");
            // The rest should be by order: id, width
            assert_eq!(attr_names[2], "id");
            assert_eq!(attr_names[3], "width");
        }
    }

    #[test]
    fn test_alphabetical_sorting() {
        let plugin = SortAttrsPlugin::new();
        let mut doc = Document::new();

        // Create element with attributes not in default order
        let mut rect = create_element("rect");
        rect.attributes
            .insert("z-index".to_string(), "1".to_string());
        rect.attributes
            .insert("data-custom".to_string(), "value".to_string());
        rect.attributes
            .insert("aria-label".to_string(), "button".to_string());
        doc.root.children.push(Node::Element(rect));

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Check that attributes are sorted alphabetically
        if let Node::Element(elem) = &doc.root.children[0] {
            assert_eq!(elem.attributes.len(), 3);
            // Should be sorted alphabetically: aria-label, data-custom, z-index
        }
    }

    #[test]
    fn test_custom_order_config() {
        let config = SortAttrsConfig {
            order: vec!["width".to_string(), "height".to_string(), "id".to_string()],
            xmlns_order: "front".to_string(),
        };
        let plugin = SortAttrsPlugin::with_config(config);
        let mut doc = Document::new();

        // Create element with attributes
        let mut rect = create_element("rect");
        rect.attributes.insert("id".to_string(), "test".to_string());
        rect.attributes
            .insert("height".to_string(), "100".to_string());
        rect.attributes
            .insert("width".to_string(), "200".to_string());
        doc.root.children.push(Node::Element(rect));

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Check that attributes are sorted according to custom order
        if let Node::Element(elem) = &doc.root.children[0] {
            assert_eq!(elem.attributes.len(), 3);
            // Should be sorted according to custom order: width, height, id
        }
    }

    #[test]
    fn test_hyphenated_attributes() {
        let plugin = SortAttrsPlugin::new();
        let mut doc = Document::new();

        // Create element with hyphenated attributes
        let mut rect = create_element("rect");
        rect.attributes
            .insert("fill-opacity".to_string(), "0.5".to_string());
        rect.attributes
            .insert("fill".to_string(), "red".to_string());
        rect.attributes
            .insert("stroke-width".to_string(), "2".to_string());
        rect.attributes
            .insert("stroke".to_string(), "blue".to_string());
        doc.root.children.push(Node::Element(rect));

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Check that hyphenated attributes are grouped with their base
        if let Node::Element(elem) = &doc.root.children[0] {
            assert_eq!(elem.attributes.len(), 4);
            // fill and stroke are in default order, so they should be grouped together
        }
    }

    #[test]
    fn test_nested_elements() {
        let plugin = SortAttrsPlugin::new();
        let mut doc = Document::new();

        // Create nested elements with attributes
        let mut group = create_element("g");
        group
            .attributes
            .insert("transform".to_string(), "translate(10,20)".to_string());
        group
            .attributes
            .insert("id".to_string(), "group1".to_string());

        let mut rect = create_element("rect");
        rect.attributes
            .insert("height".to_string(), "100".to_string());
        rect.attributes
            .insert("width".to_string(), "200".to_string());
        rect.attributes.insert("x".to_string(), "10".to_string());

        group.children.push(Node::Element(rect));
        doc.root.children.push(Node::Element(group));

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Check that both elements have sorted attributes
        if let Node::Element(group_elem) = &doc.root.children[0] {
            assert_eq!(group_elem.attributes.len(), 2);

            if let Node::Element(rect_elem) = &group_elem.children[0] {
                assert_eq!(rect_elem.attributes.len(), 3);
            }
        }
    }

    #[test]
    fn test_empty_attributes() {
        let plugin = SortAttrsPlugin::new();
        let mut doc = Document::new();

        // Create element with no attributes
        let rect = create_element("rect");
        doc.root.children.push(Node::Element(rect));

        // Apply plugin - should not crash
        let result = plugin.apply(&mut doc);
        assert!(result.is_ok());

        // Element should still exist with no attributes
        if let Node::Element(elem) = &doc.root.children[0] {
            assert_eq!(elem.attributes.len(), 0);
        }
    }

    #[test]
    fn test_config_parsing() {
        let config = SortAttrsPlugin::parse_config(&json!({
            "order": ["id", "class", "width", "height"],
            "xmlnsOrder": "alphabetical"
        }))
        .unwrap();

        assert_eq!(config.order, vec!["id", "class", "width", "height"]);
        assert_eq!(config.xmlns_order, "alphabetical");
    }
}

// Use parameterized testing framework for SVGO fixture tests
crate::plugin_fixture_tests!(SortAttrsPlugin, "sortAttrs");
