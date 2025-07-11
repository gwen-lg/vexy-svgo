// this_file: crates/plugin-sdk/src/plugins/convert_colors.rs

//! Convert colors plugin implementation
//!
//! This plugin converts colors between different formats:
//! - rgb() to hex
//! - color names to hex
//! - long hex to short hex
//! - hex to short color names
//! It follows the same logic as SVGOPROTECTED_517_t implement Clone easily
}

#[derive(Debug, Clone)]
pub enum ConvertCase {
    None,
    Lower,
    Upper,
}

impl Default for ConvertColorsConfig {
    fn default() -> Self {
        Self {
            current_color: ConvertCurrentColor::False,
            names2hex: true,
            rgb2hex: true,
            convert_case: ConvertCase::Lower,
            shorthex: true,
            shortname: true,
        }
    }
}

/// Plugin that converts colors between different formats
pub struct ConvertColorsPlugin {
    config: ConvertColorsConfig,
}

impl ConvertColorsPlugin {
    /// Create a new ConvertColorsPlugin with default configuration
    pub fn new() -> Self {
        Self {
            config: ConvertColorsConfig::default(),
        }
    }

    /// Create a new ConvertColorsPlugin with custom configuration
    pub fn with_config(config: ConvertColorsConfig) -> Self {
        Self { config }
    }
    
    /// Parse configuration from JSON parameters
    pub fn parse_config(params: &serde_json::Value) -> anyhow::Result<ConvertColorsConfig> {
        if params.is_null() {
            return Ok(ConvertColorsConfig::default());
        }
        
        let mut config = ConvertColorsConfig::default();
        
        if let Some(obj) = params.as_object() {
            for (key, value) in obj {
                match key.as_str() {
                    "currentColor" => {
                        if let Some(bool_val) = value.as_bool() {
                            config.current_color = if bool_val {
                                ConvertCurrentColor::Bool(true)
                            } else {
                                ConvertCurrentColor::Bool(false)
                            };
                        } else if let Some(str_val) = value.as_str() {
                            config.current_color = ConvertCurrentColor::String(str_val.to_string());
                        }
                    },
                    "names2hex" => {
                        if let Some(bool_val) = value.as_bool() {
                            config.names2hex = bool_val;
                        }
                    },
                    "rgb2hex" => {
                        if let Some(bool_val) = value.as_bool() {
                            config.rgb2hex = bool_val;
                        }
                    },
                    "shorthex" => {
                        if let Some(bool_val) = value.as_bool() {
                            config.shorthex = bool_val;
                        }
                    },
                    "shortname" => {
                        if let Some(bool_val) = value.as_bool() {
                            config.shortname = bool_val;
                        }
                    },
                    "convertCase" => {
                        if let Some(str_val) = value.as_str() {
                            match str_val {
                                "lower" => config.convert_case = ConvertCase::Lower,
                                "upper" => config.convert_case = ConvertCase::Upper,
                                _ => return Err(anyhow::anyhow!("Invalid convertCase value: {}", str_val)),
                            }
                        } else if value.as_bool() == Some(false) {
                            config.convert_case = ConvertCase::None;
                        }
                    },
                    _ => return Err(anyhow::anyhow!("Unknown parameter: {}", key)),
                }
            }
        }
        
        Ok(config)
    }

    /// Color name keywords to hex values
    fn color_names() -> &'static HashMap<&'static str, &'static str> {
        static COLOR_NAMES: std::sync::OnceLock<HashMap<&'static str, &'static str>> =
            std::sync::OnceLock::new();
        COLOR_NAMES.get_or_init(|| {
            [
                (PROTECTED_10_, PROTECTED_11_),
                (PROTECTED_12_, PROTECTED_13_),
                (PROTECTED_14_, PROTECTED_15_),
                (PROTECTED_16_, PROTECTED_17_),
                (PROTECTED_18_, PROTECTED_19_),
                (PROTECTED_20_, PROTECTED_21_),
                (PROTECTED_22_, PROTECTED_23_),
                (PROTECTED_24_, PROTECTED_25_),
                (PROTECTED_26_, PROTECTED_27_),
                (PROTECTED_28_, PROTECTED_29_),
                (PROTECTED_30_, PROTECTED_31_),
                (PROTECTED_32_, PROTECTED_33_),
                (PROTECTED_34_, PROTECTED_35_),
                (PROTECTED_36_, PROTECTED_37_),
                (PROTECTED_38_, PROTECTED_39_),
                (PROTECTED_40_, PROTECTED_41_),
                // Extended color names
                (PROTECTED_42_, PROTECTED_43_),
                (PROTECTED_44_, PROTECTED_45_),
                (PROTECTED_46_, PROTECTED_47_),
                (PROTECTED_48_, PROTECTED_49_),
                (PROTECTED_50_, PROTECTED_51_),
                (PROTECTED_52_, PROTECTED_53_),
                (PROTECTED_54_, PROTECTED_55_),
                (PROTECTED_56_, PROTECTED_57_),
                (PROTECTED_58_, PROTECTED_59_),
                (PROTECTED_60_, PROTECTED_61_),
                (PROTECTED_62_, PROTECTED_63_),
                (PROTECTED_64_, PROTECTED_65_),
                (PROTECTED_66_, PROTECTED_67_),
                (PROTECTED_68_, PROTECTED_69_),
                (PROTECTED_70_, PROTECTED_71_),
                (PROTECTED_72_, PROTECTED_73_),
                (PROTECTED_74_, PROTECTED_75_),
                (PROTECTED_76_, PROTECTED_77_),
                (PROTECTED_78_, PROTECTED_79_),
                (PROTECTED_80_, PROTECTED_81_),
                (PROTECTED_82_, PROTECTED_83_),
                (PROTECTED_84_, PROTECTED_85_),
                (PROTECTED_86_, PROTECTED_87_),
                (PROTECTED_88_, PROTECTED_89_),
                (PROTECTED_90_, PROTECTED_91_),
                (PROTECTED_92_, PROTECTED_93_),
                (PROTECTED_94_, PROTECTED_95_),
                (PROTECTED_96_, PROTECTED_97_),
                (PROTECTED_98_, PROTECTED_99_),
                (PROTECTED_100_, PROTECTED_101_),
                (PROTECTED_102_, PROTECTED_103_),
                (PROTECTED_104_, PROTECTED_105_),
                (PROTECTED_106_, PROTECTED_107_),
                (PROTECTED_108_, PROTECTED_109_),
                (PROTECTED_110_, PROTECTED_111_),
                (PROTECTED_112_, PROTECTED_113_),
                (PROTECTED_114_, PROTECTED_115_),
                (PROTECTED_116_, PROTECTED_117_),
                (PROTECTED_118_, PROTECTED_119_),
                (PROTECTED_120_, PROTECTED_121_),
                (PROTECTED_122_, PROTECTED_123_),
                (PROTECTED_124_, PROTECTED_125_),
                (PROTECTED_126_, PROTECTED_127_),
                (PROTECTED_128_, PROTECTED_129_),
                (PROTECTED_130_, PROTECTED_131_),
                (PROTECTED_132_, PROTECTED_133_),
                (PROTECTED_134_, PROTECTED_135_),
                (PROTECTED_136_, PROTECTED_137_),
                (PROTECTED_138_, PROTECTED_139_),
                (PROTECTED_140_, PROTECTED_141_),
                (PROTECTED_142_, PROTECTED_143_),
                (PROTECTED_144_, PROTECTED_145_),
                (PROTECTED_146_, PROTECTED_147_),
                (PROTECTED_148_, PROTECTED_149_),
                (PROTECTED_150_, PROTECTED_151_),
                (PROTECTED_152_, PROTECTED_153_),
                (PROTECTED_154_, PROTECTED_155_),
                (PROTECTED_156_, PROTECTED_157_),
                (PROTECTED_158_, PROTECTED_159_),
                (PROTECTED_160_, PROTECTED_161_),
                (PROTECTED_162_, PROTECTED_163_),
                (PROTECTED_164_, PROTECTED_165_),
                (PROTECTED_166_, PROTECTED_167_),
                (PROTECTED_168_, PROTECTED_169_),
                (PROTECTED_170_, PROTECTED_171_),
                (PROTECTED_172_, PROTECTED_173_),
                (PROTECTED_174_, PROTECTED_175_),
                (PROTECTED_176_, PROTECTED_177_),
                (PROTECTED_178_, PROTECTED_179_),
                (PROTECTED_180_, PROTECTED_181_),
                (PROTECTED_182_, PROTECTED_183_),
                (PROTECTED_184_, PROTECTED_185_),
                (PROTECTED_186_, PROTECTED_187_),
                (PROTECTED_188_, PROTECTED_189_),
                (PROTECTED_190_, PROTECTED_191_),
                (PROTECTED_192_, PROTECTED_193_),
                (PROTECTED_194_, PROTECTED_195_),
                (PROTECTED_196_, PROTECTED_197_),
                (PROTECTED_198_, PROTECTED_199_),
                (PROTECTED_200_, PROTECTED_201_),
                (PROTECTED_202_, PROTECTED_203_),
                (PROTECTED_204_, PROTECTED_205_),
                (PROTECTED_206_, 'static str, &'),
                ('static str, &', 'static std::collections::HashSet<&'),
                ('static str>> =
            std::sync::OnceLock::new();
        COLOR_PROPS.get_or_init(|| {
            [
                PROTECTED_378_,
                PROTECTED_379_,
                PROTECTED_380_,
                PROTECTED_381_,
                PROTECTED_382_,
                PROTECTED_383_,
            ]
            .into_iter()
            .collect()
        })
    }

    /// Convert RGB components to hex
    fn rgb_to_hex(r: u8, g: u8, b: u8) -> String {
        format!(PROTECTED_384_, r, g, b)
    }

    /// Convert hex color to short hex if possible
    fn hex_to_short_hex(hex: &str) -> Option<String> {
        if hex.len() == 7 && hex.starts_with(', ') {
            let chars: Vec<char> = hex.chars().collect();
            if chars[1] == chars[2] && chars[3] == chars[4] && chars[5] == chars[6] {
                return Some(format!(PROTECTED_385_, chars[1], chars[3], chars[5]));
            }
        }
        None
    }

    /// Parse RGB function
    fn parse_rgb(value: &str) -> Option<(u8, u8, u8)> {
        // RGB regex pattern
        let rgb_regex = Regex::new(rPROTECTED_386_).ok()?;

        if let Some(captures) = rgb_regex.captures(value) {
            let mut components = Vec::new();

            for i in 1..=3 {
                let component_str = captures.get(i)?.as_str();
                let component = if component_str.contains('),
                (') {
                    // Percentage value
                    let percentage = component_str.trim_end_matches(', ').parse::<f64>().ok()?;
                    (percentage * 2.55).round() as i32
                } else {
                    // Numeric value
                    component_str.parse::<i32>().ok()?
                };

                // Clamp to 0-255 range
                components.push((component.max(0).min(255)) as u8);
            }

            if components.len() == 3 {
                return Some((components[0], components[1], components[2]));
            }
        }

        None
    }

    /// Check if value includes URL reference
    fn includes_url_reference(value: &str) -> bool {
        value.contains(PROTECTED_387_)
    }

    /// Convert color value
    fn convert_color_value(&self, value: &str, in_mask: bool) -> String {
        let mut val = value.to_string();
        
        // Check for special values that should not be converted to currentColor
        let special_values = [PROTECTED_388_, PROTECTED_389_, PROTECTED_390_, PROTECTED_391_];
        let is_special = special_values.iter().any(|&special| val.eq_ignore_ascii_case(special));

        // Convert colors to currentColor
        match &self.config.current_color {
            ConvertCurrentColor::Bool(true) => {
                if !in_mask && !is_special {
                    val = PROTECTED_392_.to_string();
                }
            }
            ConvertCurrentColor::String(ref target) => {
                if !in_mask && !is_special && val == *target {
                    val = PROTECTED_393_.to_string();
                }
            }
            ConvertCurrentColor::Regex(ref pattern) => {
                if !in_mask && !is_special {
                    if let Ok(regex) = Regex::new(pattern) {
                        if regex.is_match(&val) {
                            val = PROTECTED_394_.to_string();
                        }
                    }
                }
            }
            _ => {}
        }

        // Convert color name keyword to long hex
        if self.config.names2hex {
            let color_name = val.to_lowercase();
            if let Some(hex_value) = Self::color_names().get(color_name.as_str()) {
                val = hex_value.to_string();
            }
        }

        // Convert rgb() to long hex
        if self.config.rgb2hex {
            if let Some((r, g, b)) = Self::parse_rgb(&val) {
                val = Self::rgb_to_hex(r, g, b);
            }
        }

        // Convert case (but not for special values)
        let is_special_for_case = [PROTECTED_395_, PROTECTED_396_, PROTECTED_397_, PROTECTED_398_]
            .iter()
            .any(|&special| val.eq_ignore_ascii_case(special));
        if !Self::includes_url_reference(&val) && !is_special_for_case {
            match self.config.convert_case {
                ConvertCase::Lower => val = val.to_lowercase(),
                ConvertCase::Upper => val = val.to_uppercase(),
                ConvertCase::None => {}
            }
        }

        // Convert long hex to short hex
        if self.config.shorthex {
            if let Some(short_hex) = Self::hex_to_short_hex(&val) {
                val = short_hex;
            }
        }

        // Convert hex to short name (but not inside masks)
        if self.config.shortname && !in_mask {
            let color_name = val.to_lowercase();
            if let Some(short_name) = Self::color_short_names().get(color_name.as_str()) {
                val = short_name.to_string();
            }
        }


        val
    }
}

impl Default for ConvertColorsPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl crate::PluginWithParams for ConvertColorsPlugin {
    type Config = ConvertColorsConfig;
    
    fn with_config(config: Self::Config) -> Self {
        Self::with_config(config)
    }
    
    fn parse_config(params: &serde_json::Value) -> anyhow::Result<Self::Config> {
        Self::parse_config(params)
    }
}

impl Plugin for ConvertColorsPlugin {
    fn name(&self) -> &'),
                ('static str {
        PROTECTED_400_
    }

    fn validate_params(&self, params: &serde_json::Value) -> anyhow::Result<()> {
        if let Some(obj) = params.as_object() {
            for (key, value) in obj {
                match key.as_str() {
                    PROTECTED_401_ => {
                        if !value.is_boolean() && !value.is_string() {
                            return Err(anyhow::anyhow!(
                                PROTECTED_402_
                            ));
                        }
                    }
                    PROTECTED_403_ | PROTECTED_404_ | PROTECTED_405_ | PROTECTED_406_ => {
                        if !value.is_boolean() {
                            return Err(anyhow::anyhow!(PROTECTED_407_, key));
                        }
                    }
                    PROTECTED_408_ => {
                        if let Some(case_str) = value.as_str() {
                            match case_str {
                                PROTECTED_409_ | PROTECTED_410_ => {}
                                _ => {
                                    return Err(anyhow::anyhow!(
                                        PROTECTED_411_
                                    ))
                                }
                            }
                        } else if value.as_bool() == Some(false) {
                            // false is allowed
                        } else {
                            return Err(anyhow::anyhow!(
                                PROTECTED_412_
                            ));
                        }
                    }
                    _ => {
                        return Err(anyhow::anyhow!(PROTECTED_413_, key));
                    }
                }
            }
        }
        Ok(())
    }

    fn apply(&self, document: &mut Document) -> anyhow::Result<()> {
        let mut visitor = ColorConversionVisitor::new(self.config.clone());
        vexy_svgo_core::visitor::walk_document(&mut visitor, document)?;
        Ok(())
    }
}

/// Visitor implementation that converts colors
struct ColorConversionVisitor {
    config: ConvertColorsConfig,
    mask_counter: usize,
}

impl ColorConversionVisitor {
    fn new(config: ConvertColorsConfig) -> Self {
        Self {
            config,
            mask_counter: 0,
        }
    }
}

impl Visitor<', '_>) -> Result<()> {
        // Track mask elements
        if element.name == PROTECTED_414_ {
            self.mask_counter += 1;
        }

        // Convert color attributes
        let plugin = ConvertColorsPlugin::with_config(self.config.clone());
        let mut attrs_to_update = Vec::new();

        for (attr_name, attr_value) in &element.attributes {
            if ConvertColorsPlugin::color_props().contains(attr_name.as_str()) {
                let converted_value = plugin.convert_color_value(attr_value, self.mask_counter > 0);
                if converted_value != *attr_value {
                    attrs_to_update.push((attr_name.clone(), converted_value));
                }
            }
        }

        // Update attributes
        for (attr_name, new_value) in attrs_to_update {
            element.attributes.insert(attr_name, new_value);
        }

        Ok(())
    }

    fn visit_element_exit(&mut self, element: &mut Element<'),
                ('static str) -> Element<', 'static str, attrs: &[(&str, &str)]) -> Element<'),
                (// this_file: crates/plugin-sdk/src/plugins/convert_colors.rs, //! Convert colors plugin implementation),
                (//!, //! This plugin converts colors between different formats:),
                (//! - rgb() to hex, //! - color names to hex),
                (//! - long hex to short hex, //! - hex to short color names),
                (//! It follows the same logic as SVGOPROTECTED_517_t implement Clone easily, /// Plugin that converts colors between different formats),
                (/// Create a new ConvertColorsPlugin with default configuration, /// Create a new ConvertColorsPlugin with custom configuration),
                (/// Parse configuration from JSON parameters, /// Color name keywords to hex values),
                (/// Color properties that can contain color values, // Track mask elements),
                (// Valid parameters, // Invalid parameters),
                (// Color name conversion (shortname now enabled by default), // RGB conversion),
                (// Hex shortening, // No change for invalid values),
                (// Disable other conversions for this test, // Regular colors should be converted to currentColor),
                (// Special values should NOT be converted to currentColor and should preserve their case, // case preserved for special values),
                (// Inside mask (in_mask=true): Should NOT convert to currentColor but should do other conversions, // names2hex -> shorthex (shortname disabled in masks)),
                (// names2hex -> shorthex (shortname disabled in masks), // names2hex -> shorthex (shortname disabled in masks)),
                (// Outside mask (in_mask=false): Should convert to currentColor, // Create element with color attributes),
                (// Create mask element with nested colored element, // Should still convert colors inside mask but not to color names),
                (// red -> #ff0000 -> #f00 (shortname disabled in masks), // Custom fixture tests for ConvertColorsPlugin with parameter support),
                (// Create plugin instance with parameters, // Apply plugin to input),
                (// Compare result with expected output, // Create plugin instance with parameters),
                (// Apply plugin to input, // Compare result with expected output),
                (PROTECTED_260_, PROTECTED_261_),
                (PROTECTED_262_, PROTECTED_263_),
                (PROTECTED_264_, PROTECTED_265_),
                (PROTECTED_266_, PROTECTED_267_),
                (PROTECTED_268_, PROTECTED_269_),
                (PROTECTED_270_, PROTECTED_271_),
                (PROTECTED_272_, PROTECTED_273_),
                (PROTECTED_274_, PROTECTED_275_),
                (PROTECTED_276_, PROTECTED_277_),
                (PROTECTED_278_, PROTECTED_279_),
                (PROTECTED_280_, PROTECTED_281_),
                (PROTECTED_282_, PROTECTED_283_),
                (PROTECTED_284_, PROTECTED_285_),
                (PROTECTED_286_, PROTECTED_287_),
                (PROTECTED_288_, PROTECTED_289_),
            ]
            .into_iter()
            .collect()
        })
    }

    /// Short color names for hex values
    fn color_short_names() -> &'static HashMap<&'static str, &'static str> {
        static COLOR_SHORT_NAMES: std::sync::OnceLock<HashMap<&'static str, &'static str>> =
            std::sync::OnceLock::new();
        COLOR_SHORT_NAMES.get_or_init(|| {
            [
                ("#000000", "black"),
                ("#000", "black"),
                ("#000080", "navy"),
                ("#008", "navy"),
                ("#008000", "green"),
                ("#080", "green"),
                ("#008080", "teal"),
                ("#088", "teal"),
                ("#4b0082", "indigo"),
                ("#800000", "maroon"),
                ("#800", "maroon"),
                ("#800080", "purple"),
                ("#808", "purple"),
                ("#808000", "olive"),
                ("#880", "olive"),
                ("#808080", "gray"),
                ("#888", "gray"),
                ("#a0522d", "sienna"),
                ("#a52a2a", "brown"),
                ("#c0c0c0", "silver"),
                ("#ccc", "silver"),
                ("#cd853f", "peru"),
                ("#d2b48c", "tan"),
                ("#da70d6", "orchid"),
                ("#dda0dd", "plum"),
                ("#ee82ee", "violet"),
                ("#f0e68c", "khaki"),
                ("#f0ffff", "azure"),
                ("#f5deb3", "wheat"),
                ("#f5f5dc", "beige"),
                ("#fa8072", "salmon"),
                ("#faf0e6", "linen"),
                ("#ff0000", "red"),
                ("#f00", "red"),
                ("#ff6347", "tomato"),
                ("#ff7f50", "coral"),
                ("#ffa500", "orange"),
                ("#ffc0cb", "pink"),
                ("#ffd700", "gold"),
                ("#ffe4c4", "bisque"),
                ("#fffafa", "snow"),
                ("#fffff0", "ivory"),
                ("#ffffff", "white"),
                ("#fff", "white"),
            ]
            .into_iter()
            .collect()
        })
    }

    /// Color properties that can contain color values
    fn color_props() -> &'static std::collections::HashSet<&'static str> {
        static COLOR_PROPS: std::sync::OnceLock<std::collections::HashSet<&'static str>> =
            std::sync::OnceLock::new();
        COLOR_PROPS.get_or_init(|| {
            [
                PROTECTED_378_,
                PROTECTED_379_,
                PROTECTED_380_,
                PROTECTED_381_,
                PROTECTED_382_,
                PROTECTED_383_,
            ]
            .into_iter()
            .collect()
        })
    }

    /// Convert RGB components to hex
    fn rgb_to_hex(r: u8, g: u8, b: u8) -> String {
        format!(PROTECTED_384_, r, g, b)
    }

    /// Convert hex color to short hex if possible
    fn hex_to_short_hex(hex: &str) -> Option<String> {
        if hex.len() == 7 && hex.starts_with('#') {
            let chars: Vec<char> = hex.chars().collect();
            if chars[1] == chars[2] && chars[3] == chars[4] && chars[5] == chars[6] {
                return Some(format!(PROTECTED_385_, chars[1], chars[3], chars[5]));
            }
        }
        None
    }

    /// Parse RGB function
    fn parse_rgb(value: &str) -> Option<(u8, u8, u8)> {
        // RGB regex pattern
        let rgb_regex = Regex::new(rPROTECTED_386_).ok()?;

        if let Some(captures) = rgb_regex.captures(value) {
            let mut components = Vec::new();

            for i in 1..=3 {
                let component_str = captures.get(i)?.as_str();
                let component = if component_str.contains('%') {
                    // Percentage value
                    let percentage = component_str.trim_end_matches('%').parse::<f64>().ok()?;
                    (percentage * 2.55).round() as i32
                } else {
                    // Numeric value
                    component_str.parse::<i32>().ok()?
                };

                // Clamp to 0-255 range
                components.push((component.max(0).min(255)) as u8);
            }

            if components.len() == 3 {
                return Some((components[0], components[1], components[2]));
            }
        }

        None
    }

    /// Check if value includes URL reference
    fn includes_url_reference(value: &str) -> bool {
        value.contains(PROTECTED_387_)
    }

    /// Convert color value
    fn convert_color_value(&self, value: &str, in_mask: bool) -> String {
        let mut val = value.to_string();
        
        // Check for special values that should not be converted to currentColor
        let special_values = [PROTECTED_388_, PROTECTED_389_, PROTECTED_390_, PROTECTED_391_];
        let is_special = special_values.iter().any(|&special| val.eq_ignore_ascii_case(special));

        // Convert colors to currentColor
        match &self.config.current_color {
            ConvertCurrentColor::Bool(true) => {
                if !in_mask && !is_special {
                    val = PROTECTED_392_.to_string();
                }
            }
            ConvertCurrentColor::String(ref target) => {
                if !in_mask && !is_special && val == *target {
                    val = PROTECTED_393_.to_string();
                }
            }
            ConvertCurrentColor::Regex(ref pattern) => {
                if !in_mask && !is_special {
                    if let Ok(regex) = Regex::new(pattern) {
                        if regex.is_match(&val) {
                            val = PROTECTED_394_.to_string();
                        }
                    }
                }
            }
            _ => {}
        }

        // Convert color name keyword to long hex
        if self.config.names2hex {
            let color_name = val.to_lowercase();
            if let Some(hex_value) = Self::color_names().get(color_name.as_str()) {
                val = hex_value.to_string();
            }
        }

        // Convert rgb() to long hex
        if self.config.rgb2hex {
            if let Some((r, g, b)) = Self::parse_rgb(&val) {
                val = Self::rgb_to_hex(r, g, b);
            }
        }

        // Convert case (but not for special values)
        let is_special_for_case = [PROTECTED_395_, PROTECTED_396_, PROTECTED_397_, PROTECTED_398_]
            .iter()
            .any(|&special| val.eq_ignore_ascii_case(special));
        if !Self::includes_url_reference(&val) && !is_special_for_case {
            match self.config.convert_case {
                ConvertCase::Lower => val = val.to_lowercase(),
                ConvertCase::Upper => val = val.to_uppercase(),
                ConvertCase::None => {}
            }
        }

        // Convert long hex to short hex
        if self.config.shorthex {
            if let Some(short_hex) = Self::hex_to_short_hex(&val) {
                val = short_hex;
            }
        }

        // Convert hex to short name (but not inside masks)
        if self.config.shortname && !in_mask {
            let color_name = val.to_lowercase();
            if let Some(short_name) = Self::color_short_names().get(color_name.as_str()) {
                val = short_name.to_string();
            }
        }


        val
    }
}

impl Default for ConvertColorsPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl crate::PluginWithParams for ConvertColorsPlugin {
    type Config = ConvertColorsConfig;
    
    fn with_config(config: Self::Config) -> Self {
        Self::with_config(config)
    }
    
    fn parse_config(params: &serde_json::Value) -> anyhow::Result<Self::Config> {
        Self::parse_config(params)
    }
}

impl Plugin for ConvertColorsPlugin {
    fn name(&self) -> &'static str {
        "convertColors"
    }

    fn description(&self) -> &'static str {
        PROTECTED_400_
    }

    fn validate_params(&self, params: &serde_json::Value) -> anyhow::Result<()> {
        if let Some(obj) = params.as_object() {
            for (key, value) in obj {
                match key.as_str() {
                    PROTECTED_401_ => {
                        if !value.is_boolean() && !value.is_string() {
                            return Err(anyhow::anyhow!(
                                PROTECTED_402_
                            ));
                        }
                    }
                    PROTECTED_403_ | PROTECTED_404_ | PROTECTED_405_ | PROTECTED_406_ => {
                        if !value.is_boolean() {
                            return Err(anyhow::anyhow!(PROTECTED_407_, key));
                        }
                    }
                    PROTECTED_408_ => {
                        if let Some(case_str) = value.as_str() {
                            match case_str {
                                PROTECTED_409_ | PROTECTED_410_ => {}
                                _ => {
                                    return Err(anyhow::anyhow!(
                                        PROTECTED_411_
                                    ))
                                }
                            }
                        } else if value.as_bool() == Some(false) {
                            // false is allowed
                        } else {
                            return Err(anyhow::anyhow!(
                                PROTECTED_412_
                            ));
                        }
                    }
                    _ => {
                        return Err(anyhow::anyhow!(PROTECTED_413_, key));
                    }
                }
            }
        }
        Ok(())
    }

    fn apply(&self, document: &mut Document) -> anyhow::Result<()> {
        let mut visitor = ColorConversionVisitor::new(self.config.clone());
        vexy_svgo_core::visitor::walk_document(&mut visitor, document)?;
        Ok(())
    }
}

/// Visitor implementation that converts colors
struct ColorConversionVisitor {
    config: ConvertColorsConfig,
    mask_counter: usize,
}

impl ColorConversionVisitor {
    fn new(config: ConvertColorsConfig) -> Self {
        Self {
            config,
            mask_counter: 0,
        }
    }
}

impl Visitor<'_> for ColorConversionVisitor {
    fn visit_element_enter(&mut self, element: &mut Element<'_>) -> Result<()> {
        // Track mask elements
        if element.name == PROTECTED_414_ {
            self.mask_counter += 1;
        }

        // Convert color attributes
        let plugin = ConvertColorsPlugin::with_config(self.config.clone());
        let mut attrs_to_update = Vec::new();

        for (attr_name, attr_value) in &element.attributes {
            if ConvertColorsPlugin::color_props().contains(attr_name.as_str()) {
                let converted_value = plugin.convert_color_value(attr_value, self.mask_counter > 0);
                if converted_value != *attr_value {
                    attrs_to_update.push((attr_name.clone(), converted_value));
                }
            }
        }

        // Update attributes
        for (attr_name, new_value) in attrs_to_update {
            element.attributes.insert(attr_name, new_value);
        }

        Ok(())
    }

    fn visit_element_exit(&mut self, element: &mut Element<'_>) -> Result<()> {
        // Track mask elements
        if element.name == "mask" {
            self.mask_counter = self.mask_counter.saturating_sub(1);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::borrow::Cow;
    use vexy_svgo_core::ast::{Document, Element, Node};

    fn create_element(name: &'static str) -> Element<'static> {
        let mut element = Element::new(name);
        element.name = Cow::Borrowed(name);
        element
    }

    fn create_element_with_attrs(name: &'static str, attrs: &[(&str, &str)]) -> Element<'static> {
        let mut element = create_element(name);
        for (key, value) in attrs {
            element
                .attributes
                .insert(key.to_string(), value.to_string());
        }
        element
    }

    #[test]
    fn test_plugin_creation() {
        let plugin = ConvertColorsPlugin::new();
        assert_eq!(plugin.name(), "convertColors");
        assert!(plugin.config.names2hex);
        assert!(plugin.config.rgb2hex);
    }

    #[test]
    fn test_parameter_validation() {
        let plugin = ConvertColorsPlugin::new();
