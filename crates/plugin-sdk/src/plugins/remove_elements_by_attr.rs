// this_file: crates/plugin-sdk/src/plugins/remove_elements_by_attr.rs

//! Removes elements by ID or class attribute
//!
//! This plugin removes arbitrary elements that match specified ID or class attributes.
//! Elements can be removed based on their id attribute or class attribute values.
//!
//! Reference: SVGOPROTECTED_93_static str {
        "removeElementsByAttr"
    }

    fn description(&self) -> &'static str {
        PROTECTED_7_
    }

    fn validate_params(&self, params: &Value) -> Result<()> {
        Self::parse_config(params)?;
        Ok(())
    }

    fn apply(&self, document: &mut Document) -> Result<()> {
        // Only proceed if we have something to remove
        if self.config.id.is_empty() && self.config.class.is_empty() {
            return Ok(());
        }

        self.process_element(&mut document.root);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indexmap::IndexMap;
    use serde_json::json;
    use std::collections::HashMap;
    use vexy_svgo_core::ast::{Document, DocumentMetadata, Element, Node};

    fn create_test_document() -> Document<'static> {
        Document {
            root: Element {
                name: "svg".into(),
                attributes: IndexMap::new(),
                namespaces: IndexMap::new(),
                children: vec![],
            },
            prologue: vec![],
            epilogue: vec![],
            metadata: DocumentMetadata {
                path: None,
                encoding: None,
                version: None,
            },
        }
    }

    #[test]
    fn test_plugin_info() {
        let plugin = RemoveElementsByAttrPlugin::new();
        assert_eq!(plugin.name(), "removeElementsByAttr");
        assert_eq!(
            plugin.description(),
            "removes arbitrary elements by ID or className (disabled by default)"
        );
    }

    #[test]
    fn test_param_validation() {
        let plugin = RemoveElementsByAttrPlugin::new();

        // Test null params
        assert!(plugin.validate_params(&Value::Null).is_ok());

        // Test valid params
        assert!(plugin
            .validate_params(&json!({
                "id": "test"
            }))
            .is_ok());

        assert!(plugin
            .validate_params(&json!({
                "class": "test-class"
            }))
            .is_ok());

        assert!(plugin
            .validate_params(&json!({
                "id": ["id1", "id2"],
                "class": ["class1", "class2"]
            }))
            .is_ok());
    }

    #[test]
    fn test_parse_config_single_id() {
        let config_json = json!({
            "id": "elementToRemove"
        });

        let config = RemoveElementsByAttrPlugin::parse_config(&config_json).unwrap();
        assert_eq!(config.id, vec!["elementToRemove"]);
        assert!(config.class.is_empty());
    }

    #[test]
    fn test_parse_config_multiple_ids() {
        let config_json = json!({
            "id": ["elementToRemove1", "elementToRemove2"]
        });

        let config = RemoveElementsByAttrPlugin::parse_config(&config_json).unwrap();
        assert_eq!(config.id, vec!["elementToRemove1", "elementToRemove2"]);
        assert!(config.class.is_empty());
    }

    #[test]
    fn test_parse_config_single_class() {
        let config_json = json!({
            "class": "classToRemove"
        });

        let config = RemoveElementsByAttrPlugin::parse_config(&config_json).unwrap();
        assert!(config.id.is_empty());
        assert_eq!(config.class, vec!["classToRemove"]);
    }

    #[test]
    fn test_parse_config_multiple_classes() {
        let config_json = json!({
            "class": ["classToRemove1", "classToRemove2"]
        });

        let config = RemoveElementsByAttrPlugin::parse_config(&config_json).unwrap();
        assert!(config.id.is_empty());
        assert_eq!(config.class, vec!["classToRemove1", "classToRemove2"]);
    }

    #[test]
    fn test_parse_config_mixed() {
        let config_json = json!({
            "id": "elementToRemove",
            "class": ["classToRemove1", "classToRemove2"]
        });

        let config = RemoveElementsByAttrPlugin::parse_config(&config_json).unwrap();
        assert_eq!(config.id, vec!["elementToRemove"]);
        assert_eq!(config.class, vec!["classToRemove1", "classToRemove2"]);
    }

    #[test]
    fn test_should_remove_element_by_id() {
        let config = RemoveElementsByAttrConfig {
            id: vec!["removeMe".to_string()],
            class: vec![],
        };
        let plugin = RemoveElementsByAttrPlugin::with_config(config);

        let mut element = Element {
            name: "rect".into(),
            attributes: IndexMap::new(),
            namespaces: IndexMap::new(),
            children: vec![],
        };
        element.set_attr("id", "removeMe");

        assert!(plugin.should_remove_element(&element));

        // Test element that shouldnPROTECTED_95_t be removed
        let mut element2 = Element {
            name: "rect".into(),
            attributes: IndexMap::new(),
            namespaces: IndexMap::new(),
            children: vec![],
        };
        element2.set_attr("class", "someClass keepMe anotherClass");

        assert!(!plugin.should_remove_element(&element2));
    }

    #[test]
    fn test_apply_removes_by_id() {
        let config = RemoveElementsByAttrConfig {
            id: vec!["elementToRemove".to_string()],
            class: vec![],
        };
        let plugin = RemoveElementsByAttrPlugin::with_config(config);
        let mut doc = create_test_document();

        // Add element to remove
        let mut element_to_remove = Element {
            name: "rect".into(),
            attributes: IndexMap::new(),
            namespaces: IndexMap::new(),
            children: vec![],
        };
        element_to_remove.set_attr("id", "elementToRemove");
        doc.root.children.push(Node::Element(element_to_remove));

        // Add element to keep
        let mut element_to_keep = Element {
            name: "circle".into(),
            attributes: IndexMap::new(),
            namespaces: IndexMap::new(),
            children: vec![],
        };
        element_to_keep.set_attr("id", "elementToKeep");
        doc.root.children.push(Node::Element(element_to_keep));

        let result = plugin.apply(&mut doc);
        assert!(result.is_ok());

        // Should have only one element remaining
        assert_eq!(doc.root.children.len(), 1);
        if let Node::Element(element) = &doc.root.children[0] {
            assert_eq!(element.name, "circle");
            assert_eq!(element.attr("id").map(|s| s.as_str()), Some("elementToKeep"));
        } else {
            panic!("Expected element");
        }
    }

    #[test]
    fn test_apply_removes_by_class() {
        let config = RemoveElementsByAttrConfig {
            id: vec![],
            class: vec!["removeMe".to_string()],
        };
        let plugin = RemoveElementsByAttrPlugin::with_config(config);
        let mut doc = create_test_document();

        // Add element to remove
        let mut element_to_remove = Element {
            name: "rect".into(),
            attributes: IndexMap::new(),
            namespaces: IndexMap::new(),
            children: vec![],
        };
        element_to_remove.set_attr("class", "some-class removeMe another-class");
        doc.root.children.push(Node::Element(element_to_remove));

        // Add element to keep
        let mut element_to_keep = Element {
            name: "circle".into(),
            attributes: IndexMap::new(),
            namespaces: IndexMap::new(),
            children: vec![],
        };
        element_to_keep.set_attr("class", "some-class keep-me another-class");
        doc.root.children.push(Node::Element(element_to_keep));

        let result = plugin.apply(&mut doc);
        assert!(result.is_ok());

        // Should have only one element remaining
        assert_eq!(doc.root.children.len(), 1);
        if let Node::Element(element) = &doc.root.children[0] {
            assert_eq!(element.name, "circle");
            assert_eq!(
                element.attr("class").map(|s| s.as_str()),
                Some("some-class keep-me another-class")
            );
        } else {
            panic!("Expected element");
        }
    }

    #[test]
    fn test_apply_no_config_does_nothing() {
        let plugin = RemoveElementsByAttrPlugin::new();
        let mut doc = create_test_document();

        // Add some elements
        let mut element = Element {
            name: "rect".into(),
            attributes: IndexMap::new(),
            namespaces: IndexMap::new(),
            children: vec![],
        };
        element.set_attr("id", "someId");
        doc.root.children.push(Node::Element(element));

        let result = plugin.apply(&mut doc);
        assert!(result.is_ok());

        // Should still have the element
        assert_eq!(doc.root.children.len(), 1);
    }

    #[test]
    fn test_apply_recursive() {
        let config = RemoveElementsByAttrConfig {
            id: vec!["removeMe".to_string()],
            class: vec![],
        };
        let plugin = RemoveElementsByAttrPlugin::with_config(config);
        let mut doc = create_test_document();

        // Create nested structure
        let mut nested_element = Element {
            name: "rect".into(),
            attributes: IndexMap::new(),
            namespaces: IndexMap::new(),
            children: vec![],
        };
        nested_element.set_attr("id", "removeMe");

        let mut group = Element {
            name: "g".into(),
            attributes: IndexMap::new(),
            namespaces: IndexMap::new(),
            children: vec![Node::Element(nested_element)],
        };
        group.set_attr("id", "group");

        doc.root.children.push(Node::Element(group));

        let result = plugin.apply(&mut doc);
        assert!(result.is_ok());

        // Group should remain but nested element should be removed
        assert_eq!(doc.root.children.len(), 1);
        if let Node::Element(group) = &doc.root.children[0] {
            assert_eq!(group.name, "g");
            assert_eq!(group.children.len(), 0); // Nested element removed
        } else {
            panic!("Expected group element");
        }
    }
}

        // Test element with matching class
        let mut element = Element {
            name: "rect".into(),
            attributes: IndexMap::new(),
            namespaces: IndexMap::new(),
            children: vec![],
        };
        element.set_attr("class", "someClass removeMe anotherClass");

        assert!(plugin.should_remove_element(&element));

        // Test element that shouldn't be removed
        let mut element2 = Element {
            name: "rect".into(),
            attributes: IndexMap::new(),
            namespaces: IndexMap::new(),
            children: vec![],
        };
        element2.set_attr("class", "someClass keepMe anotherClass");

        assert!(!plugin.should_remove_element(&element2));
    }

    #[test]
    fn test_apply_removes_by_id() {
        let config = RemoveElementsByAttrConfig {
            id: vec!["elementToRemove".to_string()],
            class: vec![],
        };
        let plugin = RemoveElementsByAttrPlugin::with_config(config);
        let mut doc = create_test_document();

        // Add element to remove
        let mut element_to_remove = Element {
            name: "rect".into(),
            attributes: IndexMap::new(),
            namespaces: IndexMap::new(),
            children: vec![],
        };
        element_to_remove.set_attr("id", "elementToRemove");
        doc.root.children.push(Node::Element(element_to_remove));

        // Add element to keep
        let mut element_to_keep = Element {
            name: "circle".into(),
            attributes: IndexMap::new(),
            namespaces: IndexMap::new(),
            children: vec![],
        };
        element_to_keep.set_attr("id", "elementToKeep");
        doc.root.children.push(Node::Element(element_to_keep));

        let result = plugin.apply(&mut doc);
        assert!(result.is_ok());

        // Should have only one element remaining
        assert_eq!(doc.root.children.len(), 1);
        if let Node::Element(element) = &doc.root.children[0] {
            assert_eq!(element.name, "circle");
            assert_eq!(element.attr("id").map(|s| s.as_str()), Some("elementToKeep"));
        } else {
            panic!("Expected element");
        }
    }

    #[test]
    fn test_apply_removes_by_class() {
        let config = RemoveElementsByAttrConfig {
            id: vec![],
            class: vec!["removeMe".to_string()],
        };
        let plugin = RemoveElementsByAttrPlugin::with_config(config);
        let mut doc = create_test_document();

        // Add element to remove
        let mut element_to_remove = Element {
            name: "rect".into(),
            attributes: IndexMap::new(),
            namespaces: IndexMap::new(),
            children: vec![],
        };
        element_to_remove.set_attr("class", "some-class removeMe another-class");
        doc.root.children.push(Node::Element(element_to_remove));

        // Add element to keep
        let mut element_to_keep = Element {
            name: "circle".into(),
            attributes: IndexMap::new(),
            namespaces: IndexMap::new(),
            children: vec![],
        };
        element_to_keep.set_attr("class", "some-class keep-me another-class");
        doc.root.children.push(Node::Element(element_to_keep));

        let result = plugin.apply(&mut doc);
        assert!(result.is_ok());

        // Should have only one element remaining
        assert_eq!(doc.root.children.len(), 1);
        if let Node::Element(element) = &doc.root.children[0] {
            assert_eq!(element.name, "circle");
            assert_eq!(
                element.attr("class").map(|s| s.as_str()),
                Some("some-class keep-me another-class")
            );
        } else {
            panic!("Expected element");
        }
    }

    #[test]
    fn test_apply_no_config_does_nothing() {
        let plugin = RemoveElementsByAttrPlugin::new();
        let mut doc = create_test_document();

        // Add some elements
        let mut element = Element {
            name: "rect".into(),
            attributes: IndexMap::new(),
            namespaces: IndexMap::new(),
            children: vec![],
        };
        element.set_attr("id", "someId");
        doc.root.children.push(Node::Element(element));

        let result = plugin.apply(&mut doc);
        assert!(result.is_ok());

        // Should still have the element
        assert_eq!(doc.root.children.len(), 1);
    }

    #[test]
    fn test_apply_recursive() {
        let config = RemoveElementsByAttrConfig {
            id: vec!["removeMe".to_string()],
            class: vec![],
        };
        let plugin = RemoveElementsByAttrPlugin::with_config(config);
        let mut doc = create_test_document();

        // Create nested structure
        let mut nested_element = Element {
            name: "rect".into(),
            attributes: IndexMap::new(),
            namespaces: IndexMap::new(),
            children: vec![],
        };
        nested_element.set_attr("id", "removeMe");

        let mut group = Element {
            name: "g".into(),
            attributes: IndexMap::new(),
            namespaces: IndexMap::new(),
            children: vec![Node::Element(nested_element)],
        };
        group.set_attr("id", "group");

        doc.root.children.push(Node::Element(group));

        let result = plugin.apply(&mut doc);
        assert!(result.is_ok());

        // Group should remain but nested element should be removed
        assert_eq!(doc.root.children.len(), 1);
        if let Node::Element(group) = &doc.root.children[0] {
            assert_eq!(group.name, "g");
            assert_eq!(group.children.len(), 0); // Nested element removed
        } else {
            panic!("Expected group element");
        }
    }
}
