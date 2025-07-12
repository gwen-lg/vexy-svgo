// this_file: crates/plugin-sdk/src/plugins/convert_shape_to_path.rs

//! Convert basic shapes to path elements
//!
//! This plugin converts rect, line, polyline, polygon, circle and ellipse elements
//! to path elements for better optimization potential.
//!
//! Reference: SVGO's convertShapeToPath plugin

use crate::Plugin;
use anyhow::Result;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::LazyLock;
use vexy_svgo_core::ast::{Document, Element, Node};

static NUMBER_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"[-+]?(?:\d*\.\d+|\d+\.?)(?:[eE][-+]?\d+)?").unwrap());

/// Configuration for the convert shape to path plugin
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ConvertShapeToPathConfig {
    /// Whether to convert circles and ellipses to paths using arc commands
    #[serde(default)]
    pub convert_arcs: bool,

    /// Number of decimal places for numeric values
    #[serde(skip_serializing_if = "Option::is_none")]
    pub float_precision: Option<u8>,
}

impl Default for ConvertShapeToPathConfig {
    fn default() -> Self {
        Self {
            convert_arcs: false,
            float_precision: None,
        }
    }
}

/// Plugin that converts basic shapes to path elements
pub struct ConvertShapeToPathPlugin {
    config: ConvertShapeToPathConfig,
}

impl ConvertShapeToPathPlugin {
    pub fn new() -> Self {
        Self {
            config: ConvertShapeToPathConfig::default(),
        }
    }

    pub fn with_config(config: ConvertShapeToPathConfig) -> Self {
        Self { config }
    }

    fn parse_config(params: &Value) -> Result<ConvertShapeToPathConfig> {
        if params.is_null() {
            Ok(ConvertShapeToPathConfig::default())
        } else {
            serde_json::from_value(params.clone())
                .map_err(|e| anyhow::anyhow!("Invalid plugin configuration: {}", e))
        }
    }

    /// Recursively convert shapes in an element and its children
    fn convert_shapes_in_element(&self, element: &mut Element) {
        // Process child elements first
        for child in &mut element.children {
            if let Node::Element(child_element) = child {
                self.convert_shapes_in_element(child_element);
            }
        }

        // Convert current element if it's a shape
        self.convert_shape_element(element);
    }

    /// Convert a shape element to a path if applicable
    fn convert_shape_element(&self, element: &mut Element) {
        match element.name.as_ref() {
            "rect" => self.convert_rect(element),
            "line" => self.convert_line(element),
            "polyline" => self.convert_polyline(element),
            "polygon" => self.convert_polygon(element),
            "circle" if self.config.convert_arcs => self.convert_circle(element),
            "ellipse" if self.config.convert_arcs => self.convert_ellipse(element),
            _ => {}
        }
    }

    /// Parse a coordinate value, returning None if it's not a valid number
    fn parse_coord(value: &str) -> Option<f64> {
        // Skip if contains non-numeric characters that indicate units or percentages
        if value.contains('%') || value.contains("px") || value.contains("pt") {
            return None;
        }
        value.parse().ok()
    }

    /// Convert a rectangle to a path
    fn convert_rect(&self, element: &mut Element) {
        // Don't convert rectangles with rounded corners
        if element.has_attr("rx") || element.has_attr("ry") {
            return;
        }

        // Extract required attributes
        let width_str = match element.attr("width") {
            Some(w) => w,
            None => return,
        };
        let height_str = match element.attr("height") {
            Some(h) => h,
            None => return,
        };

        let x = match Self::parse_coord(element.attr("x").unwrap_or("0")) {
            Some(x) => x,
            None => return,
        };
        let y = match Self::parse_coord(element.attr("y").unwrap_or("0")) {
            Some(y) => y,
            None => return,
        };
        let width = match Self::parse_coord(width_str) {
            Some(w) => w,
            None => return,
        };
        let height = match Self::parse_coord(height_str) {
            Some(h) => h,
            None => return,
        };

        // Build path data: M x y H x+width V y+height H x z
        let path_data = format!(
            "M{} {}H{}V{}H{}z",
            self.format_number(x),
            self.format_number(y),
            self.format_number(x + width),
            self.format_number(y + height),
            self.format_number(x)
        );

        // Update element
        element.name = "path".to_string().into();
        element.set_attr("d", &path_data);
        element.remove_attr("x");
        element.remove_attr("y");
        element.remove_attr("width");
        element.remove_attr("height");
    }

    /// Convert a line to a path
    fn convert_line(&self, element: &mut Element) {
        let x1 = match Self::parse_coord(element.attr("x1").unwrap_or("0")) {
            Some(x) => x,
            None => return,
        };
        let y1 = match Self::parse_coord(element.attr("y1").unwrap_or("0")) {
            Some(y) => y,
            None => return,
        };
        let x2 = match Self::parse_coord(element.attr("x2").unwrap_or("0")) {
            Some(x) => x,
            None => return,
        };
        let y2 = match Self::parse_coord(element.attr("y2").unwrap_or("0")) {
            Some(y) => y,
            None => return,
        };

        // Build path data: M x1 y1 x2 y2 (without L command to match SVGO)
        let path_data = format!(
            "M{} {} {} {}",
            self.format_number(x1),
            self.format_number(y1),
            self.format_number(x2),
            self.format_number(y2)
        );

        // Update element
        element.name = "path".to_string().into();
        element.set_attr("d", &path_data);
        element.remove_attr("x1");
        element.remove_attr("y1");
        element.remove_attr("x2");
        element.remove_attr("y2");
    }

    /// Convert polyline to a path
    fn convert_polyline(&self, element: &mut Element) {
        self.convert_poly(element, false);
    }

    /// Convert polygon to a path
    fn convert_polygon(&self, element: &mut Element) {
        self.convert_poly(element, true);
    }

    /// Convert polyline or polygon to a path
    fn convert_poly(&self, element: &mut Element, is_polygon: bool) {
        let points_str = match element.attr("points") {
            Some(p) => p,
            None => return,
        };

        // Extract all numbers from the points string
        let coords: Vec<f64> = NUMBER_REGEX
            .find_iter(points_str)
            .filter_map(|m| m.as_str().parse().ok())
            .collect();

        // Need at least 2 coordinate pairs (4 numbers)
        if coords.len() < 4 {
            // Remove the element by clearing all attributes and children
            element.attributes.clear();
            element.children.clear();
            // Mark for removal by parent
                        element.name = "".into();
            return;
        }

        // Build path data
        let mut path_data = String::new();

        for (i, chunk) in coords.chunks(2).enumerate() {
            if chunk.len() == 2 {
                if i == 0 {
                    path_data.push_str(&format!(
                        "M{} {}",
                        self.format_number(chunk[0]),
                        self.format_number(chunk[1])
                    ));
                } else {
                    path_data.push_str(&format!(
                        " {} {}",
                        self.format_number(chunk[0]),
                        self.format_number(chunk[1])
                    ));
                }
            }
        }

        // Add closing command for polygons
        if is_polygon {
            path_data.push('z');
        }

        // Update element
        element.name = "path".to_string().into();
        element.set_attr("d", &path_data);
        element.remove_attr("points");
    }

    /// Convert circle to a path using arc commands
    fn convert_circle(&self, element: &mut Element) {
        let cx = match Self::parse_coord(element.attr("cx").unwrap_or("0")) {
            Some(x) => x,
            None => return,
        };
        let cy = match Self::parse_coord(element.attr("cy").unwrap_or("0")) {
            Some(y) => y,
            None => return,
        };
        let r = match Self::parse_coord(element.attr("r").unwrap_or("0")) {
            Some(r) => r,
            None => return,
        };

        // Build path data using two arc commands
        let path_data = format!(
            "M{} {}A{} {} 0 1 0 {} {}A{} {} 0 1 0 {} {}z",
            self.format_number(cx),
            self.format_number(cy - r),
            self.format_number(r),
            self.format_number(r),
            self.format_number(cx),
            self.format_number(cy + r),
            self.format_number(r),
            self.format_number(r),
            self.format_number(cx),
            self.format_number(cy - r)
        );

        // Update element
        element.name = "path".to_string().into();
        element.set_attr("d", &path_data);
        element.remove_attr("cx");
        element.remove_attr("cy");
        element.remove_attr("r");
    }

    /// Convert ellipse to a path using arc commands
    fn convert_ellipse(&self, element: &mut Element) {
        let cx = match Self::parse_coord(element.attr("cx").unwrap_or("0")) {
            Some(x) => x,
            None => return,
        };
        let cy = match Self::parse_coord(element.attr("cy").unwrap_or("0")) {
            Some(y) => y,
            None => return,
        };
        let rx = match Self::parse_coord(element.attr("rx").map(|s| s.as_ref()).unwrap_or("0")) {
            Some(r) => r,
            None => return,
        };
        let ry = match Self::parse_coord(element.attr("ry").map(|s| s.as_ref()).unwrap_or("0")) {
            Some(r) => r,
            None => return,
        };

        // Build path data using two arc commands
        let path_data = format!(
            "M{} {}A{} {} 0 1 0 {} {}A{} {} 0 1 0 {} {}z",
            self.format_number(cx),
            self.format_number(cy - ry),
            self.format_number(rx),
            self.format_number(ry),
            self.format_number(cx),
            self.format_number(cy + ry),
            self.format_number(rx),
            self.format_number(ry),
            self.format_number(cx),
            self.format_number(cy - ry)
        );

        // Update element
        element.name = "path".to_string().into();
        element.set_attr("d", &path_data);
        element.remove_attr("cx");
        element.remove_attr("cy");
        element.remove_attr("rx");
        element.remove_attr("ry");
    }

    /// Format a number with optional precision
    fn format_number(&self, value: f64) -> String {
        match self.config.float_precision {
            Some(p) => {
                let formatted = format!("{:.1$}", value, p as usize);
                // Remove trailing zeros and decimal point if integer
                let trimmed = formatted.trim_end_matches('0').trim_end_matches('.');
                if trimmed.is_empty() || trimmed == "-" {
                    "0".to_string()
                } else {
                    trimmed.to_string()
                }
            }
            None => {
                // Default formatting - remove .0 from integers
                if value.fract() == 0.0 {
                    format!("{}", value as i64)
                } else {
                    value.to_string()
                }
            }
        }
    }
}

impl Default for ConvertShapeToPathPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for ConvertShapeToPathPlugin {
    fn name(&self) -> &'static str {
        "convertShapeToPath"
    }

    fn description(&self) -> &'static str {
        "converts basic shapes to more compact path form"
    }

    fn validate_params(&self, params: &Value) -> Result<()> {
        Self::parse_config(params)?;
        Ok(())
    }

    fn apply(&self, document: &mut Document) -> Result<()> {
        self.convert_shapes_in_element(&mut document.root);

        // Clean up any elements marked for removal (empty name)
        Self::remove_empty_elements(&mut document.root);

        Ok(())
    }
}

impl ConvertShapeToPathPlugin {
    /// Remove elements that were marked for deletion (empty name)
    fn remove_empty_elements(element: &mut Element) {
        element.children.retain(|child| {
            if let Node::Element(elem) = child {
                !elem.name.is_empty()
            } else {
                true
            }
        });

        // Recursively clean children
        for child in &mut element.children {
            if let Node::Element(child_elem) = child {
                Self::remove_empty_elements(child_elem);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use vexy_svgo_core::ast::Element;

    fn create_element(name: &str, attrs: Vec<(&str, &str)>) -> Element<'static> {
        let mut element = Element::new(name.to_string());
        for (key, value) in attrs {
            element.set_attr(key, value);
        }
        element
    }

    #[test]
    fn test_plugin_info() {
        let plugin = ConvertShapeToPathPlugin::new();
        assert_eq!(plugin.name(), "convertShapeToPath");
        assert_eq!(
            plugin.description(),
            "converts basic shapes to more compact path form"
        );
    }

    #[test]
    fn test_param_validation() {
        let plugin = ConvertShapeToPathPlugin::new();

        // Test null params
        assert!(plugin.validate_params(&Value::Null).is_ok());

        // Test empty object
        assert!(plugin.validate_params(&serde_json::json!({})).is_ok());

        // Test valid params
        assert!(plugin
            .validate_params(&serde_json::json!({
                "convertArcs": true,
                "floatPrecision": 3
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
    fn test_convert_rect_basic() {
        let mut element = create_element("rect", vec![("width", "32"), ("height", "32")]);
        let plugin = ConvertShapeToPathPlugin::new();

        plugin.convert_rect(&mut element);

        assert_eq!(element.name, "path");
        assert_eq!(element.attr("d").unwrap(), "M0 0H32V32H0z");
        assert!(!element.has_attr("width"));
        assert!(!element.has_attr("height"));
    }

    #[test]
    fn test_convert_rect_with_position() {
        let mut element = create_element(
            "rect",
            vec![("x", "20"), ("y", "10"), ("width", "50"), ("height", "40")],
        );
        let plugin = ConvertShapeToPathPlugin::new();

        plugin.convert_rect(&mut element);

        assert_eq!(element.name, "path");
        assert_eq!(element.attr("d").unwrap(), "M20 10H70V50H20z");
    }

    #[test]
    fn test_rect_with_rounded_corners_not_converted() {
        let mut element = create_element(
            "rect",
            vec![
                ("x", "10"),
                ("y", "10"),
                ("width", "50"),
                ("height", "50"),
                ("rx", "4"),
            ],
        );
        let plugin = ConvertShapeToPathPlugin::new();

        plugin.convert_rect(&mut element);

        // Should not be converted
        assert_eq!(element.name, "rect");
        assert!(element.has_attr("rx"));
    }

    #[test]
    fn test_convert_line() {
        let mut element = create_element(
            "line",
            vec![("x1", "10"), ("y1", "10"), ("x2", "50"), ("y2", "20")],
        );
        let plugin = ConvertShapeToPathPlugin::new();

        plugin.convert_line(&mut element);

        assert_eq!(element.name, "path");
        assert_eq!(element.attr("d").unwrap(), "M10 10 50 20");
        assert!(!element.has_attr("x1"));
        assert!(!element.has_attr("y1"));
        assert!(!element.has_attr("x2"));
        assert!(!element.has_attr("y2"));
    }

    #[test]
    fn test_convert_polyline() {
        let mut element = create_element("polyline", vec![("points", "10,80 20,50 50,20 80,10")]);
        let plugin = ConvertShapeToPathPlugin::new();

        plugin.convert_polyline(&mut element);

        assert_eq!(element.name, "path");
        assert_eq!(element.attr("d").unwrap(), "M10 80 20 50 50 20 80 10");
        assert!(!element.has_attr("points"));
    }

    #[test]
    fn test_convert_polygon() {
        let mut element = create_element("polygon", vec![("points", "20 10 50 40 30 20")]);
        let plugin = ConvertShapeToPathPlugin::new();

        plugin.convert_polygon(&mut element);

        assert_eq!(element.name, "path");
        assert_eq!(element.attr("d").unwrap(), "M20 10 50 40 30 20z");
        assert!(!element.has_attr("points"));
    }

    #[test]
    fn test_convert_circle() {
        let mut element = create_element("circle", vec![("cx", "50"), ("cy", "50"), ("r", "25")]);
        let config = ConvertShapeToPathConfig {
            convert_arcs: true,
            float_precision: None,
        };
        let plugin = ConvertShapeToPathPlugin::with_config(config);

        plugin.convert_circle(&mut element);

        assert_eq!(element.name, "path");
        assert_eq!(
            element.attr("d").unwrap(),
            "M50 25A25 25 0 1 0 50 75A25 25 0 1 0 50 25z"
        );
        assert!(!element.has_attr("cx"));
        assert!(!element.has_attr("cy"));
        assert!(!element.has_attr("r"));
    }

    #[test]
    fn test_precision_formatting() {
        let config = ConvertShapeToPathConfig {
            convert_arcs: false,
            float_precision: Some(3),
        };
        let plugin = ConvertShapeToPathPlugin::with_config(config);

        assert_eq!(plugin.format_number(10.123456), "10.123");
        assert_eq!(plugin.format_number(20.987654), "20.988");
        assert_eq!(plugin.format_number(30.0), "30");

        let plugin_no_precision = ConvertShapeToPathPlugin::new();
        assert_eq!(plugin_no_precision.format_number(40.5), "40.5");
        assert_eq!(plugin_no_precision.format_number(50.0), "50");
    }

    #[test]
    fn test_polyline_insufficient_points() {
        let mut element = create_element("polyline", vec![("points", "10 20")]);
        let plugin = ConvertShapeToPathPlugin::new();

        plugin.convert_poly(&mut element, false);

        // Should be marked for removal
        assert_eq!(element.name, "");
        assert!(element.attributes.is_empty());
    }

    #[test]
    fn test_skip_unit_values() {
        let mut element = create_element("rect", vec![("width", "100%"), ("height", "50")]);
        let plugin = ConvertShapeToPathPlugin::new();

        plugin.convert_rect(&mut element);

        // Should not be converted due to percentage width
        assert_eq!(element.name, "rect");
        assert!(element.has_attr("width"));
    }
}

// Uncomment when ready to enable fixture tests
// crate::plugin_fixture_tests!(ConvertShapeToPathPlugin, "convertShapeToPath");
