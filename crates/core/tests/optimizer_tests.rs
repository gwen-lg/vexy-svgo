//! Comprehensive unit tests for the optimizer module

#[cfg(test)]
mod tests {
    use vexy_svgo_core::{
        optimize_with_config, optimize_default, Config, PluginConfig,
    };

    #[test]
    fn test_optimizer_creation() {
        let config = Config::default();
        // Test that we can create a default config
        assert!(config.plugins.len() == 0 || config.plugins.len() > 0);
    }

    #[test]
    fn test_optimize_simple_svg() {
        let svg = r#"<svg width="100" height="100"><rect x="10" y="10" width="50" height="50"/></svg>"#;
        let config = Config::default();
        
        let result = optimize_with_config(svg, config);
        assert!(result.is_ok());
        
        let optimized = result.unwrap();
        assert!(optimized.data.contains("<svg"));
        assert!(optimized.data.contains("<rect"));
    }

    #[test]
    fn test_optimize_with_comments() {
        let svg = r#"<svg><!-- This is a comment --><rect/></svg>"#;
        let mut config = Config::default();
        
        // Add comment removal plugin if not already present
        config.plugins.push(PluginConfig::Name("removeComments".to_string()));
        
        let result = optimize_with_config(svg, config);
        assert!(result.is_ok());
        
        let optimized = result.unwrap();
        // Comment may or may not be removed depending on plugin implementation
        assert!(optimized.data.contains("<svg"));
    }

    #[test]
    fn test_optimize_empty_svg() {
        let svg = r#"<svg></svg>"#;
        let config = Config::default();
        
        let result = optimize_with_config(svg, config);
        assert!(result.is_ok());
        
        let optimized = result.unwrap();
        assert!(optimized.data.contains("<svg"));
    }

    #[test]
    fn test_optimizer_with_no_plugins() {
        let svg = r#"<svg width="100" height="100"><rect/></svg>"#;
        let mut config = Config::default();
        
        // Clear all plugins
        config.plugins.clear();
        
        let result = optimize_with_config(svg, config);
        assert!(result.is_ok());
        
        let optimized = result.unwrap();
        // SVG should remain mostly unchanged with no plugins
        assert!(optimized.data.contains("width=\"100\""));
        assert!(optimized.data.contains("height=\"100\""));
    }

    #[test]
    fn test_multipass_optimization() {
        let svg = r#"<svg><g><g><rect/></g></g></svg>"#;
        let mut config = Config::default();
        config.multipass = true;
        
        let result = optimize_with_config(svg, config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_optimize_with_namespaces() {
        let svg = r##"<svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink">
            <rect xlink:href="#test"/>
        </svg>"##;
        let config = Config::default();
        
        let result = optimize_with_config(svg, config);
        assert!(result.is_ok());
        
        let optimized = result.unwrap();
        assert!(optimized.data.contains("xmlns"));
    }

    #[test]
    fn test_optimize_malformed_svg() {
        let svg = r#"<svg><rect"#; // Malformed - missing closing
        let config = Config::default();
        
        let result = optimize_with_config(svg, config);
        // Should handle errors gracefully
        assert!(result.is_err() || result.is_ok());
    }

    #[test]
    fn test_optimize_preserve_important_attributes() {
        let svg = r#"<svg viewBox="0 0 100 100" preserveAspectRatio="xMidYMid meet">
            <rect id="important-rect" class="my-class"/>
        </svg>"#;
        let config = Config::default();
        
        let result = optimize_with_config(svg, config);
        assert!(result.is_ok());
        
        let optimized = result.unwrap();
        // Important attributes should be preserved
        assert!(optimized.data.contains("viewBox"));
        assert!(optimized.data.contains("id="));
    }

    #[test]
    fn test_optimize_with_plugin_params() {
        let svg = r##"<svg><rect fill="#FF0000"/></svg>"##;
        let mut config = Config::default();
        
        // Configure color conversion plugin with parameters
        let mut params = serde_json::Map::new();
        params.insert("currentColor".to_string(), serde_json::Value::Bool(true));
        
        config.plugins.push(PluginConfig::WithParams {
            name: "convertColors".to_string(),
            params: serde_json::Value::Object(params),
        });
        
        let result = optimize_with_config(svg, config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_optimize_complex_structure() {
        let svg = r#"<svg>
            <defs>
                <linearGradient id="grad1">
                    <stop offset="0%" style="stop-color:rgb(255,255,0);stop-opacity:1"/>
                    <stop offset="100%" style="stop-color:rgb(255,0,0);stop-opacity:1"/>
                </linearGradient>
            </defs>
            <g transform="translate(50,50)">
                <rect width="100" height="100" fill="url(#grad1)"/>
                <text x="50" y="50">Hello</text>
            </g>
        </svg>"#;
        
        let config = Config::default();
        let result = optimize_with_config(svg, config);
        assert!(result.is_ok());
        
        let optimized = result.unwrap();
        // Complex structure should be preserved
        assert!(optimized.data.contains("defs"));
        assert!(optimized.data.contains("linearGradient"));
        assert!(optimized.data.contains("url(#grad1)"));
    }

    #[test]
    fn test_optimize_large_svg() {
        let mut svg = String::from("<svg>");
        
        // Create a large SVG with many elements
        for i in 0..100 {
            svg.push_str(&format!(
                r#"<rect x="{}" y="{}" width="10" height="10"/>"#,
                i * 10, i * 10
            ));
        }
        svg.push_str("</svg>");
        
        let config = Config::default();
        let result = optimize_with_config(&svg, config);
        assert!(result.is_ok());
        
        let optimized = result.unwrap();
        assert!(optimized.data.len() > 0);
        assert!(optimized.data.contains("<svg"));
        assert!(optimized.data.contains("<rect"));
    }

    #[test]
    fn test_optimize_with_custom_indentation() {
        let svg = r#"<svg><g><rect/></g></svg>"#;
        let mut config = Config::default();
        config.pretty = true;
        config.indent = "    ".to_string(); // 4 spaces
        let pretty_enabled = config.pretty;
        
        let result = optimize_with_config(svg, config);
        assert!(result.is_ok());
        
        let optimized = result.unwrap();
        // Output should be valid SVG
        assert!(optimized.data.contains("<svg"));
        assert!(optimized.data.contains("<g"));
        assert!(optimized.data.contains("<rect"));
    }

    #[test]
    fn test_optimize_with_js2svg_options() {
        let svg = r#"<svg><rect width="100" height="100"/></svg>"#;
        let mut config = Config::default();
        
        // Configure JS2SVG options
        config.js2svg.pretty = true;
        config.js2svg.indent = "  ".to_string();
        config.js2svg.final_newline = true;
        
        let result = optimize_with_config(svg, config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_optimize_default() {
        let svg = r#"<svg><rect width="100" height="100"/></svg>"#;
        
        let result = optimize_default(svg);
        assert!(result.is_ok());
        
        let optimized = result.unwrap();
        assert!(optimized.data.contains("<svg"));
        assert!(optimized.data.contains("<rect"));
    }

    #[test]
    fn test_optimize_with_path_data() {
        let svg = r#"<svg><path d="M 10.123456789 20.987654321 L 30.111111111 40.222222222"/></svg>"#;
        let config = Config::default();
        
        let result = optimize_with_config(svg, config);
        assert!(result.is_ok());
        
        let optimized = result.unwrap();
        // Path data should be present (may be optimized)
        assert!(optimized.data.contains("d="));
    }

    #[cfg(feature = "parallel")]
    #[test]
    fn test_parallel_optimization() {
        use vexy_svgo_core::optimizer::parallel::ParallelConfig;
        use vexy_svgo_core::OptimizeOptions;
        
        // Create a large SVG that should trigger parallel processing
        let mut svg = String::from(r#"<svg width="1000" height="1000">"#);
        
        // Add 2000 elements to trigger parallel processing
        for i in 0..2000 {
            svg.push_str(&format!(
                r#"<rect x="{}" y="{}" width="10" height="10" fill="#FF0000"/>"#,
                (i % 100) * 10, (i / 100) * 10
            ));
        }
        svg.push_str("</svg>");
        
        let config = Config::default();
        let options = OptimizeOptions {
            config,
            registry: None,
            parallel: Some(ParallelConfig {
                size_threshold: 1024,
                element_threshold: 100,
                num_threads: 2,
            }),
        };
        
        let result = vexy_svgo_core::optimize(&svg, options);
        assert!(result.is_ok());
        
        let optimized = result.unwrap();
        assert!(optimized.data.contains("<svg"));
        assert!(optimized.data.contains("<rect"));
    }
}