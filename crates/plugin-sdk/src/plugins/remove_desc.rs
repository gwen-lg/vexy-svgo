// this_file: crates/plugin-sdk/src/plugins/remove_desc.rs

//! Remove <desc> elements
//!
//! This plugin removes <desc> elements from SVG documents.
//! By default, it only removes empty descriptions or those containing standard
//! editor content (e.g., "Created with..."). Can be configured to remove all
//! descriptions.
//!
//! Reference: SVGO's removeDesc plugin

use crate::Plugin;
use anyhow::Result;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::LazyLock;
use vexy_svgo_core::ast::{Document, Element, Node};

/// Configuration for the removeDesc plugin
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RemoveDescConfig {
    /// Remove any <desc> element, not just empty or standard ones
    #[serde(default)]
    pub remove_any: bool,
}

impl Default for RemoveDescConfig {
    fn default() -> Self {
        Self { remove_any: false }
    }
}

/// Plugin that removes <desc> elements
pub struct RemoveDescPlugin {
    config: RemoveDescConfig,
}

// Regex pattern for standard editor descriptions
static STANDARD_DESCS: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(Created with|Created using)").unwrap());

impl RemoveDescPlugin {
    pub fn new() -> Self {
        Self {
            config: RemoveDescConfig::default(),
        }
    }

    pub fn with_config(config: RemoveDescConfig) -> Self {
        Self { config }
    }

    fn parse_config(params: &Value) -> Result<RemoveDescConfig> {
        if params.is_null() {
            Ok(RemoveDescConfig::default())
        } else {
            serde_json::from_value(params.clone())
                .map_err(|e| anyhow::anyhow!("Invalid plugin configuration: {}", e))
        }
    }

    fn process_element(&self, element: &mut Element) {
        // Filter out desc elements based on criteria
        element.children.retain(|child| {
            match child {
                Node::Element(child_element) if child_element.name == "desc" => {
                    // Keep the desc element if we should not remove it
                    !self.should_remove_desc(child_element)
                }
                _ => true,
            }
        });

        // Process remaining child elements
        let mut i = 0;
        while i < element.children.len() {
            if let Node::Element(child) = &mut element.children[i] {
                self.process_element(child);
            }
            i += 1;
        }
    }

    /// Check if a desc element should be removed
    fn should_remove_desc(&self, desc_element: &Element) -> bool {
        if self.config.remove_any {
            return true;
        }

        // Remove if empty
        if desc_element.children.is_empty() {
            return true;
        }

        // Check if it contains only standard editor text
        if desc_element.children.len() == 1 {
            if let Some(Node::Text(text)) = desc_element.children.first() {
                if STANDARD_DESCS.is_match(text) {
                    return true;
                }
            }
        }

        false
    }
}

impl Default for RemoveDescPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for RemoveDescPlugin {
    fn name(&self) -> &'static str {
        "removeDesc"
    }

    fn description(&self) -> &'static str {
        "removes <desc> element"
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
    use vexy_svgo_core::parser::Parser;

    #[test]
    fn test_plugin_info() {
        let plugin = RemoveDescPlugin::new();
        assert_eq!(plugin.name(), "removeDesc");
        assert_eq!(plugin.description(), "removes <desc> element");
    }

    #[test]
    fn test_param_validation() {
        let plugin = RemoveDescPlugin::new();

        // Test null params
        assert!(plugin.validate_params(&Value::Null).is_ok());

        // Test valid params
        assert!(plugin
            .validate_params(&serde_json::json!({
                "removeAny": true
            }))
            .is_ok());

        // Test invalid params
        assert!(plugin
            .validate_params(&serde_json::json!({
                "invalidParam": true
            }))
            .is_err());
    }

    #[test]
    fn test_remove_empty_desc() {
        let svg = r#"<svg xmlns="http://www.w3.org/2000/svg">
            <desc></desc>
            <rect width="100" height="100"/>
        </svg>"#;

        let parser = Parser::new();
        let mut document = parser.parse(svg).unwrap();

        let plugin = RemoveDescPlugin::new();
        plugin.apply(&mut document).unwrap();

        // Check that empty desc is removed
        assert!(!has_desc_element(&document.root));
    }

    #[test]
    fn test_remove_standard_desc() {
        let svg = r#"<svg xmlns="http://www.w3.org/2000/svg">
            <desc>Created with Sketch.</desc>
            <rect width="100" height="100"/>
        </svg>"#;

        let parser = Parser::new();
        let mut document = parser.parse(svg).unwrap();

        let plugin = RemoveDescPlugin::new();
        plugin.apply(&mut document).unwrap();

        // Check that standard desc is removed
        assert!(!has_desc_element(&document.root));
    }

    #[test]
    fn test_preserve_custom_desc() {
        let svg = r#"<svg xmlns="http://www.w3.org/2000/svg">
            <desc>This is a custom description for accessibility</desc>
            <rect width="100" height="100"/>
        </svg>"#;

        let parser = Parser::new();
        let mut document = parser.parse(svg).unwrap();

        let plugin = RemoveDescPlugin::new();
        plugin.apply(&mut document).unwrap();

        // Check that custom desc is preserved
        assert!(has_desc_element(&document.root));
    }

    #[test]
    fn test_remove_any() {
        let svg = r#"<svg xmlns="http://www.w3.org/2000/svg">
            <desc>This is a custom description for accessibility</desc>
            <rect width="100" height="100"/>
        </svg>"#;

        let parser = Parser::new();
        let mut document = parser.parse(svg).unwrap();

        let plugin = RemoveDescPlugin::with_config(RemoveDescConfig { remove_any: true });
        plugin.apply(&mut document).unwrap();

        // Check that all desc elements are removed
        assert!(!has_desc_element(&document.root));
    }

    fn has_desc_element(element: &Element) -> bool {
        for child in &element.children {
            if let Node::Element(child_element) = child {
                if child_element.name == "desc" {
                    return true;
                }
                if has_desc_element(child_element) {
                    return true;
                }
            }
        }
        false
    }
}
