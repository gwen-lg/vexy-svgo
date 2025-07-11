// this_file: crates/plugin-sdk/src/plugins/remove_unknowns_and_defaults/default_attrs.rs

use std::collections::HashMap;
use std::collections::HashSet;
use vexy_svgo_core::ast::Element;
use crate::plugins::remove_unknowns_and_defaults::RemoveUnknownsAndDefaultsConfig;

/// Known SVG attributes (simplified list)
pub(crate) fn known_attributes() -> &'static HashSet<&'static str> {
    static KNOWN_ATTRIBUTES: std::sync::OnceLock<HashSet<&'static str>> =
        std::sync::OnceLock::new();
    KNOWN_ATTRIBUTES.get_or_init(|| {
        [
            PROTECTED_0_,
            PROTECTED_1_,
            PROTECTED_2_,
            PROTECTED_3_,
            PROTECTED_4_,
            PROTECTED_5_,
            PROTECTED_6_,
            PROTECTED_7_,
            PROTECTED_8_,
            PROTECTED_9_,
            PROTECTED_10_,
            PROTECTED_11_,
            PROTECTED_12_,
            PROTECTED_13_,
            PROTECTED_14_,
            PROTECTED_15_,
            PROTECTED_16_,
            PROTECTED_17_,
            PROTECTED_18_,
            PROTECTED_19_,
            PROTECTED_20_,
            PROTECTED_21_,
            PROTECTED_22_,
            PROTECTED_23_,
            PROTECTED_24_,
            PROTECTED_25_,
            PROTECTED_26_,
            PROTECTED_27_,
            PROTECTED_28_,
            PROTECTED_29_,
            PROTECTED_30_,
            PROTECTED_31_,
            PROTECTED_32_,
            PROTECTED_33_,
            PROTECTED_34_,
            PROTECTED_35_,
            PROTECTED_36_,
            PROTECTED_37_,
            PROTECTED_38_,
            PROTECTED_39_,
            PROTECTED_40_,
            PROTECTED_41_,
            PROTECTED_42_,
            PROTECTED_43_,
            PROTECTED_44_,
            PROTECTED_45_,
            PROTECTED_46_,
            PROTECTED_47_,
            PROTECTED_48_,
            PROTECTED_49_,
            PROTECTED_50_,
            PROTECTED_51_,
            PROTECTED_52_,
            PROTECTED_53_,
            PROTECTED_54_,
            PROTECTED_55_,
            PROTECTED_56_,
            PROTECTED_57_,
            PROTECTED_58_,
            PROTECTED_59_,
            PROTECTED_60_,
            PROTECTED_61_,
            PROTECTED_62_,
            PROTECTED_63_,
            PROTECTED_64_,
            PROTECTED_65_,
            PROTECTED_66_,
            PROTECTED_67_,
            PROTECTED_68_,
            PROTECTED_69_,
            PROTECTED_70_,
            // Element specific attributes
            PROTECTED_71_,
            PROTECTED_72_,
            PROTECTED_73_,
            PROTECTED_74_,
            PROTECTED_75_,
            PROTECTED_76_,
            PROTECTED_77_,
            PROTECTED_78_,
            PROTECTED_79_,
            PROTECTED_80_,
            PROTECTED_81_,
            PROTECTED_82_,
            PROTECTED_83_,
            PROTECTED_84_,
            PROTECTED_85_,
            PROTECTED_86_,
            PROTECTED_87_,
            PROTECTED_88_,
            PROTECTED_89_,
            PROTECTED_90_,
            PROTECTED_91_,
            PROTECTED_92_,
            PROTECTED_93_,
            PROTECTED_94_,
            PROTECTED_95_,
            PROTECTED_96_,
            PROTECTED_97_,
            PROTECTED_98_,
            PROTECTED_99_,
            PROTECTED_100_,
            PROTECTED_101_,
            PROTECTED_102_,
            PROTECTED_103_,
            PROTECTED_104_,
            PROTECTED_105_,
            PROTECTED_106_,
            PROTECTED_107_,
            PROTECTED_108_,
            PROTECTED_109_,
            PROTECTED_110_,
            PROTECTED_111_,
            PROTECTED_112_,
            PROTECTED_113_,
            // Animation attributes
            PROTECTED_114_,
            PROTECTED_115_,
            PROTECTED_116_,
            PROTECTED_117_,
            PROTECTED_118_,
            PROTECTED_119_,
            PROTECTED_120_,
            PROTECTED_121_,
            PROTECTED_122_,
            PROTECTED_123_,
            PROTECTED_124_,
            PROTECTED_125_,
            PROTECTED_126_,
            PROTECTED_127_,
            PROTECTED_128_,
            PROTECTED_129_,
            PROTECTED_130_,
            PROTECTED_131_,
            PROTECTED_132_,
            PROTECTED_133_,
            // Filter attributes
            PROTECTED_134_,
            PROTECTED_135_,
            PROTECTED_136_,
            PROTECTED_137_,
            PROTECTED_138_,
            PROTECTED_139_,
            PROTECTED_140_,
            PROTECTED_141_,
            PROTECTED_142_,
            PROTECTED_143_,
            PROTECTED_144_,
            PROTECTED_145_,
            PROTECTED_146_,
            PROTECTED_147_,
            PROTECTED_148_,
            PROTECTED_149_,
            PROTECTED_150_,
            PROTECTED_151_,
            PROTECTED_152_,
            PROTECTED_153_,
            PROTECTED_154_,
            PROTECTED_155_,
            PROTECTED_156_,
            // Gradient attributes
            PROTECTED_157_,
            PROTECTED_158_,
            PROTECTED_159_,
            PROTECTED_160_,
            PROTECTED_161_,
            PROTECTED_162_,
            PROTECTED_163_,
            // Pattern attributes
            PROTECTED_164_,
            PROTECTED_165_,
            PROTECTED_166_,
            // Mask attributes
            PROTECTED_167_,
            PROTECTED_168_,
            // Clip path attributes
            PROTECTED_169_,
            // Text attributes
            PROTECTED_170_,
            PROTECTED_171_,
            PROTECTED_172_,
            PROTECTED_173_,
            // Table attributes (for foreignObject)
            PROTECTED_174_,
            PROTECTED_175_,
            PROTECTED_176_,
        ]
        .into_iter()
        .collect()
    })
}

/// Default values for common SVG attributes
pub(crate) fn default_attribute_values() -> &'static HashMap<&'static str, &'static str> {
    static DEFAULT_VALUES: std::sync::OnceLock<HashMap<&'static str, &'static str>> =
        std::sync::OnceLock::new();
    DEFAULT_VALUES.get_or_init(|| {
        [
            ("x", "0"),
            ("y", "0"),
            ("width", "0"),
            ("height", "0"),
            ("fill", "black"),
            ("stroke", "none"),
            ("stroke-width", "1"),
            ("stroke-linecap", "butt"),
            ("stroke-linejoin", "miter"),
            ("stroke-miterlimit", "4"),
            ("stroke-dasharray", "none"),
            ("stroke-dashoffset", "0"),
            ("stroke-opacity", "1"),
            ("fill-opacity", "1"),
            ("opacity", "1"),
            ("visibility", "visible"),
            ("display", "inline"),
            ("overflow", "visible"),
            ("clip-rule", "nonzero"),
            ("fill-rule", "nonzero"),
            ("color-interpolation", "sRGB"),
            ("color-interpolation-filters", "linearRGB"),
            ("color-rendering", "auto"),
            ("dominant-baseline", "auto"),
            ("enable-background", "accumulate"),
            ("font-size", "medium"),
            ("font-stretch", "normal"),
            ("font-style", "normal"),
            ("font-variant", "normal"),
            ("font-weight", "normal"),
            ("glyph-orientation-horizontal", "0deg"),
            ("glyph-orientation-vertical", "auto"),
            ("image-rendering", "auto"),
            ("kerning", "auto"),
            ("letter-spacing", "normal"),
            ("lighting-color", "normal"),
            ("marker", "none"),
            ("marker-start", "none"),
            ("marker-mid", "none"),
            ("marker-end", "none"),
            ("pointer-events", "visiblePainted"),
            ("shape-rendering", "auto"),
            ("stop-color", "black"),
            ("stop-opacity", "1"),
            ("text-anchor", "start"),
            ("text-decoration", "none"),
            ("text-rendering", "auto"),
            ("unicode-bidi", "normal"),
            ("vector-effect", "none"),
            ("word-spacing", "normal"),
            ("writing-mode", "lr-tb"),
            ("direction", "ltr"),
            ("color", "black"),
            // Circle, ellipse specific
            ("cx", "0"),
            ("cy", "0"),
            ("r", "0"),
            ("rx", "0"),
            ("ry", "0"),
            // Line specific
            ("x1", "0"),
            ("y1", "0"),
            ("x2", "0"),
            ("y2", "0"),
            // Text specific
            ("dx", "0"),
            ("dy", "0"),
            ("rotate", "0"),
            ("textLength", "0"),
            ("lengthAdjust", "spacing"),
            ("startOffset", "0"),
            // Pattern/marker specific
            ("refX", "0"),
            ("refY", "0"),
            ("markerUnits", "strokeWidth"),
            ("markerWidth", "3"),
            ("markerHeight", "3"),
            ("orient", "0"),
            ("patternUnits", "objectBoundingBox"),
            ("patternContentUnits", "userSpaceOnUse"),
            ("maskUnits", "objectBoundingBox"),
            ("maskContentUnits", "userSpaceOnUse"),
            ("clipPathUnits", "userSpaceOnUse"),
            // Gradient specific
            ("gradientUnits", "objectBoundingBox"),
            ("spreadMethod", "pad"),
            ("fx", "cx"),
            ("fy", "cy"),
            // Filter specific
            ("filterUnits", "objectBoundingBox"),
            ("primitiveUnits", "userSpaceOnUse"),
            // Note: filter elements have different default x/y/width/height values
            // but we canPROTECTED_756_s an unknown attribute
    if config.unknown_attrs && !known_attributes().contains(attr_name) {
        return true;
    }

    // Check if itPROTECTED_757_t remove default values from elements with IDs as they might be referenced
            if element.attr("id").is_some() {
                return false;
            }
            
            if attr_value == *default_value {
                return true;
            }
        }
    }

    false
}
            ("k4", "0"),
            ("divisor", "1"),
            ("bias", "0"),
            ("targetX", "1"),
            ("targetY", "1"),
            ("surfaceScale", "1"),
            ("specularConstant", "1"),
            ("specularExponent", "1"),
            // Text path specific
            ("method", "align"),
            ("spacing", "exact"),
            // Transform origin
            ("transform-origin", "0 0"),
            // Misc
            ("version", "1.1"),
            ("baseProfile", "none"),
            ("preserveAspectRatio", "xMidYMid meet"),
            ("zoomAndPan", "magnify"),
            ("contentScriptType", "application/ecmascript"),
            ("contentStyleType", "text/css"),
            ("type", "text/css"),
            ("media", "all"),
            ("target", "_self"),
            ("xlink:type", "simple"),
            ("xlink:show", "other"),
            ("xlink:actuate", "onRequest"),
            ("xml:space", "default"),
            ("requiredFeatures", ""),
            ("requiredExtensions", ""),
            ("systemLanguage", ""),
            ("externalResourcesRequired", "false"),
            ("class", ""),
            ("style", ""),
            ("transform", ""),
            ("tabindex", "0"),
            ("lang", ""),
            ("xml:lang", ""),
            ("role", ""),
            ("aria-label", ""),
            ("aria-labelledby", ""),
            ("aria-describedby", ""),
            ("aria-details", ""),
            ("title", ""),
            ("desc", ""),
            // Presentation attributes
            ("alignment-baseline", "auto"),
            ("baseline-shift", "0"),
            ("clip", "auto"),
            ("clip-path", "none"),
            ("color-profile", "auto"),
            ("cursor", "auto"),
            ("filter", "none"),
            ("flood-color", "black"),
            ("flood-opacity", "1"),
            ("font-family", ""),
            ("font-size-adjust", "none"),
            ("glyph-orientation-horizontal", "0"),
            ("glyph-orientation-vertical", "auto"),
            ("image-rendering", "auto"),
            ("mask", "none"),
            ("text-overflow", "clip"),
            ("white-space", "normal"),
            ("line-height", "normal"),
            ("paint-order", "normal"),
            ("isolation", "auto"),
            ("mix-blend-mode", "normal"),
            ("solid-color", "black"),
            ("solid-opacity", "1"),
            ("vector-effect", "none"),
            ("stop-color", "black"),
            ("stop-opacity", "1"),
            ("color-interpolation", "sRGB"),
            ("color-interpolation-filters", "linearRGB"),
            ("color-rendering", "auto"),
            ("fill", "black"),
            ("fill-opacity", "1"),
            ("fill-rule", "nonzero"),
            ("image-rendering", "auto"),
            ("marker", "none"),
            ("marker-end", "none"),
            ("marker-mid", "none"),
            ("marker-start", "none"),
            ("shape-rendering", "auto"),
            ("stroke", "none"),
            ("stroke-dasharray", "none"),
            ("stroke-dashoffset", "0"),
            ("stroke-linecap", "butt"),
            ("stroke-linejoin", "miter"),
            ("stroke-miterlimit", "4"),
            ("stroke-opacity", "1"),
            ("stroke-width", "1"),
            ("text-rendering", "auto"),
            ("alignment-baseline", "auto"),
            ("baseline-shift", "baseline"),
            ("dominant-baseline", "auto"),
            ("glyph-orientation-horizontal", "0deg"),
            ("glyph-orientation-vertical", "auto"),
            ("kerning", "auto"),
            ("text-anchor", "start"),
            ("writing-mode", "lr-tb"),
            ("font-stretch", "normal"),
            ("font-style", "normal"),
            ("font-variant", "normal"),
            ("font-weight", "normal"),
            ("direction", "ltr"),
            ("letter-spacing", "normal"),
            ("text-decoration", "none"),
            ("unicode-bidi", "normal"),
            ("word-spacing", "normal"),
            ("clip-rule", "nonzero"),
            ("mask-type", "luminance"),
            ("display", "inline"),
            ("overflow", "visible"),
            ("visibility", "visible"),
            ("color-profile", "sRGB"),
            ("opacity", "1"),
            // Additional defaults
            ("lighting-color", "white"),
            ("enable-background", "accumulate"),
            ("xml:base", ""),
            ("requiredFeatures", ""),
            ("requiredExtensions", ""),
            ("systemLanguage", ""),
            ("id", ""),
            ("tabindex", "0"),
            ("focusable", "auto"),
            ("aria-hidden", "false"),
            ("role", ""),
            ("aria-label", ""),
            ("aria-labelledby", ""),
            ("aria-describedby", ""),
            ("aria-details", ""),
            ("aria-haspopup", "false"),
            ("aria-controls", ""),
            ("aria-flowto", ""),
            ("aria-owns", ""),
            ("aria-activedescendant", ""),
            ("aria-live", "off"),
            ("aria-atomic", "false"),
            ("aria-relevant", "additions text"),
            ("aria-busy", "false"),
            ("aria-disabled", "false"),
            ("aria-grabbed", "undefined"),
            ("aria-dropeffect", "none"),
            ("aria-expanded", "undefined"),
            ("aria-pressed", "undefined"),
            ("aria-checked", "undefined"),
            ("aria-selected", "undefined"),
            ("aria-required", "false"),
            ("aria-readonly", "false"),
            ("aria-multiline", "false"),
            ("aria-multiselectable", "false"),
            ("aria-orientation", "undefined"),
            ("aria-autocomplete", "none"),
            ("aria-invalid", "false"),
            ("aria-hidden", "false"),
            ("aria-level", ""),
            ("aria-valuemin", ""),
            ("aria-valuemax", ""),
            ("aria-valuenow", ""),
            ("aria-valuetext", ""),
            ("aria-setsize", ""),
            ("aria-posinset", ""),
            ("aria-colspan", ""),
            ("aria-rowspan", ""),
            ("aria-rowindex", ""),
            ("aria-colindex", ""),
            ("aria-sort", "none"),
            ("aria-modal", "false"),
            ("aria-current", "false"),
            ("aria-placeholder", ""),
            ("aria-roledescription", ""),
            ("aria-keyshortcuts", ""),
            // Newer attributes
            ("color-scheme", "normal"),
            ("rotate", "0"),
            ("scale", "1"),
            ("method", "align"),
            ("spacing", "exact"),
            ("startOffset", "0"),
            ("lengthAdjust", "spacing"),
            ("alignment-baseline", "auto"),
            ("baseline-shift", "baseline"),
            ("preserveAspectRatio", "xMidYMid meet"),
            ("zoomAndPan", "magnify"),
            ("contentScriptType", "application/ecmascript"),
            ("contentStyleType", "text/css"),
            ("type", "text/css"),
            ("media", "all"),
            ("target", "_self"),
        ]
        .into_iter()
        .collect()
    })
}

/// Check if an attribute should be removed based on configuration
pub(crate) fn should_remove_attribute(
    attr_name: &str,
    attr_value: &str,
    element: &Element,
    _parent: Option<&Element>,
    config: &RemoveUnknownsAndDefaultsConfig,
) -> bool {
    // Keep data attributes if configured
    if config.keep_data_attrs && attr_name.starts_with("data-") {
        return false;
    }

    // Keep ARIA attributes if configured
    if config.keep_aria_attrs && attr_name.starts_with("aria-") {
        return false;
    }

    // Keep role attribute if configured
    if config.keep_role_attr && attr_name == "role" {
        return false;
    }

    // Check for standard namespace prefixes
    if attr_name.starts_with("xml:") || 
       attr_name.starts_with("xlink:") || 
       attr_name.starts_with("xmlns") {
        return false;
    }

    // Check if itPROTECTED_550_s a default value
    if config.default_attrs {
        if let Some(default_value) = default_attribute_values().get(attr_name) {
            // Don't remove default values from elements with IDs as they might be referenced
            if element.attr("id").is_some() {
                return false;
            }
            
            if attr_value == *default_value {
                return true;
            }
        }
    }

    false
}
        }
    }

    false
}