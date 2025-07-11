// this_file: test/plugins/sort_attrs.rs

//! Tests for the sortAttrs plugin
//! Auto-generated from SVGO test fixtures

use vexy_svgo::{optimize, OptimizeOptions, Config, PluginConfig};
use vexy_svgo::config::Js2SvgOptions;
use serde_json::json;

fn test_plugin(input: &str, expected: &str, params: Option<serde_json::Value>) {
    let config = Config {
        plugins: vec![PluginConfig {
            name: "sortAttrs".to_string(),
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
        "\nPlugin: sortAttrs\nInput:\n{}\nExpected:\n{}\nActual:\n{}\n", 
        input, expected, output);
}

#[test]
fn test_sort_attrs_01() {
    // Sort according default list and alphabetically
    let input = r#"<svg r=\"\" b=\"\" x2=\"\" cx=\"\" y1=\"\" a=\"\" y=\"\" y2=\"\" x1=\"\" cy=\"\" x=\"\"></svg>"#;

    let expected = r#"<svg x=\"\" x1=\"\" x2=\"\" y=\"\" y1=\"\" y2=\"\" cx=\"\" cy=\"\" r=\"\" a=\"\" b=\"\"/>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_sort_attrs_02() {
    // Sort derived attributes like fill and fill-opacity
    let input = r#"<svg a=\"\" fill-opacity=\"\" stroke=\"\" fill=\"\" stroke-opacity=\"\"></svg>"#;

    let expected = r#"<svg fill=\"\" fill-opacity=\"\" stroke=\"\" stroke-opacity=\"\" a=\"\"/>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_sort_attrs_03() {
    // Put xmlns and namespace attributes before others by default
    let input = r#"<svg xmlns:editor2=\"link\" fill=\"\" b=\"\" xmlns:xlink=\"\" xmlns:editor1=\"link\" xmlns=\"\" d=\"\">
  <rect editor2:b=\"\" editor1:b=\"\" editor2:a=\"\" editor1:a=\"\" />
</svg>"#;

    let expected = r#"<svg xmlns=\"\" xmlns:editor1=\"link\" xmlns:editor2=\"link\" xmlns:xlink=\"\" fill=\"\" d=\"\" b=\"\">
    <rect editor1:a=\"\" editor1:b=\"\" editor2:a=\"\" editor2:b=\"\"/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_sort_attrs_04() {
    // Optionally sort xmlns and xmlns:* attributes alphabetically
    let input = r#"<svg foo=\"bar\" xmlns=\"http://www.w3.org/2000/svg\" height=\"10\" baz=\"quux\" width=\"10\" hello=\"world\">
    <rect x=\"0\" y=\"0\" width=\"100\" height=\"100\" stroke-width=\"1\" stroke-linejoin=\"round\" fill=\"red\" stroke=\"orange\" xmlns=\"http://www.w3.org/2000/svg\"/>
    test
</svg>"#;

    let expected = r#"<svg width=\"10\" height=\"10\" baz=\"quux\" foo=\"bar\" hello=\"world\" xmlns=\"http://www.w3.org/2000/svg\">
    <rect width=\"100\" height=\"100\" x=\"0\" y=\"0\" fill=\"red\" stroke=\"orange\" stroke-linejoin=\"round\" stroke-width=\"1\" xmlns=\"http://www.w3.org/2000/svg\"/>
    test
</svg>"#;

    let params = serde_json::from_str(r#"{
  "xmlnsOrder": "alphabetical"
}"#).ok();

    test_plugin(input, expected, params);
}
