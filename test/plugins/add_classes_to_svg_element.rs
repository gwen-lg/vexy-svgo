// this_file: test/plugins/add_classes_to_svg_element.rs

//! Tests for the addClassesToSVGElement plugin
//! Auto-generated from SVGO test fixtures

use vexy_svgo::{optimize, OptimizeOptions, Config, PluginConfig};
use vexy_svgo::config::Js2SvgOptions;
use serde_json::json;

fn test_plugin(input: &str, expected: &str, params: Option<serde_json::Value>) {
    let config = Config {
        plugins: vec![PluginConfig {
            name: "addClassesToSVGElement".to_string(),
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
        "\nPlugin: addClassesToSVGElement\nInput:\n{}\nExpected:\n{}\nActual:\n{}\n", 
        input, expected, output);
}

#[test]
fn test_add_classes_to_svg_element_01() {
    // Should add classes when passed as a classNames Array
    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    test
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" class=\"mySvg size-big\">
    test
</svg>"#;

    let params = serde_json::from_str(r#"{
  "classNames": [
    "mySvg",
    "size-big"
  ]
}"#).ok();

    test_plugin(input, expected, params);
}

#[test]
fn test_add_classes_to_svg_element_02() {
    // Should add class when passed as a className String
    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    test
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" class=\"mySvg\">
    test
</svg>"#;

    let params = serde_json::from_str(r#"{
  "className": "mySvg"
}"#).ok();

    test_plugin(input, expected, params);
}

#[test]
fn test_add_classes_to_svg_element_03() {
    // Should avoid adding existing classes
    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" class=\"mySvg\">
    test
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" class=\"mySvg size-big\">
    test
</svg>"#;

    let params = serde_json::from_str(r#"{
  "classNames": [
    "mySvg",
    "size-big"
  ]
}"#).ok();

    test_plugin(input, expected, params);
}
