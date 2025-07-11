// this_file: crates/plugin-sdk/src/plugins/remove_unknowns_and_defaults/tests.rs

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

    fn create_element_with_attrs(name: &'static str, attrs: &[(&str, &str)]) -> Element<'static> {
        let mut element = create_element(name);
        for (key, value) in attrs {
            element
                .attributes
                .insert(key.to_string(), value.to_string());
        }
        element
    }

    #[test]
    fn test_plugin_creation() {
        let plugin = RemoveUnknownsAndDefaultsPlugin::new();
        assert_eq!(plugin.name(), "removeUnknownsAndDefaults");
    }

    #[test]
    fn test_configuration_defaults() {
        let config = RemoveUnknownsAndDefaultsConfig::default();
        assert!(config.unknown_content);
        assert!(config.unknown_attrs);
        assert!(config.default_attrs);
        assert!(config.default_markup_declarations);
        assert!(config.useless_overrides);
        assert!(config.keep_data_attrs);
        assert!(config.keep_aria_attrs);
        assert!(!config.keep_role_attr);
    }

    #[test]
    fn test_parameter_validation() {
        let plugin = RemoveUnknownsAndDefaultsPlugin::new();

        // Valid parameters
        assert!(plugin.validate_params(&json!({})).is_ok());
        assert!(plugin
            .validate_params(&json!({"unknownContent": true}))
            .is_ok());
        assert!(plugin
            .validate_params(&json!({"keepDataAttrs": false}))
            .is_ok());

        // Invalid parameters
        assert!(plugin
            .validate_params(&json!({"unknownContent": "invalid"}))
            .is_err());
        assert!(plugin
            .validate_params(&json!({"invalidParam": true}))
            .is_err());
    }

    #[test]
    fn test_known_elements() {
        let known = RemoveUnknownsAndDefaultsPlugin::known_elements();
        assert!(known.contains("svg"));
        assert!(known.contains("rect"));
        assert!(known.contains("circle"));
        assert!(known.contains("path"));
        assert!(!known.contains("unknown-element"));
    }

    #[test]
    fn test_known_attributes() {
        let known = RemoveUnknownsAndDefaultsPlugin::known_attributes();
        assert!(known.contains("id"));
        assert!(known.contains("class"));
        assert!(known.contains("fill"));
        assert!(known.contains("stroke"));
        assert!(!known.contains("unknown-attr"));
    }

    #[test]
    fn test_default_values() {
        let defaults = RemoveUnknownsAndDefaultsPlugin::default_attribute_values();
        assert_eq!(defaults.get("x"), Some(&"0"));
        assert_eq!(defaults.get("y"), Some(&"0"));
        assert_eq!(defaults.get("fill"), Some(&"black"));
        assert_eq!(defaults.get("stroke"), Some(&"none"));
    }

    #[test]
    fn test_should_remove_unknown_element() {
        let plugin = RemoveUnknownsAndDefaultsPlugin::new();

        // Known elements should not be removed
        let rect = create_element("rect");
        assert!(!plugin.should_remove_unknown_element(&rect));

        // Unknown elements should be removed
        let unknown = create_element("unknown-element");
        assert!(plugin.should_remove_unknown_element(&unknown));

        // Namespaced elements should not be removed
        let mut namespaced = create_element("custom:element");
        namespaced.name = Cow::Borrowed("custom:element");
        assert!(!plugin.should_remove_unknown_element(&namespaced));
    }

    #[test]
    fn test_should_remove_attribute_unknown() {
        let plugin = RemoveUnknownsAndDefaultsPlugin::new();
        let element = create_element("rect");

        // Known attributes should not be removed
        assert!(!plugin.should_remove_attribute("fill", "red", &element, None));

        // Unknown attributes should be removed
        assert!(plugin.should_remove_attribute("unknown-attr", "value", &element, None));

        // Data attributes should be preserved by default
        assert!(!plugin.should_remove_attribute("data-test", "value", &element, None));

        // ARIA attributes should be preserved by default
        assert!(!plugin.should_remove_attribute("aria-label", "test", &element, None));

        // Role attribute should be removed by default (keepRoleAttr is false)
        assert!(plugin.should_remove_attribute("role", "button", &element, None));

        // Role attribute should be kept when keepRoleAttr is true
        let mut config = RemoveUnknownsAndDefaultsConfig::default();
        config.keep_role_attr = true;
        let plugin_keep_role = RemoveUnknownsAndDefaultsPlugin::with_config(config);
        assert!(!plugin_keep_role.should_remove_attribute("role", "button", &element, None));
    }

    #[test]
    fn test_should_remove_attribute_defaults() {
        let plugin = RemoveUnknownsAndDefaultsPlugin::new();
        let element = create_element("rect");

        // Default values should be removed
        assert!(plugin.should_remove_attribute("x", "0", &element, None));
        assert!(plugin.should_remove_attribute("fill", "black", &element, None));

        // Non-default values should not be removed
        assert!(!plugin.should_remove_attribute("x", "10", &element, None));
        assert!(!plugin.should_remove_attribute("fill", "red", &element, None));

        // Elements with id should keep their default values
        let element_with_id = create_element_with_attrs("rect", &[("id", "test")]);
        assert!(!plugin.should_remove_attribute("x", "0", &element_with_id, None));
    }

    #[test]
    fn test_should_remove_attribute_namespaced() {
        let plugin = RemoveUnknownsAndDefaultsPlugin::new();
        let element = create_element("rect");

        // xmlns attributes should be preserved
        assert!(!plugin.should_remove_attribute(
            "xmlns",
            "http://www.w3.org/2000/svg",
            &element,
            None
        ));
        assert!(!plugin.should_remove_attribute(
            "xmlns:xlink",
            "http://www.w3.org/1999/xlink",
            &element,
            None
        ));

        // xml: and xlink: attributes should be preserved
        assert!(!plugin.should_remove_attribute("xml:space", "preserve", &element, None));
        assert!(!plugin.should_remove_attribute("xlink:href", "#test", &element, None));

        // Other namespaced attributes should be removed if unknown
        assert!(plugin.should_remove_attribute("custom:attr", "value", &element, None));
    }

    #[test]
    fn test_plugin_apply_remove_unknown_elements() {
        let plugin = RemoveUnknownsAndDefaultsPlugin::new();
        let mut doc = Document::new();

        // Add known and unknown elements
        doc.root
            .children
            .push(Node::Element(create_element("rect")));
        doc.root
            .children
            .push(Node::Element(create_element("unknown-element")));
        doc.root
            .children
            .push(Node::Element(create_element("circle")));

        plugin.apply(&mut doc).unwrap();

        // Unknown element should be removed
        assert_eq!(doc.root.children.len(), 2);

        // Check that known elements remain
        let element_names: Vec<&str> = doc
            .root
            .children
            .iter()
            .filter_map(|child| match child {
                Node::Element(elem) => Some(elem.name.as_ref()),
                _ => None,
            })
            .collect();

        assert!(element_names.contains(&"rect"));
        assert!(element_names.contains(&"circle"));
        assert!(!element_names.contains(&"unknown-element"));
    }

    #[test]
    fn test_plugin_apply_remove_unknown_attributes() {
        let plugin = RemoveUnknownsAndDefaultsPlugin::new();
        let mut doc = Document::new();

        // Create element with known and unknown attributes
        let element = create_element_with_attrs(
            "rect",
            &[
                ("width", "100"),
                ("height", "100"),
                ("unknown-attr", "value"),
                ("data-test", "keep"),
            ],
        );

        doc.root.children.push(Node::Element(element));

        plugin.apply(&mut doc).unwrap();

        if let Some(Node::Element(rect)) = doc.root.children.first() {
            assert!(rect.attributes.contains_key("width"));
            assert!(rect.attributes.contains_key("height"));
            assert!(rect.attributes.contains_key("data-test"));
            assert!(!rect.attributes.contains_key("unknown-attr"));
        }
    }

    #[test]
    fn test_plugin_apply_remove_default_values() {
        let plugin = RemoveUnknownsAndDefaultsPlugin::new();
        let mut doc = Document::new();

        // Create element with default and non-default values
        let element = create_element_with_attrs(
            "rect",
            &[
                ("x", "0"),        // default
                ("y", "10"),       // non-default
                ("fill", "black"), // default
                ("stroke", "red"), // non-default
            ],
        );

        doc.root.children.push(Node::Element(element));

        plugin.apply(&mut doc).unwrap();

        if let Some(Node::Element(rect)) = doc.root.children.first() {
            assert!(!rect.attributes.contains_key("x"));
            assert!(rect.attributes.contains_key("y"));
            assert!(!rect.attributes.contains_key("fill"));
            assert!(rect.attributes.contains_key("stroke"));
        }
    }

    #[test]
    fn test_plugin_apply_preserve_elements_with_id() {
        let plugin = RemoveUnknownsAndDefaultsPlugin::new();
        let mut doc = Document::new();

        // Create element with id and default values
        let element = create_element_with_attrs(
            "rect",
            &[
                ("id", "test"),
                ("x", "0"),        // default, but should be kept
                ("fill", "black"), // default, but should be kept
            ],
        );

        doc.root.children.push(Node::Element(element));

        plugin.apply(&mut doc).unwrap();

        if let Some(Node::Element(rect)) = doc.root.children.first() {
            assert!(rect.attributes.contains_key("id"));
            assert!(rect.attributes.contains_key("x"));
            assert!(rect.attributes.contains_key("fill"));
        }
    }
}
