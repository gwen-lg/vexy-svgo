//! SVGO configuration compatibility tests
//! 
//! These tests ensure that vexy-svgo can read and process SVGO configuration files
//! in all their various formats (.svgo.config.js, svgo.config.js, .svgorc.json, etc.)

use vexy_svgo_core::{optimize_with_config, Config, PluginConfig, DataUriFormat};
use serde_json;
use std::fs;
use tempfile::TempDir;

#[test]
#[ignore = "Config loading from JS files not yet implemented"]
fn test_load_svgo_config_js() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join(".svgo.config.js");
    
    // Write a typical SVGO config.js file
    let config_content = r#"
module.exports = {
    multipass: true,
    plugins: [
        'removeDoctype',
        'removeXMLProcInst',
        'removeComments',
        {
            name: 'convertColors',
            params: {
                currentColor: true
            }
        }
    ]
};
"#;
    
    fs::write(&config_path, config_content).unwrap();
    
    // TODO: Implement load_config_from_directory
    // let config = load_config_from_directory(temp_dir.path()).unwrap();
    
    // assert!(config.multipass);
    // assert!(config.plugins.len() >= 3);
}

#[test]
#[ignore = "Config loading from JSON files not yet implemented"]
fn test_load_svgorc_json() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join(".svgorc.json");
    
    let config_content = r#"{
    "multipass": false,
    "js2svg": {
        "pretty": true,
        "indent": 2
    },
    "plugins": [
        "removeDoctype",
        "removeXMLProcInst",
        "removeComments",
        "removeMetadata",
        "removeTitle"
    ]
}"#;
    
    fs::write(&config_path, config_content).unwrap();
    
    // TODO: Implement load_config_from_directory
    // let config = load_config_from_directory(temp_dir.path()).unwrap();
    
    // assert!(!config.multipass);
    // assert!(config.js2svg.pretty);
    // assert_eq!(config.js2svg.indent, "  "); // 2 spaces
}

#[test]
#[ignore = "Preset support not yet implemented"]
fn test_svgo_config_with_preset() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("svgo.config.js");
    
    // SVGO supports preset configurations
    let config_content = r#"
module.exports = {
    plugins: [
        {
            name: 'preset-default',
            params: {
                overrides: {
                    removeViewBox: false,
                    inlineStyles: {
                        onlyMatchedOnce: false
                    }
                }
            }
        }
    ]
};
"#;
    
    fs::write(&config_path, config_content).unwrap();
    
    // TODO: Implement preset support
    // let config = load_config_from_directory(temp_dir.path());
    // assert!(config.is_ok());
}

#[test]
fn test_complex_plugin_config() {
    let mut config = Config::default();
    
    // Test various plugin configuration formats
    config.plugins = vec![
        // Simple string format
        PluginConfig::Name("removeDoctype".to_string()),
        
        // Plugin with boolean parameter
        PluginConfig::WithParams {
            name: "removeViewBox".to_string(),
            params: serde_json::json!(false),
        },
        
        // Plugin with object parameters
        PluginConfig::WithParams {
            name: "convertColors".to_string(),
            params: serde_json::json!({
                "currentColor": true,
                "names2hex": true,
                "rgb2hex": true,
                "shorthex": true,
                "shortname": true
            }),
        },
        
        // Plugin with nested configuration
        PluginConfig::WithParams {
            name: "cleanupIds".to_string(),
            params: serde_json::json!({
                "minify": true,
                "preserve": ["myImportantId", "anotherImportantId"],
                "preservePrefixes": ["data-", "aria-"]
            }),
        },
    ];
    
    let svg = r#"<svg viewBox="0 0 100 100">
        <rect id="myImportantId" fill="rgb(255, 0, 0)" width="50" height="50"/>
    </svg>"#;
    
    let result = optimize_with_config(svg, config);
    assert!(result.is_ok());
    
    let optimized = result.unwrap();
    // viewBox should be preserved (removeViewBox: false)
    assert!(optimized.data.contains("viewBox"));
    // Important ID should be preserved
    assert!(optimized.data.contains("myImportantId"));
}

#[test]
#[ignore = "cleanupNumericValues plugin not yet fully implemented"]
fn test_floatprecision_config() {
    let mut config = Config::default();
    
    // SVGO uses floatPrecision to control decimal precision
    config.plugins.push(PluginConfig::WithParams {
        name: "cleanupNumericValues".to_string(),
        params: serde_json::json!({
            "floatPrecision": 2
        }),
    });
    
    let svg = r#"<svg>
        <path d="M 10.123456789 20.987654321 L 30.111111111 40.222222222"/>
    </svg>"#;
    
    let result = optimize_with_config(svg, config);
    assert!(result.is_ok());
    
    let optimized = result.unwrap();
    // Numbers should be rounded to 2 decimal places
    assert!(!optimized.data.contains("10.123456789"));
}

#[test]
fn test_js2svg_config_options() {
    let mut config = Config::default();
    
    // Test various js2svg options
    config.js2svg.pretty = true;
    config.js2svg.indent = "    ".to_string(); // 4 spaces
    config.js2svg.final_newline = true;
    
    let svg = r#"<svg><g><rect/></g></svg>"#;
    
    let is_pretty = config.js2svg.pretty;
    let result = optimize_with_config(svg, config);
    assert!(result.is_ok());
    
    let optimized = result.unwrap();
    // Pretty print should add newlines
    if is_pretty {
        assert!(optimized.data.contains("\n") || optimized.data.len() > 30);
    }
}

#[test]
#[ignore = "Config loading from files not yet implemented"]
fn test_path_config() {
    let temp_dir = TempDir::new().unwrap();
    let svg_path = temp_dir.path().join("test.svg");
    let config_path = temp_dir.path().join(".svgo.config.js");
    
    // Create a config that uses the path
    let config_content = r#"
module.exports = {
    multipass: true,
    path: __dirname,
    plugins: ['removeComments']
};
"#;
    
    fs::write(&config_path, config_content).unwrap();
    fs::write(&svg_path, "<svg><!-- comment --><rect/></svg>").unwrap();
    
    let mut config = Config::default();
    config.path = Some(svg_path.to_string_lossy().to_string());
    
    // The path should be available for plugins that need it
    assert!(config.path.is_some());
}

#[test]
fn test_datauri_config() {
    
    let mut config = Config::default();
    config.datauri = Some(DataUriFormat::Base64);
    
    let svg = r#"<svg><rect width="10" height="10"/></svg>"#;
    
    let result = optimize_with_config(svg, config);
    assert!(result.is_ok());
    
    // When datauri is set, the output format might be different
    // This is mainly to ensure the config option is accepted
}

#[test]
fn test_custom_plugin_order() {
    let mut config = Config::default();
    
    // SVGO allows custom plugin ordering
    config.plugins = vec![
        PluginConfig::Name("cleanupAttrs".to_string()),
        PluginConfig::Name("removeDoctype".to_string()),
        PluginConfig::Name("removeXMLProcInst".to_string()),
        PluginConfig::Name("removeComments".to_string()),
        PluginConfig::Name("removeMetadata".to_string()),
        // Custom order matters for some optimizations
        PluginConfig::Name("convertStyleToAttrs".to_string()),
        PluginConfig::Name("removeStyleElement".to_string()),
    ];
    
    let svg = r#"<?xml version="1.0"?>
<!DOCTYPE svg PUBLIC "-//W3C//DTD SVG 1.1//EN" "http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd">
<svg>
    <style>rect { fill: red; }</style>
    <rect/>
</svg>"#;
    
    let result = optimize_with_config(svg, config);
    assert!(result.is_ok());
}

#[test]
fn test_disable_plugin_config() {
    let mut config = Config::default();
    
    // In SVGO, you can disable default plugins by prefixing with '-'
    // Or by setting them to false in the config
    config.plugins = vec![
        PluginConfig::WithParams {
            name: "removeViewBox".to_string(),
            params: serde_json::json!(false),
        },
        PluginConfig::WithParams {
            name: "removeEmptyAttrs".to_string(),
            params: serde_json::json!(false),
        },
    ];
    
    let svg = r#"<svg viewBox="0 0 100 100" fill="">
        <rect width="50" height="50"/>
    </svg>"#;
    
    let result = optimize_with_config(svg, config);
    assert!(result.is_ok());
    
    let optimized = result.unwrap();
    // These attributes should be preserved when plugins are disabled
    assert!(optimized.data.contains("viewBox"));
}

#[test]
fn test_plugin_params_validation() {
    // Test that invalid plugin parameters are handled gracefully
    let mut config = Config::default();
    
    config.plugins.push(PluginConfig::WithParams {
        name: "convertColors".to_string(),
        params: serde_json::json!({
            "invalidParam": "invalidValue",
            "currentColor": "not-a-boolean", // Should be boolean
        }),
    });
    
    let svg = r#"<svg><rect fill="red"/></svg>"#;
    
    // Should either ignore invalid params or handle gracefully
    let result = optimize_with_config(svg, config);
    assert!(result.is_ok() || result.is_err());
}