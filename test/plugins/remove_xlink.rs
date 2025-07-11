// this_file: test/plugins/remove_xlink.rs

//! Tests for the removeXlink plugin
//! Auto-generated from SVGO test fixtures

use vexy_svgo::{optimize, OptimizeOptions, Config, PluginConfig};
use vexy_svgo::config::Js2SvgOptions;
use serde_json::json;

fn test_plugin(input: &str, expected: &str, params: Option<serde_json::Value>) {
    let config = Config {
        plugins: vec![PluginConfig {
            name: "removeXlink".to_string(),
            params,
        }],
        multipass: false,
        js2svg: Js2SvgOptions {
            pretty: true,
            indent: 4,
            ..Default::default()
        },
        ..Default::default()
    };

    let options = OptimizeOptions::new(config);
    let result = optimize(input, options).expect("Optimization should succeed");
    let output = result.data.trim();
    let expected = expected.trim();
    
    assert_eq!(output, expected, 
        "\nPlugin: removeXlink\nInput:\n{}\nExpected:\n{}\nActual:\n{}\n", 
        input, expected, output);
}

#[test]
fn test_remove_xlink_01() {
    // Remove xmlns:xlink and replace xlink:href with href attribute
    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\" viewBox=\"0 0 348.61 100\">
  <defs>
    <linearGradient id=\"a\" x1=\"263.36\" y1=\"14.74\" x2=\"333.47\" y2=\"84.85\" gradientUnits=\"userSpaceOnUse\">
      <stop offset=\"0\" stop-color=\"#45afe4\"/>
      <stop offset=\"1\" stop-color=\"#364f9e\"/>
    </linearGradient>
    <linearGradient id=\"b\" x1=\"262.64\" y1=\"15.46\" x2=\"332.75\" y2=\"85.57\" xlink:href=\"#a\"/>
  </defs>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 348.61 100\">
    <defs>
        <linearGradient id=\"a\" x1=\"263.36\" y1=\"14.74\" x2=\"333.47\" y2=\"84.85\" gradientUnits=\"userSpaceOnUse\">
            <stop offset=\"0\" stop-color=\"#45afe4\"/>
            <stop offset=\"1\" stop-color=\"#364f9e\"/>
        </linearGradient>
        <linearGradient id=\"b\" x1=\"262.64\" y1=\"15.46\" x2=\"332.75\" y2=\"85.57\" href=\"#a\"/>
    </defs>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_xlink_02() {
    // Remove xlink namespace even if it's under another prefix.
    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" xmlns:uwu=\"http://www.w3.org/1999/xlink\" viewBox=\"0 0 348.61 100\">
  <defs>
    <linearGradient id=\"a\" x1=\"263.36\" y1=\"14.74\" x2=\"333.47\" y2=\"84.85\" gradientUnits=\"userSpaceOnUse\">
      <stop offset=\"0\" stop-color=\"#45afe4\"/>
      <stop offset=\"1\" stop-color=\"#364f9e\"/>
    </linearGradient>
    <linearGradient id=\"b\" x1=\"262.64\" y1=\"15.46\" x2=\"332.75\" y2=\"85.57\" uwu:href=\"#a\"/>
  </defs>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 348.61 100\">
    <defs>
        <linearGradient id=\"a\" x1=\"263.36\" y1=\"14.74\" x2=\"333.47\" y2=\"84.85\" gradientUnits=\"userSpaceOnUse\">
            <stop offset=\"0\" stop-color=\"#45afe4\"/>
            <stop offset=\"1\" stop-color=\"#364f9e\"/>
        </linearGradient>
        <linearGradient id=\"b\" x1=\"262.64\" y1=\"15.46\" x2=\"332.75\" y2=\"85.57\" href=\"#a\"/>
    </defs>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_xlink_03() {
    // Convert xlink:href and xlink:show to href and target, and convert xlink:title
    // to title node.
    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\" viewBox=\"0 0 50 50\">
  <a xlink:href=\"https://duckduckgo.com\" xlink:show=\"new\" xlink:title=\"DuckDuckGo Homepage\">
    <text x=\"0\" y=\"10\">uwu</text>
  </a>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 50 50\">
    <a target=\"_blank\" href=\"https://duckduckgo.com\"><title>DuckDuckGo Homepage</title>
    <text x=\"0\" y=\"10\">uwu</text>
  </a>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_xlink_04() {
    // Drops other xlink attributes.
    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\" viewBox=\"0 0 50 50\">
  <defs>
    <linearGradient id=\"a\" x1=\"263.36\" y1=\"14.74\" x2=\"333.47\" y2=\"84.85\" gradientUnits=\"userSpaceOnUse\">
      <stop offset=\"0\" stop-color=\"#45afe4\"/>
      <stop offset=\"1\" stop-color=\"#364f9e\"/>
    </linearGradient>
    <linearGradient id=\"b\" x1=\"262.64\" y1=\"15.46\" x2=\"332.75\" y2=\"85.57\" xlink:href=\"#a\" xlink:type=\"simple\"/>
  </defs>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 50 50\">
    <defs>
        <linearGradient id=\"a\" x1=\"263.36\" y1=\"14.74\" x2=\"333.47\" y2=\"84.85\" gradientUnits=\"userSpaceOnUse\">
            <stop offset=\"0\" stop-color=\"#45afe4\"/>
            <stop offset=\"1\" stop-color=\"#364f9e\"/>
        </linearGradient>
        <linearGradient id=\"b\" x1=\"262.64\" y1=\"15.46\" x2=\"332.75\" y2=\"85.57\" href=\"#a\"/>
    </defs>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}
