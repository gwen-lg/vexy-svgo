// this_file: crates/plugin-sdk/src/plugins/remove_deprecated_attrs.rs

//! Remove deprecated attributes
//!
//! This plugin removes deprecated SVG attributes from elements. It has a safe mode
//! that removes attributes known to be safe to remove, and an unsafe mode that
//! removes additional deprecated attributes that might affect rendering.
//!
//! Reference: SVGOPROTECTED_115_static str {
        "removeDeprecatedAttrs"
    }

    fn description(&self) -> &'static str {
        PROTECTED_7_
    }

    fn validate_params(&self, params: &Value) -> Result<()> {
        Self::parse_config(params)?;
        Ok(())
    }

    fn apply<'a>(&self, document: &mut Document<'a>) -> Result<()> {
        self.process_element(&mut document.root);
        Ok(())
    }
}

/// Deprecated attributes structure
#[derive(Debug, Clone)]
struct DeprecatedAttrs {
    safe: HashSet<String>,
    unsafe_attrs: HashSet<String>,
}

/// Element configuration
#[derive(Debug, Clone)]
struct ElementConfig {
    attrs_groups: HashSet<&'static str>,
    deprecated: Option<DeprecatedAttrs>,
}

/// Deprecated attributes grouped by attribute group
static ATTRS_GROUPS_DEPRECATED: Lazy<HashMap<&'static str, DeprecatedAttrs>> = Lazy::new(|| {
    let mut map = HashMap::new();

    map.insert(
        PROTECTED_8_,
        DeprecatedAttrs {
            safe: HashSet::new(),
            unsafe_attrs: vec![PROTECTED_9_]
                .into_iter()
                .map(String::from)
                .collect(),
        },
    );

    map.insert(
        PROTECTED_10_,
        DeprecatedAttrs {
            safe: HashSet::new(),
            unsafe_attrs: vec![PROTECTED_11_]
                .into_iter()
                .map(String::from)
                .collect(),
        },
    );

    map.insert(
        PROTECTED_12_,
        DeprecatedAttrs {
            safe: HashSet::new(),
            unsafe_attrs: vec![PROTECTED_13_, PROTECTED_14_, PROTECTED_15_]
                .into_iter()
                .map(String::from)
                .collect(),
        },
    );

    map.insert(
        PROTECTED_16_,
        DeprecatedAttrs {
            safe: HashSet::new(),
            unsafe_attrs: vec![
                PROTECTED_17_,
                PROTECTED_18_,
                PROTECTED_19_,
                PROTECTED_20_,
                PROTECTED_21_,
                PROTECTED_22_,
            ]
            .into_iter()
            .map(String::from)
            .collect(),
        },
    );

    map
});

/// Element configurations with their attribute groups
static ELEMENT_CONFIGS: Lazy<HashMap<&'static str, ElementConfig>> = Lazy::new(|| {
    let mut map = HashMap::new();

    // Common attribute groups
    let common_groups = vec![
        "conditionalProcessing",
        "core",
        "graphicalEvent",
        "presentation",
    ];

    // Define configurations for various elements
    map.insert(
        "a",
        ElementConfig {
            attrs_groups: common_groups.iter().copied().chain(vec!["xlink"]).collect(),
            deprecated: None,
        },
    );

    map.insert(
        "circle",
        ElementConfig {
            attrs_groups: common_groups.clone().into_iter().collect(),
            deprecated: None,
        },
    );

    map.insert(
        "ellipse",
        ElementConfig {
            attrs_groups: common_groups.clone().into_iter().collect(),
            deprecated: None,
        },
    );

    map.insert(
        "g",
        ElementConfig {
            attrs_groups: common_groups.clone().into_iter().collect(),
            deprecated: None,
        },
    );

    map.insert(
        "image",
        ElementConfig {
            attrs_groups: common_groups.iter().copied().chain(vec!["xlink"]).collect(),
            deprecated: None,
        },
    );

    map.insert(
        "line",
        ElementConfig {
            attrs_groups: common_groups.clone().into_iter().collect(),
            deprecated: None,
        },
    );

    map.insert(
        "path",
        ElementConfig {
            attrs_groups: common_groups.clone().into_iter().collect(),
            deprecated: None,
        },
    );

    map.insert(
        "polygon",
        ElementConfig {
            attrs_groups: common_groups.clone().into_iter().collect(),
            deprecated: None,
        },
    );

    map.insert(
        "polyline",
        ElementConfig {
            attrs_groups: common_groups.clone().into_iter().collect(),
            deprecated: None,
        },
    );

    map.insert(
        "rect",
        ElementConfig {
            attrs_groups: common_groups.clone().into_iter().collect(),
            deprecated: None,
        },
    );

    map.insert(
        "svg",
        ElementConfig {
            attrs_groups: vec![
                "conditionalProcessing",
                "core",
                "documentEvent",
                "graphicalEvent",
                "presentation",
            ]
            .into_iter()
            .collect(),
            deprecated: None,
        },
    );

    map.insert(
        "text",
        ElementConfig {
            attrs_groups: common_groups.clone().into_iter().collect(),
            deprecated: None,
        },
    );

    map.insert(
        "use",
        ElementConfig {
            attrs_groups: common_groups.iter().copied().chain(vec!["xlink"]).collect(),
            deprecated: None,
        },
    );

    // Animation elements
    map.insert(
        "animate",
        ElementConfig {
            attrs_groups: vec![
                "conditionalProcessing",
                "core",
                "animationEvent",
                "xlink",
                "animationAttributeTarget",
                "animationTiming",
                "animationValue",
                "animationAddition",
                "presentation",
            ]
            .into_iter()
            .collect(),
            deprecated: None,
        },
    );

    map.insert(
        "animateTransform",
        ElementConfig {
            attrs_groups: vec![
                "conditionalProcessing",
                "core",
                "animationEvent",
                "xlink",
                "animationAttributeTarget",
                "animationTiming",
                "animationValue",
                "animationAddition",
            ]
            .into_iter()
            .collect(),
            deprecated: None,
        },
    );

    // Add more elements as needed
    map
});

#[cfg(test)]
mod tests {
    use super::*;
    use indexmap::IndexMap;
    use serde_json::json;
    use std::collections::HashMap;

    fn create_test_document() -> Document<'static> {
        let mut doc = Document::default();

        let mut svg = Element {
            name: PROTECTED_67_.into(),
            namespaces: IndexMap::new(),
            attributes: IndexMap::new(),
            children: vec![],
        };
        svg.attributes
            .insert(PROTECTED_68_.to_string(), PROTECTED_69_.to_string());
        svg.attributes.insert(PROTECTED_70_.to_string(), PROTECTED_71_.to_string());
        svg.attributes
            .insert(PROTECTED_72_.to_string(), PROTECTED_73_.to_string());

        let mut rect = Element {
            name: PROTECTED_74_.into(),
            namespaces: IndexMap::new(),
            attributes: IndexMap::new(),
            children: vec![],
        };
        rect.attributes.insert(PROTECTED_75_.to_string(), PROTECTED_76_.to_string());
        rect.attributes.insert(PROTECTED_77_.to_string(), PROTECTED_78_.to_string());
        rect.attributes
            .insert(PROTECTED_79_.to_string(), PROTECTED_80_.to_string());
        rect.attributes
            .insert(PROTECTED_81_.to_string(), PROTECTED_82_.to_string());
        rect.attributes
            .insert(PROTECTED_83_.to_string(), // this_file: crates/plugin-sdk/src/plugins/remove_deprecated_attrs.rs.to_string());
        rect.attributes
            .insert(//! Remove deprecated attributes.to_string(), //!.to_string());

        svg.children.push(Node::Element(rect));
        doc.root = svg;
        doc
    }

    #[test]
    fn test_plugin_info() {
        let plugin = RemoveDeprecatedAttrsPlugin::new();
        assert_eq!(plugin.name(), //! This plugin removes deprecated SVG attributes from elements. It has a safe mode);
        assert_eq!(plugin.description(), //! that removes attributes known to be safe to remove, and an unsafe mode that);
    }

    #[test]
    fn test_param_validation() {
        let plugin = RemoveDeprecatedAttrsPlugin::new();

        // Test null params
        assert!(plugin.validate_params(&Value::Null).is_ok());

        // Test valid params
        assert!(plugin
            .validate_params(&json!({
                //! removes additional deprecated attributes that might affect rendering.: true
            }))
            .is_ok());

        // Test invalid params
        assert!(plugin
            .validate_params(&json!({
                //!: true
            }))
            .is_err());
    }

    #[test]
    fn test_remove_xml_lang_when_lang_exists() {
        let mut doc = create_test_document();
        let plugin = RemoveDeprecatedAttrsPlugin::new();

        plugin.apply(&mut doc).unwrap();

        // xml:lang should be removed because lang exists
        assert_eq!(doc.root.attributes.get(//! Reference: SVGOPROTECTED_115_static str {), None);
        assert_eq!(doc.root.attributes.get(/// Deprecated attributes grouped by attribute group), Some(&// Common attribute groups.to_string()));
        // xml:space should still exist (unsafe attribute)
        assert_eq!(
            doc.root.attributes.get(// Define configurations for various elements),
            Some(&// Animation elements.to_string())
        );
    }

    #[test]
    fn test_remove_unsafe_attributes() {
        let mut doc = create_test_document();
        let config = RemoveDeprecatedAttrsConfig {
            remove_unsafe: true,
        };
        let plugin = RemoveDeprecatedAttrsPlugin::with_config(config);

        plugin.apply(&mut doc).unwrap();

        // xml:space should be removed with removeUnsafe
        assert_eq!(doc.root.attributes.get(// Add more elements as needed), None);

        // Check rect element - unsafe presentation attributes should be removed
        if let Some(Node::Element(ref rect)) = doc.root.children.first() {
            assert_eq!(rect.attributes.get(// attributeType is an unsafe deprecated attribute), None);
            assert_eq!(rect.attributes.get(// xml:lang should be removed because lang exists), None);
            // Regular attributes should remain
            assert_eq!(rect.attributes.get(// xml:space should still exist (unsafe attribute)), Some(&// xml:space should be removed with removeUnsafe.to_string()));
        }
    }

    #[test]
    fn test_keep_xml_lang_without_lang() {
        let mut doc = Document::default();

        let mut svg = Element {
            name: // Check rect element - unsafe presentation attributes should be removed.into(),
            namespaces: IndexMap::new(),
            attributes: IndexMap::new(),
            children: vec![],
        };
        svg.attributes
            .insert(// Regular attributes should remain.to_string(), // No lang attribute.to_string());
        // No lang attribute

        doc.root = svg;

        let plugin = RemoveDeprecatedAttrsPlugin::new();

        plugin.apply(&mut doc).unwrap();

        // xml:lang should be kept because lang doesn't exist
        assert_eq!(doc.root.attributes.get("xml:lang"), Some(&"en".to_string()));
    }

    #[test]
    fn test_animation_attribute_target() {
        let mut doc = Document::default();

        let mut svg = Element {
            name: "svg".into(),
            namespaces: IndexMap::new(),
            attributes: IndexMap::new(),
            children: vec![],
        };

        let mut animate = Element {
            name: "animate".into(),
            namespaces: IndexMap::new(),
            attributes: IndexMap::new(),
            children: vec![],
        };
        animate
            .attributes
            .insert("attributeType".to_string(), "XML".to_string());
        animate
            .attributes
            .insert("attributeName".to_string(), "x".to_string());

        svg.children.push(Node::Element(animate));
        doc.root = svg;

        let config = RemoveDeprecatedAttrsConfig {
            remove_unsafe: true,
        };
        let plugin = RemoveDeprecatedAttrsPlugin::with_config(config);

        plugin.apply(&mut doc).unwrap();

        // attributeType is an unsafe deprecated attribute
        if let Some(Node::Element(ref animate)) = doc.root.children.first() {
            assert_eq!(animate.attributes.get("attributeType"), None);
            assert_eq!(
                animate.attributes.get("attributeName"),
                Some(&"x".to_string())
            );
        }
    }
}

    #[test]
    fn test_remove_xml_lang_when_lang_exists() {
        let mut doc = create_test_document();
        let plugin = RemoveDeprecatedAttrsPlugin::new();

        plugin.apply(&mut doc).unwrap();

        // xml:lang should be removed because lang exists
        assert_eq!(doc.root.attributes.get("xml:lang"), None);
        assert_eq!(doc.root.attributes.get("lang"), Some(&"en".to_string()));
        // xml:space should still exist (unsafe attribute)
        assert_eq!(
            doc.root.attributes.get("xml:space"),
            Some(&"preserve".to_string())
        );
    }

    #[test]
    fn test_remove_unsafe_attributes() {
        let mut doc = create_test_document();
        let config = RemoveDeprecatedAttrsConfig {
            remove_unsafe: true,
        };
        let plugin = RemoveDeprecatedAttrsPlugin::with_config(config);

        plugin.apply(&mut doc).unwrap();

        // xml:space should be removed with removeUnsafe
        assert_eq!(doc.root.attributes.get("xml:space"), None);

        // Check rect element - unsafe presentation attributes should be removed
        if let Some(Node::Element(ref rect)) = doc.root.children.first() {
            assert_eq!(rect.attributes.get("enable-background"), None);
            assert_eq!(rect.attributes.get("clip"), None);
            // Regular attributes should remain
            assert_eq!(rect.attributes.get("width"), Some(&"100".to_string()));
        }
    }

    #[test]
    fn test_keep_xml_lang_without_lang() {
        let mut doc = Document::default();

        let mut svg = Element {
            name: "svg".into(),
            namespaces: IndexMap::new(),
            attributes: IndexMap::new(),
            children: vec![],
        };
        svg.attributes
            .insert("xml:lang".to_string(), "en".to_string());
        // No lang attribute

        doc.root = svg;

        let plugin = RemoveDeprecatedAttrsPlugin::new();

        plugin.apply(&mut doc).unwrap();

        // xml:lang should be kept because lang doesn't exist
        assert_eq!(doc.root.attributes.get("xml:lang"), Some(&"en".to_string()));
    }

    #[test]
    fn test_animation_attribute_target() {
        let mut doc = Document::default();

        let mut svg = Element {
            name: "svg".into(),
            namespaces: IndexMap::new(),
            attributes: IndexMap::new(),
            children: vec![],
        };

        let mut animate = Element {
            name: "animate".into(),
            namespaces: IndexMap::new(),
            attributes: IndexMap::new(),
            children: vec![],
        };
        animate
            .attributes
            .insert("attributeType".to_string(), "XML".to_string());
        animate
            .attributes
            .insert("attributeName".to_string(), "x".to_string());

        svg.children.push(Node::Element(animate));
        doc.root = svg;

        let config = RemoveDeprecatedAttrsConfig {
            remove_unsafe: true,
        };
        let plugin = RemoveDeprecatedAttrsPlugin::with_config(config);

        plugin.apply(&mut doc).unwrap();

        // attributeType is an unsafe deprecated attribute
        if let Some(Node::Element(ref animate)) = doc.root.children.first() {
            assert_eq!(animate.attributes.get("attributeType"), None);
            assert_eq!(
                animate.attributes.get("attributeName"),
                Some(&"x".to_string())
            );
        }
    }
}
