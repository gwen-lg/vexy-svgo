// this_file: crates/plugin-sdk/src/plugins/remove_doctype.rs

//! Remove DOCTYPE plugin implementation
//!
//! This plugin removes DOCTYPE declarations from SVG documents.
//! DOCTYPE declarations are not needed in SVG and can cause issues.
//!
//! According to the SVG WG: "the SVG DTDs are a source of so many
//! issues that the SVG WG has decided not to write one for the upcoming
//! SVG 1.2 standard. In fact SVG WG members are even telling people not
//! to use a DOCTYPE declaration in SVG 1.0 and 1.1 documents"
//! https://jwatt.org/svg/authoring/#doctype-declaration

use crate::Plugin;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use vexy_svgo_core::ast::{Document, Node};

/// Configuration parameters for remove doctype plugin (currently empty)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveDoctypeConfig {
    // No configuration options - matches SVGO behavior
}

impl Default for RemoveDoctypeConfig {
    fn default() -> Self {
        Self {}
    }
}

/// Plugin that removes DOCTYPE declarations
pub struct RemoveDoctypePlugin {
    #[allow(dead_code)]
    config: RemoveDoctypeConfig,
}

impl RemoveDoctypePlugin {
    /// Create a new RemoveDoctypePlugin
    pub fn new() -> Self {
        Self {
            #[allow(dead_code)]
            config: RemoveDoctypeConfig::default(),
        }
    }

    /// Create a new RemoveDoctypePlugin with config
    pub fn with_config(config: RemoveDoctypeConfig) -> Self {
        Self { config }
    }

    /// Parse configuration from JSON
    fn _parse_config(params: &Value) -> Result<RemoveDoctypeConfig> {
        if params.is_object() {
            serde_json::from_value(params.clone())
                .map_err(|e| anyhow::anyhow!("Invalid configuration: {}", e))
        } else {
            Ok(RemoveDoctypeConfig::default())
        }
    }
}

impl Default for RemoveDoctypePlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for RemoveDoctypePlugin {
    fn name(&self) -> &'static str {
        "removeDoctype"
    }

    fn description(&self) -> &'static str {
        "removes doctype declaration"
    }

    fn validate_params(&self, params: &Value) -> Result<()> {
        if let Some(obj) = params.as_object() {
            if !obj.is_empty() {
                return Err(anyhow::anyhow!(
                    "removeDoctype plugin does not accept any parameters"
                ));
            }
        }
        Ok(())
    }

    fn apply(&self, document: &mut Document) -> Result<()> {
        // Remove all DOCTYPE nodes from the document prologue
        document
            .prologue
            .retain(|child| !matches!(child, Node::DocType(_)));

        // Also remove DOCTYPE nodes from root children (in case they were misplaced)
        document
            .root
            .children
            .retain(|child| !matches!(child, Node::DocType(_)));

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
        let plugin = RemoveDoctypePlugin::new();
        assert_eq!(plugin.name(), "removeDoctype");
        assert_eq!(plugin.description(), "removes doctype declaration");
    }

    #[test]
    fn test_parameter_validation() {
        let plugin = RemoveDoctypePlugin::new();

        // Valid parameters (empty object)
        assert!(plugin.validate_params(&json!({})).is_ok());

        // Invalid parameters (non-empty object)
        assert!(plugin.validate_params(&json!({"param": "value"})).is_err());
    }

    #[test]
    fn test_remove_doctype() {
        let plugin = RemoveDoctypePlugin::new();
        let mut doc = Document::new();

        // Add a DOCTYPE node
        doc.root.children.push(Node::DocType("svg PUBLIC \"-//W3C//DTD SVG 1.1//EN\" \"http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd\"".to_string()));

        // Add a regular element
        let svg = create_element("svg");
        doc.root.children.push(Node::Element(svg));

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Should have removed the DOCTYPE node
        assert_eq!(doc.root.children.len(), 1);
        assert!(matches!(doc.root.children[0], Node::Element(_)));
    }

    #[test]
    fn test_multiple_doctypes() {
        let plugin = RemoveDoctypePlugin::new();
        let mut doc = Document::new();

        // Add multiple DOCTYPE nodes
        doc.root.children.push(Node::DocType("svg PUBLIC \"-//W3C//DTD SVG 1.1//EN\" \"http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd\"".to_string()));
        doc.root.children.push(Node::DocType("html PUBLIC \"-//W3C//DTD XHTML 1.0 Transitional//EN\" \"http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd\"".to_string()));

        // Add a regular element
        let svg = create_element("svg");
        doc.root.children.push(Node::Element(svg));

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Should have removed all DOCTYPE nodes
        assert_eq!(doc.root.children.len(), 1);
        assert!(matches!(doc.root.children[0], Node::Element(_)));
    }

    #[test]
    fn test_no_doctype() {
        let plugin = RemoveDoctypePlugin::new();
        let mut doc = Document::new();

        // Add only regular elements
        let svg = create_element("svg");
        doc.root.children.push(Node::Element(svg));

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Should have no changes
        assert_eq!(doc.root.children.len(), 1);
        assert!(matches!(doc.root.children[0], Node::Element(_)));
    }

    #[test]
    fn test_doctype_with_text() {
        let plugin = RemoveDoctypePlugin::new();
        let mut doc = Document::new();

        // Add DOCTYPE, text, and element
        doc.root.children.push(Node::DocType(
            "svg PUBLIC \"-//W3C//DTD SVG 1.1//EN\"".to_string(),
        ));
        doc.root.children.push(Node::Text("Some text".to_string()));
        let svg = create_element("svg");
        doc.root.children.push(Node::Element(svg));

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Should have removed only the DOCTYPE node
        assert_eq!(doc.root.children.len(), 2);
        assert!(matches!(doc.root.children[0], Node::Text(_)));
        assert!(matches!(doc.root.children[1], Node::Element(_)));
    }

    #[test]
    fn test_config_parsing() {
        let config = RemoveDoctypePlugin::_parse_config(&json!({})).unwrap();
        // No fields to check since config is empty
        let _ = config;
    }
}

// Use parameterized testing framework for SVGO fixture tests
crate::plugin_fixture_tests!(RemoveDoctypePlugin, "removeDoctype");
