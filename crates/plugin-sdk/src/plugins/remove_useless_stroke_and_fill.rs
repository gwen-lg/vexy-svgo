// this_file: crates/plugin-sdk/src/plugins/remove_useless_stroke_and_fill.rs

//! Remove useless stroke and fill attributes
//!
//! This plugin removes stroke and fill attributes that are either:
//! - Set to PROTECTED_18_ when no parent element has these attributes
//! - Set to transparent (opacity 0)
//! - Stroke width set to 0
//!
//! It also handles inheritance and can optionally remove elements that have
//! no visible stroke or fill (removeNone parameter).
//!
//! Reference: SVGOPROTECTED_94_static str>> = Lazy::new(|| {
    HashSet::from([
        "rect",
        "circle",
        "ellipse",
        "line",
        "polyline",
        "polygon",
        "path",
        "text",
        "tspan",
        "textPath",
        "altGlyph",
        "glyph",
        "missing-glyph",
    ])
});

/// Configuration for the removeUselessStrokeAndFill plugin
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RemoveUselessStrokeAndFillConfig {
    #[serde(default = "default_true")]
    pub stroke: bool,
    #[serde(default = "default_true")]
    pub fill: bool,
    #[serde(default)]
    pub remove_none: bool,
}

fn default_true() -> bool {
    true
}

impl Default for RemoveUselessStrokeAndFillConfig {
    fn default() -> Self {
        Self {
            stroke: true,
            fill: true,
            remove_none: false,
        }
    }
}

/// Remove useless stroke and fill attributes
pub struct RemoveUselessStrokeAndFillPlugin {
    config: RemoveUselessStrokeAndFillConfig,
}

impl RemoveUselessStrokeAndFillPlugin {
    pub fn new() -> Self {
        Self {
            config: RemoveUselessStrokeAndFillConfig::default(),
        }
    }

    pub fn with_config(config: RemoveUselessStrokeAndFillConfig) -> Self {
        Self { config }
    }

    fn parse_config(params: &Value) -> Result<RemoveUselessStrokeAndFillConfig> {
        if params.is_null() {
            Ok(RemoveUselessStrokeAndFillConfig::default())
        } else {
            serde_json::from_value(params.clone())
                .map_err(|e| anyhow::anyhow!("Invalid plugin configuration: {}", e))
        }
    }

    fn has_style_or_script(&self, element: &Element) -> bool {
        if element.name == "style" || element.name == "script" {
            return true;
        }

        for child in &element.children {
            if let Node::Element(child_elem) = child {
                if self.has_style_or_script(child_elem) {
                    return true;
                }
            }
        }

        false
    }

    fn process_element(
        &self,
        element: &mut Element,
        parent_styles: &HashMap<String, String>,
    ) -> (HashMap<String, String>, bool) {
        // Skip elements with ID (they might be referenced by CSS)
        if element.has_attr("id") {
            return (HashMap::new(), false);
        }

        // Only process shape elements
        if !SHAPE_ELEMENTS.contains(&element.name.as_ref()) {
            return (HashMap::new(), false);
        }

        // Compute current element styles
        let current_styles = self.compute_element_styles(element, parent_styles);

        // Process stroke attributes
        if self.config.stroke {
            self.process_stroke_attributes(element, &current_styles, parent_styles);
        }

        // Process fill attributes
        if self.config.fill {
            self.process_fill_attributes(element, &current_styles);
        }

        // Check if element should be removed (has no visible stroke or fill)
        let should_remove = self.config.remove_none && self.should_remove_element(element, &current_styles);

        (current_styles, should_remove)
    }

    fn compute_element_styles(
        &self,
        element: &Element,
        parent_styles: &HashMap<String, String>,
    ) -> HashMap<String, String> {
        let mut styles = parent_styles.clone();

        // Override with elementPROTECTED_95_;PROTECTED_96_:PROTECTED_97_t remove stroke=PROTECTED_63_ when overriding inheritance
                        } else {
                            k.starts_with("stroke")
                        }
                    })
                    .cloned()
                    .collect();

                for attr in stroke_attrs {
                    element.remove_attr(&attr);
                }

                // If we had stroke-width=PROTECTED_65_ with a non-none stroke, set stroke=PROTECTED_66_
                if has_zero_width_with_stroke {
                    element.set_attr("stroke", "none");
                }
            }
        }
    }

    fn process_fill_attributes(
        &self,
        element: &mut Element,
        current_styles: &HashMap<String, String>,
    ) {
        let fill = current_styles.get("fill");
        let fill_opacity = current_styles.get("fill-opacity");

        let should_remove_fill =
            fill.is_some_and(|f| f == "none") || fill_opacity.is_some_and(|op| op == "0");

        if should_remove_fill {
            // Remove all fill-related attributes except fill itself
            let fill_attrs: Vec<String> = element.attributes
                .keys()
                .filter(|k| k.starts_with("fill-"))
                .cloned()
                .collect();

            for attr in fill_attrs {
                element.remove_attr(&attr);
            }

            // Set explicit PROTECTED_74_ if not already set
            if fill.is_none_or(|f| f != "none") {
                element.set_attr("fill", "none");
            }
        }
    }

    fn should_remove_element(
        &self,
        element: &Element,
        current_styles: &HashMap<String, String>,
    ) -> bool {
        let stroke = current_styles.get("stroke");
        let fill = current_styles.get("fill");

        let no_stroke = stroke.is_none_or(|s| s == "none")
            || element.attr("stroke").is_some_and(|s| s == "none");
        let no_fill =
            fill.is_some_and(|f| f == "none") || element.attr("fill").is_some_and(|f| f == "none");

        no_stroke && no_fill
    }

    fn remove_marked_elements(&self, element: &mut Element) {
        let mut i = 0;
        while i < element.children.len() {
            let mut remove = false;
            if let Node::Element(child_elem) = &mut element.children[i] {
                // First, process the childPROTECTED_98_static str {
        "removeUselessStrokeAndFill"
    }

    fn description(&self) -> &'static str {
        "remove useless stroke and fill attributes"
    }

    fn validate_params(&self, params: &Value) -> Result<()> {
        Self::parse_config(params)?;
        Ok(())
    }

    fn apply(&self, document: &mut Document) -> Result<()> {
        // Skip optimization if there are style or script elements
        if self.has_style_or_script(&document.root) {
            return Ok(());
        }

        // First pass: process attributes
        self.process_element_recursive(&mut document.root, &HashMap::new());

        // Second pass: remove elements with no visible stroke or fill
        if self.config.remove_none {
            self.remove_marked_elements(&mut document.root);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;

    #[test]
    fn test_plugin_info() {
        let plugin = RemoveUselessStrokeAndFillPlugin::new();
        assert_eq!(plugin.name(), "removeUselessStrokeAndFill");
        assert_eq!(
            plugin.description(),
            "remove useless stroke and fill attributes"
        );
    }

    #[test]
    fn test_param_validation() {
        let plugin = RemoveUselessStrokeAndFillPlugin::new();

        // Test null params
        assert!(plugin.validate_params(&Value::Null).is_ok());

        // Test valid params
        assert!(plugin
            .validate_params(&serde_json::json!({
                "stroke": true,
                "fill": false,
                "removeNone": true
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
    fn test_remove_stroke_none() {
        let input = r#"<svg><rect stroke="none" fill="red" width="100" height="100"/></svg>"#;
        let expected = r#"<svg><rect fill="red" width="100" height="100"/></svg>"#;

        assert_plugin_output(&RemoveUselessStrokeAndFillPlugin::new(), input, expected);
    }

    #[test]
    fn test_remove_fill_none() {
        let input = r#"<svg><rect stroke="blue" fill="none" width="100" height="100"/></svg>"#;
        let expected = r#"<svg><rect stroke="blue" fill="none" width="100" height="100"/></svg>"#;

        assert_plugin_output(&RemoveUselessStrokeAndFillPlugin::new(), input, expected);
    }

    #[test]
    fn test_remove_zero_opacity() {
        let input =
            r#"<svg><rect stroke-opacity="0" fill-opacity="0" width="100" height="100"/></svg>"#;
        let expected = r#"<svg><rect fill="none" width="100" height="100"/></svg>"#;

        assert_plugin_output(&RemoveUselessStrokeAndFillPlugin::new(), input, expected);
    }

    #[test]
    fn test_preserve_with_id() {
        let input =
            r#"<svg><rect id="test" stroke="none" fill="none" width="100" height="100"/></svg>"#;
        let expected =
            r#"<svg><rect id="test" stroke="none" fill="none" width="100" height="100"/></svg>"#;

        assert_plugin_output(&RemoveUselessStrokeAndFillPlugin::new(), input, expected);
    }

    #[test]
    fn test_skip_with_style_element() {
        let input = r#"<svg><style>.test { fill: red; }</style><rect stroke="none" fill="none" width="100" height="100"/></svg>"#;
        let expected = r#"<svg><style>.test { fill: red; }</style><rect stroke="none" fill="none" width="100" height="100"/></svg>"#;

        assert_plugin_output(&RemoveUselessStrokeAndFillPlugin::new(), input, expected);
    }

    #[test]
    fn test_with_stroke_width_zero() {
        let input = r#"<svg><rect stroke-width="0" stroke="red" fill="blue"/></svg>"#;
        let expected = r#"<svg><rect stroke="none" fill="blue"/></svg>"#;

        assert_plugin_output(&RemoveUselessStrokeAndFillPlugin::new(), input, expected);
    }

    #[test]
    fn test_inheritance() {
        let input = r#"<svg><g stroke="red"><rect stroke="none"/></g></svg>"#;
        let expected = r#"<svg><g stroke="red"><rect stroke="none"/></g></svg>"#;

        assert_plugin_output(&RemoveUselessStrokeAndFillPlugin::new(), input, expected);
    }

    #[test]
    fn test_config_stroke_false() {
        let config = RemoveUselessStrokeAndFillConfig {
            stroke: false,
            fill: true,
            remove_none: false,
        };
        let plugin = RemoveUselessStrokeAndFillPlugin::with_config(config);

        let input = r#"<svg><rect stroke="none" fill="none" width="100" height="100"/></svg>"#;
        let expected = r#"<svg><rect stroke="none" fill="none" width="100" height="100"/></svg>"#;

        assert_plugin_output(&plugin, input, expected);
    }

    #[test]
    fn test_config_fill_false() {
        let config = RemoveUselessStrokeAndFillConfig {
            stroke: true,
            fill: false,
            remove_none: false,
        };
        let plugin = RemoveUselessStrokeAndFillPlugin::with_config(config);

        let input = r#"<svg><rect stroke="none" fill="none" width="100" height="100"/></svg>"#;
        let expected = r#"<svg><rect fill="none" width="100" height="100"/></svg>"#;

        assert_plugin_output(&plugin, input, expected);
    }
}
            "remove useless stroke and fill attributes"
        );
    }

    #[test]
    fn test_param_validation() {
        let plugin = RemoveUselessStrokeAndFillPlugin::new();

        // Test null params
        assert!(plugin.validate_params(&Value::Null).is_ok());

        // Test valid params
        assert!(plugin
            .validate_params(&serde_json::json!({
                "stroke": true,
                "fill": false,
                "removeNone": true
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
    fn test_remove_stroke_none() {
        let input = r#"<svg><rect stroke="none" fill="red" width="100" height="100"/></svg>"#;
        let expected = r#"<svg><rect fill="red" width="100" height="100"/></svg>"#;

        assert_plugin_output(&RemoveUselessStrokeAndFillPlugin::new(), input, expected);
    }

    #[test]
    fn test_remove_fill_none() {
        let input = r#"<svg><rect stroke="blue" fill="none" width="100" height="100"/></svg>"#;
        let expected = r#"<svg><rect stroke="blue" fill="none" width="100" height="100"/></svg>"#;

        assert_plugin_output(&RemoveUselessStrokeAndFillPlugin::new(), input, expected);
    }

    #[test]
    fn test_remove_zero_opacity() {
        let input =
            r#"<svg><rect stroke-opacity="0" fill-opacity="0" width="100" height="100"/></svg>"#;
        let expected = r#"<svg><rect fill="none" width="100" height="100"/></svg>"#;

        assert_plugin_output(&RemoveUselessStrokeAndFillPlugin::new(), input, expected);
    }

    #[test]
    fn test_preserve_with_id() {
        let input =
            r#"<svg><rect id="test" stroke="none" fill="none" width="100" height="100"/></svg>"#;
        let expected =
            r#"<svg><rect id="test" stroke="none" fill="none" width="100" height="100"/></svg>"#;

        assert_plugin_output(&RemoveUselessStrokeAndFillPlugin::new(), input, expected);
    }

    #[test]
    fn test_skip_with_style_element() {
        let input = r#"<svg><style>.test { fill: red; }</style><rect stroke="none" fill="none" width="100" height="100"/></svg>"#;
        let expected = r#"<svg><style>.test { fill: red; }</style><rect stroke="none" fill="none" width="100" height="100"/></svg>"#;

        assert_plugin_output(&RemoveUselessStrokeAndFillPlugin::new(), input, expected);
    }

    #[test]
    fn test_with_stroke_width_zero() {
        let input = r#"<svg><rect stroke-width="0" stroke="red" fill="blue"/></svg>"#;
        let expected = r#"<svg><rect stroke="none" fill="blue"/></svg>"#;

        assert_plugin_output(&RemoveUselessStrokeAndFillPlugin::new(), input, expected);
    }

    #[test]
    fn test_inheritance() {
        let input = r#"<svg><g stroke="red"><rect stroke="none"/></g></svg>"#;
        let expected = r#"<svg><g stroke="red"><rect stroke="none"/></g></svg>"#;

        assert_plugin_output(&RemoveUselessStrokeAndFillPlugin::new(), input, expected);
    }

    #[test]
    fn test_config_stroke_false() {
        let config = RemoveUselessStrokeAndFillConfig {
            stroke: false,
            fill: true,
            remove_none: false,
        };
        let plugin = RemoveUselessStrokeAndFillPlugin::with_config(config);

        let input = r#"<svg><rect stroke="none" fill="none" width="100" height="100"/></svg>"#;
        let expected = r#"<svg><rect stroke="none" fill="none" width="100" height="100"/></svg>"#;

        assert_plugin_output(&plugin, input, expected);
    }

    #[test]
    fn test_config_fill_false() {
        let config = RemoveUselessStrokeAndFillConfig {
            stroke: true,
            fill: false,
            remove_none: false,
        };
        let plugin = RemoveUselessStrokeAndFillPlugin::with_config(config);

        let input = r#"<svg><rect stroke="none" fill="none" width="100" height="100"/></svg>"#;
        let expected = r#"<svg><rect fill="none" width="100" height="100"/></svg>"#;

        assert_plugin_output(&plugin, input, expected);
    }
}
