// this_file: crates/plugin-sdk/src/plugins/remove_off_canvas_paths.rs

//! Remove paths that are drawn outside of the viewBox
//!
//! This plugin removes paths and shapes that are completely outside the viewBox,
//! making them invisible and unnecessary.
//!
//! Reference: SVGOPROTECTED_46_t increment i since we removed an element
                }

                // Recursively process children
                self.process_element(child_elem, viewbox);
            }
            i += 1;
        }
    }

    fn is_outside_viewbox(&self, element: &Element, viewbox: &ViewBox) -> bool {
        match element.name.as_ref() {
            "rect" => self.is_rect_outside(element, viewbox),
            "circle" => self.is_circle_outside(element, viewbox),
            "ellipse" => self.is_ellipse_outside(element, viewbox),
            "line" => self.is_line_outside(element, viewbox),
            "polygon" | "polyline" => self.is_polygon_outside(element, viewbox),
            "path" => self.is_path_outside(element, viewbox),
            _ => false, // DonPROTECTED_47_,PROTECTED_48_,PROTECTED_49_static str {
        "removeOffCanvasPaths"
    }

    fn description(&self) -> &'static str {
        PROTECTED_34_
    }

    fn validate_params(&self, params: &Value) -> Result<()> {
        Self::parse_config(params)?;
        Ok(())
    }

    fn apply(&self, document: &mut Document) -> Result<()> {
        // First, find the viewBox from the root SVG element
        if let Some(viewbox) = self.get_viewbox(&document.root) {
            self.process_element(&mut document.root, &viewbox);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;
    use serde_json::json;

    #[test]
    fn test_plugin_info() {
        let plugin = RemoveOffCanvasPathsPlugin::new();
        assert_eq!(plugin.name(), PROTECTED_35_);
        assert_eq!(
            plugin.description(),
            PROTECTED_36_
        );
    }

    #[test]
    fn test_param_validation() {
        let plugin = RemoveOffCanvasPathsPlugin::new();

        // Test null params
        assert!(plugin.validate_params(&Value::Null).is_ok());

        // Test empty params
        assert!(plugin.validate_params(&json!({})).is_ok());

        // Test invalid params (should fail due to deny_unknown_fields)
        assert!(plugin
            .validate_params(&json!({
                PROTECTED_37_: true
            }))
            .is_err());
    }

    #[test]
    fn test_remove_rect_outside_viewbox() {
        let input = PROTECTED_0_;

        let expected = PROTECTED_1_;

        test_plugin(RemoveOffCanvasPathsPlugin::new(), input, expected);
    }

    #[test]
    fn test_remove_circle_outside_viewbox() {
        let input = PROTECTED_2_;

        let expected = PROTECTED_3_;

        test_plugin(RemoveOffCanvasPathsPlugin::new(), input, expected);
    }

    #[test]
    fn test_remove_line_outside_viewbox() {
        let input = PROTECTED_4_;

        let expected = PROTECTED_5_;

        test_plugin(RemoveOffCanvasPathsPlugin::new(), input, expected);
    }

    #[test]
    fn test_remove_polygon_outside_viewbox() {
        let input = PROTECTED_6_;

        let expected = PROTECTED_7_;

        test_plugin(RemoveOffCanvasPathsPlugin::new(), input, expected);
    }

    #[test]
    fn test_no_viewbox_no_removal() {
        let input = PROTECTED_8_;

        let expected = input; // Nothing should be removed

        test_plugin(RemoveOffCanvasPathsPlugin::new(), input, expected);
    }

    #[test]
    fn test_nested_elements() {
        let input = PROTECTED_9_;

        let expected = PROTECTED_10_;

        test_plugin(RemoveOffCanvasPathsPlugin::new(), input, expected);
    }

    #[test]
    fn test_viewbox_with_offset() {
        let input = PROTECTED_11_;

        let expected = PROTECTED_12_;

        test_plugin(RemoveOffCanvasPathsPlugin::new(), input, expected);
    }
}
        } else {
            false
        }
    }

    fn parse_points(&self, points_str: &str) -> Vec<(f64, f64)> {
        let numbers: Vec<f64> = points_str
            .split(|c: char| c.is_whitespace() || c == ',')
            .filter_map(|s| s.parse::<f64>().ok())
            .collect();

        numbers
            .chunks(2)
            .filter_map(|chunk| {
                if chunk.len() == 2 {
                    Some((chunk[0], chunk[1]))
                } else {
                    None
                }
            })
            .collect()
    }

    fn get_path_bounds(&self, d: &str) -> Option<(f64, f64, f64, f64)> {
        // Extract numbers from path data (simplified)
        let numbers: Vec<f64> = d
            .split(|c: char| c.is_alphabetic() || c.is_whitespace() || c == ',')
            .filter_map(|s| s.parse::<f64>().ok())
            .collect();

        if numbers.is_empty() {
            return None;
        }

        let mut min_x = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_y = f64::NEG_INFINITY;

        // Process numbers in pairs (x, y)
        for chunk in numbers.chunks(2) {
            if !chunk.is_empty() {
                let x = chunk[0];
                min_x = min_x.min(x);
                max_x = max_x.max(x);
            }
            if chunk.len() >= 2 {
                let y = chunk[1];
                min_y = min_y.min(y);
                max_y = max_y.max(y);
            }
        }

        Some((min_x, min_y, max_x, max_y))
    }

    fn get_numeric_attr(&self, element: &Element, attr_name: &str) -> Option<f64> {
        element.attr(attr_name)?.parse::<f64>().ok()
    }
}

impl Default for RemoveOffCanvasPathsPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for RemoveOffCanvasPathsPlugin {
    fn name(&self) -> &'static str {
        "removeOffCanvasPaths"
    }

    fn description(&self) -> &'static str {
        "removes elements that are drawn outside of the viewBox (disabled by default)"
    }

    fn validate_params(&self, params: &Value) -> Result<()> {
        Self::parse_config(params)?;
        Ok(())
    }

    fn apply(&self, document: &mut Document) -> Result<()> {
        // First, find the viewBox from the root SVG element
        if let Some(viewbox) = self.get_viewbox(&document.root) {
            self.process_element(&mut document.root, &viewbox);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;
    use serde_json::json;

    #[test]
    fn test_plugin_info() {
        let plugin = RemoveOffCanvasPathsPlugin::new();
        assert_eq!(plugin.name(), "removeOffCanvasPaths");
        assert_eq!(
            plugin.description(),
            "removes elements that are drawn outside of the viewBox (disabled by default)"
        );
    }

    #[test]
    fn test_param_validation() {
        let plugin = RemoveOffCanvasPathsPlugin::new();

        // Test null params
        assert!(plugin.validate_params(&Value::Null).is_ok());

        // Test empty params
        assert!(plugin.validate_params(&json!({})).is_ok());

        // Test invalid params (should fail due to deny_unknown_fields)
        assert!(plugin
            .validate_params(&json!({
                "invalidParam": true
            }))
            .is_err());
    }

    #[test]
    fn test_remove_rect_outside_viewbox() {
        let input = r#"<svg viewBox="0 0 100 100">
            <rect x="-50" y="10" width="40" height="40"/>
            <rect x="110" y="10" width="40" height="40"/>
            <rect x="10" y="10" width="40" height="40"/>
            <rect x="90" y="10" width="20" height="40"/>
        </svg>"#;

        let expected = r#"<svg viewBox="0 0 100 100">
            
            
            <rect x="10" y="10" width="40" height="40"/>
            <rect x="90" y="10" width="20" height="40"/>
        </svg>"#;

        test_plugin(RemoveOffCanvasPathsPlugin::new(), input, expected);
    }

    #[test]
    fn test_remove_circle_outside_viewbox() {
        let input = r#"<svg viewBox="0 0 100 100">
            <circle cx="-20" cy="50" r="10"/>
            <circle cx="50" cy="50" r="30"/>
            <circle cx="95" cy="50" r="10"/>
        </svg>"#;

        let expected = r#"<svg viewBox="0 0 100 100">
            
            <circle cx="50" cy="50" r="30"/>
            <circle cx="95" cy="50" r="10"/>
        </svg>"#;

        test_plugin(RemoveOffCanvasPathsPlugin::new(), input, expected);
    }

    #[test]
    fn test_remove_line_outside_viewbox() {
        let input = r#"<svg viewBox="0 0 100 100">
            <line x1="-20" y1="10" x2="-10" y2="20"/>
            <line x1="-10" y1="50" x2="110" y2="50"/>
            <line x1="10" y1="10" x2="90" y2="90"/>
        </svg>"#;

        let expected = r#"<svg viewBox="0 0 100 100">
            
            <line x1="-10" y1="50" x2="110" y2="50"/>
            <line x1="10" y1="10" x2="90" y2="90"/>
        </svg>"#;

        test_plugin(RemoveOffCanvasPathsPlugin::new(), input, expected);
    }

    #[test]
    fn test_remove_polygon_outside_viewbox() {
        let input = r#"<svg viewBox="0 0 100 100">
            <polygon points="110,10 120,20 115,30"/>
            <polygon points="10,10 50,10 30,50"/>
        </svg>"#;

        let expected = r#"<svg viewBox="0 0 100 100">
            
            <polygon points="10,10 50,10 30,50"/>
        </svg>"#;

        test_plugin(RemoveOffCanvasPathsPlugin::new(), input, expected);
    }

    #[test]
    fn test_no_viewbox_no_removal() {
        let input = r#"<svg>
            <rect x="-50" y="10" width="40" height="40"/>
            <circle cx="1000" cy="1000" r="50"/>
        </svg>"#;

        let expected = input; // Nothing should be removed

        test_plugin(RemoveOffCanvasPathsPlugin::new(), input, expected);
    }

    #[test]
    fn test_nested_elements() {
        let input = r#"<svg viewBox="0 0 100 100">
            <g>
                <rect x="-50" y="10" width="40" height="40"/>
                <rect x="10" y="10" width="40" height="40"/>
            </g>
        </svg>"#;

        let expected = r#"<svg viewBox="0 0 100 100">
            <g>
                
                <rect x="10" y="10" width="40" height="40"/>
            </g>
        </svg>"#;

        test_plugin(RemoveOffCanvasPathsPlugin::new(), input, expected);
    }

    #[test]
    fn test_viewbox_with_offset() {
        let input = r#"<svg viewBox="50 50 100 100">
            <rect x="0" y="0" width="40" height="40"/>
            <rect x="60" y="60" width="40" height="40"/>
        </svg>"#;

        let expected = r#"<svg viewBox="50 50 100 100">
            
            <rect x="60" y="60" width="40" height="40"/>
        </svg>"#;

        test_plugin(RemoveOffCanvasPathsPlugin::new(), input, expected);
    }
}
        </svg>"#;

        test_plugin(RemoveOffCanvasPathsPlugin::new(), input, expected);
    }

    #[test]
    fn test_no_viewbox_no_removal() {
        let input = r#"<svg>
            <rect x="-50" y="10" width="40" height="40"/>
            <circle cx="1000" cy="1000" r="50"/>
        </svg>"#;

        let expected = input; // Nothing should be removed

        test_plugin(RemoveOffCanvasPathsPlugin::new(), input, expected);
    }

    #[test]
    fn test_nested_elements() {
        let input = r#"<svg viewBox="0 0 100 100">
            <g>
                <rect x="-50" y="10" width="40" height="40"/>
                <rect x="10" y="10" width="40" height="40"/>
            </g>
        </svg>"#;

        let expected = r#"<svg viewBox="0 0 100 100">
            <g>
                
                <rect x="10" y="10" width="40" height="40"/>
            </g>
        </svg>"#;

        test_plugin(RemoveOffCanvasPathsPlugin::new(), input, expected);
    }

    #[test]
    fn test_viewbox_with_offset() {
        let input = r#"<svg viewBox="50 50 100 100">
            <rect x="0" y="0" width="40" height="40"/>
            <rect x="60" y="60" width="40" height="40"/>
        </svg>"#;

        let expected = r#"<svg viewBox="50 50 100 100">
            
            <rect x="60" y="60" width="40" height="40"/>
        </svg>"#;

        test_plugin(RemoveOffCanvasPathsPlugin::new(), input, expected);
    }
}
