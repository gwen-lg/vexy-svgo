// this_file: crates/plugin-sdk/src/plugins/remove_scripts.rs

//! Remove scripts plugin implementation
//!
//! This plugin removes all script elements and event attributes from SVG documents
//! to improve security. It removes:
//! - All <script> elements
//! - All event handler attributes (onclick, onload, etc.)
//! - JavaScript URLs in href attributes
//!
//! Reference: SVGO's removeScripts plugin

use crate::Plugin;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashSet;
use vexy_svgo_core::ast::{Document, Element, Node};

/// Configuration parameters for remove scripts plugin
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RemoveScriptsConfig {
    // No configuration options for this plugin
}

impl Default for RemoveScriptsConfig {
    fn default() -> Self {
        Self {}
    }
}

/// Plugin that removes scripts and event attributes
pub struct RemoveScriptsPlugin {
    config: RemoveScriptsConfig,
    event_attrs: HashSet<&'static str>,
}

impl RemoveScriptsPlugin {
    /// Create a new RemoveScriptsPlugin
    pub fn new() -> Self {
        // Initialize with all event attributes from SVGO's collections
        let mut event_attrs = HashSet::new();

        // animationEvent
        event_attrs.extend(&["onbegin", "onend", "onrepeat", "onload"]);

        // documentEvent
        event_attrs.extend(&[
            "onabort", "onerror", "onresize", "onscroll", "onunload", "onzoom",
        ]);

        // documentElementEvent
        event_attrs.extend(&["oncopy", "oncut", "onpaste"]);

        // globalEvent
        event_attrs.extend(&[
            "oncancel",
            "oncanplay",
            "oncanplaythrough",
            "onchange",
            "onclick",
            "onclose",
            "oncuechange",
            "ondblclick",
            "ondrag",
            "ondragend",
            "ondragenter",
            "ondragleave",
            "ondragover",
            "ondragstart",
            "ondrop",
            "ondurationchange",
            "onemptied",
            "onended",
            "onerror",
            "onfocus",
            "oninput",
            "oninvalid",
            "onkeydown",
            "onkeypress",
            "onkeyup",
            "onload",
            "onloadeddata",
            "onloadedmetadata",
            "onloadstart",
            "onmousedown",
            "onmouseenter",
            "onmouseleave",
            "onmousemove",
            "onmouseout",
            "onmouseover",
            "onmouseup",
            "onmousewheel",
            "onpause",
            "onplay",
            "onplaying",
            "onprogress",
            "onratechange",
            "onreset",
            "onresize",
            "onscroll",
            "onseeked",
            "onseeking",
            "onselect",
            "onshow",
            "onstalled",
            "onsubmit",
            "onsuspend",
            "ontimeupdate",
            "ontoggle",
            "onvolumechange",
            "onwaiting",
        ]);

        // graphicalEvent
        event_attrs.extend(&[
            "onactivate",
            "onclick",
            "onfocusin",
            "onfocusout",
            "onload",
            "onmousedown",
            "onmousemove",
            "onmouseout",
            "onmouseover",
            "onmouseup",
        ]);

        Self {
            config: RemoveScriptsConfig::default(),
            event_attrs,
        }
    }

    /// Create a new RemoveScriptsPlugin with config
    pub fn with_config(config: RemoveScriptsConfig) -> Self {
        let mut plugin = Self::new();
        plugin.config = config;
        plugin
    }

    /// Parse configuration from JSON
    fn parse_config(params: &Value) -> Result<RemoveScriptsConfig> {
        if params.is_null() {
            Ok(RemoveScriptsConfig::default())
        } else if params.is_object() {
            serde_json::from_value(params.clone())
                .map_err(|e| anyhow::anyhow!("Invalid configuration: {}", e))
        } else {
            Err(anyhow::anyhow!("Configuration must be an object"))
        }
    }

    /// Process element to remove scripts and event attributes
    fn process_element(&self, element: &mut Element) {
        // Remove all event attributes
        element
            .attributes
            .retain(|name, _| !self.event_attrs.contains(name.as_ref()));

        // Special handling for <a> elements with javascript: hrefs
        if element.name == "a" {
            let has_javascript_href = element.attributes.iter().any(|(name, value)| {
                (name == "href" || name.ends_with(":href"))
                    && value.trim_start().starts_with("javascript:")
            });

            if has_javascript_href {
                // Extract useful children (non-text nodes)
                let useful_children: Vec<Node> = element
                    .children
                    .iter()
                    .filter(|child| !matches!(child, Node::Text(_)))
                    .cloned()
                    .collect();

                // Replace the element's children with only useful children
                element.children = useful_children;

                // Remove the href attribute
                element
                    .attributes
                    .retain(|name, _| !(name == "href" || name.ends_with(":href")));
            }
        }

        // Remove script elements
        element.children.retain(|child| {
            if let Node::Element(elem) = child {
                elem.name != "script"
            } else {
                true
            }
        });

        // Recursively process children
        for child in &mut element.children {
            if let Node::Element(elem) = child {
                self.process_element(elem);
            }
        }
    }
}

impl Default for RemoveScriptsPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for RemoveScriptsPlugin {
    fn name(&self) -> &'static str {
        "removeScripts"
    }

    fn description(&self) -> &'static str {
        "removes scripts (disabled by default)"
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
        let plugin = RemoveScriptsPlugin::new();
        assert_eq!(plugin.name(), "removeScripts");
        assert_eq!(
            plugin.description(),
            "removes scripts (disabled by default)"
        );
    }

    #[test]
    fn test_removes_script_elements() {
        let plugin = RemoveScriptsPlugin::new();

        let mut doc = Document::new();
        doc.root = create_element("svg");

        // Add script element
        let mut script = create_element("script");
        script
            .children
            .push(Node::Text("alert('hello');".to_string()));
        doc.root.children.push(Node::Element(script));

        // Add non-script element
        let rect = create_element("rect");
        doc.root.children.push(Node::Element(rect));

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Check that script was removed
        assert_eq!(doc.root.children.len(), 1);
        if let Node::Element(elem) = &doc.root.children[0] {
            assert_eq!(elem.name, "rect");
        } else {
            panic!("Expected element node");
        }
    }

    #[test]
    fn test_removes_event_attributes() {
        let plugin = RemoveScriptsPlugin::new();

        let mut doc = Document::new();
        doc.root = create_element("svg");
        doc.root
            .attributes
            .insert("onclick".to_string(), "alert('clicked')".to_string());
        doc.root
            .attributes
            .insert("onload".to_string(), "init()".to_string());
        doc.root
            .attributes
            .insert("width".to_string(), "100".to_string());

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Check that event attributes were removed
        assert!(!doc.root.attributes.contains_key("onclick"));
        assert!(!doc.root.attributes.contains_key("onload"));
        assert_eq!(doc.root.attributes.get("width"), Some(&"100".to_string()));
    }

    #[test]
    fn test_removes_javascript_hrefs() {
        let plugin = RemoveScriptsPlugin::new();

        let mut doc = Document::new();
        doc.root = create_element("svg");

        // Add anchor with javascript href
        let mut anchor = create_element("a");
        anchor
            .attributes
            .insert("href".to_string(), "javascript:void(0)".to_string());

        // Add children to anchor
        anchor.children.push(Node::Text("Click me".to_string()));
        let rect = create_element("rect");
        anchor.children.push(Node::Element(rect));

        doc.root.children.push(Node::Element(anchor));

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Check that javascript href was removed
        if let Node::Element(elem) = &doc.root.children[0] {
            assert!(!elem.attributes.contains_key("href"));
            // Check that only non-text children remain
            assert_eq!(elem.children.len(), 1);
            if let Node::Element(child) = &elem.children[0] {
                assert_eq!(child.name, "rect");
            }
        } else {
            panic!("Expected element node");
        }
    }

    #[test]
    fn test_removes_xlink_javascript_hrefs() {
        let plugin = RemoveScriptsPlugin::new();

        let mut doc = Document::new();
        doc.root = create_element("svg");

        // Add anchor with xlink:href javascript
        let mut anchor = create_element("a");
        anchor.attributes.insert(
            "xlink:href".to_string(),
            "  javascript:alert('test')".to_string(),
        );
        anchor
            .children
            .push(Node::Element(create_element("circle")));

        doc.root.children.push(Node::Element(anchor));

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Check that xlink:href was removed
        if let Node::Element(elem) = &doc.root.children[0] {
            assert!(!elem.attributes.contains_key("xlink:href"));
        }
    }

    #[test]
    fn test_preserves_non_javascript_hrefs() {
        let plugin = RemoveScriptsPlugin::new();

        let mut doc = Document::new();
        doc.root = create_element("svg");

        // Add anchor with normal href
        let mut anchor = create_element("a");
        anchor
            .attributes
            .insert("href".to_string(), "https://example.com".to_string());
        anchor.children.push(Node::Text("Link".to_string()));

        doc.root.children.push(Node::Element(anchor));

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Check that normal href was preserved
        if let Node::Element(elem) = &doc.root.children[0] {
            assert_eq!(
                elem.attributes.get("href"),
                Some(&"https://example.com".to_string())
            );
            assert_eq!(elem.children.len(), 1); // Text node preserved
        }
    }

    #[test]
    fn test_nested_script_removal() {
        let plugin = RemoveScriptsPlugin::new();

        let mut doc = Document::new();
        doc.root = create_element("svg");

        // Create nested structure
        let mut group = create_element("g");
        group
            .attributes
            .insert("onclick".to_string(), "handleClick()".to_string());

        let mut script = create_element("script");
        script
            .children
            .push(Node::Text("console.log('test');".to_string()));
        group.children.push(Node::Element(script));

        let mut rect = create_element("rect");
        rect.attributes
            .insert("onmouseover".to_string(), "highlight()".to_string());
        group.children.push(Node::Element(rect));

        doc.root.children.push(Node::Element(group));

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Check nested removal
        if let Node::Element(g) = &doc.root.children[0] {
            assert!(!g.attributes.contains_key("onclick"));
            assert_eq!(g.children.len(), 1); // Only rect remains

            if let Node::Element(rect) = &g.children[0] {
                assert!(!rect.attributes.contains_key("onmouseover"));
            }
        }
    }

    #[test]
    fn test_removes_all_event_types() {
        let plugin = RemoveScriptsPlugin::new();

        let mut doc = Document::new();
        doc.root = create_element("svg");

        // Add various event attributes
        doc.root
            .attributes
            .insert("onbegin".to_string(), "startAnim()".to_string()); // animationEvent
        doc.root
            .attributes
            .insert("onzoom".to_string(), "handleZoom()".to_string()); // documentEvent
        doc.root
            .attributes
            .insert("oncopy".to_string(), "handleCopy()".to_string()); // documentElementEvent
        doc.root
            .attributes
            .insert("ondrag".to_string(), "handleDrag()".to_string()); // globalEvent
        doc.root
            .attributes
            .insert("onfocusin".to_string(), "handleFocus()".to_string()); // graphicalEvent
        doc.root
            .attributes
            .insert("viewBox".to_string(), "0 0 100 100".to_string()); // non-event

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Check that all event attributes were removed
        assert_eq!(doc.root.attributes.len(), 1);
        assert_eq!(
            doc.root.attributes.get("viewBox"),
            Some(&"0 0 100 100".to_string())
        );
    }

    #[test]
    fn test_empty_document() {
        let plugin = RemoveScriptsPlugin::new();

        let mut doc = Document::new();
        doc.root = create_element("svg");

        // Apply plugin to empty document
        let result = plugin.apply(&mut doc);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parameter_validation() {
        let plugin = RemoveScriptsPlugin::new();

        // Empty object is valid
        assert!(plugin.validate_params(&json!({})).is_ok());

        // Null is valid
        assert!(plugin.validate_params(&Value::Null).is_ok());

        // Non-object is invalid
        assert!(plugin.validate_params(&json!("invalid")).is_err());
    }
}

// Use parameterized testing framework for SVGO fixture tests
crate::plugin_fixture_tests!(RemoveScriptsPlugin, "removeScripts");
