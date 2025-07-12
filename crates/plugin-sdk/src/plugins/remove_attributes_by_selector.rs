// this_file: crates/plugin-sdk/src/plugins/remove_attributes_by_selector.rs

//! Plugin to remove attributes of elements that match a CSS selector
//!
//! This plugin removes attributes from elements that match specified CSS selectors.
//! It supports single selectors or multiple selectors with different attribute removals.
//!
//! Reference: SVGO's removeAttributesBySelector plugin

use crate::Plugin;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use vexy_svgo_core::ast::{Document, Element, Node};

/// Configuration for a single selector
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SelectorConfig {
    /// CSS selector string
    pub selector: String,
    /// Attributes to remove (can be a single attribute or list)
    #[serde(deserialize_with = "deserialize_attributes")]
    pub attributes: Vec<String>,
}

/// Configuration for the RemoveAttributesBySelector plugin
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields, untagged)]
pub enum RemoveAttributesBySelectorConfig {
    /// Single selector configuration
    Single {
        selector: String,
        #[serde(deserialize_with = "deserialize_attributes")]
        attributes: Vec<String>,
    },
    /// Multiple selector configurations
    Multiple { selectors: Vec<SelectorConfig> },
}

impl Default for RemoveAttributesBySelectorConfig {
    fn default() -> Self {
        Self::Multiple {
            selectors: Vec::new(),
        }
    }
}

/// Custom deserializer for attributes that can be a string or array of strings
fn deserialize_attributes<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrVec {
        String(String),
        Vec(Vec<String>),
    }

    match StringOrVec::deserialize(deserializer)? {
        StringOrVec::String(s) => Ok(vec![s]),
        StringOrVec::Vec(v) => Ok(v),
    }
}

/// Plugin to remove attributes by CSS selector
pub struct RemoveAttributesBySelectorPlugin {
    config: RemoveAttributesBySelectorConfig,
}

impl RemoveAttributesBySelectorPlugin {
    pub fn new() -> Self {
        Self {
            config: RemoveAttributesBySelectorConfig::default(),
        }
    }

    pub fn with_config(config: RemoveAttributesBySelectorConfig) -> Self {
        Self { config }
    }

    fn parse_config(params: &Value) -> Result<RemoveAttributesBySelectorConfig> {
        if params.is_null() {
            Ok(RemoveAttributesBySelectorConfig::default())
        } else {
            serde_json::from_value(params.clone())
                .map_err(|e| anyhow::anyhow!("Invalid plugin configuration: {}", e))
        }
    }

    /// Get all selector configurations as a unified list
    fn get_selectors(&self) -> Vec<SelectorConfig> {
        match &self.config {
            RemoveAttributesBySelectorConfig::Single {
                selector,
                attributes,
            } => {
                vec![SelectorConfig {
                    selector: selector.clone(),
                    attributes: attributes.clone(),
                }]
            }
            RemoveAttributesBySelectorConfig::Multiple { selectors } => selectors.clone(),
        }
    }

    /// Simple fallback function to remove attributes using basic selector matching
    fn remove_attributes_simple(
        &self,
        element: &mut Element,
        selector: &str,
        attributes: &[String],
    ) -> Result<bool> {
        // Simple element matching - check if element matches selector
        let matches = if let Some(class_name) = selector.strip_prefix('.') {
            // Class selector: .className
            element
                .attr("class")
                .is_some_and(|classes| classes.split_whitespace().any(|c| c == class_name))
        } else if let Some(id) = selector.strip_prefix('#') {
            // ID selector: #elementId
            element.attr("id").is_some_and(|elem_id| elem_id == id)
        } else if selector.starts_with('[') && selector.ends_with(']') {
            // Attribute selector: [attr='value'] or [attr="value"] or [attr=value]
            self.parse_and_match_attribute_selector(element, selector)?
        } else if selector.contains('[') || selector.contains(']') {
            // Malformed attribute selector - should error
            return Err(anyhow::anyhow!("Malformed CSS selector: {}", selector));
        } else {
            // Element selector: elementName
            element.name == selector
        };

        if matches {
            // Remove specified attributes
            for attr in attributes {
                element.remove_attr(attr);
            }
            return Ok(true);
        }

        // Recursively process children
        let mut found = false;
        let mut i = 0;
        while i < element.children.len() {
            if let Node::Element(child_element) = &mut element.children[i] {
                if self.remove_attributes_simple(child_element, selector, attributes)? {
                    found = true;
                }
            }
            i += 1;
        }

        Ok(found)
    }

    /// Parse and match attribute selectors like [attr='value']
    fn parse_and_match_attribute_selector(
        &self,
        element: &Element,
        selector: &str,
    ) -> Result<bool> {
        // Remove brackets and parse content
        let content = &selector[1..selector.len() - 1];

        // Handle different attribute selector formats:
        // [attr='value'], [attr="value"], [attr=value]
        if let Some(eq_pos) = content.find('=') {
            let attr_name = content[..eq_pos].trim();
            let attr_value = content[eq_pos + 1..].trim();

            // Remove quotes if present
            let attr_value = if (attr_value.starts_with('"') && attr_value.ends_with('"'))
                || (attr_value.starts_with('\'') && attr_value.ends_with('\''))
            {
                &attr_value[1..attr_value.len() - 1]
            } else {
                attr_value
            };

            // Check if element has the attribute with the specified value
            Ok(element
                .attr(attr_name)
                .is_some_and(|elem_value| elem_value == attr_value))
        } else {
            // Simple attribute existence check: [attr]
            let attr_name = content.trim();
            Ok(element.has_attr(attr_name))
        }
    }
}

impl Default for RemoveAttributesBySelectorPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for RemoveAttributesBySelectorPlugin {
    fn name(&self) -> &'static str {
        "removeAttributesBySelector"
    }

    fn description(&self) -> &'static str {
        "removes attributes of elements that match a css selector"
    }

    fn validate_params(&self, params: &Value) -> Result<()> {
        Self::parse_config(params)?;
        Ok(())
    }

    fn apply<'a>(&self, document: &mut Document<'a>) -> Result<()> {
        // Process each selector configuration
        for config in self.get_selectors().iter() {
            // For now, use the simple fallback matching for all selectors
            // This covers the most common use cases: class, ID, element, and attribute selectors
            self.remove_attributes_simple(
                &mut document.root,
                &config.selector,
                &config.attributes,
            )?;
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
        let mut doc = Document::default();

        // Create a simple SVG structure
        let mut svg = Element {
            name: "svg".into(),
            attributes: IndexMap::new(),
            namespaces: Default::default(),
            children: vec![],
        };

        // Add rect with fill="#00ff00"
        let mut rect = Element {
            name: "rect".into(),
            attributes: IndexMap::new(),
            namespaces: Default::default(),
            children: vec![],
        };
        rect.set_attr("x", "0");
        rect.set_attr("y", "0");
        rect.set_attr("width", "100");
        rect.set_attr("height", "100");
        rect.set_attr("fill", "#00ff00");
        rect.set_attr("stroke", "#00ff00");

        svg.children.push(Node::Element(rect));
        doc.root = svg;
        doc
    }

    #[test]
    fn test_plugin_info() {
        let plugin = RemoveAttributesBySelectorPlugin::new();
        assert_eq!(plugin.name(), "removeAttributesBySelector");
        assert_eq!(
            plugin.description(),
            "removes attributes of elements that match a css selector"
        );
    }

    #[test]
    fn test_param_validation() {
        let plugin = RemoveAttributesBySelectorPlugin::new();

        // Test null params
        assert!(plugin.validate_params(&Value::Null).is_ok());

        // Test valid single selector params
        assert!(plugin
            .validate_params(&serde_json::json!({
                "selector": "[fill='#00ff00']",
                "attributes": "fill"
            }))
            .is_ok());

        // Test valid multiple selectors params
        assert!(plugin
            .validate_params(&serde_json::json!({
                "selectors": [
                    {
                        "selector": "[fill='#00ff00']",
                        "attributes": ["fill", "stroke"]
                    }
                ]
            }))
            .is_ok());

        // Test invalid params - unknown field
        assert!(plugin
            .validate_params(&serde_json::json!({
                "selector": "rect",
                "attributes": "fill",
                "unknownField": true
            }))
            .is_err());
    }

    #[test]
    fn test_single_attribute_removal() {
        let mut doc = create_test_document();
        let config = RemoveAttributesBySelectorConfig::Single {
            selector: "[fill='#00ff00']".to_string(),
            attributes: vec!["fill".to_string()],
        };
        let plugin = RemoveAttributesBySelectorPlugin::with_config(config);

        plugin.apply(&mut doc).unwrap();

        // Check that fill was removed but stroke remains
        if let Some(Node::Element(ref rect)) = doc.root.children.first() {
            assert_eq!(rect.attr("fill"), None);
            assert_eq!(rect.attr("stroke"), Some("#00ff00"));
        }
    }

    #[test]
    fn test_multiple_attributes_removal() {
        let mut doc = create_test_document();
        let config = RemoveAttributesBySelectorConfig::Single {
            selector: "[fill='#00ff00']".to_string(),
            attributes: vec!["fill".to_string(), "stroke".to_string()],
        };
        let plugin = RemoveAttributesBySelectorPlugin::with_config(config);

        plugin.apply(&mut doc).unwrap();

        // Check that both fill and stroke were removed
        if let Some(Node::Element(ref rect)) = doc.root.children.first() {
            assert_eq!(rect.attr("fill"), None);
            assert_eq!(rect.attr("stroke"), None);
            // Other attributes should remain
            assert_eq!(rect.attr("width"), Some("100"));
        }
    }

    #[test]
    fn test_multiple_selectors() {
        let mut doc = create_test_document();

        // Add an element with id="remove"
        let mut circle = Element {
            name: "circle".into(),
            attributes: IndexMap::new(),
            namespaces: Default::default(),
            children: vec![],
        };
        circle.set_attr("id", "remove");
        circle.set_attr("cx", "50");
        circle.set_attr("cy", "50");
        circle.set_attr("r", "25");
        circle.set_attr("stroke", "black");

        doc.root.children.push(Node::Element(circle));

        let config = RemoveAttributesBySelectorConfig::Multiple {
            selectors: vec![
                SelectorConfig {
                    selector: "[fill='#00ff00']".to_string(),
                    attributes: vec!["fill".to_string()],
                },
                SelectorConfig {
                    selector: "#remove".to_string(),
                    attributes: vec!["stroke".to_string(), "id".to_string()],
                },
            ],
        };
        let plugin = RemoveAttributesBySelectorPlugin::with_config(config);

        plugin.apply(&mut doc).unwrap();

        // Check results
        // Check rect
        if let Some(Node::Element(ref rect)) = doc.root.children.first() {
            assert_eq!(rect.attr("fill"), None);
            assert_eq!(rect.attr("stroke"), Some("#00ff00"));
        }

        // Check circle
        if let Some(Node::Element(ref circle)) = doc.root.children.get(1) {
            assert_eq!(circle.attr("id"), None);
            assert_eq!(circle.attr("stroke"), None);
            assert_eq!(circle.attr("cx"), Some("50"));
        }
    }

    #[test]
    fn test_element_name_selector() {
        let mut doc = create_test_document();
        let config = RemoveAttributesBySelectorConfig::Single {
            selector: "rect".to_string(),
            attributes: vec!["fill".to_string()],
        };
        let plugin = RemoveAttributesBySelectorPlugin::with_config(config);

        plugin.apply(&mut doc).unwrap();

        // Check that fill was removed from rect
        if let Some(Node::Element(ref rect)) = doc.root.children.first() {
            assert_eq!(rect.attr("fill"), None);
        }
    }

    #[test]
    fn test_class_selector() {
        let mut doc = create_test_document();

        // Add class to rect
        if let Some(Node::Element(ref mut rect)) = doc.root.children.get_mut(0) {
            rect.set_attr("class", "remove-me another-class");
        }

        let config = RemoveAttributesBySelectorConfig::Single {
            selector: ".remove-me".to_string(),
            attributes: vec!["fill".to_string()],
        };
        let plugin = RemoveAttributesBySelectorPlugin::with_config(config);

        plugin.apply(&mut doc).unwrap();

        // Check that fill was removed
        if let Some(Node::Element(ref rect)) = doc.root.children.first() {
            assert_eq!(rect.attr("fill"), None);
            assert_eq!(
                rect.attr("class"),
                Some("remove-me another-class")
            );
        }
    }

    #[test]
    fn test_invalid_selector() {
        let mut doc = create_test_document();
        let config = RemoveAttributesBySelectorConfig::Single {
            selector: "[invalid selector".to_string(),
            attributes: vec!["fill".to_string()],
        };
        let plugin = RemoveAttributesBySelectorPlugin::with_config(config);

        let result = plugin.apply(&mut doc);
        assert!(result.is_err());
    }
}
