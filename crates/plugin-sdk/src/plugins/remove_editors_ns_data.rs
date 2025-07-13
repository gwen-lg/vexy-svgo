// this_file: crates/plugin-sdk/src/plugins/remove_editors_ns_data.rs

//! Remove editors namespace data plugin implementation
//!
//! This plugin removes namespace declarations, attributes, and elements from various
//! SVG editors like Inkscape, Illustrator, Sketch, etc. These editor-specific data
//! are not needed for SVG rendering and can significantly reduce file size.
//!
//! SVGO parameters supported:
//! - `additionalNamespaces` (default: []) - Additional namespace URIs to remove

use crate::Plugin;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashSet;
use std::sync::LazyLock;
use vexy_svgo_core::ast::{Document, Element, Node};
use vexy_svgo_core::visitor::Visitor;

/// Default editor namespaces to remove
static EDITOR_NAMESPACES: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    HashSet::from([
        "http://creativecommons.org/ns#",
        "http://inkscape.sourceforge.net/DTD/sodipodi-0.dtd",
        "http://krita.org/namespaces/svg/krita",
        "http://ns.adobe.com/AdobeIllustrator/10.0/",
        "http://ns.adobe.com/AdobeSVGViewerExtensions/3.0/",
        "http://ns.adobe.com/Extensibility/1.0/",
        "http://ns.adobe.com/Flows/1.0/",
        "http://ns.adobe.com/GenericCustomNamespace/1.0/",
        "http://ns.adobe.com/Graphs/1.0/",
        "http://ns.adobe.com/ImageReplacement/1.0/",
        "http://ns.adobe.com/SaveForWeb/1.0/",
        "http://ns.adobe.com/Variables/1.0/",
        "http://ns.adobe.com/XPath/1.0/",
        "http://purl.org/dc/elements/1.1/",
        "http://schemas.microsoft.com/visio/2003/SVGExtensions/",
        "http://sodipodi.sourceforge.net/DTD/sodipodi-0.dtd",
        "http://taptrix.com/vectorillustrator/svg_extensions",
        "http://www.bohemiancoding.com/sketch/ns",
        "http://www.figma.com/figma/ns",
        "http://www.inkscape.org/namespaces/inkscape",
        "http://www.serif.com/",
        "http://www.vector.evaxdesign.sk",
        "http://www.w3.org/1999/02/22-rdf-syntax-ns#",
        "https://boxy-svg.com",
    ])
});

/// Configuration parameters for remove editors ns data plugin
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveEditorsNSDataConfig {
    /// Additional namespace URIs to remove
    #[serde(default)]
    pub additional_namespaces: Vec<String>,
}

impl Default for RemoveEditorsNSDataConfig {
    fn default() -> Self {
        Self {
            additional_namespaces: Vec::new(),
        }
    }
}

/// Plugin that removes editor namespace data
pub struct RemoveEditorsNSDataPlugin {
    #[allow(dead_code)]
    config: RemoveEditorsNSDataConfig,
}

impl RemoveEditorsNSDataPlugin {
    /// Create a new RemoveEditorsNSDataPlugin
    pub fn new() -> Self {
        Self {
            #[allow(dead_code)]
            config: RemoveEditorsNSDataConfig::default(),
        }
    }

    /// Create a new RemoveEditorsNSDataPlugin with config
    pub fn with_config(config: RemoveEditorsNSDataConfig) -> Self {
        Self { config }
    }

    /// Parse configuration from JSON
    fn _parse_config(params: &Value) -> Result<RemoveEditorsNSDataConfig> {
        if let Some(_obj) = params.as_object() {
            serde_json::from_value(params.clone())
                .map_err(|e| anyhow!("Invalid configuration: {}", e))
        } else {
            Ok(RemoveEditorsNSDataConfig::default())
        }
    }
}

impl Default for RemoveEditorsNSDataPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for RemoveEditorsNSDataPlugin {
    fn name(&self) -> &'static str {
        "removeEditorsNSData"
    }

    fn description(&self) -> &'static str {
        "Remove editors namespaces, elements and attributes"
    }

    fn validate_params(&self, params: &Value) -> Result<()> {
        if let Some(obj) = params.as_object() {
            // Validate parameters
            for (key, value) in obj {
                match key.as_str() {
                    "additionalNamespaces" => {
                        if !value.is_array() {
                            return Err(anyhow!("{} must be an array", key));
                        }
                        if let Some(arr) = value.as_array() {
                            for item in arr {
                                if !item.is_string() {
                                    return Err(anyhow!(
                                        "additionalNamespaces must contain only strings"
                                    ));
                                }
                            }
                        }
                    }
                    _ => return Err(anyhow!("Unknown parameter: {}", key)),
                }
            }
        }
        Ok(())
    }

    fn apply(&self, document: &mut Document) -> Result<()> {
        let mut visitor = EditorsNSDataRemovalVisitor::new(self.config.clone());
        vexy_svgo_core::visitor::walk_document(&mut visitor, document)?;
        Ok(())
    }
}

/// State for tracking discovered namespace prefixes
#[derive(Debug)]
struct NamespaceState {
    /// Namespace URIs to remove
    namespaces_to_remove: HashSet<String>,
    /// Discovered prefixes that map to editor namespaces
    prefixes_to_remove: HashSet<String>,
}

impl NamespaceState {
    fn new(additional_namespaces: &[String]) -> Self {
        let mut namespaces_to_remove = HashSet::new();

        // Add default editor namespaces
        for ns in EDITOR_NAMESPACES.iter() {
            namespaces_to_remove.insert((*ns).to_string());
        }

        // Add additional namespaces
        for ns in additional_namespaces {
            namespaces_to_remove.insert(ns.clone());
        }

        Self {
            namespaces_to_remove,
            prefixes_to_remove: HashSet::new(),
        }
    }
}

/// Visitor implementation that removes editor namespace data
struct EditorsNSDataRemovalVisitor {
    #[allow(dead_code)]
    config: RemoveEditorsNSDataConfig,
    state: NamespaceState,
}

impl EditorsNSDataRemovalVisitor {
    fn new(config: RemoveEditorsNSDataConfig) -> Self {
        let state = NamespaceState::new(&config.additional_namespaces);
        Self { config, state }
    }

    /// Process namespace declarations on SVG element
    fn process_namespace_declarations(&mut self, element: &mut Element) {
        if element.name == "svg" {
            let mut attrs_to_remove = Vec::new();

            // Find xmlns declarations that match editor namespaces
            for (name, value) in &element.attributes {
                if name.starts_with("xmlns:") {
                    if self.state.namespaces_to_remove.contains(value.as_ref()) {
                        // Extract the prefix
                        let prefix = &name[6..];
                        self.state.prefixes_to_remove.insert(prefix.to_string());
                        attrs_to_remove.push(name.clone());
                    }
                } else if name == "xmlns" && self.state.namespaces_to_remove.contains(value.as_ref()) {
                    // Default namespace is an editor namespace
                    attrs_to_remove.push(name.clone());
                }
            }

            // Remove the xmlns declarations
            for attr in attrs_to_remove {
                element.attributes.shift_remove(&attr);
            }
        }
    }

    /// Remove attributes with editor namespace prefixes
    fn remove_prefixed_attributes(&self, element: &mut Element) {
        let mut attrs_to_remove = Vec::new();

        for name in element.attributes.keys() {
            if let Some(colon_pos) = name.find(':') {
                let prefix = &name[..colon_pos];
                if self.state.prefixes_to_remove.contains(prefix) {
                    attrs_to_remove.push(name.clone());
                }
            }
        }

        // Remove the editor attributes
        for attr in attrs_to_remove {
            element.attributes.shift_remove(&attr);
        }
    }

    /// Check if an element should be removed based on its namespace prefix
    fn should_remove_element(&self, element: &Element) -> bool {
        if let Some(colon_pos) = element.name.find(':') {
            let prefix = &element.name[..colon_pos];
            return self.state.prefixes_to_remove.contains(prefix);
        }
        false
    }
}

impl Visitor<'_> for EditorsNSDataRemovalVisitor {
    fn visit_element_enter(&mut self, element: &mut Element<'_>) -> Result<()> {
        // Process namespace declarations on SVG root
        self.process_namespace_declarations(element);

        // Remove prefixed attributes
        self.remove_prefixed_attributes(element);

        Ok(())
    }

    fn visit_element_exit(&mut self, element: &mut Element<'_>) -> Result<()> {
        // Remove child elements with editor namespace prefixes
        element.children.retain(|child| {
            if let Node::Element(child_element) = child {
                !self.should_remove_element(child_element)
            } else {
                true // Keep non-element nodes
            }
        });

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
        let plugin = RemoveEditorsNSDataPlugin::new();
        assert_eq!(plugin.name(), "removeEditorsNSData");
    }

    #[test]
    fn test_parameter_validation() {
        let plugin = RemoveEditorsNSDataPlugin::new();

        // Valid parameters
        assert!(plugin.validate_params(&json!({})).is_ok());
        assert!(plugin
            .validate_params(&json!({
                "additionalNamespaces": ["http://example.com/ns"]
            }))
            .is_ok());

        // Invalid parameters
        assert!(plugin
            .validate_params(&json!({"additionalNamespaces": "invalid"}))
            .is_err());
        assert!(plugin
            .validate_params(&json!({"additionalNamespaces": [123]}))
            .is_err());
        assert!(plugin
            .validate_params(&json!({"unknownParam": true}))
            .is_err());
    }

    #[test]
    fn test_remove_inkscape_namespace() {
        let plugin = RemoveEditorsNSDataPlugin::new();
        let mut doc = Document::new();

        // Set up SVG with Inkscape namespace
        doc.root.attributes.insert(
            "xmlns:inkscape".to_string(),
            "http://www.inkscape.org/namespaces/inkscape".to_string(),
        );

        // Add element with Inkscape attributes
        let mut rect = create_element("rect");
        rect.attributes
            .insert("inkscape:label".to_string(), "Layer 1".to_string());
        rect.attributes
            .insert("width".to_string(), "100".to_string());

        doc.root.children.push(Node::Element(rect));

        plugin.apply(&mut doc).unwrap();

        // Namespace declaration should be removed
        assert!(!doc.root.attributes.contains_key("xmlns:inkscape"));

        // Inkscape attribute should be removed
        if let Some(Node::Element(rect)) = doc.root.children.get(0) {
            assert!(!rect.attributes.contains_key("inkscape:label"));
            assert_eq!(rect.attributes.get("width"), Some(&"100".to_string()));
        }
    }

    #[test]
    fn test_remove_illustrator_namespace() {
        let plugin = RemoveEditorsNSDataPlugin::new();
        let mut doc = Document::new();

        // Set up SVG with Illustrator namespace
        doc.root.attributes.insert(
            "xmlns:i".to_string(),
            "http://ns.adobe.com/AdobeIllustrator/10.0/".to_string(),
        );

        // Add Illustrator-specific element
        let mut ai_element = create_element("i:pgf");
        ai_element
            .attributes
            .insert("id".to_string(), "adobe_illustrator".to_string());

        doc.root.children.push(Node::Element(ai_element));

        plugin.apply(&mut doc).unwrap();

        // Namespace declaration should be removed
        assert!(!doc.root.attributes.contains_key("xmlns:i"));

        // Illustrator element should be removed
        assert!(doc.root.children.is_empty());
    }

    #[test]
    fn test_remove_multiple_namespaces() {
        let plugin = RemoveEditorsNSDataPlugin::new();
        let mut doc = Document::new();

        // Set up SVG with multiple editor namespaces
        doc.root.attributes.insert(
            "xmlns:inkscape".to_string(),
            "http://www.inkscape.org/namespaces/inkscape".to_string(),
        );
        doc.root.attributes.insert(
            "xmlns:sodipodi".to_string(),
            "http://sodipodi.sourceforge.net/DTD/sodipodi-0.dtd".to_string(),
        );
        doc.root.attributes.insert(
            "xmlns:sketch".to_string(),
            "http://www.bohemiancoding.com/sketch/ns".to_string(),
        );

        plugin.apply(&mut doc).unwrap();

        // All editor namespace declarations should be removed
        assert!(!doc.root.attributes.contains_key("xmlns:inkscape"));
        assert!(!doc.root.attributes.contains_key("xmlns:sodipodi"));
        assert!(!doc.root.attributes.contains_key("xmlns:sketch"));
    }

    #[test]
    fn test_additional_namespaces() {
        let config = RemoveEditorsNSDataConfig {
            additional_namespaces: vec!["http://custom.editor/ns".to_string()],
        };
        let plugin = RemoveEditorsNSDataPlugin::with_config(config);
        let mut doc = Document::new();

        // Set up SVG with custom namespace
        doc.root.attributes.insert(
            "xmlns:custom".to_string(),
            "http://custom.editor/ns".to_string(),
        );

        // Add element with custom namespace
        let mut elem = create_element("custom:data");
        elem.attributes
            .insert("value".to_string(), "test".to_string());

        doc.root.children.push(Node::Element(elem));

        plugin.apply(&mut doc).unwrap();

        // Custom namespace should be removed
        assert!(!doc.root.attributes.contains_key("xmlns:custom"));

        // Custom element should be removed
        assert!(doc.root.children.is_empty());
    }

    #[test]
    fn test_preserve_standard_namespaces() {
        let plugin = RemoveEditorsNSDataPlugin::new();
        let mut doc = Document::new();

        // Set up SVG with standard namespaces
        doc.root.attributes.insert(
            "xmlns".to_string(),
            "http://www.w3.org/2000/svg".to_string(),
        );
        doc.root.attributes.insert(
            "xmlns:xlink".to_string(),
            "http://www.w3.org/1999/xlink".to_string(),
        );

        // Add Inkscape namespace to be removed
        doc.root.attributes.insert(
            "xmlns:inkscape".to_string(),
            "http://www.inkscape.org/namespaces/inkscape".to_string(),
        );

        plugin.apply(&mut doc).unwrap();

        // Standard namespaces should be preserved
        assert_eq!(
            doc.root.attributes.get("xmlns"),
            Some(&"http://www.w3.org/2000/svg".to_string())
        );
        assert_eq!(
            doc.root.attributes.get("xmlns:xlink"),
            Some(&"http://www.w3.org/1999/xlink".to_string())
        );

        // Editor namespace should be removed
        assert!(!doc.root.attributes.contains_key("xmlns:inkscape"));
    }

    #[test]
    fn test_nested_elements() {
        let plugin = RemoveEditorsNSDataPlugin::new();
        let mut doc = Document::new();

        // Set up SVG with Inkscape namespace
        doc.root.attributes.insert(
            "xmlns:inkscape".to_string(),
            "http://www.inkscape.org/namespaces/inkscape".to_string(),
        );

        // Create nested structure with mixed elements
        let mut g = create_element("g");

        let rect = create_element("rect");
        g.children.push(Node::Element(rect));

        let inkscape_elem = create_element("inkscape:perspective");
        g.children.push(Node::Element(inkscape_elem));

        let circle = create_element("circle");
        g.children.push(Node::Element(circle));

        doc.root.children.push(Node::Element(g));

        plugin.apply(&mut doc).unwrap();

        // Check that only non-editor elements remain
        if let Some(Node::Element(g)) = doc.root.children.get(0) {
            assert_eq!(g.children.len(), 2);

            if let Some(Node::Element(elem1)) = g.children.get(0) {
                assert_eq!(elem1.name, "rect");
            }
            if let Some(Node::Element(elem2)) = g.children.get(1) {
                assert_eq!(elem2.name, "circle");
            }
        }
    }

    #[test]
    fn test_config_parsing() {
        let config = RemoveEditorsNSDataPlugin::_parse_config(&json!({
            "additionalNamespaces": ["http://example.com/ns1", "http://example.com/ns2"]
        }))
        .unwrap();

        assert_eq!(config.additional_namespaces.len(), 2);
        assert_eq!(config.additional_namespaces[0], "http://example.com/ns1");
        assert_eq!(config.additional_namespaces[1], "http://example.com/ns2");
    }
}

// Use parameterized testing framework for SVGO fixture tests
crate::plugin_fixture_tests!(RemoveEditorsNSDataPlugin, "removeEditorsNSData");
