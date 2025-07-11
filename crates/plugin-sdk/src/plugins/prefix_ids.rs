// this_file: crates/plugin-sdk/src/plugins/prefix_ids.rs

//! Prefix IDs and class names with a given string or auto-generated prefix
//!
//! This plugin adds a prefix to all IDs and optionally class names in the SVG document.
//! It also updates all references to these IDs in attributes like href, fill, etc.
//!
//! Reference: SVGOPROTECTED_126_.PROTECTED_127_ PROTECTED_128_#PROTECTED_129_;PROTECTED_130_.PROTECTED_131_static str {
        "prefixIds"
    }

    fn description(&self) -> &'static str {
        PROTECTED_42_
    }

    fn validate_params(&self, params: &Value) -> Result<()> {
        Self::parse_config(params)?;
        Ok(())
    }

    fn apply<'a>(&self, document: &mut Document<'a>) -> Result<()> {
        let prefix = self.generate_prefix(document);
        self.process_element(&mut document.root, &prefix);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indexmap::IndexMap;
    use vexy_svgo_core::ast::{Document, DocumentMetadata, Element};

    fn create_test_document() -> Document<'static> {
        Document {
            root: Element {
                name: "svg".into(),
                attributes: IndexMap::new(),
                namespaces: IndexMap::new(),
                children: vec![],
            },
            prologue: vec![],
            epilogue: vec![],
            metadata: DocumentMetadata {
                path: None,
                encoding: None,
                version: None,
            },
        }
    }

    #[test]
    fn test_plugin_info() {
        let plugin = PrefixIdsPlugin::new();
        assert_eq!(plugin.name(), "prefixIds");
        assert_eq!(plugin.description(), "prefix IDs");
    }

    #[test]
    fn test_param_validation() {
        let plugin = PrefixIdsPlugin::new();

        // Test null params
        assert!(plugin.validate_params(&Value::Null).is_ok());

        // Test valid params
        assert!(plugin
            .validate_params(&serde_json::json!({
                "prefix": "custom",
                "delim": "_",
                "prefixIds": false,
                "prefixClassNames": true
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
    fn test_get_basename() {
        assert_eq!(
            PrefixIdsPlugin::get_basename("/path/to/file.svg"),
            "file.svg"
        );
        assert_eq!(
            PrefixIdsPlugin::get_basename("C:\\path\\to\\file.svg"),
            "file.svg"
        );
        assert_eq!(PrefixIdsPlugin::get_basename("file.svg"), "file.svg");
        assert_eq!(PrefixIdsPlugin::get_basename(""), "");
    }

    #[test]
    fn test_escape_identifier_name() {
        assert_eq!(
            PrefixIdsPlugin::escape_identifier_name("my file.svg"),
            "my_file_svg"
        );
        assert_eq!(PrefixIdsPlugin::escape_identifier_name("normal"), "normal");
    }

    #[test]
    fn test_generate_prefix() {
        // Test with custom prefix
        let config = PrefixIdsConfig {
            prefix: Some("custom".to_string()),
            delim: "__".to_string(),
            prefix_ids: true,
            prefix_class_names: true,
        };
        let plugin = PrefixIdsPlugin::with_config(config);
        let doc = create_test_document();
        assert_eq!(plugin.generate_prefix(&doc), "custom__");

        // Test with file path
        let plugin = PrefixIdsPlugin::new();
        let mut doc = create_test_document();
        doc.metadata.path = Some("/path/to/test.svg".to_string());
        assert_eq!(plugin.generate_prefix(&doc), "test_svg__");

        // Test default
        let plugin = PrefixIdsPlugin::new();
        let doc = create_test_document();
        assert_eq!(plugin.generate_prefix(&doc), "prefix__");
    }

    #[test]
    fn test_prefix_id() {
        let plugin = PrefixIdsPlugin::new();

        // Test normal prefixing
        assert_eq!(plugin.prefix_id("test__", "myid"), "test__myid");

        // Test when already prefixed
        assert_eq!(plugin.prefix_id("test__", "test__myid"), "test__myid");
    }

    #[test]
    fn test_prefix_reference() {
        let plugin = PrefixIdsPlugin::new();

        // Test valid reference
        assert_eq!(
            plugin.prefix_reference("test__", "#myid"),
            Some("#test__myid".to_string())
        );

        // Test invalid reference
        assert_eq!(plugin.prefix_reference("test__", "myid"), None);
    }

    #[test]
    fn test_apply_with_ids() {
        let plugin = PrefixIdsPlugin::new();
        let mut doc = create_test_document();

        // Add element with ID
        let mut attrs = IndexMap::new();
        attrs.insert("id".to_string(), "myId".to_string());
        doc.root.children.push(Node::Element(Element {
            name: "rect".into(),
            attributes: attrs,
            namespaces: IndexMap::new(),
            children: vec![],
        }));

        let result = plugin.apply(&mut doc);
        assert!(result.is_ok());

        // Check that ID was prefixed
        if let Node::Element(rect) = &doc.root.children[0] {
            assert_eq!(rect.attr("id").map(|s| s.as_str()), Some("prefix__myId"));
        } else {
            panic!("Expected element");
        }
    }

    #[test]
    fn test_apply_with_href() {
        let plugin = PrefixIdsPlugin::new();
        let mut doc = create_test_document();

        // Add element with href
        let mut attrs = IndexMap::new();
        attrs.insert("href".to_string(), "#myTarget".to_string());
        doc.root.children.push(Node::Element(Element {
            name: "use".into(),
            attributes: attrs,
            namespaces: IndexMap::new(),
            children: vec![],
        }));

        let result = plugin.apply(&mut doc);
        assert!(result.is_ok());

        // Check that href was prefixed
        if let Node::Element(use_elem) = &doc.root.children[0] {
            assert_eq!(use_elem.attr("href").map(|s| s.as_str()), Some("#prefix__myTarget"));
        } else {
            panic!("Expected element");
        }
    }

    #[test]
    fn test_apply_with_custom_config() {
        let config = PrefixIdsConfig {
            prefix: Some("custom".to_string()),
            delim: "_".to_string(),
            prefix_ids: true,
            prefix_class_names: true,
        };
        let plugin = PrefixIdsPlugin::with_config(config);
        let mut doc = create_test_document();

        // Add element with ID
        let mut attrs = IndexMap::new();
        attrs.insert("id".to_string(), "myId".to_string());
        doc.root.children.push(Node::Element(Element {
            name: "rect".into(),
            attributes: attrs,
            namespaces: IndexMap::new(),
            children: vec![],
        }));

        let result = plugin.apply(&mut doc);
        assert!(result.is_ok());

        // Check that ID was prefixed with custom config
        if let Node::Element(rect) = &doc.root.children[0] {
            assert_eq!(rect.attr("id").map(|s| s.as_str()), Some("custom_myId"));
        } else {
            panic!("Expected element");
        }
    }

    #[test]
    fn test_process_url_references() {
        let plugin = PrefixIdsPlugin::new();

        // Test double-quoted URL
        assert_eq!(
            plugin.process_url_references("url(\"#icon\")", "pre_"),
            "url(\"#pre_icon\")"
        );

        // Test single-quoted URL
        assert_eq!(
            plugin.process_url_references(r#"url('#icon')"#, "pre_"),
            r#"url('#pre_icon')"#
        );

        // Test unquoted URL
        assert_eq!(
            plugin.process_url_references("url(#icon)", "pre_"),
            "url(#pre_icon)"
        );

        // Test non-reference URL
        assert_eq!(
            plugin.process_url_references("url(http://example.com)", "pre_"),
            "url(http://example.com)"
        );
    }

    #[test]
    fn test_process_animation_references() {
        let plugin = PrefixIdsPlugin::new();

        assert_eq!(
            plugin.process_animation_references("elem1.end", "pre_"),
            "pre_elem1.end"
        );

        assert_eq!(
            plugin.process_animation_references("elem1.start; elem2.end", "pre_"),
            "pre_elem1.start; pre_elem2.end"
        );

        assert_eq!(plugin.process_animation_references("5s", "pre_"), "5s");
    }

    #[test]
    fn test_process_style_content() {
        let plugin = PrefixIdsPlugin::new();

        let style = "#myId { fill: red; } .myClass { stroke: blue; } rect { fill: url(#grad); }";
        let processed = plugin.process_style_content(style, "pre_");

        assert!(processed.contains("#pre_myId"));
        assert!(processed.contains(".pre_myClass"));
        assert!(processed.contains("url(#pre_grad)"));
    }
}
                .to_string();
        }

        // Also handle url() references in CSS
        result = self.process_url_references(&result, prefix);

        result
    }
}

impl Default for PrefixIdsPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for PrefixIdsPlugin {
    fn name(&self) -> &'static str {
        PROTECTED_88_
    }

    fn description(&self) -> &'static str {
        "prefix IDs"
    }

    fn validate_params(&self, params: &Value) -> Result<()> {
        Self::parse_config(params)?;
        Ok(())
    }

    fn apply<'a>(&self, document: &mut Document<'a>) -> Result<()> {
        let prefix = self.generate_prefix(document);
        self.process_element(&mut document.root, &prefix);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indexmap::IndexMap;
    use vexy_svgo_core::ast::{Document, DocumentMetadata, Element};

    fn create_test_document() -> Document<'static> {
        Document {
            root: Element {
                name: "svg".into(),
                attributes: IndexMap::new(),
                namespaces: IndexMap::new(),
                children: vec![],
            },
            prologue: vec![],
            epilogue: vec![],
            metadata: DocumentMetadata {
                path: None,
                encoding: None,
                version: None,
            },
        }
    }

    #[test]
    fn test_plugin_info() {
        let plugin = PrefixIdsPlugin::new();
        assert_eq!(plugin.name(), "prefixIds");
        assert_eq!(plugin.description(), "prefix IDs");
    }

    #[test]
    fn test_param_validation() {
        let plugin = PrefixIdsPlugin::new();

        // Test null params
        assert!(plugin.validate_params(&Value::Null).is_ok());

        // Test valid params
        assert!(plugin
            .validate_params(&serde_json::json!({
                "prefix": "custom",
                "delim": "_",
                "prefixIds": false,
                "prefixClassNames": true
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
    fn test_get_basename() {
        assert_eq!(
            PrefixIdsPlugin::get_basename("/path/to/file.svg"),
            "file.svg"
        );
        assert_eq!(
            PrefixIdsPlugin::get_basename("C:\\path\\to\\file.svg"),
            "file.svg"
        );
        assert_eq!(PrefixIdsPlugin::get_basename("file.svg"), "file.svg");
        assert_eq!(PrefixIdsPlugin::get_basename(""), "");
    }

    #[test]
    fn test_escape_identifier_name() {
        assert_eq!(
            PrefixIdsPlugin::escape_identifier_name("my file.svg"),
            "my_file_svg"
        );
        assert_eq!(PrefixIdsPlugin::escape_identifier_name("normal"), "normal");
    }

    #[test]
    fn test_generate_prefix() {
        // Test with custom prefix
        let config = PrefixIdsConfig {
            prefix: Some("custom".to_string()),
            delim: "__".to_string(),
            prefix_ids: true,
            prefix_class_names: true,
        };
        let plugin = PrefixIdsPlugin::with_config(config);
        let doc = create_test_document();
        assert_eq!(plugin.generate_prefix(&doc), "custom__");

        // Test with file path
        let plugin = PrefixIdsPlugin::new();
        let mut doc = create_test_document();
        doc.metadata.path = Some("/path/to/test.svg".to_string());
        assert_eq!(plugin.generate_prefix(&doc), "test_svg__");

        // Test default
        let plugin = PrefixIdsPlugin::new();
        let doc = create_test_document();
        assert_eq!(plugin.generate_prefix(&doc), "prefix__");
    }

    #[test]
    fn test_prefix_id() {
        let plugin = PrefixIdsPlugin::new();

        // Test normal prefixing
        assert_eq!(plugin.prefix_id("test__", "myid"), "test__myid");

        // Test when already prefixed
        assert_eq!(plugin.prefix_id("test__", "test__myid"), "test__myid");
    }

    #[test]
    fn test_prefix_reference() {
        let plugin = PrefixIdsPlugin::new();

        // Test valid reference
        assert_eq!(
            plugin.prefix_reference("test__", "#myid"),
            Some("#test__myid".to_string())
        );

        // Test invalid reference
        assert_eq!(plugin.prefix_reference("test__", "myid"), None);
    }

    #[test]
    fn test_apply_with_ids() {
        let plugin = PrefixIdsPlugin::new();
        let mut doc = create_test_document();

        // Add element with ID
        let mut attrs = IndexMap::new();
        attrs.insert("id".to_string(), "myId".to_string());
        doc.root.children.push(Node::Element(Element {
            name: "rect".into(),
            attributes: attrs,
            namespaces: IndexMap::new(),
            children: vec![],
        }));

        let result = plugin.apply(&mut doc);
        assert!(result.is_ok());

        // Check that ID was prefixed
        if let Node::Element(rect) = &doc.root.children[0] {
            assert_eq!(rect.attr("id").map(|s| s.as_str()), Some("prefix__myId"));
        } else {
            panic!("Expected element");
        }
    }

    #[test]
    fn test_apply_with_href() {
        let plugin = PrefixIdsPlugin::new();
        let mut doc = create_test_document();

        // Add element with href
        let mut attrs = IndexMap::new();
        attrs.insert("href".to_string(), "#myTarget".to_string());
        doc.root.children.push(Node::Element(Element {
            name: "use".into(),
            attributes: attrs,
            namespaces: IndexMap::new(),
            children: vec![],
        }));

        let result = plugin.apply(&mut doc);
        assert!(result.is_ok());

        // Check that href was prefixed
        if let Node::Element(use_elem) = &doc.root.children[0] {
            assert_eq!(use_elem.attr("href").map(|s| s.as_str()), Some("#prefix__myTarget"));
        } else {
            panic!("Expected element");
        }
    }

    #[test]
    fn test_apply_with_custom_config() {
        let config = PrefixIdsConfig {
            prefix: Some("custom".to_string()),
            delim: "_".to_string(),
            prefix_ids: true,
            prefix_class_names: true,
        };
        let plugin = PrefixIdsPlugin::with_config(config);
        let mut doc = create_test_document();

        // Add element with ID
        let mut attrs = IndexMap::new();
        attrs.insert("id".to_string(), "myId".to_string());
        doc.root.children.push(Node::Element(Element {
            name: "rect".into(),
            attributes: attrs,
            namespaces: IndexMap::new(),
            children: vec![],
        }));

        let result = plugin.apply(&mut doc);
        assert!(result.is_ok());

        // Check that ID was prefixed with custom config
        if let Node::Element(rect) = &doc.root.children[0] {
            assert_eq!(rect.attr("id").map(|s| s.as_str()), Some("custom_myId"));
        } else {
            panic!("Expected element");
        }
    }

    #[test]
    fn test_process_url_references() {
        let plugin = PrefixIdsPlugin::new();

        // Test double-quoted URL
        assert_eq!(
            plugin.process_url_references("url(\"#icon\")", "pre_"),
            "url(\"#pre_icon\")"
        );

        // Test single-quoted URL
        assert_eq!(
            plugin.process_url_references(r#"url('#icon')"#, "pre_"),
            r#"url('#pre_icon')"#
        );

        // Test unquoted URL
        assert_eq!(
            plugin.process_url_references("url(#icon)", "pre_"),
            "url(#pre_icon)"
        );

        // Test non-reference URL
        assert_eq!(
            plugin.process_url_references("url(http://example.com)", "pre_"),
            "url(http://example.com)"
        );
    }

    #[test]
    fn test_process_animation_references() {
        let plugin = PrefixIdsPlugin::new();

        assert_eq!(
            plugin.process_animation_references("elem1.end", "pre_"),
            "pre_elem1.end"
        );

        assert_eq!(
            plugin.process_animation_references("elem1.start; elem2.end", "pre_"),
            "pre_elem1.start; pre_elem2.end"
        );

        assert_eq!(plugin.process_animation_references("5s", "pre_"), "5s");
    }

    #[test]
    fn test_process_style_content() {
        let plugin = PrefixIdsPlugin::new();

        let style = "#myId { fill: red; } .myClass { stroke: blue; } rect { fill: url(#grad); }";
        let processed = plugin.process_style_content(style, "pre_");

        assert!(processed.contains("#pre_myId"));
        assert!(processed.contains(".pre_myClass"));
        assert!(processed.contains("url(#pre_grad)"));
    }
}
