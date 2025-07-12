// this_file: crates/plugin-sdk/src/plugins/sort_defs_children.rs

//! Sort defs children plugin implementation
//!
//! This plugin sorts children of <defs> elements to improve compression.
//! Elements are sorted first by frequency (most frequent first), then by
//! element name length (longest first), then by element name (alphabetically).
//!
//! Reference: SVGO's sortDefsChildren plugin

use crate::Plugin;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use vexy_svgo_core::ast::{Document, Element, Node};

/// Configuration parameters for sort defs children plugin (currently empty)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SortDefsChildrenConfig {
    // No configuration options - matches SVGO behavior
}

impl Default for SortDefsChildrenConfig {
    fn default() -> Self {
        Self {}
    }
}

/// Plugin that sorts children of defs elements
pub struct SortDefsChildrenPlugin {
    config: SortDefsChildrenConfig,
}

impl SortDefsChildrenPlugin {
    /// Create a new SortDefsChildrenPlugin
    pub fn new() -> Self {
        Self {
            config: SortDefsChildrenConfig::default(),
        }
    }

    /// Create a new SortDefsChildrenPlugin with config
    pub fn with_config(config: SortDefsChildrenConfig) -> Self {
        Self { config }
    }

    /// Parse configuration from JSON
    fn parse_config(params: &Value) -> Result<SortDefsChildrenConfig> {
        if params.is_null() || (params.is_object() && params.as_object().unwrap().is_empty()) {
            Ok(SortDefsChildrenConfig::default())
        } else if params.is_object() {
            serde_json::from_value(params.clone())
                .map_err(|e| anyhow::anyhow!("Invalid configuration: {}", e))
        } else {
            Ok(SortDefsChildrenConfig::default())
        }
    }

    /// Sort children of defs elements
    fn sort_defs_children_recursive(&self, element: &mut Element) {
        // Process this element if it's a defs element
        if element.name == "defs" {
            // Count frequencies of each element name
            let mut frequencies: HashMap<String, u32> = HashMap::new();
            for child in &element.children {
                if let Node::Element(elem) = child {
                    let count = frequencies.entry(elem.name.to_string()).or_insert(0);
                    *count += 1;
                }
            }

            // Sort children according to the algorithm
            element.children.sort_by(|a, b| {
                // Only sort elements, leave other nodes as is
                if let (Node::Element(elem_a), Node::Element(elem_b)) = (a, b) {
                    let name_a = elem_a.name.as_ref();
                    let name_b = elem_b.name.as_ref();

                    // Get frequencies
                    let freq_a = frequencies.get(name_a).unwrap_or(&0);
                    let freq_b = frequencies.get(name_b).unwrap_or(&0);

                    // First, sort by frequency (most frequent first)
                    let frequency_comparison = freq_b.cmp(freq_a);
                    if frequency_comparison != std::cmp::Ordering::Equal {
                        return frequency_comparison;
                    }

                    // Then, sort by element name length (longest first)
                    let length_comparison = name_b.len().cmp(&name_a.len());
                    if length_comparison != std::cmp::Ordering::Equal {
                        return length_comparison;
                    }

                    // Finally, sort by element name (alphabetically)
                    name_a.cmp(name_b)
                } else {
                    // Keep non-element nodes in their original order
                    std::cmp::Ordering::Equal
                }
            });
        }

        // Process child elements recursively
        for child in &mut element.children {
            if let Node::Element(elem) = child {
                self.sort_defs_children_recursive(elem);
            }
        }
    }
}

impl Default for SortDefsChildrenPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for SortDefsChildrenPlugin {
    fn name(&self) -> &'static str {
        "sortDefsChildren"
    }

    fn description(&self) -> &'static str {
        "Sorts children of <defs> to improve compression"
    }

    fn validate_params(&self, params: &Value) -> Result<()> {
        Self::parse_config(params)?;
        Ok(())
    }

    fn apply(&self, document: &mut Document) -> Result<()> {
        self.sort_defs_children_recursive(&mut document.root);
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
        let plugin = SortDefsChildrenPlugin::new();
        assert_eq!(plugin.name(), "sortDefsChildren");
        assert_eq!(
            plugin.description(),
            "Sorts children of <defs> to improve compression"
        );
    }

    #[test]
    fn test_parameter_validation() {
        let plugin = SortDefsChildrenPlugin::new();

        // Valid parameters (empty object)
        assert!(plugin.validate_params(&json!({})).is_ok());

        // Invalid parameters (non-empty object)
        assert!(plugin.validate_params(&json!({"param": "value"})).is_err());
    }

    #[test]
    fn test_sort_by_frequency() {
        let plugin = SortDefsChildrenPlugin::new();
        let mut doc = Document::new();

        // Create defs with children of different frequencies
        let mut defs = create_element("defs");

        // Add elements: 3 rects, 2 circles, 1 path
        // Should be sorted: rects first, then circles, then path
        defs.children.push(Node::Element(create_element("path")));
        defs.children.push(Node::Element(create_element("circle")));
        defs.children.push(Node::Element(create_element("rect")));
        defs.children.push(Node::Element(create_element("circle")));
        defs.children.push(Node::Element(create_element("rect")));
        defs.children.push(Node::Element(create_element("rect")));

        doc.root.children.push(Node::Element(defs));

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Check that elements are sorted by frequency
        if let Node::Element(defs_elem) = &doc.root.children[0] {
            assert_eq!(defs_elem.children.len(), 6);

            // First 3 should be rects (frequency 3)
            if let Node::Element(elem) = &defs_elem.children[0] {
                assert_eq!(elem.name, "rect");
            }
            if let Node::Element(elem) = &defs_elem.children[1] {
                assert_eq!(elem.name, "rect");
            }
            if let Node::Element(elem) = &defs_elem.children[2] {
                assert_eq!(elem.name, "rect");
            }

            // Next 2 should be circles (frequency 2)
            if let Node::Element(elem) = &defs_elem.children[3] {
                assert_eq!(elem.name, "circle");
            }
            if let Node::Element(elem) = &defs_elem.children[4] {
                assert_eq!(elem.name, "circle");
            }

            // Last should be path (frequency 1)
            if let Node::Element(elem) = &defs_elem.children[5] {
                assert_eq!(elem.name, "path");
            }
        }
    }

    #[test]
    fn test_sort_by_length_when_same_frequency() {
        let plugin = SortDefsChildrenPlugin::new();
        let mut doc = Document::new();

        // Create defs with children of same frequency but different lengths
        let mut defs = create_element("defs");

        // Add elements with same frequency (1 each) but different name lengths
        // Should be sorted by length: "linearGradient" (14), "rect" (4), "g" (1)
        defs.children.push(Node::Element(create_element("g")));
        defs.children.push(Node::Element(create_element("rect")));
        defs.children
            .push(Node::Element(create_element("linearGradient")));

        doc.root.children.push(Node::Element(defs));

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Check that elements are sorted by length (longest first)
        if let Node::Element(defs_elem) = &doc.root.children[0] {
            assert_eq!(defs_elem.children.len(), 3);

            if let Node::Element(elem) = &defs_elem.children[0] {
                assert_eq!(elem.name, "linearGradient");
            }
            if let Node::Element(elem) = &defs_elem.children[1] {
                assert_eq!(elem.name, "rect");
            }
            if let Node::Element(elem) = &defs_elem.children[2] {
                assert_eq!(elem.name, "g");
            }
        }
    }

    #[test]
    fn test_sort_alphabetically_when_same_frequency_and_length() {
        let plugin = SortDefsChildrenPlugin::new();
        let mut doc = Document::new();

        // Create defs with children of same frequency and length
        let mut defs = create_element("defs");

        // Add elements with same frequency (1 each) and same length (4 chars)
        // Should be sorted alphabetically: "path", "rect"
        defs.children.push(Node::Element(create_element("rect")));
        defs.children.push(Node::Element(create_element("path")));

        doc.root.children.push(Node::Element(defs));

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Check that elements are sorted alphabetically
        if let Node::Element(defs_elem) = &doc.root.children[0] {
            assert_eq!(defs_elem.children.len(), 2);

            if let Node::Element(elem) = &defs_elem.children[0] {
                assert_eq!(elem.name, "path");
            }
            if let Node::Element(elem) = &defs_elem.children[1] {
                assert_eq!(elem.name, "rect");
            }
        }
    }

    #[test]
    fn test_ignores_non_defs_elements() {
        let plugin = SortDefsChildrenPlugin::new();
        let mut doc = Document::new();

        // Create non-defs element with children
        let mut group = create_element("g");

        // Add children in reverse order
        group.children.push(Node::Element(create_element("rect")));
        group.children.push(Node::Element(create_element("circle")));
        group.children.push(Node::Element(create_element("path")));

        doc.root.children.push(Node::Element(group));

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Check that children order is unchanged
        if let Node::Element(group_elem) = &doc.root.children[0] {
            assert_eq!(group_elem.children.len(), 3);

            if let Node::Element(elem) = &group_elem.children[0] {
                assert_eq!(elem.name, "rect");
            }
            if let Node::Element(elem) = &group_elem.children[1] {
                assert_eq!(elem.name, "circle");
            }
            if let Node::Element(elem) = &group_elem.children[2] {
                assert_eq!(elem.name, "path");
            }
        }
    }

    #[test]
    fn test_nested_defs() {
        let plugin = SortDefsChildrenPlugin::new();
        let mut doc = Document::new();

        // Create nested defs elements
        let mut outer_defs = create_element("defs");
        let mut inner_defs = create_element("defs");

        // Add children to inner defs
        inner_defs
            .children
            .push(Node::Element(create_element("rect")));
        inner_defs
            .children
            .push(Node::Element(create_element("circle")));
        inner_defs
            .children
            .push(Node::Element(create_element("circle")));

        // Add children to outer defs
        outer_defs
            .children
            .push(Node::Element(create_element("path")));
        outer_defs.children.push(Node::Element(inner_defs));
        outer_defs
            .children
            .push(Node::Element(create_element("path")));

        doc.root.children.push(Node::Element(outer_defs));

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Check that both defs are sorted
        if let Node::Element(outer_defs_elem) = &doc.root.children[0] {
            assert_eq!(outer_defs_elem.children.len(), 3);

            // Outer defs should have paths first (frequency 2), then inner defs (frequency 1)
            if let Node::Element(elem) = &outer_defs_elem.children[0] {
                assert_eq!(elem.name, "path");
            }
            if let Node::Element(elem) = &outer_defs_elem.children[1] {
                assert_eq!(elem.name, "path");
            }
            if let Node::Element(elem) = &outer_defs_elem.children[2] {
                assert_eq!(elem.name, "defs");

                // Inner defs should have circles first (frequency 2), then rect (frequency 1)
                if let Node::Element(inner_elem) = &elem.children[0] {
                    assert_eq!(inner_elem.name, "circle");
                }
                if let Node::Element(inner_elem) = &elem.children[1] {
                    assert_eq!(inner_elem.name, "circle");
                }
                if let Node::Element(inner_elem) = &elem.children[2] {
                    assert_eq!(inner_elem.name, "rect");
                }
            }
        }
    }

    #[test]
    fn test_empty_defs() {
        let plugin = SortDefsChildrenPlugin::new();
        let mut doc = Document::new();

        // Create empty defs
        let defs = create_element("defs");
        doc.root.children.push(Node::Element(defs));

        // Apply plugin - should not crash
        let result = plugin.apply(&mut doc);
        assert!(result.is_ok());

        // Defs should still exist and be empty
        if let Node::Element(defs_elem) = &doc.root.children[0] {
            assert_eq!(defs_elem.name, "defs");
            assert_eq!(defs_elem.children.len(), 0);
        }
    }

    #[test]
    fn test_complex_sorting() {
        let plugin = SortDefsChildrenPlugin::new();
        let mut doc = Document::new();

        // Create defs with complex mix of elements
        let mut defs = create_element("defs");

        // Add elements with various frequencies and lengths
        defs.children.push(Node::Element(create_element("g"))); // freq 1, len 1
        defs.children.push(Node::Element(create_element("rect"))); // freq 2, len 4
        defs.children
            .push(Node::Element(create_element("linearGradient"))); // freq 1, len 14
        defs.children.push(Node::Element(create_element("rect"))); // freq 2, len 4
        defs.children.push(Node::Element(create_element("circle"))); // freq 3, len 6
        defs.children.push(Node::Element(create_element("circle"))); // freq 3, len 6
        defs.children.push(Node::Element(create_element("circle"))); // freq 3, len 6
        defs.children.push(Node::Element(create_element("path"))); // freq 1, len 4

        doc.root.children.push(Node::Element(defs));

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Check expected order:
        // 1. circles (freq 3, len 6) - 3 elements
        // 2. rects (freq 2, len 4) - 2 elements
        // 3. linearGradient (freq 1, len 14) - 1 element
        // 4. path (freq 1, len 4) - 1 element
        // 5. g (freq 1, len 1) - 1 element
        if let Node::Element(defs_elem) = &doc.root.children[0] {
            assert_eq!(defs_elem.children.len(), 8);

            let element_names: Vec<&str> = defs_elem
                .children
                .iter()
                .filter_map(|child| {
                    if let Node::Element(elem) = child {
                        Some(elem.name.as_ref())
                    } else {
                        None
                    }
                })
                .collect();

            assert_eq!(
                element_names,
                vec![
                    "circle",
                    "circle",
                    "circle", // frequency 3
                    "rect",
                    "rect",           // frequency 2
                    "linearGradient", // frequency 1, length 14
                    "path",           // frequency 1, length 4
                    "g"               // frequency 1, length 1
                ]
            );
        }
    }

    #[test]
    fn test_config_parsing() {
        let config = SortDefsChildrenPlugin::parse_config(&json!({})).unwrap();
        // No fields to check since config is empty
        let _ = config;
    }
}

// Use parameterized testing framework for SVGO fixture tests
// TODO: Re-enable after fixing XML parsing and text content handling
// crate::plugin_fixture_tests!(SortDefsChildrenPlugin, "sortDefsChildren");
