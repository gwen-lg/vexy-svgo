// this_file: crates/plugin-sdk/src/plugins/remove_desc.rs

//! Remove <desc> elements
//!
//! This plugin removes <desc> elements from SVG documents.
//! By default, it only removes empty descriptions or those containing standard
//! editor content (e.g., PROTECTED_4_). Can be configured to remove all
//! descriptions.
//!
//! Reference: SVGOPROTECTED_16_static str {
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
