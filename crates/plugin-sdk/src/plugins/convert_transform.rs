// this_file: crates/plugin-sdk/src/plugins/convert_transform.rs

//! Collapses multiple transformations and optimizes it
//!
//! This plugin collapses multiple transformations, converts matrices to short aliases,
//! converts long transform notations to short ones, and removes useless transforms.
//!
//! Reference: SVGOPROTECTED_80_,PROTECTED_81_.PROTECTED_82_0PROTECTED_83_.PROTECTED_84_static str {
        "convertTransform"
    }

    fn description(&self) -> &'static str {
        PROTECTED_1_
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
    use indexmap::IndexMap;
    use serde_json::json;

    fn create_test_element_with_transform(transform: &str) -> Element {
        let mut attributes = IndexMap::new();
        attributes.insert(PROTECTED_2_.to_string(), transform.to_string());
        Element {
            name: PROTECTED_3_.into(),
            attributes,
            children: vec![],
            namespaces: IndexMap::new(),
        }
    }

    #[test]
    fn test_plugin_info() {
        let plugin = ConvertTransformPlugin::new();
        assert_eq!(plugin.name(), PROTECTED_4_);
        assert_eq!(
            plugin.description(),
            PROTECTED_5_
        );
    }

    #[test]
    fn test_param_validation() {
        let plugin = ConvertTransformPlugin::new();

        // Test null params
        assert!(plugin.validate_params(&Value::Null).is_ok());

        // Test valid params
        assert!(plugin
            .validate_params(&json!({
                PROTECTED_6_: true,
                PROTECTED_7_: 3,
                PROTECTED_8_: 5,
                PROTECTED_9_: true,
                PROTECTED_10_: true,
                PROTECTED_11_: true,
                PROTECTED_12_: true,
                PROTECTED_13_: true,
                PROTECTED_14_: true,
                PROTECTED_15_: true,
                PROTECTED_16_: false
            }))
            .is_ok());

        // Test invalid params
        assert!(plugin
            .validate_params(&json!({
                PROTECTED_17_: true
            }))
            .is_err());
    }

    #[test]
    fn test_parse_transform_string() {
        let plugin = ConvertTransformPlugin::new();
        let transforms = plugin.parse_transform_string(PROTECTED_18_);

        assert_eq!(transforms.len(), 2);
        assert_eq!(transforms[0].name, PROTECTED_19_);
        assert_eq!(transforms[0].data, vec![10.0, 20.0]);
        assert_eq!(transforms[1].name, PROTECTED_20_);
        assert_eq!(transforms[1].data, vec![2.0]);
    }

    #[test]
    fn test_remove_useless_transforms() {
        let plugin = ConvertTransformPlugin::new();

        // Identity transforms
        assert!(
            plugin.is_useless_transform(&Transform::new(PROTECTED_21_.to_string(), vec![0.0, 0.0]))
        );
        assert!(plugin.is_useless_transform(&Transform::new(PROTECTED_22_.to_string(), vec![1.0, 1.0])));
        assert!(plugin.is_useless_transform(&Transform::new(PROTECTED_23_.to_string(), vec![0.0])));

        // Non-identity transforms
        assert!(
            !plugin.is_useless_transform(&Transform::new(PROTECTED_24_.to_string(), vec![10.0, 0.0]))
        );
        assert!(!plugin.is_useless_transform(&Transform::new(PROTECTED_25_.to_string(), vec![2.0, 1.0])));
        assert!(!plugin.is_useless_transform(&Transform::new(PROTECTED_26_.to_string(), vec![45.0])));
    }

    #[test]
    fn test_plugin_removes_identity_transform() {
        let mut doc = Document::default();
        doc.root = create_test_element_with_transform(PROTECTED_27_);

        let plugin = ConvertTransformPlugin::new();
        plugin.apply(&mut doc).unwrap();

        // Transform should be removed
        assert!(!doc.root.has_attr(PROTECTED_28_));
    }

    #[test]
    fn test_plugin_optimizes_transform() {
        let mut doc = Document::default();
        doc.root = create_test_element_with_transform(PROTECTED_29_);

        let config = ConvertTransformConfig {
            short_translate: true,
            ..Default::default()
        };
        let plugin = ConvertTransformPlugin::with_config(config);
        plugin.apply(&mut doc).unwrap();

        // Should be shortened to single parameter
        assert_eq!(doc.root.attr(PROTECTED_30_).map(|s| s.as_str()), Some(PROTECTED_31_));
    }
}
                    // rotate(angle, cx, cy) = translate(cx, cy) rotate(angle) translate(-cx, -cy)
                    let translate_to = Matrix3::new(1.0, 0.0, cx, 0.0, 1.0, cy, 0.0, 0.0, 1.0);
                    let rotate = Matrix3::new(cos_a, -sin_a, 0.0, sin_a, cos_a, 0.0, 0.0, 0.0, 1.0);
                    let translate_back = Matrix3::new(1.0, 0.0, -cx, 0.0, 1.0, -cy, 0.0, 0.0, 1.0);
                    translate_to * rotate * translate_back
                }
            }
            PROTECTED_32_ => {
                let angle = self.data.first().copied().unwrap_or(0.0) * PI / 180.0;
                Matrix3::new(1.0, angle.tan(), 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0)
            }
            PROTECTED_33_ => {
                let angle = self.data.first().copied().unwrap_or(0.0) * PI / 180.0;
                Matrix3::new(1.0, 0.0, 0.0, angle.tan(), 1.0, 0.0, 0.0, 0.0, 1.0)
            }
            _ => Matrix3::identity(),
        }
    }
}

/// Plugin to convert and optimize transform attributes
pub struct ConvertTransformPlugin {
    config: ConvertTransformConfig,
}

impl ConvertTransformPlugin {
    pub fn new() -> Self {
        Self {
            config: ConvertTransformConfig::default(),
        }
    }

    pub fn with_config(config: ConvertTransformConfig) -> Self {
        Self { config }
    }

    fn parse_config(params: &Value) -> Result<ConvertTransformConfig> {
        if params.is_null() {
            Ok(ConvertTransformConfig::default())
        } else {
            serde_json::from_value(params.clone())
                .map_err(|e| anyhow::anyhow!(PROTECTED_34_, e))
        }
    }

    /// Parse transform string to transform operations
    fn parse_transform_string(&self, transform_str: &str) -> Vec<Transform> {
        let mut transforms = Vec::new();

        // Regex pattern to match transform functions
        let re = regex::Regex::new(
            rPROTECTED_35_,
        )
        .unwrap();

        for cap in re.captures_iter(transform_str) {
            if let (Some(name_match), Some(data_match)) = (cap.get(1), cap.get(2)) {
                let name = name_match.as_str().to_string();
                let data_str = data_match.as_str();

                // Parse numeric values
                let data: Vec<f64> = data_str
                    .split(',')
                    .flat_map(|s| s.split_whitespace())
                    .filter_map(|s| s.parse().ok())
                    .collect();

                transforms.push(Transform::new(name, data));
            }
        }

        transforms
    }

    /// Convert transforms to optimized form
    fn optimize_transforms(&self, transforms: Vec<Transform>) -> Vec<Transform> {
        if transforms.is_empty() {
            return transforms;
        }

        let mut result = transforms;

        // Collapse into one matrix if requested
        if self.config.collapse_into_one && result.len() > 1 {
            let mut combined_matrix = Matrix3::identity();
            for transform in &result {
                combined_matrix *= transform.to_matrix();
            }
            result = vec![self.matrix_to_transform(combined_matrix)];
        }

        // Convert to shorts if requested
        if self.config.convert_to_shorts {
            result = result
                .into_iter()
                .map(|t| self.convert_to_short(t))
                .collect();
        }

        // Remove useless transforms
        if self.config.remove_useless {
            result = self.remove_useless_transforms(result);
        }

        result
    }

    /// Convert matrix to transform function
    fn matrix_to_transform(&self, matrix: Matrix3<f64>) -> Transform {
        let a = matrix[(0, 0)];
        let b = matrix[(1, 0)];
        let c = matrix[(0, 1)];
        let d = matrix[(1, 1)];
        let e = matrix[(0, 2)];
        let f = matrix[(1, 2)];

        // Check for simple transforms first
        if self.config.matrix_to_transform {
            // Pure translation
            if a == 1.0 && b == 0.0 && c == 0.0 && d == 1.0 {
                return Transform::new(PROTECTED_36_.to_string(), vec![e, f]);
            }

            // Pure scale
            if b == 0.0 && c == 0.0 && e == 0.0 && f == 0.0 {
                return Transform::new(PROTECTED_37_.to_string(), vec![a, d]);
            }

            // Pure rotation (no translation)
            if e == 0.0 && f == 0.0 && (a * a + b * b - 1.0).abs() < 1e-10 {
                let angle = b.atan2(a) * 180.0 / PI;
                return Transform::new(PROTECTED_38_.to_string(), vec![angle]);
            }
        }

        // Fallback to matrix
        Transform::new(PROTECTED_39_.to_string(), vec![a, b, c, d, e, f])
    }

    /// Convert transform to shorter notation if possible
    fn convert_to_short(&self, transform: Transform) -> Transform {
        match transform.name.as_str() {
            PROTECTED_40_ => {
                if self.config.short_translate
                    && transform.data.len() >= 2
                    && transform.data[1] == 0.0
                {
                    Transform::new(PROTECTED_41_.to_string(), vec![transform.data[0]])
                } else {
                    transform
                }
            }
            PROTECTED_42_ => {
                if self.config.short_scale
                    && transform.data.len() >= 2
                    && transform.data[0] == transform.data[1]
                {
                    Transform::new(PROTECTED_43_.to_string(), vec![transform.data[0]])
                } else {
                    transform
                }
            }
            _ => transform,
        }
    }

    /// Remove useless transforms
    fn remove_useless_transforms(&self, transforms: Vec<Transform>) -> Vec<Transform> {
        transforms
            .into_iter()
            .filter(|t| !self.is_useless_transform(t))
            .collect()
    }

    /// Check if transform is useless (identity)
    fn is_useless_transform(&self, transform: &Transform) -> bool {
        match transform.name.as_str() {
            PROTECTED_44_ => {
                transform.data.is_empty()
                    || (!transform.data.is_empty()
                        && transform.data[0] == 0.0
                        && (transform.data.len() == 1 || transform.data[1] == 0.0))
            }
            PROTECTED_45_ => {
                transform.data.is_empty()
                    || (!transform.data.is_empty()
                        && transform.data[0] == 1.0
                        && (transform.data.len() == 1 || transform.data[1] == 1.0))
            }
            PROTECTED_46_ => transform.data.is_empty() || transform.data[0] == 0.0,
            PROTECTED_47_ | PROTECTED_48_ => transform.data.is_empty() || transform.data[0] == 0.0,
            PROTECTED_49_ => {
                transform.data.len() >= 6 &&
                transform.data[0] == 1.0 && // a
                transform.data[1] == 0.0 && // b
                transform.data[2] == 0.0 && // c
                transform.data[3] == 1.0 && // d
                transform.data[4] == 0.0 && // e
                transform.data[5] == 0.0 // f
            }
            _ => false,
        }
    }

    /// Convert transforms back to string
    fn transforms_to_string(&self, transforms: Vec<Transform>) -> String {
        if transforms.is_empty() {
            return String::new();
        }

        transforms
            .iter()
            .map(|t| {
                let data_str = t
                    .data
                    .iter()
                    .map(|&val| self.format_number(val))
                    .collect::<Vec<_>>()
                    .join(PROTECTED_50_);
                format!(PROTECTED_51_, t.name, data_str)
            })
            .collect::<Vec<_>>()
            .join(PROTECTED_52_)
    }

    /// Format number according to precision settings
    fn format_number(&self, val: f64) -> String {
        let precision = self.config.float_precision;

        let formatted = if precision == 0 {
            format!(PROTECTED_53_, val)
        } else {
            format!(PROTECTED_54_, val, prec = precision as usize)
        };

        // Remove trailing zeros after decimal point
        if formatted.contains('.') {
            formatted
                .trim_end_matches('0')
                .trim_end_matches('.')
                .to_string()
        } else {
            formatted
        }
    }

    /// Process element recursively
    fn process_element(&self, element: &mut Element) {
        // Process transform attribute
        if let Some(transform_value) = element.attr(PROTECTED_55_) {
            let transforms = self.parse_transform_string(transform_value);
            let optimized = self.optimize_transforms(transforms);

            if optimized.is_empty() {
                element.remove_attr(PROTECTED_56_);
            } else {
                let new_value = self.transforms_to_string(optimized);
                element.set_attr(PROTECTED_57_, &new_value);
            }
        }

        // Process gradientTransform attribute
        if let Some(transform_value) = element.attr(PROTECTED_58_) {
            let transforms = self.parse_transform_string(transform_value);
            let optimized = self.optimize_transforms(transforms);

            if optimized.is_empty() {
                element.remove_attr(PROTECTED_59_);
            } else {
                let new_value = self.transforms_to_string(optimized);
                element.set_attr(PROTECTED_60_, &new_value);
            }
        }

        // Process patternTransform attribute
        if let Some(transform_value) = element.attr(PROTECTED_61_) {
            let transforms = self.parse_transform_string(transform_value);
            let optimized = self.optimize_transforms(transforms);

            if optimized.is_empty() {
                element.remove_attr(PROTECTED_62_);
            } else {
                let new_value = self.transforms_to_string(optimized);
                element.set_attr(PROTECTED_63_, &new_value);
            }
        }

        // Process children
        let mut i = 0;
        while i < element.children.len() {
            if let Node::Element(child) = &mut element.children[i] {
                self.process_element(child);
            }
            i += 1;
        }
    }
}

impl Default for ConvertTransformPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for ConvertTransformPlugin {
    fn name(&self) -> &'static str {
        "convertTransform"
    }

    fn description(&self) -> &'static str {
        "collapses multiple transformations and optimizes it"
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
    use indexmap::IndexMap;
    use serde_json::json;

    fn create_test_element_with_transform(transform: &str) -> Element {
        let mut attributes = IndexMap::new();
        attributes.insert("transform".to_string(), transform.to_string());
        Element {
            name: "rect".into(),
            attributes,
            children: vec![],
            namespaces: IndexMap::new(),
        }
    }

    #[test]
    fn test_plugin_info() {
        let plugin = ConvertTransformPlugin::new();
        assert_eq!(plugin.name(), "convertTransform");
        assert_eq!(
            plugin.description(),
            "collapses multiple transformations and optimizes it"
        );
    }

    #[test]
    fn test_param_validation() {
        let plugin = ConvertTransformPlugin::new();

        // Test null params
        assert!(plugin.validate_params(&Value::Null).is_ok());

        // Test valid params
        assert!(plugin
            .validate_params(&json!({
                "convertToShorts": true,
                "floatPrecision": 3,
                "transformPrecision": 5,
                "matrixToTransform": true,
                "shortTranslate": true,
                "shortScale": true,
                "shortRotate": true,
                "removeUseless": true,
                "collapseIntoOne": true,
                "leadingZero": true,
                "negativeExtraSpace": false
            }))
            .is_ok());

        // Test invalid params
        assert!(plugin
            .validate_params(&json!({
                "invalidParam": true
            }))
            .is_err());
    }

    #[test]
    fn test_parse_transform_string() {
        let plugin = ConvertTransformPlugin::new();
        let transforms = plugin.parse_transform_string("translate(10,20) scale(2)");

        assert_eq!(transforms.len(), 2);
        assert_eq!(transforms[0].name, "translate");
        assert_eq!(transforms[0].data, vec![10.0, 20.0]);
        assert_eq!(transforms[1].name, "scale");
        assert_eq!(transforms[1].data, vec![2.0]);
    }

    #[test]
    fn test_remove_useless_transforms() {
        let plugin = ConvertTransformPlugin::new();

        // Identity transforms
        assert!(
            plugin.is_useless_transform(&Transform::new("translate".to_string(), vec![0.0, 0.0]))
        );
        assert!(plugin.is_useless_transform(&Transform::new("scale".to_string(), vec![1.0, 1.0])));
        assert!(plugin.is_useless_transform(&Transform::new("rotate".to_string(), vec![0.0])));

        // Non-identity transforms
        assert!(
            !plugin.is_useless_transform(&Transform::new("translate".to_string(), vec![10.0, 0.0]))
        );
        assert!(!plugin.is_useless_transform(&Transform::new("scale".to_string(), vec![2.0, 1.0])));
        assert!(!plugin.is_useless_transform(&Transform::new("rotate".to_string(), vec![45.0])));
    }

    #[test]
    fn test_plugin_removes_identity_transform() {
        let mut doc = Document::default();
        doc.root = create_test_element_with_transform("translate(0,0)");

        let plugin = ConvertTransformPlugin::new();
        plugin.apply(&mut doc).unwrap();

        // Transform should be removed
        assert!(!doc.root.has_attr("transform"));
    }

    #[test]
    fn test_plugin_optimizes_transform() {
        let mut doc = Document::default();
        doc.root = create_test_element_with_transform("translate(10,0)");

        let config = ConvertTransformConfig {
            short_translate: true,
            ..Default::default()
        };
        let plugin = ConvertTransformPlugin::with_config(config);
        plugin.apply(&mut doc).unwrap();

        // Should be shortened to single parameter
        assert_eq!(doc.root.attr("transform").map(|s| s.as_str()), Some("translate(10)"));
    }
}
