// this_file: crates/plugin-sdk/src/plugins/convert_one_stop_gradients.rs

//! Converts one-stop (single color) gradients to a plain color
//!
//! This plugin identifies linear and radial gradients that contain only one stop
//! and replaces all references to these gradients with the solid color from that stop.
//! It also removes the gradient definitions and any empty defs elements that result.
//!
//! Reference: SVGOPROTECTED_70_;PROTECTED_71_)PROTECTED_72_static str {
        "convertOneStopGradients"
    }

    fn description(&self) -> &'static str {
        PROTECTED_29_
    }

    fn validate_params(&self, params: &Value) -> Result<()> {
        Self::parse_config(params)?;
        Ok(())
    }

    fn apply(&self, document: &mut Document) -> Result<()> {
        let mut gradients_to_remove = HashMap::new();
        let mut affected_defs = HashSet::new();

        // First pass: identify gradients with only one stop
        self.process_element(
            &mut document.root,
            &mut gradients_to_remove,
            false,
            &mut affected_defs,
        );

        // Second pass: replace gradient references with solid colors
        if !gradients_to_remove.is_empty() {
            self.replace_gradient_references(&mut document.root, &gradients_to_remove);

            // Third pass: remove the gradient elements
            self.remove_gradients(&mut document.root, &gradients_to_remove);

            // Fourth pass: remove empty defs elements
            self.remove_empty_defs(&mut document.root);

            // Remove unused xlink namespace
            self.remove_unused_xlink_namespace(document);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indexmap::IndexMap;
    use vexy_svgo_core::ast::{Document, Element, Node};

    fn create_test_document() -> Document<'static> {
        use std::collections::HashMap;
        Document {
            root: Element {
                name: "svg".into(),
                attributes: IndexMap::new(),
                namespaces: IndexMap::new(),
                children: vec![],
            },
            prologue: vec![],
            epilogue: vec![],
            metadata: vexy_svgo_core::ast::DocumentMetadata {
                path: None,
                encoding: None,
                version: None,
            },
        }
    }

    #[test]
    fn test_plugin_info() {
        let plugin = ConvertOneStopGradientsPlugin::new();
        assert_eq!(plugin.name(), "convertOneStopGradients");
        assert_eq!(
            plugin.description(),
            "converts one-stop (single color) gradients to a plain color"
        );
    }

    #[test]
    fn test_param_validation() {
        let plugin = ConvertOneStopGradientsPlugin::new();

        // Test null params
        assert!(plugin.validate_params(&Value::Null).is_ok());

        // Test empty object params
        assert!(plugin.validate_params(&serde_json::json!({})).is_ok());

        // Test invalid params
        assert!(plugin
            .validate_params(&serde_json::json!({
                "invalidParam": true
            }))
            .is_err());
    }

    #[test]
    fn test_extract_gradient_id() {
        let plugin = ConvertOneStopGradientsPlugin::new();

        // Test valid gradient ID extraction
        assert_eq!(
            plugin.extract_gradient_id("url(#myGradient)"),
            Some("myGradient".to_string())
        );
        assert_eq!(
            plugin.extract_gradient_id("url(#grad1)"),
            Some("grad1".to_string())
        );

        // Test invalid formats
        assert_eq!(plugin.extract_gradient_id("red"), None);
        assert_eq!(plugin.extract_gradient_id("url(myGradient)"), None);
        assert_eq!(plugin.extract_gradient_id("#myGradient"), None);
    }

    #[test]
    fn test_apply_with_empty_document() {
        let plugin = ConvertOneStopGradientsPlugin::new();
        let mut doc = create_test_document();

        // Should not panic with empty document
        let result = plugin.apply(&mut doc);
        assert!(result.is_ok());
    }

    #[test]
    fn test_apply_with_no_gradients() {
        let plugin = ConvertOneStopGradientsPlugin::new();
        let mut doc = create_test_document();

        // Add a simple rect element
        let mut rect_attrs = IndexMap::new();
        rect_attrs.insert("fill".to_string(), "red".to_string());
        rect_attrs.insert("width".to_string(), "100".to_string());
        rect_attrs.insert("height".to_string(), "100".to_string());

        doc.root.children.push(Node::Element(Element {
            name: "rect".into(),
            attributes: rect_attrs,
            namespaces: IndexMap::new(),
            children: vec![],
        }));

        let result = plugin.apply(&mut doc);
        assert!(result.is_ok());

        // Document should remain unchanged
        assert_eq!(doc.root.children.len(), 1);
        if let Node::Element(rect) = &doc.root.children[0] {
            assert_eq!(rect.name, "rect");
            assert_eq!(rect.attr("fill").map(|s| s.as_str()), Some("red"));
        }
    }

    #[test]
    fn test_one_stop_gradient_conversion() {
        let plugin = ConvertOneStopGradientsPlugin::new();
        let mut doc = create_test_document();

        // Add defs with a one-stop gradient
        let mut defs_elem = Element {
            name: "defs".into(),
            attributes: IndexMap::new(),
            namespaces: IndexMap::new(),
            children: vec![],
        };

        let mut gradient_attrs = IndexMap::new();
        gradient_attrs.insert("id".to_string(), "grad1".to_string());

        let mut stop_attrs = IndexMap::new();
        stop_attrs.insert("stop-color".to_string(), "#ff0000".to_string());

        let gradient_elem = Element {
            name: "linearGradient".into(),
            attributes: gradient_attrs,
            namespaces: IndexMap::new(),
            children: vec![Node::Element(Element {
                name: "stop".into(),
                attributes: stop_attrs,
                namespaces: IndexMap::new(),
                children: vec![],
            })],
        };

        defs_elem.children.push(Node::Element(gradient_elem));
        doc.root.children.push(Node::Element(defs_elem));

        // Add rect using the gradient
        let mut rect_attrs = IndexMap::new();
        rect_attrs.insert("fill".to_string(), "url(#grad1)".to_string());
        rect_attrs.insert("width".to_string(), "100".to_string());
        rect_attrs.insert("height".to_string(), "100".to_string());

        doc.root.children.push(Node::Element(Element {
            name: "rect".into(),
            attributes: rect_attrs,
            namespaces: IndexMap::new(),
            children: vec![],
        }));

        let result = plugin.apply(&mut doc);
        assert!(result.is_ok());

        // Check that gradient was removed and rect now has solid fill
        // The defs should be empty and removed
        let has_gradient = doc.root.children.iter().any(|child| {
            if let Node::Element(elem) = child {
                elem.children.iter().any(|child| {
                    if let Node::Element(e) = child {
                        e.name == "linearGradient" || e.name == "radialGradient"
                    } else {
                        false
                    }
                })
            } else {
                false
            }
        });
        assert!(!has_gradient);

        // Find the rect and check its fill
        let rect = doc.root.children.iter().find_map(|child| {
            if let Node::Element(elem) = child {
                if elem.name == "rect" {
                    Some(elem)
                } else {
                    None
                }
            } else {
                None
            }
        });

        assert!(rect.is_some());
        let rect = rect.unwrap();
        assert_eq!(rect.attr("fill").map(|s| s.as_str()), Some("#ff0000"));
    }
}
        }

        let has_xlink = check_xlink(&document.root);

        // Remove xmlns:xlink if no xlink:href attributes remain
        if !has_xlink {
            document.root.namespaces.remove("xlink");
            document.root.remove_attr("xmlns:xlink");
        }
    }
}

impl Default for ConvertOneStopGradientsPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for ConvertOneStopGradientsPlugin {
    fn name(&self) -> &'static str {
        PROTECTED_43_
    }

    fn description(&self) -> &'static str {
        "converts one-stop (single color) gradients to a plain color"
    }

    fn validate_params(&self, params: &Value) -> Result<()> {
        Self::parse_config(params)?;
        Ok(())
    }

    fn apply(&self, document: &mut Document) -> Result<()> {
        let mut gradients_to_remove = HashMap::new();
        let mut affected_defs = HashSet::new();

        // First pass: identify gradients with only one stop
        self.process_element(
            &mut document.root,
            &mut gradients_to_remove,
            false,
            &mut affected_defs,
        );

        // Second pass: replace gradient references with solid colors
        if !gradients_to_remove.is_empty() {
            self.replace_gradient_references(&mut document.root, &gradients_to_remove);

            // Third pass: remove the gradient elements
            self.remove_gradients(&mut document.root, &gradients_to_remove);

            // Fourth pass: remove empty defs elements
            self.remove_empty_defs(&mut document.root);

            // Remove unused xlink namespace
            self.remove_unused_xlink_namespace(document);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indexmap::IndexMap;
    use vexy_svgo_core::ast::{Document, Element, Node};

    fn create_test_document() -> Document<'static> {
        use std::collections::HashMap;
        Document {
            root: Element {
                name: "svg".into(),
                attributes: IndexMap::new(),
                namespaces: IndexMap::new(),
                children: vec![],
            },
            prologue: vec![],
            epilogue: vec![],
            metadata: vexy_svgo_core::ast::DocumentMetadata {
                path: None,
                encoding: None,
                version: None,
            },
        }
    }

    #[test]
    fn test_plugin_info() {
        let plugin = ConvertOneStopGradientsPlugin::new();
        assert_eq!(plugin.name(), "convertOneStopGradients");
        assert_eq!(
            plugin.description(),
            "converts one-stop (single color) gradients to a plain color"
        );
    }

    #[test]
    fn test_param_validation() {
        let plugin = ConvertOneStopGradientsPlugin::new();

        // Test null params
        assert!(plugin.validate_params(&Value::Null).is_ok());

        // Test empty object params
        assert!(plugin.validate_params(&serde_json::json!({})).is_ok());

        // Test invalid params
        assert!(plugin
            .validate_params(&serde_json::json!({
                "invalidParam": true
            }))
            .is_err());
    }

    #[test]
    fn test_extract_gradient_id() {
        let plugin = ConvertOneStopGradientsPlugin::new();

        // Test valid gradient ID extraction
        assert_eq!(
            plugin.extract_gradient_id("url(#myGradient)"),
            Some("myGradient".to_string())
        );
        assert_eq!(
            plugin.extract_gradient_id("url(#grad1)"),
            Some("grad1".to_string())
        );

        // Test invalid formats
        assert_eq!(plugin.extract_gradient_id("red"), None);
        assert_eq!(plugin.extract_gradient_id("url(myGradient)"), None);
        assert_eq!(plugin.extract_gradient_id("#myGradient"), None);
    }

    #[test]
    fn test_apply_with_empty_document() {
        let plugin = ConvertOneStopGradientsPlugin::new();
        let mut doc = create_test_document();

        // Should not panic with empty document
        let result = plugin.apply(&mut doc);
        assert!(result.is_ok());
    }

    #[test]
    fn test_apply_with_no_gradients() {
        let plugin = ConvertOneStopGradientsPlugin::new();
        let mut doc = create_test_document();

        // Add a simple rect element
        let mut rect_attrs = IndexMap::new();
        rect_attrs.insert("fill".to_string(), "red".to_string());
        rect_attrs.insert("width".to_string(), "100".to_string());
        rect_attrs.insert("height".to_string(), "100".to_string());

        doc.root.children.push(Node::Element(Element {
            name: "rect".into(),
            attributes: rect_attrs,
            namespaces: IndexMap::new(),
            children: vec![],
        }));

        let result = plugin.apply(&mut doc);
        assert!(result.is_ok());

        // Document should remain unchanged
        assert_eq!(doc.root.children.len(), 1);
        if let Node::Element(rect) = &doc.root.children[0] {
            assert_eq!(rect.name, "rect");
            assert_eq!(rect.attr("fill").map(|s| s.as_str()), Some("red"));
        }
    }

    #[test]
    fn test_one_stop_gradient_conversion() {
        let plugin = ConvertOneStopGradientsPlugin::new();
        let mut doc = create_test_document();

        // Add defs with a one-stop gradient
        let mut defs_elem = Element {
            name: "defs".into(),
            attributes: IndexMap::new(),
            namespaces: IndexMap::new(),
            children: vec![],
        };

        let mut gradient_attrs = IndexMap::new();
        gradient_attrs.insert("id".to_string(), "grad1".to_string());

        let mut stop_attrs = IndexMap::new();
        stop_attrs.insert("stop-color".to_string(), "#ff0000".to_string());

        let gradient_elem = Element {
            name: "linearGradient".into(),
            attributes: gradient_attrs,
            namespaces: IndexMap::new(),
            children: vec![Node::Element(Element {
                name: "stop".into(),
                attributes: stop_attrs,
                namespaces: IndexMap::new(),
                children: vec![],
            })],
        };

        defs_elem.children.push(Node::Element(gradient_elem));
        doc.root.children.push(Node::Element(defs_elem));

        // Add rect using the gradient
        let mut rect_attrs = IndexMap::new();
        rect_attrs.insert("fill".to_string(), "url(#grad1)".to_string());
        rect_attrs.insert("width".to_string(), "100".to_string());
        rect_attrs.insert("height".to_string(), "100".to_string());

        doc.root.children.push(Node::Element(Element {
            name: "rect".into(),
            attributes: rect_attrs,
            namespaces: IndexMap::new(),
            children: vec![],
        }));

        let result = plugin.apply(&mut doc);
        assert!(result.is_ok());

        // Check that gradient was removed and rect now has solid fill
        // The defs should be empty and removed
        let has_gradient = doc.root.children.iter().any(|child| {
            if let Node::Element(elem) = child {
                elem.children.iter().any(|child| {
                    if let Node::Element(e) = child {
                        e.name == "linearGradient" || e.name == "radialGradient"
                    } else {
                        false
                    }
                })
            } else {
                false
            }
        });
        assert!(!has_gradient);

        // Find the rect and check its fill
        let rect = doc.root.children.iter().find_map(|child| {
            if let Node::Element(elem) = child {
                if elem.name == "rect" {
                    Some(elem)
                } else {
                    None
                }
            } else {
                None
            }
        });

        assert!(rect.is_some());
        let rect = rect.unwrap();
        assert_eq!(rect.attr("fill").map(|s| s.as_str()), Some("#ff0000"));
    }
}
