// this_file: test/plugins/remove_elements_by_attr.rs

//! Tests for the removeElementsByAttr plugin
//! Auto-generated from SVGO test fixtures

use vexy_svgo::{optimize, OptimizeOptions, Config, PluginConfig};
use vexy_svgo::config::Js2SvgOptions;
use serde_json::json;

fn test_plugin(input: &str, expected: &str, params: Option<serde_json::Value>) {
    let config = Config {
        plugins: vec![PluginConfig {
            name: "removeElementsByAttr".to_string(),
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
        "\nPlugin: removeElementsByAttr\nInput:\n{}\nExpected:\n{}\nActual:\n{}\n", 
        input, expected, output);
}

#[test]
fn test_remove_elements_by_attr_01() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"19\" height=\"19\" viewBox=\"0 0 19 19\">
    <rect id=\"someID\" width=\"19\" height=\"19\"/>
    <path id=\"close\" d=\"M1093.5,31.792l-0.72.721-8.27-8.286-8.28,8.286-0.72-.721,8.28-8.286-8.28-8.286,0.72-.721,8.28,8.286,8.27-8.286,0.72,0.721-8.27,8.286Z\" transform=\"translate(-1075 -14)\"/>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"19\" height=\"19\" viewBox=\"0 0 19 19\">
    <rect id=\"someID\" width=\"19\" height=\"19\"/>
    <path id=\"close\" d=\"M1093.5,31.792l-0.72.721-8.27-8.286-8.28,8.286-0.72-.721,8.28-8.286-8.28-8.286,0.72-.721,8.28,8.286,8.27-8.286,0.72,0.721-8.27,8.286Z\" transform=\"translate(-1075 -14)\"/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_elements_by_attr_02() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"19\" height=\"19\" viewBox=\"0 0 19 19\">
    <rect id=\"someID\" width=\"19\" height=\"19\"/>
    <path id=\"close\" d=\"M1093.5,31.792l-0.72.721-8.27-8.286-8.28,8.286-0.72-.721,8.28-8.286-8.28-8.286,0.72-.721,8.28,8.286,8.27-8.286,0.72,0.721-8.27,8.286Z\" transform=\"translate(-1075 -14)\"/>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"19\" height=\"19\" viewBox=\"0 0 19 19\">
    <path id=\"close\" d=\"M1093.5,31.792l-0.72.721-8.27-8.286-8.28,8.286-0.72-.721,8.28-8.286-8.28-8.286,0.72-.721,8.28,8.286,8.27-8.286,0.72,0.721-8.27,8.286Z\" transform=\"translate(-1075 -14)\"/>
</svg>"#;

    let params = serde_json::from_str(r#"{
  "id": "someID"
}"#).ok();

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_elements_by_attr_03() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"19\" height=\"19\" viewBox=\"0 0 19 19\">
    <rect id=\"someID\" width=\"19\" height=\"19\"/>
    <path id=\"anotherID\" d=\"M1093.5,31.792l-0.72.721-8.27-8.286-8.28,8.286-0.72-.721,8.28-8.286-8.28-8.286,0.72-.721,8.28,8.286,8.27-8.286,0.72,0.721-8.27,8.286Z\" transform=\"translate(-1075 -14)\"/>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"19\" height=\"19\" viewBox=\"0 0 19 19\"/>"#;

    let params = serde_json::from_str(r#"{
  "id": [
    "someID",
    "anotherID"
  ]
}"#).ok();

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_elements_by_attr_04() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"19\" height=\"19\" viewBox=\"0 0 19 19\">
    <rect class=\"someClass\" width=\"19\" height=\"19\"/>
    <path class=\"close\" d=\"M1093.5,31.792l-0.72.721-8.27-8.286-8.28,8.286-0.72-.721,8.28-8.286-8.28-8.286,0.72-.721,8.28,8.286,8.27-8.286,0.72,0.721-8.27,8.286Z\" transform=\"translate(-1075 -14)\"/>
    <rect class=\"someClass extraClass\"/>
    <rect class=\"SOMEclass case-sensitive\"/>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"19\" height=\"19\" viewBox=\"0 0 19 19\">
    <path class=\"close\" d=\"M1093.5,31.792l-0.72.721-8.27-8.286-8.28,8.286-0.72-.721,8.28-8.286-8.28-8.286,0.72-.721,8.28,8.286,8.27-8.286,0.72,0.721-8.27,8.286Z\" transform=\"translate(-1075 -14)\"/>
    <rect class=\"SOMEclass case-sensitive\"/>
</svg>"#;

    let params = serde_json::from_str(r#"{
  "class": "someClass"
}"#).ok();

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_elements_by_attr_05() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"19\" height=\"19\" viewBox=\"0 0 19 19\">
    <rect class=\"someClass\" width=\"19\" height=\"19\"/>
    <path class=\"anotherClass\" d=\"M1093.5,31.792l-0.72.721-8.27-8.286-8.28,8.286-0.72-.721,8.28-8.286-8.28-8.286,0.72-.721,8.28,8.286,8.27-8.286,0.72,0.721-8.27,8.286Z\" transform=\"translate(-1075 -14)\"/>
    <rect class=\"someClass extraClass\"/>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"19\" height=\"19\" viewBox=\"0 0 19 19\"/>"#;

    let params = serde_json::from_str(r#"{
  "class": [
    "someClass",
    "anotherClass"
  ]
}"#).ok();

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_elements_by_attr_06() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"19\" height=\"19\" viewBox=\"0 0 19 19\">
    <rect class=\"someClass\" width=\"19\" height=\"19\"/>
    <path class=\"someClass extraClass\" d=\"M1093.5,31.792l-0.72.721-8.27-8.286-8.28,8.286-0.72-.721,8.28-8.286-8.28-8.286,0.72-.721,8.28,8.286,8.27-8.286,0.72,0.721-8.27,8.286Z\" transform=\"translate(-1075 -14)\"/>
    <rect class=\"anotherClass\"/>
    <path id=\"someID\" class=\"anotherID\"/>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"19\" height=\"19\" viewBox=\"0 0 19 19\">
    <rect class=\"anotherClass\"/>
</svg>"#;

    let params = serde_json::from_str(r#"{
  "id": "someID",
  "class": "someClass"
}"#).ok();

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_elements_by_attr_07() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"19\" height=\"19\" viewBox=\"0 0 19 19\">
    <rect class=\"some-class\" width=\"19\" height=\"19\"/>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"19\" height=\"19\" viewBox=\"0 0 19 19\">
    <rect class=\"some-class\" width=\"19\" height=\"19\"/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}
