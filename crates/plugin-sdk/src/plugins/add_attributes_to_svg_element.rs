// this_file: crates/plugin-sdk/src/plugins/add_attributes_to_svg_element.rs

//! Add attributes to SVG element plugin implementation
//!
//! This plugin adds attributes to the outer <svg> element.
//! It can add either attribute names (with empty values) or
//! attribute name-value pairs.
//!
//! Reference: SVGO addAttributesToSVGElement plugin


use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use vexy_svgo_core::ast::{Document, Element};
use vexy_svgo_core::Plugin;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AttributeValue {
    String(String),
    Object(HashMap<String, String>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddAttributesToSVGElementConfig {
    pub attribute: Option<AttributeValue>,
    pub attributes: Option<Vec<AttributeValue>>,
}

impl Default for AddAttributesToSVGElementConfig {
    fn default() -> Self {
        Self {
            attribute: None,
            attributes: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AddAttributesToSVGElementPlugin {
    config: AddAttributesToSVGElementConfig,
}

impl AddAttributesToSVGElementPlugin {
    pub fn new() -> Self {
        Self {
            config: AddAttributesToSVGElementConfig::default(),
        }
    }

    pub fn with_config(config: AddAttributesToSVGElementConfig) -> Self {
        Self { config }
    }

    pub fn parse_config(params: &Value) -> Result<AddAttributesToSVGElementConfig, anyhow::Error> {
        let config: AddAttributesToSVGElementConfig = serde_json::from_value(params.clone())?;
        Ok(config)
    }

    fn apply_attributes(&self, element: &mut Element) {
        // Apply single attribute if specified
        if let Some(ref attr) = self.config.attribute {
            self.apply_attribute_value(element, attr);
        }

        // Apply multiple attributes if specified
        if let Some(ref attrs) = self.config.attributes {
            for attr in attrs {
                self.apply_attribute_value(element, attr);
            }
        }
    }

    fn apply_attribute_value(&self, element: &mut Element, attr: &AttributeValue) {
        match attr {
            AttributeValue::String(name) => {
                // Add attribute name with empty value if it doesn't exist
                if !element.attributes.contains_key(name.as_str()) {
                    element.attributes.insert(name.clone().into(), String::new().into());
                }
            }
            AttributeValue::Object(attrs) => {
                // Add each attribute-value pair if the attribute doesn't exist
                for (name, value) in attrs {
                    if !element.attributes.contains_key(name.as_str()) {
                        element.attributes.insert(name.clone().into(), value.clone().into());
                    }
                }
            }
        }
    }
}

impl Default for AddAttributesToSVGElementPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for AddAttributesToSVGElementPlugin {
    fn name(&self) -> &'static str {
        "addAttributesToSVGElement"
    }

    fn description(&self) -> &'static str {
        "adds attributes to an outer <svg> element"
    }

    fn validate_params(&self, params: &Value) -> anyhow::Result<()> {
        let config = Self::parse_config(params)?;

        // Validate that at least one of attribute or attributes is specified
        if config.attribute.is_none() && config.attributes.is_none() {
            return Err(anyhow::anyhow!(
                "Error in plugin \"addAttributesToSVGElement\": absent parameters.\n\
                It should have a list of \"attributes\" or one \"attribute\"."
            ));
        }

        Ok(())
    }

    fn apply(&self, document: &mut Document) -> anyhow::Result<()> {
        // Only apply to root SVG element
        if document.root.name == "svg" {
            self.apply_attributes(&mut document.root);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::borrow::Cow;
    use vexy_svgo_core::ast::{Document, Element};

    fn create_element(name: &'static str) -> Element<'static> {
        let mut element = Element::new(name);
        element.name = Cow::Borrowed(name);
        element
    }

    #[test]
    fn test_plugin_creation() {
        let plugin = AddAttributesToSVGElementPlugin::new();
        assert_eq!(plugin.name(), "addAttributesToSVGElement");
        assert_eq!(
            plugin.description(),
            "adds attributes to an outer <svg> element"
        );
    }

    #[test]
    fn test_parameter_validation_missing_params() {
        let plugin = AddAttributesToSVGElementPlugin::new();

        // Invalid - no parameters
        assert!(plugin.validate_params(&json!({})).is_err());
    }

    #[test]
    fn test_parameter_validation_single_attribute() {
        let plugin = AddAttributesToSVGElementPlugin::new();

        // Valid - single attribute as string
        assert!(plugin
            .validate_params(&json!({
                "attribute": "myAttribute"
            }))
            .is_ok());

        // Valid - single attribute as object
        assert!(plugin
            .validate_params(&json!({
                "attribute": {"data-name": "value"}
            }))
            .is_ok());
    }

    #[test]
    fn test_parameter_validation_multiple_attributes() {
        let plugin = AddAttributesToSVGElementPlugin::new();

        // Valid - array of attributes
        assert!(plugin
            .validate_params(&json!({
                "attributes": ["attr1", "attr2"]
            }))
            .is_ok());

        // Valid - array with mixed types
        assert!(plugin
            .validate_params(&json!({
                "attributes": ["attr1", {"data-name": "value"}]
            }))
            .is_ok());
    }

    #[test]
    fn test_add_single_string_attribute() {
        let config = AddAttributesToSVGElementConfig {
            attribute: Some(AttributeValue::String("myAttribute".to_string())),
            attributes: None,
        };
        let plugin = AddAttributesToSVGElementPlugin::with_config(config);

        let mut doc = Document::new();
        doc.root = create_element("svg");

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Check that attribute was added with empty value
        assert_eq!(doc.root.attributes.get("myAttribute"), Some(&String::new()));
    }

    #[test]
    fn test_add_single_object_attribute() {
        let mut attrs = HashMap::new();
        attrs.insert("data-name".to_string(), "myValue".to_string());
        attrs.insert("data-id".to_string(), "123".to_string());

        let config = AddAttributesToSVGElementConfig {
            attribute: Some(AttributeValue::Object(attrs)),
            attributes: None,
        };
        let plugin = AddAttributesToSVGElementPlugin::with_config(config);

        let mut doc = Document::new();
        doc.root = create_element("svg");

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Check that attributes were added
        assert_eq!(
            doc.root.attributes.get("data-name"),
            Some(&"myValue".to_string())
        );
        assert_eq!(doc.root.attributes.get("data-id"), Some(&"123".to_string()));
    }

    #[test]
    fn test_add_multiple_attributes() {
        let mut object_attrs = HashMap::new();
        object_attrs.insert("data-test".to_string(), "value".to_string());

        let config = AddAttributesToSVGElementConfig {
            attribute: None,
            attributes: Some(vec![
                AttributeValue::String("class".to_string()),
                AttributeValue::Object(object_attrs),
                AttributeValue::String("id".to_string()),
            ]),
        };
        let plugin = AddAttributesToSVGElementPlugin::with_config(config);

        let mut doc = Document::new();
        doc.root = create_element("svg");

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Check that all attributes were added
        assert_eq!(doc.root.attributes.get("class"), Some(&String::new()));
        assert_eq!(doc.root.attributes.get("id"), Some(&String::new()));
        assert_eq!(
            doc.root.attributes.get("data-test"),
            Some(&"value".to_string())
        );
    }

    #[test]
    fn test_does_not_override_existing_attributes() {
        let config = AddAttributesToSVGElementConfig {
            attribute: Some(AttributeValue::String("class".to_string())),
            attributes: None,
        };
        let plugin = AddAttributesToSVGElementPlugin::with_config(config);

        let mut doc = Document::new();
        doc.root = create_element("svg");
        doc.root
            .attributes
            .insert("class".to_string(), "existing-class".to_string());

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Check that existing attribute was not overridden
        assert_eq!(
            doc.root.attributes.get("class"),
            Some(&"existing-class".to_string())
        );
    }

    #[test]
    fn test_only_applies_to_svg_element() {
        let config = AddAttributesToSVGElementConfig {
            attribute: Some(AttributeValue::String("myAttr".to_string())),
            attributes: None,
        };
        let plugin = AddAttributesToSVGElementPlugin::with_config(config);

        let mut doc = Document::new();
        doc.root = create_element("div"); // Not an SVG element

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Check that no attributes were added
        assert_eq!(doc.root.attributes.len(), 0);
    }

    #[test]
    fn test_both_attribute_and_attributes() {
        let mut object_attrs = HashMap::new();
        object_attrs.insert("data-value".to_string(), "test".to_string());

        let config = AddAttributesToSVGElementConfig {
            attribute: Some(AttributeValue::String("single".to_string())),
            attributes: Some(vec![
                AttributeValue::String("multiple".to_string()),
                AttributeValue::Object(object_attrs),
            ]),
        };
        let plugin = AddAttributesToSVGElementPlugin::with_config(config);

        let mut doc = Document::new();
        doc.root = create_element("svg");

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Check that all attributes were added
        assert_eq!(doc.root.attributes.get("single"), Some(&String::new()));
        assert_eq!(doc.root.attributes.get("multiple"), Some(&String::new()));
        assert_eq!(
            doc.root.attributes.get("data-value"),
            Some(&"test".to_string())
        );
    }

    #[test]
    fn test_config_parsing() {
        // Test single string attribute
        let config = AddAttributesToSVGElementPlugin::parse_config(&json!({
            "attribute": "test"
        }))
        .unwrap();

        if let Some(AttributeValue::String(s)) = config.attribute {
            assert_eq!(s, "test");
        } else {
            panic!("Expected string attribute");
        }

        // Test array of attributes
        let config = AddAttributesToSVGElementPlugin::parse_config(&json!({
            "attributes": ["attr1", {"key": "value"}]
        }))
        .unwrap();

        if let Some(attrs) = config.attributes {
            assert_eq!(attrs.len(), 2);
        } else {
            panic!("Expected attributes array");
        }
    }
}

// Use parameterized testing framework for SVGO fixture tests
crate::plugin_fixture_tests_with_params!(AddAttributesToSVGElementPlugin, "addAttributesToSVGElement");