// this_file: crates/plugin-sdk/src/plugins/convert_ellipse_to_circle.rs

//! Convert ellipse to circle plugin implementation
//!
//! This plugin converts non-eccentric `<ellipse>` elements to `<circle>` elements.
//! When an ellipse has equal rx and ry attributes, it can be more efficiently
//! represented as a circle.
//!
//! Reference: https://www.w3.org/TR/SVG11/shapes.html

use crate::Plugin;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use vexy_svgo_core::ast::{Document, Element, Node};

/// Configuration parameters for convert ellipse to circle plugin (currently empty)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ConvertEllipseToCircleConfig {
    // No configuration options - matches SVGO behavior
}

impl Default for ConvertEllipseToCircleConfig {
    fn default() -> Self {
        Self {}
    }
}

/// Plugin that converts ellipse elements to circle elements when appropriate
pub struct ConvertEllipseToCirclePlugin {
    config: ConvertEllipseToCircleConfig,
}

impl ConvertEllipseToCirclePlugin {
    /// Create a new ConvertEllipseToCirclePlugin
    pub fn new() -> Self {
        Self {
            config: ConvertEllipseToCircleConfig::default(),
        }
    }

    /// Create a new ConvertEllipseToCirclePlugin with config
    pub fn with_config(config: ConvertEllipseToCircleConfig) -> Self {
        Self { config }
    }

    /// Parse configuration from JSON
    fn parse_config(params: &Value) -> Result<ConvertEllipseToCircleConfig> {
        if params.is_null() || (params.is_object() && params.as_object().unwrap().is_empty()) {
            Ok(ConvertEllipseToCircleConfig::default())
        } else if params.is_object() {
            serde_json::from_value(params.clone())
                .map_err(|e| anyhow::anyhow!("Invalid configuration: {}", e))
        } else {
            Ok(ConvertEllipseToCircleConfig::default())
        }
    }

    /// Recursively convert ellipse elements to circle elements
    fn convert_ellipse_to_circle_recursive(&self, element: &mut Element) {
        // Process this element if it's an ellipse
        if element.name == "ellipse" {
            let rx = element
                .attributes
                .get("rx")
                .cloned()
                .unwrap_or_else(|| "0".to_string());
            let ry = element
                .attributes
                .get("ry")
                .cloned()
                .unwrap_or_else(|| "0".into());

            // Convert to circle if rx == ry or either is "auto"
            if rx == ry || rx == "auto" || ry == "auto" {
                element.name = "circle".into();

                // Choose the appropriate radius value
                let radius = if rx == "auto" { ry } else { rx };

                // Remove rx and ry attributes
                element.attributes.remove("rx");
                element.attributes.remove("ry");

                // Add r attribute
                element.attributes.insert("r".into(), radius.into());
            }
        }

        // Process child elements recursively
        for child in &mut element.children {
            if let Node::Element(elem) = child {
                self.convert_ellipse_to_circle_recursive(elem);
            }
        }
    }
}

impl Default for ConvertEllipseToCirclePlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for ConvertEllipseToCirclePlugin {
    fn name(&self) -> &'static str {
        "convertEllipseToCircle"
    }

    fn description(&self) -> &'static str {
        "converts non-eccentric <ellipse>s to <circle>s"
    }

    fn validate_params(&self, params: &Value) -> Result<()> {
        Self::parse_config(params)?;
        Ok(())
    }

    fn apply(&self, document: &mut Document) -> Result<()> {
        // Convert ellipse elements to circle elements
        self.convert_ellipse_to_circle_recursive(&mut document.root);
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

    fn count_elements_by_name(element: &Element, name: &str) -> usize {
        let mut count = 0;
        for child in &element.children {
            if let Node::Element(elem) = child {
                if elem.name == name {
                    count += 1;
                }
                count += count_elements_by_name(elem, name);
            }
        }
        count
    }

    #[test]
    fn test_plugin_creation() {
        let plugin = ConvertEllipseToCirclePlugin::new();
        assert_eq!(plugin.name(), "convertEllipseToCircle");
        assert_eq!(
            plugin.description(),
            "converts non-eccentric <ellipse>s to <circle>s"
        );
    }

    #[test]
    fn test_parameter_validation() {
        let plugin = ConvertEllipseToCirclePlugin::new();

        // Valid parameters (empty object)
        assert!(plugin.validate_params(&json!({})).is_ok());

        // Invalid parameters (non-empty object)
        assert!(plugin.validate_params(&json!({"param": "value"})).is_err());
    }

    #[test]
    fn test_convert_ellipse_equal_radii() {
        let plugin = ConvertEllipseToCirclePlugin::new();
        let mut doc = Document::new();

        // Create ellipse with equal rx and ry
        let mut ellipse = create_element("ellipse");
        ellipse
            .attributes
            .insert("rx".to_string(), "10".to_string());
        ellipse
            .attributes
            .insert("ry".to_string(), "10".to_string());
        ellipse
            .attributes
            .insert("cx".to_string(), "50".to_string());
        ellipse
            .attributes
            .insert("cy".to_string(), "50".to_string());
        doc.root.children.push(Node::Element(ellipse));

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Should have converted to circle
        assert_eq!(count_elements_by_name(&doc.root, "ellipse"), 0);
        assert_eq!(count_elements_by_name(&doc.root, "circle"), 1);

        // Check attributes
        if let Node::Element(elem) = &doc.root.children[0] {
            assert_eq!(elem.name, "circle");
            assert_eq!(elem.attributes.get("r"), Some(&"10".to_string()));
            assert_eq!(elem.attributes.get("cx"), Some(&"50".to_string()));
            assert_eq!(elem.attributes.get("cy"), Some(&"50".to_string()));
            assert!(!elem.attributes.contains_key("rx"));
            assert!(!elem.attributes.contains_key("ry"));
        }
    }

    #[test]
    fn test_convert_ellipse_auto_rx() {
        let plugin = ConvertEllipseToCirclePlugin::new();
        let mut doc = Document::new();

        // Create ellipse with rx="auto"
        let mut ellipse = create_element("ellipse");
        ellipse
            .attributes
            .insert("rx".to_string(), "auto".to_string());
        ellipse
            .attributes
            .insert("ry".to_string(), "15".to_string());
        doc.root.children.push(Node::Element(ellipse));

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Should have converted to circle with r=ry
        assert_eq!(count_elements_by_name(&doc.root, "ellipse"), 0);
        assert_eq!(count_elements_by_name(&doc.root, "circle"), 1);

        // Check attributes
        if let Node::Element(elem) = &doc.root.children[0] {
            assert_eq!(elem.name, "circle");
            assert_eq!(elem.attributes.get("r"), Some(&"15".to_string()));
            assert!(!elem.attributes.contains_key("rx"));
            assert!(!elem.attributes.contains_key("ry"));
        }
    }

    #[test]
    fn test_convert_ellipse_auto_ry() {
        let plugin = ConvertEllipseToCirclePlugin::new();
        let mut doc = Document::new();

        // Create ellipse with ry="auto"
        let mut ellipse = create_element("ellipse");
        ellipse
            .attributes
            .insert("rx".to_string(), "20".to_string());
        ellipse
            .attributes
            .insert("ry".to_string(), "auto".to_string());
        doc.root.children.push(Node::Element(ellipse));

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Should have converted to circle with r=rx
        assert_eq!(count_elements_by_name(&doc.root, "ellipse"), 0);
        assert_eq!(count_elements_by_name(&doc.root, "circle"), 1);

        // Check attributes
        if let Node::Element(elem) = &doc.root.children[0] {
            assert_eq!(elem.name, "circle");
            assert_eq!(elem.attributes.get("r"), Some(&"20".to_string()));
            assert!(!elem.attributes.contains_key("rx"));
            assert!(!elem.attributes.contains_key("ry"));
        }
    }

    #[test]
    fn test_keep_ellipse_different_radii() {
        let plugin = ConvertEllipseToCirclePlugin::new();
        let mut doc = Document::new();

        // Create ellipse with different rx and ry
        let mut ellipse = create_element("ellipse");
        ellipse
            .attributes
            .insert("rx".to_string(), "10".to_string());
        ellipse
            .attributes
            .insert("ry".to_string(), "20".to_string());
        doc.root.children.push(Node::Element(ellipse));

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Should keep as ellipse
        assert_eq!(count_elements_by_name(&doc.root, "ellipse"), 1);
        assert_eq!(count_elements_by_name(&doc.root, "circle"), 0);

        // Check attributes unchanged
        if let Node::Element(elem) = &doc.root.children[0] {
            assert_eq!(elem.name, "ellipse");
            assert_eq!(elem.attributes.get("rx"), Some(&"10".to_string()));
            assert_eq!(elem.attributes.get("ry"), Some(&"20".to_string()));
        }
    }

    #[test]
    fn test_ellipse_default_attributes() {
        let plugin = ConvertEllipseToCirclePlugin::new();
        let mut doc = Document::new();

        // Create ellipse without rx/ry (defaults to 0)
        let ellipse = create_element("ellipse");
        doc.root.children.push(Node::Element(ellipse));

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Should have converted to circle with r=0
        assert_eq!(count_elements_by_name(&doc.root, "ellipse"), 0);
        assert_eq!(count_elements_by_name(&doc.root, "circle"), 1);

        // Check attributes
        if let Node::Element(elem) = &doc.root.children[0] {
            assert_eq!(elem.name, "circle");
            assert_eq!(elem.attributes.get("r"), Some(&"0".to_string()));
        }
    }

    #[test]
    fn test_ellipse_with_only_rx() {
        let plugin = ConvertEllipseToCirclePlugin::new();
        let mut doc = Document::new();

        // Create ellipse with only rx (ry defaults to 0)
        let mut ellipse = create_element("ellipse");
        ellipse.attributes.insert("rx".to_string(), "0".to_string());
        doc.root.children.push(Node::Element(ellipse));

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Should have converted to circle with r=0
        assert_eq!(count_elements_by_name(&doc.root, "ellipse"), 0);
        assert_eq!(count_elements_by_name(&doc.root, "circle"), 1);

        // Check attributes
        if let Node::Element(elem) = &doc.root.children[0] {
            assert_eq!(elem.name, "circle");
            assert_eq!(elem.attributes.get("r"), Some(&"0".to_string()));
        }
    }

    #[test]
    fn test_nested_ellipses() {
        let plugin = ConvertEllipseToCirclePlugin::new();
        let mut doc = Document::new();

        // Create a group with nested ellipses
        let mut group = create_element("g");

        // Ellipse that should be converted
        let mut ellipse1 = create_element("ellipse");
        ellipse1
            .attributes
            .insert("rx".to_string(), "5".to_string());
        ellipse1
            .attributes
            .insert("ry".to_string(), "5".to_string());
        group.children.push(Node::Element(ellipse1));

        // Ellipse that should remain unchanged
        let mut ellipse2 = create_element("ellipse");
        ellipse2
            .attributes
            .insert("rx".to_string(), "5".to_string());
        ellipse2
            .attributes
            .insert("ry".to_string(), "10".to_string());
        group.children.push(Node::Element(ellipse2));

        doc.root.children.push(Node::Element(group));

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Should have converted one ellipse to circle
        assert_eq!(count_elements_by_name(&doc.root, "ellipse"), 1);
        assert_eq!(count_elements_by_name(&doc.root, "circle"), 1);
    }

    #[test]
    fn test_no_ellipses() {
        let plugin = ConvertEllipseToCirclePlugin::new();
        let mut doc = Document::new();

        // Add non-ellipse elements
        let rect = create_element("rect");
        doc.root.children.push(Node::Element(rect));

        let circle = create_element("circle");
        doc.root.children.push(Node::Element(circle));

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Should have no changes
        assert_eq!(count_elements_by_name(&doc.root, "ellipse"), 0);
        assert_eq!(count_elements_by_name(&doc.root, "circle"), 1);
        assert_eq!(count_elements_by_name(&doc.root, "rect"), 1);
    }

    #[test]
    fn test_config_parsing() {
        let config = ConvertEllipseToCirclePlugin::parse_config(&json!({})).unwrap();
        // No fields to check since config is empty
        let _ = config;
    }
}

// Use parameterized testing framework for SVGO fixture tests
crate::plugin_fixture_tests!(ConvertEllipseToCirclePlugin, "convertEllipseToCircle");
