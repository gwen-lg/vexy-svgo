// this_file: crates/plugin-sdk/src/plugins/remove_non_inheritable_group_attrs.rs

//! Remove non-inheritable group attributes plugin implementation
//!
//! This plugin removes non-inheritable groupPROTECTED_230_s removeNonInheritableGroupAttrs plugin

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
            PROTECTED_1_,
            PROTECTED_2_,
            PROTECTED_3_,
            PROTECTED_4_,
            PROTECTED_5_,
            PROTECTED_6_,
            PROTECTED_7_,
            PROTECTED_8_,
            PROTECTED_9_,
            PROTECTED_10_,
            PROTECTED_11_,
            PROTECTED_12_,
            PROTECTED_13_,
            PROTECTED_14_,
            PROTECTED_15_,
            PROTECTED_16_,
            PROTECTED_17_,
            PROTECTED_18_,
            PROTECTED_19_,
            PROTECTED_20_,
            PROTECTED_21_,
            PROTECTED_22_,
            PROTECTED_23_,
            PROTECTED_24_,
            PROTECTED_25_,
            PROTECTED_26_,
            PROTECTED_27_,
            PROTECTED_28_,
            PROTECTED_29_,
            PROTECTED_30_,
            PROTECTED_31_,
            PROTECTED_32_,
            PROTECTED_33_,
            PROTECTED_34_,
            PROTECTED_35_,
            PROTECTED_36_,
            PROTECTED_37_,
            PROTECTED_38_,
            PROTECTED_39_,
            PROTECTED_40_,
            PROTECTED_41_,
            PROTECTED_42_,
            PROTECTED_43_,
            PROTECTED_44_,
            PROTECTED_45_,
            PROTECTED_46_,
            PROTECTED_47_,
            PROTECTED_48_,
            PROTECTED_49_,
            PROTECTED_50_,
            PROTECTED_51_,
            PROTECTED_52_,
            PROTECTED_53_,
            PROTECTED_54_,
            PROTECTED_55_,
            PROTECTED_56_,
            PROTECTED_57_,
            PROTECTED_58_,
            PROTECTED_59_,
            PROTECTED_60_,
            PROTECTED_61_,
        ]
        .iter()
        .cloned()
        .collect();

        // Inheritable attributes (can be inherited by child elements)
        let inheritable_attrs = [
            PROTECTED_62_,
            PROTECTED_63_,
            PROTECTED_64_,
            PROTECTED_65_,
            PROTECTED_66_,
            PROTECTED_67_,
            PROTECTED_68_,
            PROTECTED_69_,
            PROTECTED_70_,
            PROTECTED_71_,
            PROTECTED_72_,
            PROTECTED_73_,
            PROTECTED_74_,
            PROTECTED_75_,
            PROTECTED_76_,
            PROTECTED_77_,
            PROTECTED_78_,
            PROTECTED_79_,
            PROTECTED_80_,
            PROTECTED_81_,
            PROTECTED_82_,
            PROTECTED_83_,
            PROTECTED_84_,
            PROTECTED_85_,
            PROTECTED_86_,
            PROTECTED_87_,
            PROTECTED_88_,
            PROTECTED_89_,
            PROTECTED_90_,
            PROTECTED_91_,
            PROTECTED_92_,
            PROTECTED_93_,
            PROTECTED_94_,
            PROTECTED_95_,
            PROTECTED_96_,
            PROTECTED_97_,
            PROTECTED_98_,
            PROTECTED_99_,
            PROTECTED_100_,
            PROTECTED_101_,
            PROTECTED_102_,
            PROTECTED_103_,
            PROTECTED_104_,
            PROTECTED_105_,
        ]
        .iter()
        .cloned()
        .collect();

        // Presentation attributes that are allowed on groups even if non-inheritable
        let presentation_non_inheritable_group_attrs = [
            PROTECTED_106_,
            PROTECTED_107_,
            PROTECTED_108_,
            PROTECTED_109_,
            PROTECTED_110_,
            PROTECTED_111_,
            PROTECTED_112_,
            PROTECTED_113_,
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
                .map_err(|e| anyhow::anyhow!(PROTECTED_114_, e))
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
                // 1. ItPROTECTED_233_s NOT inheritable AND
                // 3. ItPROTECTED_234_static str {
        "removeNonInheritableGroupAttrs"
    }

    fn description(&self) -> &'static str {
        PROTECTED_117_
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
            .insert("alignment-baseline".to_string(), "central".to_string());
        group
            .attributes
            .insert("baseline-shift".to_string(), "10px".to_string());
        group
            .attributes
            .insert("lighting-color".to_string(), "red".to_string());
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
            .insert("fill".to_string(), "red".to_string());
        group
            .attributes
            .insert("stroke".to_string(), "blue".to_string());
        group
            .attributes
            .insert("font-family".to_string(), "Arial".to_string());
        group
            .attributes
            .insert("color".to_string(), "green".to_string());
        doc.root.children.push(Node::Element(group));

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Check that inheritable attributes were preserved
        if let Node::Element(elem) = &doc.root.children[0] {
            assert_eq!(elem.attributes.get("fill"), Some(&"red".to_string()));
            assert_eq!(elem.attributes.get("stroke"), Some(&"blue".to_string()));
            assert_eq!(
                elem.attributes.get("font-family"),
                Some(&"Arial".to_string())
            );
            assert_eq!(elem.attributes.get("color"), Some(&"green".to_string()));
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
            .insert("opacity".to_string(), "0.5".to_string());
        group
            .attributes
            .insert("transform".to_string(), "translate(10,20)".to_string());
        group
            .attributes
            .insert("clip-path".to_string(), "url(#clip)".to_string());
        group
            .attributes
            .insert("filter".to_string(), "url(#filter)".to_string());
        doc.root.children.push(Node::Element(group));

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Check that allowed group attributes were preserved
        if let Node::Element(elem) = &doc.root.children[0] {
            assert_eq!(elem.attributes.get("opacity"), Some(&"0.5".to_string()));
            assert_eq!(
                elem.attributes.get("transform"),
                Some(&"translate(10,20)".to_string())
            );
            assert_eq!(
                elem.attributes.get("clip-path"),
                Some(&"url(#clip)".to_string())
            );
            assert_eq!(
                elem.attributes.get("filter"),
                Some(&"url(#filter)".to_string())
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
            .insert("id".to_string(), "mygroup".to_string());
        group
            .attributes
            .insert("class".to_string(), "groupclass".to_string());
        group
            .attributes
            .insert("data-custom".to_string(), "value".to_string());
        doc.root.children.push(Node::Element(group));

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Check that non-presentation attributes were preserved
        if let Node::Element(elem) = &doc.root.children[0] {
            assert_eq!(elem.attributes.get("id"), Some(&"mygroup".to_string()));
            assert_eq!(
                elem.attributes.get("class"),
                Some(&"groupclass".to_string())
            );
            assert_eq!(
                elem.attributes.get("data-custom"),
                Some(&"value".to_string())
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
            .insert("alignment-baseline".to_string(), "central".to_string());
        rect.attributes
            .insert("baseline-shift".to_string(), "10px".to_string());
        doc.root.children.push(Node::Element(rect));

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Check that attributes on non-group elements are preserved
        if let Node::Element(elem) = &doc.root.children[0] {
            assert_eq!(
                elem.attributes.get("alignment-baseline"),
                Some(&"central".to_string())
            );
            assert_eq!(
                elem.attributes.get("baseline-shift"),
                Some(&"10px".to_string())
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
            .insert("alignment-baseline".to_string(), "central".to_string());
        group
            .attributes
            .insert("stop-color".to_string(), "red".to_string());
        // Should be preserved (inheritable)
        group
            .attributes
            .insert("fill".to_string(), "blue".to_string());
        group
            .attributes
            .insert("stroke".to_string(), "green".to_string());
        // Should be preserved (allowed group attribute)
        group
            .attributes
            .insert("opacity".to_string(), "0.8".to_string());
        // Should be preserved (non-presentation)
        group
            .attributes
            .insert("id".to_string(), "test".to_string());
        doc.root.children.push(Node::Element(group));

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        if let Node::Element(elem) = &doc.root.children[0] {
            // Removed attributes
            assert!(!elem.attributes.contains_key("alignment-baseline"));
            assert!(!elem.attributes.contains_key("stop-color"));

            // Preserved attributes
            assert_eq!(elem.attributes.get("fill"), Some(&"blue".to_string()));
            assert_eq!(elem.attributes.get("stroke"), Some(&"green".to_string()));
            assert_eq!(elem.attributes.get("opacity"), Some(&"0.8".to_string()));
            assert_eq!(elem.attributes.get("id"), Some(&"test".to_string()));
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
            .insert("alignment-baseline".to_string(), "central".to_string());
        outer_group
            .attributes
            .insert("fill".to_string(), "red".to_string());

        let mut inner_group = create_element("g");
        inner_group
            .attributes
            .insert("baseline-shift".to_string(), "10px".to_string());
        inner_group
            .attributes
            .insert("stroke".to_string(), "blue".to_string());

        outer_group.children.push(Node::Element(inner_group));
        doc.root.children.push(Node::Element(outer_group));

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        if let Node::Element(outer_elem) = &doc.root.children[0] {
            // Outer group: removed non-inheritable, kept inheritable
            assert!(!outer_elem.attributes.contains_key("alignment-baseline"));
            assert_eq!(outer_elem.attributes.get("fill"), Some(&"red".to_string()));

            if let Node::Element(inner_elem) = &outer_elem.children[0] {
                // Inner group: removed non-inheritable, kept inheritable
                assert!(!inner_elem.attributes.contains_key("baseline-shift"));
                assert_eq!(
                    inner_elem.attributes.get("stroke"),
                    Some(&"blue".to_string())
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
            assert_eq!(outer_elem.attributes.get("fill"), Some(&"red".to_string()));

            if let Node::Element(inner_elem) = &outer_elem.children[0] {
                // Inner group: removed non-inheritable, kept inheritable
                assert!(!inner_elem.attributes.contains_key("baseline-shift"));
                assert_eq!(
                    inner_elem.attributes.get("stroke"),
                    Some(&"blue".to_string())
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
