// this_file: crates/plugin-sdk/src/plugins/remove_attrs.rs

//! Remove specified attributes based on patterns
//!
//! This plugin removes attributes from elements based on flexible pattern matching.
//! Patterns can specify element names, attribute names, and attribute values using regex.
//!
//! Reference: SVGOPROTECTED_139_static str {
        "removeAttrs"
    }

    fn description(&self) -> &'static str {
        "removes specified attributes"
    }

    fn validate_params(&self, params: &Value) -> Result<()> {
        Self::parse_config(params)?;
        Ok(())
    }

    fn apply(&self, document: &mut Document) -> Result<()> {
        // Parse the configuration fresh for this apply call
        let config = if self.config.attrs.is_empty() {
            return Err(anyhow::anyhow!(
                "removeAttrs plugin requires 'attrs' parameter"
            ));
        } else {
            self.config.clone()
        };

        // Compile all patterns
        let mut compiled_patterns = Vec::new();
        for pattern in &config.attrs {
            compiled_patterns.push(CompiledPattern::compile(pattern, &config.elem_separator)?);
        }

        self.process_element(&mut document.root, &compiled_patterns);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_plugin_info() {
        let plugin = RemoveAttrsPlugin::new();
        assert_eq!(plugin.name(), "removeAttrs");
        assert_eq!(plugin.description(), "removes specified attributes");
    }

    #[test]
    fn test_param_validation() {
        let plugin = RemoveAttrsPlugin::new();

        // Test null params - should fail
        assert!(plugin.validate_params(&Value::Null).is_err());

        // Test empty object - should fail
        assert!(plugin.validate_params(&json!({})).is_err());

        // Test valid string param
        assert!(plugin.validate_params(&json!("fill")).is_ok());

        // Test valid array param
        assert!(plugin.validate_params(&json!(["fill", "stroke"])).is_ok());

        // Test valid object param
        assert!(plugin
            .validate_params(&json!({
                "attrs": ["fill"],
                "preserveCurrentColor": true
            }))
            .is_ok());

        // Test invalid param type in array
        assert!(plugin.validate_params(&json!(["fill", 123])).is_err());

        // Test unknown field
        assert!(plugin
            .validate_params(&json!({
                "attrs": ["fill"],
                "unknownField": true
            }))
            .is_err());
    }

    #[test]
    fn test_simple_attribute_removal() {
        let mut document = Document::new();
        document.root.name = "svg".into();

        let mut rect = Element::new("rect");
        rect.set_attr("fill", "red");
        rect.set_attr("stroke", "blue");
        rect.set_attr("width", "100");

        document.root.children.push(Node::Element(rect));

        let config = RemoveAttrsConfig {
            attrs: vec!["fill".to_string()],
            ..Default::default()
        };
        let plugin = RemoveAttrsPlugin::with_config(config);
        plugin.apply(&mut document).unwrap();

        if let Node::Element(rect) = &document.root.children[0] {
            assert!(!rect.has_attr("fill"));
            assert!(rect.has_attr("stroke"));
            assert!(rect.has_attr("width"));
        }
    }

    #[test]
    fn test_multiple_attribute_removal() {
        let mut document = Document::new();
        document.root.name = "svg".into();

        let mut rect = Element::new("rect");
        rect.set_attr("fill", "red");
        rect.set_attr("stroke", "blue");
        rect.set_attr("width", "100");

        document.root.children.push(Node::Element(rect));

        let config = RemoveAttrsConfig {
            attrs: vec!["fill".to_string(), "stroke".to_string()],
            ..Default::default()
        };
        let plugin = RemoveAttrsPlugin::with_config(config);
        plugin.apply(&mut document).unwrap();

        if let Node::Element(rect) = &document.root.children[0] {
            assert!(!rect.has_attr("fill"));
            assert!(!rect.has_attr("stroke"));
            assert!(rect.has_attr("width"));
        }
    }

    #[test]
    fn test_regex_pattern_removal() {
        let mut document = Document::new();
        document.root.name = "svg".into();

        let mut rect = Element::new("rect");
        rect.set_attr("fill", "red");
        rect.set_attr("stroke", "blue");
        rect.set_attr("stroke-width", "2");
        rect.set_attr("stroke-opacity", "0.5");

        document.root.children.push(Node::Element(rect));

        let config = RemoveAttrsConfig {
            attrs: vec!["stroke.*".to_string()],
            ..Default::default()
        };
        let plugin = RemoveAttrsPlugin::with_config(config);
        plugin.apply(&mut document).unwrap();

        if let Node::Element(rect) = &document.root.children[0] {
            assert!(rect.has_attr("fill"));
            assert!(!rect.has_attr("stroke"));
            assert!(!rect.has_attr("stroke-width"));
            assert!(!rect.has_attr("stroke-opacity"));
        }
    }

    #[test]
    fn test_element_specific_removal() {
        let mut document = Document::new();
        document.root.name = "svg".into();

        let mut circle = Element::new("circle");
        circle.set_attr("fill", "red");

        let mut rect = Element::new("rect");
        rect.set_attr("fill", "blue");

        document.root.children.push(Node::Element(circle));
        document.root.children.push(Node::Element(rect));

        let config = RemoveAttrsConfig {
            attrs: vec!["circle:fill".to_string()],
            ..Default::default()
        };
        let plugin = RemoveAttrsPlugin::with_config(config);
        plugin.apply(&mut document).unwrap();

        // Circle should have fill removed, rect should keep it
        if let Node::Element(circle_elem) = &document.root.children[0] {
            assert!(!circle_elem.has_attr("fill"));
        }
        if let Node::Element(rect_elem) = &document.root.children[1] {
            assert!(rect_elem.has_attr("fill"));
        }
    }

    #[test]
    fn test_value_specific_removal() {
        let mut document = Document::new();
        document.root.name = "svg".into();

        let mut rect = Element::new("rect");
        rect.set_attr("fill", "red");
        rect.set_attr("stroke", "blue");

        document.root.children.push(Node::Element(rect));

        let config = RemoveAttrsConfig {
            attrs: vec!["*:fill:red".to_string()],
            ..Default::default()
        };
        let plugin = RemoveAttrsPlugin::with_config(config);
        plugin.apply(&mut document).unwrap();

        if let Node::Element(rect) = &document.root.children[0] {
            assert!(!rect.has_attr("fill"));
            assert!(rect.has_attr("stroke"));
        }
    }

    #[test]
    fn test_preserve_current_color() {
        let mut document = Document::new();
        document.root.name = "svg".into();

        let mut rect = Element::new("rect");
        rect.set_attr("fill", "currentColor");
        rect.set_attr("stroke", "red");

        document.root.children.push(Node::Element(rect));

        let config = RemoveAttrsConfig {
            attrs: vec!["(fill|stroke)".to_string()],
            preserve_current_color: true,
            ..Default::default()
        };
        let plugin = RemoveAttrsPlugin::with_config(config);
        plugin.apply(&mut document).unwrap();

        if let Node::Element(rect) = &document.root.children[0] {
            assert!(rect.has_attr("fill")); // currentColor preserved
            assert!(!rect.has_attr("stroke")); // red removed
        }
    }

    #[test]
    fn test_preserve_current_color_case_insensitive() {
        let mut document = Document::new();
        document.root.name = "svg".into();

        let mut rect = Element::new("rect");
        rect.set_attr("fill", "currentcolor");
        rect.set_attr("stroke", "CURRENTCOLOR");

        document.root.children.push(Node::Element(rect));

        let config = RemoveAttrsConfig {
            attrs: vec!["(fill|stroke)".to_string()],
            preserve_current_color: true,
            ..Default::default()
        };
        let plugin = RemoveAttrsPlugin::with_config(config);
        plugin.apply(&mut document).unwrap();

        if let Node::Element(rect) = &document.root.children[0] {
            assert!(rect.has_attr("fill")); // currentcolor preserved
            assert!(rect.has_attr("stroke")); // CURRENTCOLOR preserved
        }
    }

    #[test]
    fn test_custom_separator() {
        let mut document = Document::new();
        document.root.name = "svg".into();

        let mut rect = Element::new("rect");
        rect.set_attr("fill", "red");
        rect.set_attr("stroke", "blue");

        document.root.children.push(Node::Element(rect));

        let config = RemoveAttrsConfig {
            attrs: vec!["rect|fill".to_string()],
            elem_separator: "|".to_string(),
            ..Default::default()
        };
        let plugin = RemoveAttrsPlugin::with_config(config);
        plugin.apply(&mut document).unwrap();

        if let Node::Element(rect) = &document.root.children[0] {
            assert!(!rect.has_attr("fill"));
            assert!(rect.has_attr("stroke"));
        }
    }

    #[test]
    fn test_nested_elements() {
        let mut document = Document::new();
        document.root.name = "svg".into();

        let mut group = Element::new("g");
        group.set_attr("fill", "red");

        let mut rect = Element::new("rect");
        rect.set_attr("fill", "blue");
        rect.set_attr("stroke", "green");

        group.children.push(Node::Element(rect));
        document.root.children.push(Node::Element(group));

        let config = RemoveAttrsConfig {
            attrs: vec!["fill".to_string()],
            ..Default::default()
        };
        let plugin = RemoveAttrsPlugin::with_config(config);
        plugin.apply(&mut document).unwrap();

        if let Node::Element(group_elem) = &document.root.children[0] {
            assert!(!group_elem.has_attr("fill"));

            if let Node::Element(rect_elem) = &group_elem.children[0] {
                assert!(!rect_elem.has_attr("fill"));
                assert!(rect_elem.has_attr("stroke"));
            }
        }
    }
}

        let mut rect = Element::new("rect");
        rect.set_attr("fill", "red");
        rect.set_attr("stroke", "blue");
        rect.set_attr("width", "100");

        document.root.children.push(Node::Element(rect));

        let config = RemoveAttrsConfig {
            attrs: vec!["fill".to_string(), "stroke".to_string()],
            ..Default::default()
        };
        let plugin = RemoveAttrsPlugin::with_config(config);
        plugin.apply(&mut document).unwrap();

        if let Node::Element(rect) = &document.root.children[0] {
            assert!(!rect.has_attr("fill"));
            assert!(!rect.has_attr("stroke"));
            assert!(rect.has_attr("width"));
        }
    }

    #[test]
    fn test_regex_pattern_removal() {
        let mut document = Document::new();
        document.root.name = "svg".into();

        let mut rect = Element::new("rect");
        rect.set_attr("fill", "red");
        rect.set_attr("stroke", "blue");
        rect.set_attr("stroke-width", "2");
        rect.set_attr("stroke-opacity", "0.5");

        document.root.children.push(Node::Element(rect));

        let config = RemoveAttrsConfig {
            attrs: vec!["stroke.*".to_string()],
            ..Default::default()
        };
        let plugin = RemoveAttrsPlugin::with_config(config);
        plugin.apply(&mut document).unwrap();

        if let Node::Element(rect) = &document.root.children[0] {
            assert!(rect.has_attr("fill"));
            assert!(!rect.has_attr("stroke"));
            assert!(!rect.has_attr("stroke-width"));
            assert!(!rect.has_attr("stroke-opacity"));
        }
    }

    #[test]
    fn test_element_specific_removal() {
        let mut document = Document::new();
        document.root.name = "svg".into();

        let mut circle = Element::new("circle");
        circle.set_attr("fill", "red");

        let mut rect = Element::new("rect");
        rect.set_attr("fill", "blue");

        document.root.children.push(Node::Element(circle));
        document.root.children.push(Node::Element(rect));

        let config = RemoveAttrsConfig {
            attrs: vec!["circle:fill".to_string()],
            ..Default::default()
        };
        let plugin = RemoveAttrsPlugin::with_config(config);
        plugin.apply(&mut document).unwrap();

        // Circle should have fill removed, rect should keep it
        if let Node::Element(circle_elem) = &document.root.children[0] {
            assert!(!circle_elem.has_attr("fill"));
        }
        if let Node::Element(rect_elem) = &document.root.children[1] {
            assert!(rect_elem.has_attr("fill"));
        }
    }

    #[test]
    fn test_value_specific_removal() {
        let mut document = Document::new();
        document.root.name = "svg".into();

        let mut rect = Element::new("rect");
        rect.set_attr("fill", "red");
        rect.set_attr("stroke", "blue");

        document.root.children.push(Node::Element(rect));

        let config = RemoveAttrsConfig {
            attrs: vec!["*:fill:red".to_string()],
            ..Default::default()
        };
        let plugin = RemoveAttrsPlugin::with_config(config);
        plugin.apply(&mut document).unwrap();

        if let Node::Element(rect) = &document.root.children[0] {
            assert!(!rect.has_attr("fill"));
            assert!(rect.has_attr("stroke"));
        }
    }

    #[test]
    fn test_preserve_current_color() {
        let mut document = Document::new();
        document.root.name = "svg".into();

        let mut rect = Element::new("rect");
        rect.set_attr("fill", "currentColor");
        rect.set_attr("stroke", "red");

        document.root.children.push(Node::Element(rect));

        let config = RemoveAttrsConfig {
            attrs: vec!["(fill|stroke)".to_string()],
            preserve_current_color: true,
            ..Default::default()
        };
        let plugin = RemoveAttrsPlugin::with_config(config);
        plugin.apply(&mut document).unwrap();

        if let Node::Element(rect) = &document.root.children[0] {
            assert!(rect.has_attr("fill")); // currentColor preserved
            assert!(!rect.has_attr("stroke")); // red removed
        }
    }

    #[test]
    fn test_preserve_current_color_case_insensitive() {
        let mut document = Document::new();
        document.root.name = "svg".into();

        let mut rect = Element::new("rect");
        rect.set_attr("fill", "currentcolor");
        rect.set_attr("stroke", "CURRENTCOLOR");

        document.root.children.push(Node::Element(rect));

        let config = RemoveAttrsConfig {
            attrs: vec!["(fill|stroke)".to_string()],
            preserve_current_color: true,
            ..Default::default()
        };
        let plugin = RemoveAttrsPlugin::with_config(config);
        plugin.apply(&mut document).unwrap();

        if let Node::Element(rect) = &document.root.children[0] {
            assert!(rect.has_attr("fill")); // currentcolor preserved
            assert!(rect.has_attr("stroke")); // CURRENTCOLOR preserved
        }
    }

    #[test]
    fn test_custom_separator() {
        let mut document = Document::new();
        document.root.name = "svg".into();

        let mut rect = Element::new("rect");
        rect.set_attr("fill", "red");
        rect.set_attr("stroke", "blue");

        document.root.children.push(Node::Element(rect));

        let config = RemoveAttrsConfig {
            attrs: vec!["rect|fill".to_string()],
            elem_separator: "|".to_string(),
            ..Default::default()
        };
        let plugin = RemoveAttrsPlugin::with_config(config);
        plugin.apply(&mut document).unwrap();

        if let Node::Element(rect) = &document.root.children[0] {
            assert!(!rect.has_attr("fill"));
            assert!(rect.has_attr("stroke"));
        }
    }

    #[test]
    fn test_nested_elements() {
        let mut document = Document::new();
        document.root.name = "svg".into();

        let mut group = Element::new("g");
        group.set_attr("fill", "red");

        let mut rect = Element::new("rect");
        rect.set_attr("fill", "blue");
        rect.set_attr("stroke", "green");

        group.children.push(Node::Element(rect));
        document.root.children.push(Node::Element(group));

        let config = RemoveAttrsConfig {
            attrs: vec!["fill".to_string()],
            ..Default::default()
        };
        let plugin = RemoveAttrsPlugin::with_config(config);
        plugin.apply(&mut document).unwrap();

        if let Node::Element(group_elem) = &document.root.children[0] {
            assert!(!group_elem.has_attr("fill"));

            if let Node::Element(rect_elem) = &group_elem.children[0] {
                assert!(!rect_elem.has_attr("fill"));
                assert!(rect_elem.has_attr("stroke"));
            }
        }
    }
}
