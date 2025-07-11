// this_file: test/plugins/remove_view_box.rs

//! Tests for the removeViewBox plugin
//! Auto-generated from SVGO test fixtures

use vexy_svgo::{optimize, OptimizeOptions, Config, PluginConfig};
use vexy_svgo::config::Js2SvgOptions;
use serde_json::json;

fn test_plugin(input: &str, expected: &str, params: Option<serde_json::Value>) {
    let config = Config {
        plugins: vec![PluginConfig {
            name: "removeViewBox".to_string(),
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
        "\nPlugin: removeViewBox\nInput:\n{}\nExpected:\n{}\nActual:\n{}\n", 
        input, expected, output);
}

#[test]
fn test_remove_view_box_01() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"100.5\" height=\".5\" viewBox=\"0 0 100.5 .5\">
    test
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"100.5\" height=\".5\">
    test
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_view_box_02() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"50\" height=\"50\" viewBox=\"0 0 100 50\">
    test
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"50\" height=\"50\" viewBox=\"0 0 100 50\">
    test
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_view_box_03() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"100\" height=\"50\" viewBox=\"0, 0, 100, 50\">
    test
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"100\" height=\"50\">
    test
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_view_box_04() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"50\" height=\"50\" viewBox=\"-25 -25 50 50\">
    test
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"50\" height=\"50\" viewBox=\"-25 -25 50 50\">
    test
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_view_box_05() {
    // ViewBox in nested <svg> should be preserved to not break scale
    let input = r#"<svg width=\"480\" height=\"360\" viewBox=\"0 0 480 360\" xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\">
  <defs>
    <svg id=\"svg-sub-root\" viewBox=\"0 0 450 450\" width=\"450\" height=\"450\">
      <rect x=\"225\" y=\"0\" width=\"220\" height=\"220\" style=\"fill:magenta\"/>
      <rect x=\"0\" y=\"225\" width=\"220\" height=\"220\" style=\"fill:#f0f\"/>
      <rect x=\"225\" y=\"225\" width=\"220\" height=\"220\" fill=\"#f0f\"/>
    </svg>
  </defs>
  <use x=\"60\" y=\"50\" width=\"240\" height=\"240\" xlink:href=\"#svg-sub-root\"/>
  <rect x=\"300\" y=\"170\" width=\"118\" height=\"118\" fill=\"magenta\"/>
</svg>"#;

    let expected = r#"<svg width=\"480\" height=\"360\" xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\">
    <defs>
        <svg id=\"svg-sub-root\" viewBox=\"0 0 450 450\" width=\"450\" height=\"450\">
            <rect x=\"225\" y=\"0\" width=\"220\" height=\"220\" style=\"fill:magenta\"/>
            <rect x=\"0\" y=\"225\" width=\"220\" height=\"220\" style=\"fill:#f0f\"/>
            <rect x=\"225\" y=\"225\" width=\"220\" height=\"220\" fill=\"#f0f\"/>
        </svg>
    </defs>
    <use x=\"60\" y=\"50\" width=\"240\" height=\"240\" xlink:href=\"#svg-sub-root\"/>
    <rect x=\"300\" y=\"170\" width=\"118\" height=\"118\" fill=\"magenta\"/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}
