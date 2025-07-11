// this_file: test/plugins/add_attributes_to_svg_element.rs

//! Tests for the addAttributesToSVGElement plugin
//! Auto-generated from SVGO test fixtures

use vexy_svgo::{optimize, OptimizeOptions, Config, PluginConfig};
use vexy_svgo::config::Js2SvgOptions;
use serde_json::json;

fn test_plugin(input: &str, expected: &str, params: Option<serde_json::Value>) {
    let config = Config {
        plugins: vec![PluginConfig {
            name: "addAttributesToSVGElement".to_string(),
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
        "\nPlugin: addAttributesToSVGElement\nInput:\n{}\nExpected:\n{}\nActual:\n{}\n", 
        input, expected, output);
}

#[test]
fn test_add_attributes_to_svg_element_01() {
    // Add multiple attributes without value
    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    test
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" data-icon className={classes}>
    test
</svg>"#;

    let params = serde_json::from_str(r#"{
  "attributes": [
    "data-icon",
    "className={classes}"
  ]
}"#).ok();

    test_plugin(input, expected, params);
}

#[test]
fn test_add_attributes_to_svg_element_02() {
    // Add single attribute without value
    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    test
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" data-icon>
    test
</svg>"#;

    let params = serde_json::from_str(r#"{
  "attribute": "data-icon"
}"#).ok();

    test_plugin(input, expected, params);
}

#[test]
fn test_add_attributes_to_svg_element_03() {
    // Add multiple attributes with values
    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    test
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" focusable=\"false\" data-image=\"icon\">
    test
</svg>"#;

    let params = serde_json::from_str(r#"{
  "attributes": [
    {
      "focusable": "false"
    },
    {
      "data-image": "icon"
    }
  ]
}"#).ok();

    test_plugin(input, expected, params);
}

#[test]
fn test_add_attributes_to_svg_element_04() {
    // Ignore nested <svg> elements
    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    test
    <svg />
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" data-icon>
    test
    <svg/>
</svg>"#;

    let params = serde_json::from_str(r#"{
  "attributes": [
    "data-icon"
  ]
}"#).ok();

    test_plugin(input, expected, params);
}
