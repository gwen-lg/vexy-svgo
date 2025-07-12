// this_file: crates/plugin-sdk/src/plugins/apply_transforms.rs

//! Apply transform matrices to path data
//!
//! This plugin applies transform matrices directly to path coordinates,
//! eliminating the need for transform attributes on elements.
//!
//! This is particularly useful for:
//! - Reducing file size by removing transform attributes
//! - Simplifying path data for further optimization
//! - Improving rendering performance
//!
//! Based on SVGO's applyTransforms plugin

use crate::Plugin;
use anyhow::Result;
use nalgebra::{Matrix3, Point2};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use vexy_svgo_core::ast::{Document, Element, Node};

/// Configuration for the applyTransforms plugin
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ApplyTransformsConfig {
    /// Apply transforms to stroked paths (may change stroke width)
    #[serde(default = "default_true")]
    pub apply_to_stroked: bool,

    /// Decimal precision for transformed coordinates
    #[serde(default = "default_float_precision")]
    pub float_precision: u8,
}

impl Default for ApplyTransformsConfig {
    fn default() -> Self {
        Self {
            apply_to_stroked: true,
            float_precision: 3,
        }
    }
}

fn default_true() -> bool {
    true
}

fn default_float_precision() -> u8 {
    3
}

pub struct ApplyTransformsPlugin {
    config: ApplyTransformsConfig,
}

impl Default for ApplyTransformsPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl ApplyTransformsPlugin {
    pub fn new() -> Self {
        Self {
            config: ApplyTransformsConfig::default(),
        }
    }

    pub fn with_config(config: ApplyTransformsConfig) -> Self {
        Self { config }
    }

    fn parse_config(params: &Value) -> Result<ApplyTransformsConfig> {
        if params.is_null() {
            Ok(ApplyTransformsConfig::default())
        } else {
            serde_json::from_value(params.clone())
                .map_err(|e| anyhow::anyhow!("Invalid configuration: {}", e))
        }
    }

    /// Parse a transform attribute into a transformation matrix
    fn parse_transform(transform: &str) -> Option<Matrix3<f64>> {
        let mut matrix = Matrix3::identity();
        
        // Simple parser for transform functions
        // This is a basic implementation - a full implementation would handle all transform types
        let transforms = transform.trim().split(')').filter(|s| !s.is_empty());
        
        for transform_fn in transforms {
            let transform_fn = transform_fn.trim();
            if transform_fn.is_empty() {
                continue;
            }
            
            if let Some(params_start) = transform_fn.find('(') {
                let fn_name = transform_fn[..params_start].trim();
                let params_str = &transform_fn[params_start + 1..];
                let params: Vec<f64> = params_str
                    .split(|c: char| c == ',' || c.is_whitespace())
                    .filter(|s| !s.is_empty())
                    .filter_map(|s| s.trim().parse().ok())
                    .collect();
                
                match fn_name {
                    "translate" => {
                        if params.len() >= 1 {
                            let tx = params[0];
                            let ty = params.get(1).copied().unwrap_or(0.0);
                            let translate = Matrix3::new(
                                1.0, 0.0, tx,
                                0.0, 1.0, ty,
                                0.0, 0.0, 1.0
                            );
                            matrix = translate * matrix;
                        }
                    }
                    "scale" => {
                        if params.len() >= 1 {
                            let sx = params[0];
                            let sy = params.get(1).copied().unwrap_or(sx);
                            let scale = Matrix3::new(
                                sx, 0.0, 0.0,
                                0.0, sy, 0.0,
                                0.0, 0.0, 1.0
                            );
                            matrix = scale * matrix;
                        }
                    }
                    "rotate" => {
                        if params.len() >= 1 {
                            let angle = params[0].to_radians();
                            let cos_a = angle.cos();
                            let sin_a = angle.sin();
                            
                            if params.len() >= 3 {
                                // Rotation around a point
                                let cx = params[1];
                                let cy = params[2];
                                let t1 = Matrix3::new(
                                    1.0, 0.0, cx,
                                    0.0, 1.0, cy,
                                    0.0, 0.0, 1.0
                                );
                                let r = Matrix3::new(
                                    cos_a, -sin_a, 0.0,
                                    sin_a, cos_a, 0.0,
                                    0.0, 0.0, 1.0
                                );
                                let t2 = Matrix3::new(
                                    1.0, 0.0, -cx,
                                    0.0, 1.0, -cy,
                                    0.0, 0.0, 1.0
                                );
                                matrix = t1 * r * t2 * matrix;
                            } else {
                                let rotate = Matrix3::new(
                                    cos_a, -sin_a, 0.0,
                                    sin_a, cos_a, 0.0,
                                    0.0, 0.0, 1.0
                                );
                                matrix = rotate * matrix;
                            }
                        }
                    }
                    "matrix" => {
                        if params.len() >= 6 {
                            let custom = Matrix3::new(
                                params[0], params[2], params[4],
                                params[1], params[3], params[5],
                                0.0, 0.0, 1.0
                            );
                            matrix = custom * matrix;
                        }
                    }
                    _ => {} // Skip unknown transform functions
                }
            }
        }
        
        Some(matrix)
    }

    /// Apply transforms to the document recursively
    fn apply_transforms_recursive(&self, element: &mut Element) {
        // Check if element has both transform and is a path
        if element.name == "path" {
            if let Some(transform) = element.attributes.get("transform") {
                // Check if we should skip stroked paths
                let has_stroke = element.attributes.get("stroke")
                    .map(|s| s != "none")
                    .unwrap_or(false);
                
                if !has_stroke || self.config.apply_to_stroked {
                    // Parse transform matrix
                    if let Some(matrix) = Self::parse_transform(transform) {
                        // Apply to path data
                        if let Some(d) = element.attributes.get("d") {
                            let transformed = self.transform_path_data(d, &matrix);
                            element.attributes.insert("d".to_string(), transformed);
                            // Remove transform attribute after applying
                            element.attributes.shift_remove("transform");
                        }
                    }
                }
            }
        }
        
        // Process child elements recursively
        for child in &mut element.children {
            if let Node::Element(child_element) = child {
                self.apply_transforms_recursive(child_element);
            }
        }
    }

    /// Apply transformation matrix to path data
    fn transform_path_data(&self, path_data: &str, matrix: &Matrix3<f64>) -> String {
        let mut result = String::new();
        let mut chars = path_data.chars().peekable();
        let mut current_x = 0.0;
        let mut current_y = 0.0;
        
        while let Some(ch) = chars.next() {
            if ch.is_alphabetic() {
                result.push(ch);
                result.push(' ');
                
                // Parse coordinates based on command type
                match ch {
                    'M' | 'L' => {
                        // Absolute moveto/lineto
                        if let (Some(x), Some(y)) = (self.parse_coord(&mut chars), self.parse_coord(&mut chars)) {
                            let point = Point2::new(x, y);
                            let transformed = matrix.transform_point(&point);
                            result.push_str(&format!("{} {} ", 
                                self.format_number(transformed.x),
                                self.format_number(transformed.y)
                            ));
                            current_x = transformed.x;
                            current_y = transformed.y;
                        }
                    }
                    'm' | 'l' => {
                        // Relative moveto/lineto
                        if let (Some(dx), Some(dy)) = (self.parse_coord(&mut chars), self.parse_coord(&mut chars)) {
                            // For relative commands, we need to transform the delta
                            let start = Point2::new(current_x, current_y);
                            let end = Point2::new(current_x + dx, current_y + dy);
                            let transformed_start = matrix.transform_point(&start);
                            let transformed_end = matrix.transform_point(&end);
                            let new_dx = transformed_end.x - transformed_start.x;
                            let new_dy = transformed_end.y - transformed_start.y;
                            result.push_str(&format!("{} {} ", 
                                self.format_number(new_dx),
                                self.format_number(new_dy)
                            ));
                            current_x = transformed_end.x;
                            current_y = transformed_end.y;
                        }
                    }
                    'H' => {
                        // Absolute horizontal lineto
                        if let Some(x) = self.parse_coord(&mut chars) {
                            let point = Point2::new(x, current_y);
                            let transformed = matrix.transform_point(&point);
                            // After transformation, H command might need to become L
                            if (transformed.y - current_y).abs() > 1e-6 {
                                // Convert to L command
                                result.pop(); // Remove 'H'
                                result.pop(); // Remove space
                                result.push_str(&format!("L {} {} ", 
                                    self.format_number(transformed.x),
                                    self.format_number(transformed.y)
                                ));
                            } else {
                                result.push_str(&format!("{} ", self.format_number(transformed.x)));
                            }
                            current_x = transformed.x;
                            current_y = transformed.y;
                        }
                    }
                    'V' => {
                        // Absolute vertical lineto
                        if let Some(y) = self.parse_coord(&mut chars) {
                            let point = Point2::new(current_x, y);
                            let transformed = matrix.transform_point(&point);
                            // After transformation, V command might need to become L
                            if (transformed.x - current_x).abs() > 1e-6 {
                                // Convert to L command
                                result.pop(); // Remove 'V'
                                result.pop(); // Remove space
                                result.push_str(&format!("L {} {} ", 
                                    self.format_number(transformed.x),
                                    self.format_number(transformed.y)
                                ));
                            } else {
                                result.push_str(&format!("{} ", self.format_number(transformed.y)));
                            }
                            current_x = transformed.x;
                            current_y = transformed.y;
                        }
                    }
                    'C' => {
                        // Absolute cubic bezier
                        for _ in 0..3 {
                            if let (Some(x), Some(y)) = (self.parse_coord(&mut chars), self.parse_coord(&mut chars)) {
                                let point = Point2::new(x, y);
                                let transformed = matrix.transform_point(&point);
                                result.push_str(&format!("{} {} ", 
                                    self.format_number(transformed.x),
                                    self.format_number(transformed.y)
                                ));
                                current_x = transformed.x;
                                current_y = transformed.y;
                            }
                        }
                    }
                    'Z' | 'z' => {
                        // Close path - no coordinates to transform
                    }
                    _ => {
                        // For other commands, just copy coordinates as-is for now
                        // A complete implementation would handle all SVG path commands
                        while let Some(next_ch) = chars.peek() {
                            if next_ch.is_alphabetic() {
                                break;
                            }
                            result.push(chars.next().unwrap());
                        }
                    }
                }
            }
        }
        
        result.trim().to_string()
    }

    /// Parse a coordinate from the character stream
    fn parse_coord(&self, chars: &mut std::iter::Peekable<std::str::Chars>) -> Option<f64> {
        let mut coord = String::new();
        let mut has_dot = false;
        
        // Skip whitespace and commas
        while let Some(&ch) = chars.peek() {
            if ch.is_whitespace() || ch == ',' {
                chars.next();
            } else {
                break;
            }
        }
        
        // Handle sign
        if let Some(&ch) = chars.peek() {
            if ch == '-' || ch == '+' {
                coord.push(chars.next().unwrap());
            }
        }
        
        // Parse number
        while let Some(&ch) = chars.peek() {
            if ch.is_numeric() {
                coord.push(chars.next().unwrap());
            } else if ch == '.' && !has_dot {
                coord.push(chars.next().unwrap());
                has_dot = true;
            } else {
                break;
            }
        }
        
        coord.parse().ok()
    }

    /// Format a number with the configured precision
    fn format_number(&self, num: f64) -> String {
        format!("{:.prec$}", num, prec = self.config.float_precision as usize)
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_string()
    }
}

impl Plugin for ApplyTransformsPlugin {
    fn name(&self) -> &'static str {
        "applyTransforms"
    }

    fn description(&self) -> &'static str {
        "Apply transform matrices to path data"
    }

    fn apply(&self, document: &mut Document) -> Result<()> {
        self.apply_transforms_recursive(&mut document.root);
        Ok(())
    }

    fn validate_params(&self, params: &Value) -> Result<()> {
        Self::parse_config(params)?;
        Ok(())
    }
}

impl Clone for ApplyTransformsPlugin {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_transform_translate() {
        let transform = "translate(10, 20)";
        let matrix = ApplyTransformsPlugin::parse_transform(transform).unwrap();
        let point = Point2::new(0.0, 0.0);
        let transformed = matrix.transform_point(&point);
        assert_eq!(transformed.x, 10.0);
        assert_eq!(transformed.y, 20.0);
    }

    #[test]
    fn test_parse_transform_scale() {
        let transform = "scale(2)";
        let matrix = ApplyTransformsPlugin::parse_transform(transform).unwrap();
        let point = Point2::new(5.0, 5.0);
        let transformed = matrix.transform_point(&point);
        assert_eq!(transformed.x, 10.0);
        assert_eq!(transformed.y, 10.0);
    }

    #[test]
    fn test_format_number() {
        let plugin = ApplyTransformsPlugin::new();
        assert_eq!(plugin.format_number(1.0), "1");
        assert_eq!(plugin.format_number(1.500), "1.5");
        assert_eq!(plugin.format_number(1.234567), "1.235");
    }
}