// this_file: crates/plugin-sdk/src/plugins/add_classes_to_svg_element.rs

//! Add classes to SVG element plugin implementation
//!
//! This plugin adds class names to the outer <svg> element.
//! It adds classes to the existing class attribute, preserving
//! any classes that are already present.
//!
//! Reference: SVGOPROTECTED_75_static str {
        "addClassesToSVGElement"
    }

    fn description(&self) -> &'static str {
        "adds classnames to an outer <svg> element"
    }

    fn validate_params(&self, params: &Value) -> Result<()> {
        let config = Self::parse_config(params)?;

        // Validate that at least one of className or classNames is specified
        if config.class_name.is_none()
            && (config.class_names.is_none() || config.class_names.as_ref().unwrap().is_empty())
        {
            return Err(anyhow::anyhow!(
                "Error in plugin \"addClassesToSVGElement\": absent parameters.\n\
                It should have a list of classes in \"classNames\" or one \"className\"."
            ));
        }

        Ok(())
    }

    fn apply(&self, document: &mut Document) -> Result<()> {
        // Only apply to root SVG element
        if document.root.name == "svg" {
            self.apply_classes(&mut document.root);
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
        let plugin = AddClassesToSVGElementPlugin::new();
        assert_eq!(plugin.name(), "addClassesToSVGElement");
        assert_eq!(
            plugin.description(),
            "adds classnames to an outer <svg> element"
        );
    }

    #[test]
    fn test_parameter_validation_missing_params() {
        let plugin = AddClassesToSVGElementPlugin::new();

        // Invalid - no parameters
        assert!(plugin.validate_params(&json!({})).is_err());

        // Invalid - empty classNames array
        assert!(plugin
            .validate_params(&json!({
                "classNames": []
            }))
            .is_err());
    }

    #[test]
    fn test_parameter_validation_single_class() {
        let plugin = AddClassesToSVGElementPlugin::new();

        // Valid - single class
        assert!(plugin
            .validate_params(&json!({
                "className": "myClass"
            }))
            .is_ok());
    }

    #[test]
    fn test_parameter_validation_multiple_classes() {
        let plugin = AddClassesToSVGElementPlugin::new();

        // Valid - array of classes
        assert!(plugin
            .validate_params(&json!({
                "classNames": ["class1", "class2"]
            }))
            .is_ok());
    }

    #[test]
    fn test_add_single_class() {
        let config = AddClassesToSVGElementConfig {
            class_name: Some("myClass".to_string()),
            class_names: None,
        };
        let plugin = AddClassesToSVGElementPlugin::with_config(config);

        let mut doc = Document::new();
        doc.root = create_element("svg");

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Check that class was added
        assert_eq!(
            doc.root.attributes.get("class"),
            Some(&"myClass".to_string())
        );
    }

    #[test]
    fn test_add_multiple_classes() {
        let config = AddClassesToSVGElementConfig {
            class_name: None,
            class_names: Some(vec![
                "class1".to_string(),
                "class2".to_string(),
                "class3".to_string(),
            ]),
        };
        let plugin = AddClassesToSVGElementPlugin::with_config(config);

        let mut doc = Document::new();
        doc.root = create_element("svg");

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Check that all classes were added
        let class_attr = doc.root.attributes.get("class").unwrap();
        let classes: HashSet<&str> = class_attr.split_whitespace().collect();
        assert!(classes.contains("class1"));
        assert!(classes.contains("class2"));
        assert!(classes.contains("class3"));
    }

    #[test]
    fn test_preserves_existing_classes() {
        let config = AddClassesToSVGElementConfig {
            class_name: Some("newClass".to_string()),
            class_names: None,
        };
        let plugin = AddClassesToSVGElementPlugin::with_config(config);

        let mut doc = Document::new();
        doc.root = create_element("svg");
        doc.root.attributes.insert(
            "class".to_string(),
            "existingClass1 existingClass2".to_string(),
        );

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Check that both existing and new classes are present
        let class_attr = doc.root.attributes.get("class").unwrap();
        let classes: HashSet<&str> = class_attr.split_whitespace().collect();
        assert!(classes.contains("existingClass1"));
        assert!(classes.contains("existingClass2"));
        assert!(classes.contains("newClass"));
    }

    #[test]
    fn test_deduplicates_classes() {
        let config = AddClassesToSVGElementConfig {
            class_name: Some("duplicateClass".to_string()),
            class_names: Some(vec![
                "duplicateClass".to_string(),
                "uniqueClass".to_string(),
            ]),
        };
        let plugin = AddClassesToSVGElementPlugin::with_config(config);

        let mut doc = Document::new();
        doc.root = create_element("svg");
        doc.root
            .attributes
            .insert("class".to_string(), "duplicateClass".to_string());

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Check that duplicate class appears only once
        let class_attr = doc.root.attributes.get("class").unwrap();
        let classes: Vec<&str> = class_attr.split_whitespace().collect();
        assert_eq!(
            classes.iter().filter(|&&c| c == "duplicateClass").count(),
            1
        );
        assert!(classes.contains(&"uniqueClass"));
    }

    #[test]
    fn test_only_applies_to_svg_element() {
        let config = AddClassesToSVGElementConfig {
            class_name: Some("myClass".to_string()),
            class_names: None,
        };
        let plugin = AddClassesToSVGElementPlugin::with_config(config);

        let mut doc = Document::new();
        doc.root = create_element("div"); // Not an SVG element

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Check that no class was added
        assert!(!doc.root.attributes.contains_key("class"));
    }

    #[test]
    fn test_both_class_name_and_class_names() {
        let config = AddClassesToSVGElementConfig {
            class_name: Some("single".to_string()),
            class_names: Some(vec!["multiple1".to_string(), "multiple2".to_string()]),
        };
        let plugin = AddClassesToSVGElementPlugin::with_config(config);

        let mut doc = Document::new();
        doc.root = create_element("svg");

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Check that all classes were added
        let class_attr = doc.root.attributes.get("class").unwrap();
        let classes: HashSet<&str> = class_attr.split_whitespace().collect();
        assert!(classes.contains("single"));
        assert!(classes.contains("multiple1"));
        assert!(classes.contains("multiple2"));
    }

    #[test]
    fn test_empty_class_names_are_ignored() {
        let config = AddClassesToSVGElementConfig {
            class_name: Some("".to_string()), // Empty string
            class_names: Some(vec!["valid".to_string(), "".to_string()]), // Contains empty
        };
        let plugin = AddClassesToSVGElementPlugin::with_config(config);

        let mut doc = Document::new();
        doc.root = create_element("svg");

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Check that only valid class was added
        let class_attr = doc.root.attributes.get("class").unwrap();
        assert_eq!(class_attr, "valid");
    }

    #[test]
    fn test_config_parsing() {
        // Test single class
        let config = AddClassesToSVGElementPlugin::parse_config(&json!({
            "className": "test"
        }))
        .unwrap();
        assert_eq!(config.class_name, Some("test".to_string()));

        // Test array of classes
        let config = AddClassesToSVGElementPlugin::parse_config(&json!({
            "classNames": ["class1", "class2"]
        }))
        .unwrap();
        assert_eq!(
            config.class_names,
            Some(vec!["class1".to_string(), "class2".to_string()])
        );
    }
}

// Use parameterized testing framework for SVGO fixture tests
crate::plugin_fixture_tests!(AddClassesToSVGElementPlugin, "addClassesToSVGElement");
            class_names: Some(vec![
                "duplicateClass".to_string(),
                "uniqueClass".to_string(),
            ]),
        };
        let plugin = AddClassesToSVGElementPlugin::with_config(config);

        let mut doc = Document::new();
        doc.root = create_element("svg");
        doc.root
            .attributes
            .insert("class".to_string(), "duplicateClass".to_string());

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Check that duplicate class appears only once
        let class_attr = doc.root.attributes.get("class").unwrap();
        let classes: Vec<&str> = class_attr.split_whitespace().collect();
        assert_eq!(
            classes.iter().filter(|&&c| c == "duplicateClass").count(),
            1
        );
        assert!(classes.contains(&"uniqueClass"));
    }

    #[test]
    fn test_only_applies_to_svg_element() {
        let config = AddClassesToSVGElementConfig {
            class_name: Some("myClass".to_string()),
            class_names: None,
        };
        let plugin = AddClassesToSVGElementPlugin::with_config(config);

        let mut doc = Document::new();
        doc.root = create_element("div"); // Not an SVG element

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Check that no class was added
        assert!(!doc.root.attributes.contains_key("class"));
    }

    #[test]
    fn test_both_class_name_and_class_names() {
        let config = AddClassesToSVGElementConfig {
            class_name: Some("single".to_string()),
            class_names: Some(vec!["multiple1".to_string(), "multiple2".to_string()]),
        };
        let plugin = AddClassesToSVGElementPlugin::with_config(config);

        let mut doc = Document::new();
        doc.root = create_element("svg");

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Check that all classes were added
        let class_attr = doc.root.attributes.get("class").unwrap();
        let classes: HashSet<&str> = class_attr.split_whitespace().collect();
        assert!(classes.contains("single"));
        assert!(classes.contains("multiple1"));
        assert!(classes.contains("multiple2"));
    }

    #[test]
    fn test_empty_class_names_are_ignored() {
        let config = AddClassesToSVGElementConfig {
            class_name: Some("".to_string()), // Empty string
            class_names: Some(vec!["valid".to_string(), "".to_string()]), // Contains empty
        };
        let plugin = AddClassesToSVGElementPlugin::with_config(config);

        let mut doc = Document::new();
        doc.root = create_element("svg");

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Check that only valid class was added
        let class_attr = doc.root.attributes.get("class").unwrap();
        assert_eq!(class_attr, "valid");
    }

    #[test]
    fn test_config_parsing() {
        // Test single class
        let config = AddClassesToSVGElementPlugin::parse_config(&json!({
            "className": "test"
        }))
        .unwrap();
        assert_eq!(config.class_name, Some("test".to_string()));

        // Test array of classes
        let config = AddClassesToSVGElementPlugin::parse_config(&json!({
            "classNames": ["class1", "class2"]
        }))
        .unwrap();
        assert_eq!(
            config.class_names,
            Some(vec!["class1".to_string(), "class2".to_string()])
        );
    }
}

// Use parameterized testing framework for SVGO fixture tests
crate::plugin_fixture_tests!(AddClassesToSVGElementPlugin, "addClassesToSVGElement");
