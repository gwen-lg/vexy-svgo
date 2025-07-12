// this_file: crates/plugin-sdk/src/plugins/remove_useless_transforms.rs

//! Removes transform attributes that are no-op
//!
//! Removes identity/no-op transforms from SVG elements such as translate(0,0), scale(1), rotate(0)
//!
//! Reference: SVGO's removeUselessTransforms plugin

use crate::Plugin;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use vexy_svgo_core::ast::{Document, Element, Node};

/// Configuration for the removeUselessTransforms plugin
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RemoveUselessTransformsConfig {}

impl Default for RemoveUselessTransformsConfig {
    fn default() -> Self {
        Self {}
    }
}

/// Main plugin struct
pub struct RemoveUselessTransformsPlugin {
    config: RemoveUselessTransformsConfig,
}

impl RemoveUselessTransformsPlugin {
    pub fn new() -> Self {
        Self {
            config: RemoveUselessTransformsConfig::default(),
        }
    }

    pub fn with_config(config: RemoveUselessTransformsConfig) -> Self {
        Self { config }
    }

    fn parse_config(params: &Value) -> Result<RemoveUselessTransformsConfig> {
        if params.is_null() {
            Ok(RemoveUselessTransformsConfig::default())
        } else {
            serde_json::from_value(params.clone())
                .map_err(|e| anyhow::anyhow!("Invalid plugin configuration: {}", e))
        }
    }

    fn process_element(&self, element: &mut Element) {
        // Process children first
        let mut i = 0;
        while i < element.children.len() {
            if let Node::Element(child) = &mut element.children[i] {
                self.process_element(child);
            }
            i += 1;
        }

        // Check and remove if transform is no-op
        if let Some(transform_str) = element.attr("transform") {
            if self.is_useless_transform(&transform_str) {
                element.remove_attr("transform");
            }
        }
    }

    /// Returns true if the transform string matches a no-op transform
    fn is_useless_transform(&self, s: &str) -> bool {
        let t = s.trim();

        // Check for various no-op transforms with different syntaxes
        matches!(
            t,
            "translate(0,0)"
                | "translate(0, 0)"
                | "translate(0 0)"
                | "rotate(0)"
                | "scale(1)"
                | "scale(1,1)"
                | "scale(1, 1)"
                | "scale(1 1)"
                | "skewX(0)"
                | "skewY(0)"
                | "matrix(1,0,0,1,0,0)"
                | "matrix(1, 0, 0, 1, 0, 0)"
                | "matrix(1 0 0 1 0 0)"
        )
    }
}

impl Default for RemoveUselessTransformsPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for RemoveUselessTransformsPlugin {
    fn name(&self) -> &'static str {
        "removeUselessTransforms"
    }

    fn description(&self) -> &'static str {
        "removes transform attributes that are no-op"
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
    use vexy_svgo_core::ast::{Document, Element, Node};

    #[test]
    fn test_plugin_info() {
        let plugin = RemoveUselessTransformsPlugin::new();
        assert_eq!(plugin.name(), "removeUselessTransforms");
        assert_eq!(
            plugin.description(),
            "removes transform attributes that are no-op"
        );
    }

    #[test]
    fn test_param_validation() {
        let plugin = RemoveUselessTransformsPlugin::new();

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
    fn test_remove_identity_translate() {
        let mut doc = Document::new();
        let mut g = Element::new("g");
        g.set_attr("transform", "translate(0,0)");
        doc.root.children.push(Node::Element(g));

        let plugin = RemoveUselessTransformsPlugin::new();
        plugin.apply(&mut doc).unwrap();

        let g = match &doc.root.children[0] {
            Node::Element(e) => e,
            _ => panic!("Expected element"),
        };
        assert!(!g.has_attr("transform"));
    }

    #[test]
    fn test_remove_identity_scale() {
        let mut doc = Document::new();
        let mut g = Element::new("g");
        g.set_attr("transform", "scale(1)");
        doc.root.children.push(Node::Element(g));

        let plugin = RemoveUselessTransformsPlugin::new();
        plugin.apply(&mut doc).unwrap();

        let g = match &doc.root.children[0] {
            Node::Element(e) => e,
            _ => panic!("Expected element"),
        };
        assert!(!g.has_attr("transform"));
    }

    #[test]
    fn test_preserve_non_identity_transform() {
        let mut doc = Document::new();
        let mut g = Element::new("g");
        g.set_attr("transform", "translate(10,20)");
        doc.root.children.push(Node::Element(g));

        let plugin = RemoveUselessTransformsPlugin::new();
        plugin.apply(&mut doc).unwrap();

        let g = match &doc.root.children[0] {
            Node::Element(e) => e,
            _ => panic!("Expected element"),
        };
        assert_eq!(g.attr("transform").map(|s| s.as_str()), Some("translate(10,20)"));
    }

    #[test]
    fn test_is_useless_transform() {
        let plugin = RemoveUselessTransformsPlugin::new();

        assert!(plugin.is_useless_transform("translate(0,0)"));
        assert!(plugin.is_useless_transform("translate(0, 0)"));
        assert!(plugin.is_useless_transform("translate(0 0)"));
        assert!(plugin.is_useless_transform("rotate(0)"));
        assert!(plugin.is_useless_transform("scale(1)"));
        assert!(plugin.is_useless_transform("scale(1,1)"));
        assert!(plugin.is_useless_transform("scale(1, 1)"));
        assert!(plugin.is_useless_transform("skewX(0)"));
        assert!(plugin.is_useless_transform("skewY(0)"));
        assert!(plugin.is_useless_transform("matrix(1 0 0 1 0 0)"));
        assert!(plugin.is_useless_transform(" translate(0,0) "));

        assert!(!plugin.is_useless_transform("translate(10,0)"));
        assert!(!plugin.is_useless_transform("translate(0,10)"));
        assert!(!plugin.is_useless_transform("rotate(45)"));
        assert!(!plugin.is_useless_transform("scale(2)"));
        assert!(!plugin.is_useless_transform("scale(1,2)"));
    }

    #[test]
    fn test_nested_elements() {
        let mut doc = Document::new();
        let mut g = Element::new("g");
        g.set_attr("transform", "translate(0,0)");

        let mut inner_g = Element::new("g");
        inner_g.set_attr("transform", "scale(1)");

        let mut rect = Element::new("rect");
        rect.set_attr("transform", "rotate(0)");

        inner_g.children.push(Node::Element(rect));
        g.children.push(Node::Element(inner_g));
        doc.root.children.push(Node::Element(g));

        let plugin = RemoveUselessTransformsPlugin::new();
        plugin.apply(&mut doc).unwrap();

        // Check that all useless transforms were removed
        let g = match &doc.root.children[0] {
            Node::Element(e) => e,
            _ => panic!("Expected element"),
        };
        assert!(!g.has_attr("transform"));

        let inner_g = match &g.children[0] {
            Node::Element(e) => e,
            _ => panic!("Expected element"),
        };
        assert!(!inner_g.has_attr("transform"));

        let rect = match &inner_g.children[0] {
            Node::Element(e) => e,
            _ => panic!("Expected element"),
        };
        assert!(!rect.has_attr("transform"));
    }
}
