// this_file: crates/plugin-sdk/src/plugins/remove_xmlns.rs

//! Plugin to remove xmlns attribute from SVG elements
//!
//! This plugin removes the xmlns attribute when present, which is useful for inline SVG
//! where the namespace declaration is not needed. This plugin is disabled by default.
//!
//! Reference: SVGO// xmlns should be preserved on non-SVG elementsstatic str {
        "removeXmlns"
    }

    fn description(&self) -> &'static str {
        PROTECTED_5_
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
    use vexy_svgo_core::ast::Element;
    use std::borrow::Cow;

    #[test]
    fn test_plugin_info() {
        let plugin = RemoveXmlnsPlugin::new();
        assert_eq!(plugin.name(), PROTECTED_6_);
        assert_eq!(
            plugin.description(),
            PROTECTED_7_
        );
    }

    #[test]
    fn test_param_validation() {
        let plugin = RemoveXmlnsPlugin::new();

        // Test null params
        assert!(plugin.validate_params(&Value::Null).is_ok());

        // Test empty object params
        assert!(plugin.validate_params(&serde_json::json!({})).is_ok());

        // Test invalid params
        assert!(plugin
            .validate_params(&serde_json::json!({
                PROTECTED_8_: true
            }))
            .is_err());
    }

    #[test]
    fn test_remove_xmlns_from_svg() {
        let mut document = Document::default();
        document.root.name = Cow::Borrowed(PROTECTED_9_);
        document
            .root
            .set_attr(PROTECTED_10_, PROTECTED_11_);
        document.root.set_attr(PROTECTED_12_, PROTECTED_13_);

        let plugin = RemoveXmlnsPlugin::new();
        let result = plugin.apply(&mut document);
        assert!(result.is_ok());

        // xmlns should be removed, other attributes preserved
        assert!(!document.root.has_attr(PROTECTED_14_));
        assert!(document.root.has_attr(PROTECTED_15_));
        assert_eq!(document.root.attr(PROTECTED_16_).unwrap(), PROTECTED_17_);
    }

    #[test]
    fn test_remove_xmlns_from_nested_svg() {
        let mut document = Document::default();
        document.root.name = Cow::Borrowed(PROTECTED_18_);

        // Add nested SVG element with xmlns
        let mut nested_svg = Element::new(PROTECTED_19_);
        nested_svg.set_attr(PROTECTED_20_, PROTECTED_21_);
        nested_svg.set_attr(PROTECTED_22_, PROTECTED_23_);

        document.root.children.push(Node::Element(nested_svg));

        let plugin = RemoveXmlnsPlugin::new();
        let result = plugin.apply(&mut document);
        assert!(result.is_ok());

        // xmlns should be removed from nested SVG
        if let Node::Element(ref nested) = document.root.children[0] {
            assert_eq!(nested.name, PROTECTED_24_);
            assert!(!nested.has_attr(PROTECTED_25_));
            assert!(nested.has_attr(PROTECTED_26_));
            assert_eq!(nested.attr(PROTECTED_27_).unwrap(), PROTECTED_28_);
        } else {
            panic!(PROTECTED_29_);
        }
    }

    #[test]
    fn test_preserve_other_xmlns_attributes() {
        let mut document = Document::default();
        document.root.name = Cow::Borrowed(PROTECTED_30_);
        document
            .root
            .set_attr(PROTECTED_31_, PROTECTED_32_);
        document
            .root
            .set_attr(PROTECTED_33_, PROTECTED_34_);
        document
            .root
            .set_attr(PROTECTED_35_, PROTECTED_36_);

        let plugin = RemoveXmlnsPlugin::new();
        let result = plugin.apply(&mut document);
        assert!(result.is_ok());

        // Only xmlns should be removed, namespaced xmlns attributes preserved
        assert!(!document.root.has_attr(PROTECTED_37_));
        assert!(document.root.has_attr(PROTECTED_38_));
        assert!(document.root.has_attr(PROTECTED_39_));
    }

    #[test]
    fn test_ignore_non_svg_elements() {
        let mut document = Document::default();
        document.root.name = Cow::Borrowed(PROTECTED_40_);

        // Add a non-SVG element with xmlns (shouldn't happen but test anyway)
        let mut rect_element = Element::new("rect");
        rect_element.set_attr("xmlns", "http://www.w3.org/2000/svg");
        rect_element.set_attr("width", "100");

        document.root.children.push(Node::Element(rect_element));

        let plugin = RemoveXmlnsPlugin::new();
        let result = plugin.apply(&mut document);
        assert!(result.is_ok());

        // xmlns should be preserved on non-SVG elements
        if let Node::Element(ref rect) = document.root.children[0] {
            assert_eq!(rect.name, "rect");
            assert!(rect.has_attr("xmlns"));
            assert_eq!(rect.attr("xmlns").unwrap(), "http://www.w3.org/2000/svg");
        } else {
            panic!("Expected rect element");
        }
    }

    #[test]
    fn test_no_xmlns_attribute() {
        let mut document = Document::default();
        document.root.name = Cow::Borrowed("svg");
        document.root.set_attr("viewBox", "0 0 100 100");
        document.root.set_attr("width", "100");

        let plugin = RemoveXmlnsPlugin::new();
        let result = plugin.apply(&mut document);
        assert!(result.is_ok());

        // Should work fine even without xmlns attribute
        assert!(document.root.has_attr("viewBox"));
        assert!(document.root.has_attr("width"));
    }

    #[test]
    fn test_complex_nested_structure() {
        let mut document = Document::default();
        document.root.name = Cow::Borrowed("svg");
        document
            .root
            .set_attr("xmlns", "http://www.w3.org/2000/svg");

        // Nested structure: svg -> g -> svg
        let mut inner_svg = Element::new("svg");
        inner_svg.set_attr("xmlns", "http://www.w3.org/2000/svg");
        inner_svg.set_attr("x", "10");

        let mut g_element = Element::new("g");
        g_element.children.push(Node::Element(inner_svg));

        document.root.children.push(Node::Element(g_element));

        let plugin = RemoveXmlnsPlugin::new();
        let result = plugin.apply(&mut document);
        assert!(result.is_ok());

        // Both root and nested SVG should have xmlns removed
        assert!(!document.root.has_attr("xmlns"));

        if let Node::Element(ref g) = document.root.children[0] {
            if let Node::Element(ref inner_svg) = g.children[0] {
                assert_eq!(inner_svg.name, "svg");
                assert!(!inner_svg.has_attr("xmlns"));
                assert_eq!(inner_svg.attr("x").unwrap(), "10");
            } else {
                panic!("Expected inner svg element");
            }
        } else {
            panic!("Expected g element");
        }
    }
}
        let plugin = RemoveXmlnsPlugin::new();
        let result = plugin.apply(&mut document);
        assert!(result.is_ok());

        // xmlns should be preserved on non-SVG elements
        if let Node::Element(ref rect) = document.root.children[0] {
            assert_eq!(rect.name, "rect");
            assert!(rect.has_attr("xmlns"));
            assert_eq!(rect.attr("xmlns").unwrap(), "http://www.w3.org/2000/svg");
        } else {
            panic!("Expected rect element");
        }
    }

    #[test]
    fn test_no_xmlns_attribute() {
        let mut document = Document::default();
        document.root.name = Cow::Borrowed("svg");
        document.root.set_attr("viewBox", "0 0 100 100");
        document.root.set_attr("width", "100");

        let plugin = RemoveXmlnsPlugin::new();
        let result = plugin.apply(&mut document);
        assert!(result.is_ok());

        // Should work fine even without xmlns attribute
        assert!(document.root.has_attr("viewBox"));
        assert!(document.root.has_attr("width"));
    }

    #[test]
    fn test_complex_nested_structure() {
        let mut document = Document::default();
        document.root.name = Cow::Borrowed("svg");
        document
            .root
            .set_attr("xmlns", "http://www.w3.org/2000/svg");

        // Nested structure: svg -> g -> svg
        let mut inner_svg = Element::new("svg");
        inner_svg.set_attr("xmlns", "http://www.w3.org/2000/svg");
        inner_svg.set_attr("x", "10");

        let mut g_element = Element::new("g");
        g_element.children.push(Node::Element(inner_svg));

        document.root.children.push(Node::Element(g_element));

        let plugin = RemoveXmlnsPlugin::new();
        let result = plugin.apply(&mut document);
        assert!(result.is_ok());

        // Both root and nested SVG should have xmlns removed
        assert!(!document.root.has_attr("xmlns"));

        if let Node::Element(ref g) = document.root.children[0] {
            if let Node::Element(ref inner_svg) = g.children[0] {
                assert_eq!(inner_svg.name, "svg");
                assert!(!inner_svg.has_attr("xmlns"));
                assert_eq!(inner_svg.attr("x").unwrap(), "10");
            } else {
                panic!("Expected inner svg element");
            }
        } else {
            panic!("Expected g element");
        }
    }
}
