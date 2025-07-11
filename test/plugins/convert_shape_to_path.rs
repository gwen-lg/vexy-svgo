// this_file: test/plugins/convert_shape_to_path.rs

//! Tests for the convertShapeToPath plugin
//! Auto-generated from SVGO test fixtures

use vexy_svgo::{optimize, OptimizeOptions, Config, PluginConfig};
use vexy_svgo::config::Js2SvgOptions;
use serde_json::json;

fn test_plugin(input: &str, expected: &str, params: Option<serde_json::Value>) {
    let config = Config {
        plugins: vec![PluginConfig {
            name: "convertShapeToPath".to_string(),
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
        "\nPlugin: convertShapeToPath\nInput:\n{}\nExpected:\n{}\nActual:\n{}\n", 
        input, expected, output);
}

#[test]
fn test_convert_shape_to_path_01() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <rect width=\"100%\"/>
    <rect width=\"100%\" height=\"100%\"/>
    <rect x=\"25%\" y=\"25%\" width=\"50%\" height=\"50%\"/>
    <rect x=\"25pt\" y=\"25pt\" width=\"50pt\" height=\"50pt\"/>
    <rect x=\"10\" y=\"10\" width=\"50\" height=\"50\" rx=\"4\"/>
    <rect x=\"0\" y=\"0\" width=\"20\" height=\"20\" ry=\"5\"/>
    <rect width=\"32\" height=\"32\"/>
    <rect x=\"20\" y=\"10\" width=\"50\" height=\"40\"/>
    <rect fill=\"#666\" x=\"10\" y=\"10\" width=\"10\" height=\"10\"/>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <rect width=\"100%\"/>
    <rect width=\"100%\" height=\"100%\"/>
    <rect x=\"25%\" y=\"25%\" width=\"50%\" height=\"50%\"/>
    <rect x=\"25pt\" y=\"25pt\" width=\"50pt\" height=\"50pt\"/>
    <rect x=\"10\" y=\"10\" width=\"50\" height=\"50\" rx=\"4\"/>
    <rect x=\"0\" y=\"0\" width=\"20\" height=\"20\" ry=\"5\"/>
    <path d=\"M0 0H32V32H0z\"/>
    <path d=\"M20 10H70V50H20z\"/>
    <path fill=\"#666\" d=\"M10 10H20V20H10z\"/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_convert_shape_to_path_02() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <line x2=\"100%\" y2=\"100%\"/>
    <line x1=\"24\" y2=\"24\"/>
    <line x1=\"10\" y1=\"10\" x2=\"50\" y2=\"20\"/>
    <line stroke=\"#000\" x1=\"10\" y1=\"10\" x2=\"50\" y2=\"20\"/>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <line x2=\"100%\" y2=\"100%\"/>
    <path d=\"M24 0 0 24\"/>
    <path d=\"M10 10 50 20\"/>
    <path stroke=\"#000\" d=\"M10 10 50 20\"/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_convert_shape_to_path_03() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <polyline points=\"10,10 20\"/>
    <polyline points=\"10,80 20,50 50,20 80,10\"/>
    <polyline points=\"20 ,10  50    40 30.5-1e-1 , 20 10\"/>
    <polyline stroke=\"#000\" points=\"10,10 20,20 10,20\"/>
    <polygon points=\"10,10 20\"/>
    <polygon points=\"10,80 20,50 50,20 80,10\"/>
    <polygon points=\"20 10  50 40 30,20\"/>
    <polygon stroke=\"#000\" points=\"10,10 20,20 10,20\"/>
    <polygon stroke=\"none\" points=\"10,10 20,20 10,20\"/>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <path d=\"M10 80 20 50 50 20 80 10\"/>
    <path d=\"M20 10 50 40 30.5-.1 20 10\"/>
    <path stroke=\"#000\" d=\"M10 10 20 20 10 20\"/>
    <path d=\"M10 80 20 50 50 20 80 10z\"/>
    <path d=\"M20 10 50 40 30 20z\"/>
    <path stroke=\"#000\" d=\"M10 10 20 20 10 20z\"/>
    <path stroke=\"none\" d=\"M10 10 20 20 10 20z\"/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_convert_shape_to_path_04() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <circle cx=\"10\" cy=\"10\" r=\"5\"/>
    <ellipse cx=\"10\" cy=\"10\" rx=\"5\" ry=\"5\"/>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <path d=\"M10 5A5 5 0 1 0 10 15 5 5 0 1 0 10 5z\"/>
    <path d=\"M10 5A5 5 0 1 0 10 15 5 5 0 1 0 10 5z\"/>
</svg>"#;

    let params = serde_json::from_str(r#"{
  "convertArcs": true
}"#).ok();

    test_plugin(input, expected, params);
}

#[test]
fn test_convert_shape_to_path_05() {
    // Precision should be applied to all converted shapes
    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"65mm\" height=\"45mm\" viewBox=\"0 0 65 45\">
  <rect x=\"26.614\" y=\"29.232\" width=\"34.268\" height=\"8.1757\"/>
  <line x1=\"26.6142\" y1=\"29.2322\" x2=\"34.2682\" y2=\"8.1757\"/>
  <polyline points=\"26.6142,29.2322 34.2682,8.1757\"/>
  <polygon points=\"26.6142,29.2322 34.2682,8.1757\"/>
  <circle cx=\"26.6142\" cy=\"29.2322\" r=\"34.2682\"/>
  <ellipse cx=\"26.6142\" cy=\"29.2322\" rx=\"34.2682\" ry=\"8.1757\"/>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"65mm\" height=\"45mm\" viewBox=\"0 0 65 45\">
    <path d=\"M26.614 29.232H60.882V37.408H26.614z\"/>
    <path d=\"M26.614 29.232 34.268 8.176\"/>
    <path d=\"M26.614 29.232 34.268 8.176\"/>
    <path d=\"M26.614 29.232 34.268 8.176z\"/>
    <path d=\"M26.614-5.036A34.268 34.268 0 1 0 26.614 63.5 34.268 34.268 0 1 0 26.614-5.036z\"/>
    <path d=\"M26.614 21.057A34.268 8.176 0 1 0 26.614 37.408 34.268 8.176 0 1 0 26.614 21.057z\"/>
</svg>"#;

    let params = serde_json::from_str(r#"{
  "floatPrecision": 3,
  "convertArcs": true
}"#).ok();

    test_plugin(input, expected, params);
}
