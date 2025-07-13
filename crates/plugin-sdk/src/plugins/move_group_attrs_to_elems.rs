// this_file: crates/plugin-sdk/src/plugins/move_group_attrs_to_elems.rs

//! Move group attributes to children when the group only has one child
//!
//! This plugin moves inheritable presentation attributes from a <g> element
//! to its single child element when the group contains exactly one element child.
//!
//! Reference: SVGO's moveGroupAttrsToElems plugin

use crate::Plugin;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use vexy_svgo_core::ast::{Document, Element, Node};

/// Configuration for the moveGroupAttrsToElems plugin
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MoveGroupAttrsToElemsConfig {}

impl Default for MoveGroupAttrsToElemsConfig {
    fn default() -> Self {
        Self {}
    }
}

/// Plugin to move group attributes to children when the group only has one child
pub struct MoveGroupAttrsToElemsPlugin {
    #[allow(dead_code)]
    config: MoveGroupAttrsToElemsConfig,
}

impl MoveGroupAttrsToElemsPlugin {
    pub fn new() -> Self {
        Self {
            #[allow(dead_code)]
            config: MoveGroupAttrsToElemsConfig::default(),
        }
    }

    pub fn with_config(config: MoveGroupAttrsToElemsConfig) -> Self {
        Self { config }
    }

    fn parse_config(params: &Value) -> Result<MoveGroupAttrsToElemsConfig> {
        if params.is_null() {
            Ok(MoveGroupAttrsToElemsConfig::default())
        } else {
            serde_json::from_value(params.clone())
                .map_err(|e| anyhow::anyhow!("Invalid plugin configuration: {}", e))
        }
    }

    fn process_element(&self, element: &mut Element) {
        // Process children first (depth-first)
        let mut i = 0;
        while i < element.children.len() {
            if let Node::Element(child_elem) = &mut element.children[i] {
                self.process_element(child_elem);
            }
            i += 1;
        }

        // Check if this is a group with exactly one element child
        if element.name == "g" {
            let element_children_count = element
                .children
                .iter()
                .filter(|node| matches!(node, Node::Element(_)))
                .count();

            if element_children_count == 1 {
                // We have exactly one child element, move applicable attributes
                self.move_attributes_to_single_child(element);
            }
        }
    }

    fn move_attributes_to_single_child(&self, group: &mut Element) {
        const MOVABLE_ATTRS: &[&str] = &[
            "fill",
            "stroke",
            "stroke-width",
            "stroke-linecap",
            "stroke-linejoin",
            "stroke-miterlimit",
            "stroke-dasharray",
            "stroke-dashoffset",
            "stroke-opacity",
            "fill-opacity",
            "opacity",
            "color",
            "font-family",
            "font-size",
            "font-style",
            "font-variant",
            "font-weight",
            "text-anchor",
            "text-decoration",
            "letter-spacing",
            "word-spacing",
        ];

        // Find the single child element
        let mut child_index = None;
        for (i, node) in group.children.iter().enumerate() {
            if matches!(node, Node::Element(_)) {
                child_index = Some(i);
                break;
            }
        }

        if let Some(index) = child_index {
            let mut attrs_to_move = Vec::new();

            // Collect attributes to move
            for attr_name in MOVABLE_ATTRS {
                if let Some(attr_value) = group.attr(attr_name) {
                    if let Node::Element(child) = &group.children[index] {
                        // Only move if child doesn't already have this attribute
                        if !child.has_attr(attr_name) {
                            attrs_to_move.push((attr_name.to_string(), attr_value.to_string()));
                        }
                    }
                }
            }

            // Apply the moves
            if !attrs_to_move.is_empty() {
                if let Node::Element(child) = &mut group.children[index] {
                    for (attr_name, attr_value) in &attrs_to_move {
                        child.set_attr(attr_name, attr_value);
                    }
                }

                // Remove moved attributes from group
                for (attr_name, _) in attrs_to_move {
                    group.remove_attr(&attr_name);
                }
            }
        }
    }
}

impl Default for MoveGroupAttrsToElemsPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for MoveGroupAttrsToElemsPlugin {
    fn name(&self) -> &'static str {
        "moveGroupAttrsToElems"
    }

    fn description(&self) -> &'static str {
        "Move group attributes to children when group has single child"
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
    use vexy_svgo_core::ast::{Element, Node};

    #[test]
    fn test_plugin_info() {
        let plugin = MoveGroupAttrsToElemsPlugin::new();
        assert_eq!(plugin.name(), "moveGroupAttrsToElems");
        assert_eq!(
            plugin.description(),
            "Move group attributes to children when group has single child"
        );
    }

    #[test]
    fn test_param_validation() {
        let plugin = MoveGroupAttrsToElemsPlugin::new();

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
    fn test_move_attrs_to_single_child() {
        let plugin = MoveGroupAttrsToElemsPlugin::new();
        let mut doc = Document::new();

        // Create a group with attributes and a single child
        let mut group = Element::new("g");
        group.set_attr("fill", "red");
        group.set_attr("stroke", "blue");
        group.set_attr("id", "group1"); // This should not be moved

        let child = Element::new("rect");
        group.children.push(Node::Element(child));

        doc.root = group;

        // Apply the plugin
        plugin.apply(&mut doc).unwrap();

        // Check that attributes were moved
        match &doc.root.children[0] {
            Node::Element(child) => {
                assert_eq!(child.attr("fill"), Some("red"));
                assert_eq!(child.attr("stroke"), Some("blue"));
            }
            _ => panic!("Expected element child"),
        }

        // Check that movable attributes were removed from group
        assert!(!doc.root.has_attr("fill"));
        assert!(!doc.root.has_attr("stroke"));

        // Check that non-movable attributes remain on group
        assert_eq!(doc.root.attr("id"), Some("group1"));
    }

    #[test]
    fn test_no_move_when_child_has_attr() {
        let plugin = MoveGroupAttrsToElemsPlugin::new();
        let mut doc = Document::new();

        // Create a group with attributes and a single child that already has some attributes
        let mut group = Element::new("g");
        group.set_attr("fill", "red");
        group.set_attr("stroke", "blue");

        let mut child = Element::new("rect");
        child.set_attr("fill", "green"); // Child already has fill
        group.children.push(Node::Element(child));

        doc.root = group;

        // Apply the plugin
        plugin.apply(&mut doc).unwrap();

        // Check that fill was not moved (child already had it)
        match &doc.root.children[0] {
            Node::Element(child) => {
                assert_eq!(child.attr("fill"), Some("green")); // Original value preserved
                assert_eq!(child.attr("stroke"), Some("blue")); // This was moved
            }
            _ => panic!("Expected element child"),
        }

        // Check that fill remains on group (not moved)
        assert_eq!(doc.root.attr("fill"), Some("red"));
        assert!(!doc.root.has_attr("stroke")); // This was moved
    }

    #[test]
    fn test_no_move_with_multiple_children() {
        let plugin = MoveGroupAttrsToElemsPlugin::new();
        let mut doc = Document::new();

        // Create a group with attributes and multiple children
        let mut group = Element::new("g");
        group.set_attr("fill", "red");

        group.children.push(Node::Element(Element::new("rect")));
        group.children.push(Node::Element(Element::new("circle")));

        doc.root = group;

        // Apply the plugin
        plugin.apply(&mut doc).unwrap();

        // Check that attributes were not moved
        assert_eq!(doc.root.attr("fill"), Some("red"));
    }
}
