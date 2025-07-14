// this_file: crates/plugin-sdk/src/plugins/remove_non_inheritable_group_attrs.rs

//! Remove non-inheritable group attributes plugin implementation
//!
//! This plugin removes non-inheritable group's presentation attributes.
//! Group elements can only inherit certain presentation attributes,
//! and this plugin removes attributes that cannot be inherited.
//!
//! The plugin checks if an attribute is:
//! 1. A presentation attribute (can affect visual rendering)
//! 2. NOT inheritable by child elements
//! 3. NOT allowed as a group attribute (exceptions for groups)
//!
//! Reference: SVGO's removeNonInheritableGroupAttrs plugin

use crate::Plugin;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashSet;
use vexy_svgo_core::ast::{Document, Element, Node};

/// Configuration parameters for remove non-inheritable group attributes plugin (currently empty)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RemoveNonInheritableGroupAttrsConfig {
    // No configuration options - matches SVGO behavior
}

impl Default for RemoveNonInheritableGroupAttrsConfig {
    fn default() -> Self {
        Self {}
    }
}

/// Plugin that removes non-inheritable group attributes
pub struct RemoveNonInheritableGroupAttrsPlugin {
    config: RemoveNonInheritableGroupAttrsConfig,
    presentation_attrs: HashSet<&'static str>,
    inheritable_attrs: HashSet<&'static str>,
    presentation_non_inheritable_group_attrs: HashSet<&'static str>,
}

impl RemoveNonInheritableGroupAttrsPlugin {
    /// Create a new RemoveNonInheritableGroupAttrsPlugin
    pub fn new() -> Self {
        // Presentation attributes (can affect visual rendering)
        let presentation_attrs = [
            "alignment-baseline",
            "baseline-shift",
            "clip-path",
            "clip-rule",
            "clip",
            "color-interpolation-filters",
            "color-interpolation",
            "color-profile",
            "color-rendering",
            "color",
            "cursor",
            "direction",
            "display",
            "dominant-baseline",
            "enable-background",
            "fill-opacity",
            "fill-rule",
            "fill",
            "filter",
            "flood-color",
            "flood-opacity",
            "font-family",
            "font-size-adjust",
            "font-size",
            "font-stretch",
            "font-style",
            "font-variant",
            "font-weight",
            "glyph-orientation-horizontal",
            "glyph-orientation-vertical",
            "image-rendering",
            "letter-spacing",
            "lighting-color",
            "marker-end",
            "marker-mid",
            "marker-start",
            "mask",
            "opacity",
            "overflow",
            "paint-order",
            "pointer-events",
            "shape-rendering",
            "stop-color",
            "stop-opacity",
            "stroke-dasharray",
            "stroke-dashoffset",
            "stroke-linecap",
            "stroke-linejoin",
            "stroke-miterlimit",
            "stroke-opacity",
            "stroke-width",
            "stroke",
            "text-anchor",
            "text-decoration",
            "text-rendering",
            "transform",
            "unicode-bidi",
            "vector-effect",
            "visibility",
            "word-spacing",
            "writing-mode",
        ]
        .iter()
        .cloned()
        .collect();

        // Inheritable attributes (can be inherited by child elements)
        let inheritable_attrs = [
            "clip-rule",
            "color-interpolation-filters",
            "color-interpolation",
            "color-profile",
            "color-rendering",
            "color",
            "cursor",
            "direction",
            "dominant-baseline",
            "fill-opacity",
            "fill-rule",
            "fill",
            "font-family",
            "font-size-adjust",
            "font-size",
            "font-stretch",
            "font-style",
            "font-variant",
            "font-weight",
            "font",
            "glyph-orientation-horizontal",
            "glyph-orientation-vertical",
            "image-rendering",
            "letter-spacing",
            "marker-end",
            "marker-mid",
            "marker-start",
            "marker",
            "paint-order",
            "pointer-events",
            "shape-rendering",
            "stroke-dasharray",
            "stroke-dashoffset",
            "stroke-linecap",
            "stroke-linejoin",
            "stroke-miterlimit",
            "stroke-opacity",
            "stroke-width",
            "stroke",
            "text-anchor",
            "text-rendering",
            "visibility",
            "word-spacing",
            "writing-mode",
        ]
        .iter()
        .cloned()
        .collect();

        // Presentation attributes that are allowed on groups even if non-inheritable
        let presentation_non_inheritable_group_attrs = [
            "clip-path",
            "display",
            "filter",
            "mask",
            "opacity",
            "text-decoration",
            "transform",
            "unicode-bidi",
        ]
        .iter()
        .cloned()
        .collect();

        Self {
            config: RemoveNonInheritableGroupAttrsConfig::default(),
            presentation_attrs,
            inheritable_attrs,
            presentation_non_inheritable_group_attrs,
        }
    }

    /// Create a new RemoveNonInheritableGroupAttrsPlugin with config
    pub fn with_config(config: RemoveNonInheritableGroupAttrsConfig) -> Self {
        let mut plugin = Self::new();
        plugin.config = config;
        plugin
    }

    /// Parse configuration from JSON
    fn parse_config(params: &Value) -> Result<RemoveNonInheritableGroupAttrsConfig> {
        if params.is_null() || (params.is_object() && params.as_object().unwrap().is_empty()) {
            Ok(RemoveNonInheritableGroupAttrsConfig::default())
        } else if params.is_object() {
            serde_json::from_value(params.clone())
                .map_err(|e| anyhow::anyhow!("Invalid configuration: {}", e))
        } else {
            Ok(RemoveNonInheritableGroupAttrsConfig::default())
        }
    }

    /// Remove non-inheritable attributes from group elements
    fn remove_non_inheritable_group_attrs_recursive(&self, element: &mut Element) {
        // Process this element if it's a group
        if element.name == "g" {
            let mut attrs_to_remove = Vec::new();

            for attr_name in element.attributes.keys() {
                // Remove if:
                // 1. It's a presentation attribute AND
                // 2. It's NOT inheritable AND
                // 3. It's NOT allowed as a group attribute
                if self.presentation_attrs.contains(attr_name.as_ref())
                    && !self.inheritable_attrs.contains(attr_name.as_ref())
                    && !self
                        .presentation_non_inheritable_group_attrs
                        .contains(attr_name.as_ref())
                {
                    attrs_to_remove.push(attr_name.clone());
                }
            }

            // Remove the identified attributes
            for attr_name in attrs_to_remove {
                element.attributes.shift_remove(&attr_name);
            }
        }

        // Process child elements recursively
        for child in &mut element.children {
            if let Node::Element(elem) = child {
                self.remove_non_inheritable_group_attrs_recursive(elem);
            }
        }
    }
}

impl Default for RemoveNonInheritableGroupAttrsPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for RemoveNonInheritableGroupAttrsPlugin {
    fn name(&self) -> &'static str {
        "removeNonInheritableGroupAttrs"
    }

    fn description(&self) -> &'static str {
        "removes non-inheritable group's presentational attributes"
    }

    fn validate_params(&self, params: &Value) -> Result<()> {
        Self::parse_config(params)?;
        Ok(())
    }

    fn apply(&self, document: &mut Document) -> Result<()> {
        self.remove_non_inheritable_group_attrs_recursive(&mut document.root);
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
        let plugin = RemoveNonInheritableGroupAttrsPlugin::new();
        assert_eq!(plugin.name(), "removeNonInheritableGroupAttrs");
        assert_eq!(
            plugin.description(),
            "removes non-inheritable group's presentational attributes"
        );
    }

    #[test]
    fn test_parameter_validation() {
        let plugin = RemoveNonInheritableGroupAttrsPlugin::new();

        // Valid parameters (empty object)
        assert!(plugin.validate_params(&json!({})).is_ok());

        // Invalid parameters (non-empty object)
        assert!(plugin.validate_params(&json!({"param": "value"})).is_err());
    }

    #[test]
    fn test_removes_non_inheritable_presentation_attrs() {
        let plugin = RemoveNonInheritableGroupAttrsPlugin::new();
        let mut doc = Document::new();

        // Create group with non-inheritable presentation attributes
        let mut group = create_element("g");
        // These should be removed (presentation, non-inheritable, not group-allowed)
        group
            .attributes
            .insert("alignment-baseline".into(), "central".into());
        group
            .attributes
            .insert("baseline-shift".into(), "10px".into());
        group
            .attributes
            .insert("lighting-color".into(), "red".into());
        doc.root.children.push(Node::Element(group));

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Check that non-inheritable attributes were removed
        if let Node::Element(elem) = &doc.root.children[0] {
            assert!(!elem.attributes.contains_key("alignment-baseline"));
            assert!(!elem.attributes.contains_key("baseline-shift"));
            assert!(!elem.attributes.contains_key("lighting-color"));
        }
    }

    #[test]
    fn test_preserves_inheritable_attrs() {
        let plugin = RemoveNonInheritableGroupAttrsPlugin::new();
        let mut doc = Document::new();

        // Create group with inheritable presentation attributes
        let mut group = create_element("g");
        // These should be preserved (presentation and inheritable)
        group
            .attributes
            .insert("fill".into(), "red".into());
        group
            .attributes
            .insert("stroke".into(), "blue".into());
        group
            .attributes
            .insert("font-family".into(), "Arial".into());
        group
            .attributes
            .insert("color".into(), "green".into());
        doc.root.children.push(Node::Element(group));

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Check that inheritable attributes were preserved
        if let Node::Element(elem) = &doc.root.children[0] {
            assert_eq!(elem.attributes.get("fill"), Some(&"red".into()));
            assert_eq!(elem.attributes.get("stroke"), Some(&"blue".into()));
            assert_eq!(
                elem.attributes.get("font-family"),
                Some(&"Arial".into())
            );
            assert_eq!(elem.attributes.get("color"), Some(&"green".into()));
        }
    }

    #[test]
    fn test_preserves_allowed_group_attrs() {
        let plugin = RemoveNonInheritableGroupAttrsPlugin::new();
        let mut doc = Document::new();

        // Create group with allowed non-inheritable group attributes
        let mut group = create_element("g");
        // These should be preserved (non-inheritable but allowed on groups)
        group
            .attributes
            .insert("opacity".into(), "0.5".into());
        group
            .attributes
            .insert("transform".into(), "translate(10,20)".into());
        group
            .attributes
            .insert("clip-path".into(), "url(#clip)".into());
        group
            .attributes
            .insert("filter".into(), "url(#filter)".into());
        doc.root.children.push(Node::Element(group));

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Check that allowed group attributes were preserved
        if let Node::Element(elem) = &doc.root.children[0] {
            assert_eq!(elem.attributes.get("opacity"), Some(&"0.5".into()));
            assert_eq!(
                elem.attributes.get("transform"),
                Some(&"translate(10,20)".into())
            );
            assert_eq!(
                elem.attributes.get("clip-path"),
                Some(&"url(#clip)".into())
            );
            assert_eq!(
                elem.attributes.get("filter"),
                Some(&"url(#filter)".into())
            );
        }
    }

    #[test]
    fn test_preserves_non_presentation_attrs() {
        let plugin = RemoveNonInheritableGroupAttrsPlugin::new();
        let mut doc = Document::new();

        // Create group with non-presentation attributes
        let mut group = create_element("g");
        // These should be preserved (not presentation attributes)
        group
            .attributes
            .insert("id".into(), "mygroup".into());
        group
            .attributes
            .insert("class".into(), "groupclass".into());
        group
            .attributes
            .insert("data-custom".into(), "value".into());
        doc.root.children.push(Node::Element(group));

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Check that non-presentation attributes were preserved
        if let Node::Element(elem) = &doc.root.children[0] {
            assert_eq!(elem.attributes.get("id"), Some(&"mygroup".into()));
            assert_eq!(
                elem.attributes.get("class"),
                Some(&"groupclass".into())
            );
            assert_eq!(
                elem.attributes.get("data-custom"),
                Some(&"value".into())
            );
        }
    }

    #[test]
    fn test_ignores_non_group_elements() {
        let plugin = RemoveNonInheritableGroupAttrsPlugin::new();
        let mut doc = Document::new();

        // Create non-group element with non-inheritable attributes
        let mut rect = create_element("rect");
        rect.attributes
            .insert("alignment-baseline".into(), "central".into());
        rect.attributes
            .insert("baseline-shift".into(), "10px".into());
        doc.root.children.push(Node::Element(rect));

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Check that attributes on non-group elements are preserved
        if let Node::Element(elem) = &doc.root.children[0] {
            assert_eq!(
                elem.attributes.get("alignment-baseline"),
                Some(&"central".into())
            );
            assert_eq!(
                elem.attributes.get("baseline-shift"),
                Some(&"10px".into())
            );
        }
    }

    #[test]
    fn test_mixed_attributes() {
        let plugin = RemoveNonInheritableGroupAttrsPlugin::new();
        let mut doc = Document::new();

        // Create group with mixed attributes
        let mut group = create_element("g");
        // Should be removed
        group
            .attributes
            .insert("alignment-baseline".into(), "central".into());
        group
            .attributes
            .insert("stop-color".into(), "red".into());
        // Should be preserved (inheritable)
        group
            .attributes
            .insert("fill".into(), "blue".into());
        group
            .attributes
            .insert("stroke".into(), "green".into());
        // Should be preserved (allowed group attribute)
        group
            .attributes
            .insert("opacity".into(), "0.8".into());
        // Should be preserved (non-presentation)
        group
            .attributes
            .insert("id".into(), "test".into());
        doc.root.children.push(Node::Element(group));

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        if let Node::Element(elem) = &doc.root.children[0] {
            // Removed attributes
            assert!(!elem.attributes.contains_key("alignment-baseline"));
            assert!(!elem.attributes.contains_key("stop-color"));

            // Preserved attributes
            assert_eq!(elem.attributes.get("fill"), Some(&"blue".into()));
            assert_eq!(elem.attributes.get("stroke"), Some(&"green".into()));
            assert_eq!(elem.attributes.get("opacity"), Some(&"0.8".into()));
            assert_eq!(elem.attributes.get("id"), Some(&"test".into()));
        }
    }

    #[test]
    fn test_nested_groups() {
        let plugin = RemoveNonInheritableGroupAttrsPlugin::new();
        let mut doc = Document::new();

        // Create nested groups
        let mut outer_group = create_element("g");
        outer_group
            .attributes
            .insert("alignment-baseline".into(), "central".into());
        outer_group
            .attributes
            .insert("fill".into(), "red".into());

        let mut inner_group = create_element("g");
        inner_group
            .attributes
            .insert("baseline-shift".into(), "10px".into());
        inner_group
            .attributes
            .insert("stroke".into(), "blue".into());

        outer_group.children.push(Node::Element(inner_group));
        doc.root.children.push(Node::Element(outer_group));

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        if let Node::Element(outer_elem) = &doc.root.children[0] {
            // Outer group: removed non-inheritable, kept inheritable
            assert!(!outer_elem.attributes.contains_key("alignment-baseline"));
            assert_eq!(outer_elem.attributes.get("fill"), Some(&"red".into()));

            if let Node::Element(inner_elem) = &outer_elem.children[0] {
                // Inner group: removed non-inheritable, kept inheritable
                assert!(!inner_elem.attributes.contains_key("baseline-shift"));
                assert_eq!(
                    inner_elem.attributes.get("stroke"),
                    Some(&"blue".into())
                );
            }
        }
    }

    #[test]
    fn test_empty_group() {
        let plugin = RemoveNonInheritableGroupAttrsPlugin::new();
        let mut doc = Document::new();

        // Create empty group
        let group = create_element("g");
        doc.root.children.push(Node::Element(group));

        // Apply plugin - should not crash
        let result = plugin.apply(&mut doc);
        assert!(result.is_ok());

        // Group should still exist
        assert_eq!(doc.root.children.len(), 1);
        if let Node::Element(elem) = &doc.root.children[0] {
            assert_eq!(elem.name, "g");
        }
    }

    #[test]
    fn test_config_parsing() {
        let config = RemoveNonInheritableGroupAttrsPlugin::parse_config(&json!({})).unwrap();
        // No fields to check since config is empty
        let _ = config;
    }
}

// Use parameterized testing framework for SVGO fixture tests
crate::plugin_fixture_tests!(RemoveNonInheritableGroupAttrsPlugin, "removeNonInheritableGroupAttrs");
