// this_file: crates/plugin-sdk/src/registry.rs

//! Plugin registry with migrated plugins
//!
//! This module provides a registry pre-populated with all migrated plugins
//! using the new visitor-based architecture.

use crate::plugins::*;
use vexy_svgo_core::plugin_registry::PluginRegistry;

/// Create a plugin registry with all migrated plugins
pub fn create_migrated_plugin_registry() -> PluginRegistry {
    let mut registry = PluginRegistry::new();

    // Register all migrated plugins
    registry.register(RemoveCommentsPlugin::new());
    registry.register(RemoveEmptyAttrsPlugin::new());
    registry.register(RemoveUselessDefsPlugin::new());
    registry.register(CollapseGroupsPlugin::new());
    registry.register(MoveGroupAttrsToElemsPlugin::new());
    registry.register(RemoveUnknownsAndDefaultsPlugin::new());
    registry.register(ConvertColorsPlugin::new());
    registry.register(RemoveViewBoxPlugin::new());
    registry.register(MergePathsPlugin::new());
    registry.register(InlineStylesPlugin::new());
    registry.register(CleanupIdsPlugin::new());
    registry.register(ConvertStyleToAttrsPlugin::new());
    registry.register(RemoveEmptyContainersPlugin::new());
    registry.register(RemoveHiddenElemsPlugin::new());
    registry.register(RemoveEditorsNSDataPlugin::new());
    registry.register(RemoveElementsByAttrPlugin::new());
    registry.register(RemoveUnusedNSPlugin::new());
    registry.register(CleanupAttrsPlugin::new());
    registry.register(CleanupEnableBackgroundPlugin::new());
    registry.register(CleanupListOfValuesPlugin::new());
    registry.register(MergeStylesPlugin::new());
    registry.register(RemoveDoctypePlugin::new());
    registry.register(RemoveDimensionsPlugin::new());
    registry.register(RemoveXMLProcInstPlugin::new());
    registry.register(RemoveMetadataPlugin::new());
    registry.register(RemoveEmptyTextPlugin::new());
    registry.register(ConvertEllipseToCirclePlugin::new());
    registry.register(ConvertOneStopGradientsPlugin::new());
    registry.register(ConvertShapeToPathPlugin::new());
    registry.register(ConvertPathDataPlugin::new());
    registry.register(ConvertTransformPlugin::new());
    registry.register(ApplyTransformsPlugin::new());
    registry.register(CleanupNumericValuesPlugin::new());
    registry.register(MinifyStylesPlugin::new());
    registry.register(RemoveNonInheritableGroupAttrsPlugin::new());
    registry.register(SortAttrsPlugin::new());
    registry.register(SortDefsChildrenPlugin::new());
    registry.register(RemoveTitlePlugin::new());
    registry.register(RemoveDescPlugin::new());
    registry.register(AddAttributesToSVGElementPlugin::new());
    registry.register(AddClassesToSVGElementPlugin::new());
    registry.register(RemoveScriptsPlugin::new());
    registry.register(RemoveStyleElementPlugin::new());
    registry.register(RemoveRasterImagesPlugin::new());
    registry.register(RemoveOffCanvasPathsPlugin::new());
    registry.register(RemoveAttrsPlugin::new());
    registry.register(RemoveDeprecatedAttrsPlugin::new());
    registry.register(RemoveUselessTransformsPlugin::new());
    registry.register(RemoveUselessStrokeAndFillPlugin::new());
    registry.register(RemoveXlinkPlugin::new());
    registry.register(RemoveXmlnsPlugin::new());
    registry.register(PrefixIdsPlugin::new());
    registry.register(ReusePathsPlugin::new());
    registry.register(RemoveAttributesBySelectorPlugin::new());

    registry
}

/// Get the default plugin configuration for migrated plugins
pub fn get_default_plugin_configs() -> Vec<vexy_svgo_core::plugin_registry::PluginConfig> {
    use serde_json::json;
    use vexy_svgo_core::plugin_registry::PluginConfig;

    vec![
        PluginConfig {
            name: "removeComments".to_string(),
            params: json!({"preservePatterns": true}),
            enabled: true,
        },
        PluginConfig {
            name: "removeEmptyAttrs".to_string(),
            params: json!({}),
            enabled: true,
        },
        PluginConfig {
            name: "removeUselessDefs".to_string(),
            params: json!({}),
            enabled: true,
        },
        PluginConfig {
            name: "collapseGroups".to_string(),
            params: json!({}),
            enabled: true,
        },
        PluginConfig {
            name: "moveGroupAttrsToElems".to_string(),
            params: json!({}),
            enabled: true,
        },
        PluginConfig {
            name: "removeUnknownsAndDefaults".to_string(),
            params: json!({
                "unknownContent": true,
                "unknownAttrs": true,
                "defaultAttrs": true,
                "defaultMarkupDeclarations": true,
                "uselessOverrides": true,
                "keepDataAttrs": true,
                "keepAriaAttrs": true,
                "keepRoleAttr": false
            }),
            enabled: true,
        },
        PluginConfig {
            name: "convertColors".to_string(),
            params: json!({
                "currentColor": false,
                "names2hex": true,
                "rgb2hex": true,
                "convertCase": "lower",
                "shorthex": true,
                "shortname": true
            }),
            enabled: true,
        },
        PluginConfig {
            name: "removeViewBox".to_string(),
            params: json!({}),
            enabled: true,
        },
        PluginConfig {
            name: "mergePaths".to_string(),
            params: json!({
                "force": false,
                "floatPrecision": 3,
                "noSpaceAfterFlags": false
            }),
            enabled: true,
        },
        PluginConfig {
            name: "inlineStyles".to_string(),
            params: json!({
                "onlyMatchedOnce": true,
                "removeMatchedSelectors": true,
                "useMqs": true,
                "usePseudos": true
            }),
            enabled: true,
        },
        PluginConfig {
            name: "cleanupIds".to_string(),
            params: json!({
                "remove": true,
                "minify": true,
                "preserve": [],
                "preservePrefixes": [],
                "force": false
            }),
            enabled: true,
        },
        PluginConfig {
            name: "convertStyleToAttrs".to_string(),
            params: json!({
                "keepImportant": false
            }),
            enabled: true,
        },
        PluginConfig {
            name: "removeEmptyContainers".to_string(),
            params: json!({}),
            enabled: true,
        },
        PluginConfig {
            name: "removeHiddenElems".to_string(),
            params: json!({
                "displayNone": true,
                "opacity0": true,
                "circleR0": true,
                "ellipseRX0": true,
                "ellipseRY0": true,
                "rectWidth0": true,
                "rectHeight0": true,
                "patternWidth0": true,
                "patternHeight0": true,
                "imageWidth0": true,
                "imageHeight0": true,
                "pathEmptyD": true,
                "polylineEmptyPoints": true,
                "polygonEmptyPoints": true
            }),
            enabled: true,
        },
        PluginConfig {
            name: "removeEditorsNSData".to_string(),
            params: json!({
                "additionalNamespaces": []
            }),
            enabled: true,
        },
        PluginConfig {
            name: "removeElementsByAttr".to_string(),
            params: json!({}),
            enabled: false, // Disabled by default since it requires configuration
        },
        PluginConfig {
            name: "removeUnusedNS".to_string(),
            params: json!({}),
            enabled: true,
        },
        PluginConfig {
            name: "cleanupAttrs".to_string(),
            params: json!({
                "newlines": true,
                "trim": true,
                "spaces": true
            }),
            enabled: true,
        },
        PluginConfig {
            name: "cleanupEnableBackground".to_string(),
            params: json!({}),
            enabled: true,
        },
        PluginConfig {
            name: "mergeStyles".to_string(),
            params: json!({}),
            enabled: true,
        },
        PluginConfig {
            name: "removeDoctype".to_string(),
            params: json!({}),
            enabled: true,
        },
        PluginConfig {
            name: "removeDimensions".to_string(),
            params: json!({}),
            enabled: false, // Disabled by default as it conflicts with removeViewBox
        },
        PluginConfig {
            name: "removeXMLProcInst".to_string(),
            params: json!({}),
            enabled: true,
        },
        PluginConfig {
            name: "removeMetadata".to_string(),
            params: json!({}),
            enabled: true,
        },
        PluginConfig {
            name: "removeEmptyText".to_string(),
            params: json!({
                "text": true,
                "tspan": true,
                "tref": true
            }),
            enabled: true,
        },
        PluginConfig {
            name: "convertEllipseToCircle".to_string(),
            params: json!({}),
            enabled: true,
        },
        PluginConfig {
            name: "convertOneStopGradients".to_string(),
            params: json!({}),
            enabled: false, // Not in SVGO default preset
        },
        PluginConfig {
            name: "convertShapeToPath".to_string(),
            params: json!({
                "convertArcs": false,
                "floatPrecision": null
            }),
            enabled: true,
        },
        PluginConfig {
            name: "convertPathData".to_string(),
            params: json!({
                "floatPrecision": 3,
                "transformPrecision": 5,
                "removeUseless": true,
                "collapseRepeated": true,
                "utilizeAbsolute": true,
                "leadingZero": true,
                "negativeExtraSpace": true
            }),
            enabled: true,
        },
        PluginConfig {
            name: "convertTransform".to_string(),
            params: json!({
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
            }),
            enabled: true,
        },
        PluginConfig {
            name: "cleanupNumericValues".to_string(),
            params: json!({
                "floatPrecision": 3,
                "leadingZero": true,
                "defaultPx": true,
                "convertToPx": true
            }),
            enabled: true,
        },
        PluginConfig {
            name: "minifyStyles".to_string(),
            params: json!({
                "restructure": true,
                "forceMediaMerge": false,
                "comments": false,
                "usage": null
            }),
            enabled: true,
        },
        PluginConfig {
            name: "removeNonInheritableGroupAttrs".to_string(),
            params: json!({}),
            enabled: true,
        },
        PluginConfig {
            name: "sortAttrs".to_string(),
            params: json!({
                "order": ["id", "width", "height", "x", "x1", "x2", "y", "y1", "y2", "cx", "cy", "r", "fill", "stroke", "marker", "d", "points"],
                "xmlnsOrder": "front"
            }),
            enabled: true,
        },
        PluginConfig {
            name: "sortDefsChildren".to_string(),
            params: json!({}),
            enabled: true,
        },
        PluginConfig {
            name: "removeTitle".to_string(),
            params: json!({}),
            enabled: true,
        },
        PluginConfig {
            name: "removeDesc".to_string(),
            params: json!({}),
            enabled: true,
        },
        PluginConfig {
            name: "addAttributesToSVGElement".to_string(),
            params: json!({}),
            enabled: false, // Disabled by default since it requires configuration
        },
        PluginConfig {
            name: "addClassesToSVGElement".to_string(),
            params: json!({}),
            enabled: false, // Disabled by default since it requires configuration
        },
        PluginConfig {
            name: "removeScripts".to_string(),
            params: json!({}),
            enabled: false, // Disabled by default for security reasons
        },
        PluginConfig {
            name: "removeStyleElement".to_string(),
            params: json!({}),
            enabled: false, // Disabled by default
        },
        PluginConfig {
            name: "removeRasterImages".to_string(),
            params: json!({}),
            enabled: false, // Disabled by default
        },
        PluginConfig {
            name: "removeOffCanvasPaths".to_string(),
            params: json!({}),
            enabled: false, // Disabled by default (as stated in original plugin description)
        },
        PluginConfig {
            name: "removeAttrs".to_string(),
            params: json!({}),
            enabled: false, // Disabled by default since it requires configuration
        },
        PluginConfig {
            name: "removeDeprecatedAttrs".to_string(),
            params: json!({}),
            enabled: false, // Disabled by default to preserve backward compatibility
        },
        PluginConfig {
            name: "removeUselessTransforms".to_string(),
            params: json!({}),
            enabled: true,
        },
        PluginConfig {
            name: "removeUselessStrokeAndFill".to_string(),
            params: json!({}),
            enabled: true,
        },
        PluginConfig {
            name: "removeXlink".to_string(),
            params: json!({
                "includeLegacy": true
            }),
            enabled: false, // Disabled by default
        },
        PluginConfig {
            name: "removeXmlns".to_string(),
            params: json!({}),
            enabled: false, // Disabled by default as mentioned in the description
        },
        PluginConfig {
            name: "prefixIds".to_string(),
            params: json!({}),
            enabled: false, // Disabled by default since it requires configuration
        },
        PluginConfig {
            name: "reusePaths".to_string(),
            params: json!({}),
            enabled: false, // Disabled by default as not in SVGO default preset
        },
        PluginConfig {
            name: "removeAttributesBySelector".to_string(),
            params: json!({}),
            enabled: false, // Disabled by default since it requires configuration
        },
    ]
}

/// Get plugin names for all migrated plugins
pub fn get_migrated_plugin_names() -> Vec<&'static str> {
    vec![
        "removeComments",
        "removeEmptyAttrs",
        "removeUselessDefs",
        "collapseGroups",
        "moveGroupAttrsToElems",
        "removeUnknownsAndDefaults",
        "convertColors",
        "removeViewBox",
        "mergePaths",
        "inlineStyles",
        "cleanupIds",
        "convertStyleToAttrs",
        "removeEmptyContainers",
        "removeHiddenElems",
        "removeEditorsNSData",
        "removeElementsByAttr",
        "removeUnusedNS",
        "cleanupAttrs",
        "cleanupEnableBackground",
        "mergeStyles",
        "removeDoctype",
        "removeDimensions",
        "removeXMLProcInst",
        "removeMetadata",
        "removeEmptyText",
        "convertEllipseToCircle",
        "convertOneStopGradients",
        "convertShapeToPath",
        "convertPathData",
        "convertTransform",
        "cleanupNumericValues",
        "minifyStyles",
        "removeNonInheritableGroupAttrs",
        "sortAttrs",
        "sortDefsChildren",
        "removeTitle",
        "removeDesc",
        "addAttributesToSVGElement",
        "addClassesToSVGElement",
        "removeScripts",
        "removeStyleElement",
        "removeRasterImages",
        "removeOffCanvasPaths",
        "removeAttrs",
        "removeDeprecatedAttrs",
        "removeUselessTransforms",
        "removeUselessStrokeAndFill",
        "removeXlink",
        "removeXmlns",
        "prefixIds",
        "reusePaths",
        "removeAttributesBySelector",
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_create_migrated_plugin_registry() {
        let registry = create_migrated_plugin_registry();

        // Check that all migrated plugins are registered
        for plugin_name in get_migrated_plugin_names() {
            assert!(
                registry.get_plugin(plugin_name).is_some(),
                "Plugin {} should be registered",
                plugin_name
            );
        }
    }

    #[test]
    fn test_get_default_plugin_configs() {
        let configs = get_default_plugin_configs();

        // Check that we have configs for all migrated plugins
        assert_eq!(configs.len(), get_migrated_plugin_names().len());

        // Check that we have a config for each plugin name
        let config_names: Vec<&str> = configs.iter().map(|c| c.name.as_str()).collect();
        for plugin_name in get_migrated_plugin_names() {
            assert!(
                config_names.contains(&plugin_name),
                "Plugin {} should have a config",
                plugin_name
            );
        }
    }

    #[test]
    fn test_apply_migrated_plugins() {
        let registry = create_migrated_plugin_registry();
        let configs = get_default_plugin_configs();

        // Create a test document
        let mut doc = vexy_svgo_core::ast::Document::new();

        // Apply all plugins - should not error
        let result = registry.apply_plugins(&mut doc, &configs);
        assert!(result.is_ok(), "Applying migrated plugins should succeed");
    }

    #[test]
    fn test_plugin_parameter_validation() {
        let registry = create_migrated_plugin_registry();

        // Test valid parameters
        let valid_config = vexy_svgo_core::plugin_registry::PluginConfig {
            name: "removeComments".to_string(),
            params: json!({"preservePatterns": false}),
            enabled: true,
        };

        let mut doc = vexy_svgo_core::ast::Document::new();
        let result = registry.apply_plugin(&mut doc, &valid_config);
        assert!(result.is_ok(), "Valid parameters should be accepted");

        // Test invalid parameters
        let invalid_config = vexy_svgo_core::plugin_registry::PluginConfig {
            name: "removeComments".to_string(),
            params: json!({"preservePatterns": "invalid"}),
            enabled: true,
        };

        let mut doc2 = vexy_svgo_core::ast::Document::new();
        let result = registry.apply_plugin(&mut doc2, &invalid_config);
        assert!(result.is_err(), "Invalid parameters should be rejected");
    }
}
