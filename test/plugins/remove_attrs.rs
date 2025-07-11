// this_file: test/plugins/remove_attrs.rs

//! Tests for the removeAttrs plugin
//! Auto-generated from SVGO test fixtures

use vexy_svgo::{optimize, OptimizeOptions, Config, PluginConfig};
use vexy_svgo::config::Js2SvgOptions;
use serde_json::json;

fn test_plugin(input: &str, expected: &str, params: Option<serde_json::Value>) {
    let config = Config {
        plugins: vec![PluginConfig {
            name: "removeAttrs".to_string(),
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
        "\nPlugin: removeAttrs\nInput:\n{}\nExpected:\n{}\nActual:\n{}\n", 
        input, expected, output);
}

#[test]
fn test_remove_attrs_01() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <circle fill=\"red\" stroke-width=\"6\" stroke-dashoffset=\"5\" cx=\"60\" cy=\"60\" r=\"50\"/>
    <circle fill=\"red\" stroke=\"#000\" stroke-width=\"6\" stroke-dashoffset=\"5\" stroke-opacity=\"0\" cx=\"60\" cy=\"60\" r=\"50\"/>
    <circle stroke=\"#000\" stroke-width=\"6\" stroke-dashoffset=\"5\" stroke-opacity=\"0\" cx=\"60\" cy=\"60\" r=\"50\"/>
    <path fill=\"red\" stroke=\"red\" d=\"M100,200 300,400 H100 V300 C100,100 250,100 250,200 S400,300 400,200 Q400,50 600,300 T1000,300 z\"/>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <circle fill=\"red\" cx=\"60\" cy=\"60\" r=\"50\"/>
    <circle fill=\"red\" cx=\"60\" cy=\"60\" r=\"50\"/>
    <circle cx=\"60\" cy=\"60\" r=\"50\"/>
    <path stroke=\"red\" d=\"M100,200 300,400 H100 V300 C100,100 250,100 250,200 S400,300 400,200 Q400,50 600,300 T1000,300 z\"/>
</svg>"#;

    let params = serde_json::from_str(r#"{
  "attrs": [
    "circle:stroke.*",
    "path:fill"
  ]
}"#).ok();

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_attrs_02() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <circle fill=\"red\" stroke-width=\"6\" stroke-dashoffset=\"5\" cx=\"60\" cy=\"60\" r=\"50\"/>
    <circle fill=\"red\" stroke=\"#000\" stroke-width=\"6\" stroke-dashoffset=\"5\" stroke-opacity=\"0\" cx=\"60\" cy=\"60\" r=\"50\"/>
    <circle stroke=\"#000\" stroke-width=\"6\" stroke-dashoffset=\"5\" stroke-opacity=\"0\" cx=\"60\" cy=\"60\" r=\"50\"/>
    <path fill=\"red\" stroke=\"red\" d=\"M100,200 300,400 H100 V300 C100,100 250,100 250,200 S400,300 400,200 Q400,50 600,300 T1000,300 z\"/>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <circle stroke-width=\"6\" stroke-dashoffset=\"5\" cx=\"60\" cy=\"60\" r=\"50\"/>
    <circle stroke-width=\"6\" stroke-dashoffset=\"5\" stroke-opacity=\"0\" cx=\"60\" cy=\"60\" r=\"50\"/>
    <circle stroke-width=\"6\" stroke-dashoffset=\"5\" stroke-opacity=\"0\" cx=\"60\" cy=\"60\" r=\"50\"/>
    <path d=\"M100,200 300,400 H100 V300 C100,100 250,100 250,200 S400,300 400,200 Q400,50 600,300 T1000,300 z\"/>
</svg>"#;

    let params = serde_json::from_str(r#"{
  "attrs": "(fill|stroke)"
}"#).ok();

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_attrs_03() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <circle fill=\"currentColor\" stroke-width=\"6\" stroke-dashoffset=\"5\" cx=\"60\" cy=\"60\" r=\"50\"/>
    <circle fill=\"red\" stroke=\"#000\" stroke-width=\"6\" stroke-dashoffset=\"5\" stroke-opacity=\"0\" cx=\"60\" cy=\"60\" r=\"50\"/>
    <circle stroke=\"currentColor\" stroke-width=\"6\" stroke-dashoffset=\"5\" stroke-opacity=\"0\" cx=\"60\" cy=\"60\" r=\"50\"/>
    <path fill=\"red\" stroke=\"red\" d=\"M100,200 300,400 H100 V300 C100,100 250,100 250,200 S400,300 400,200 Q400,50 600,300 T1000,300 z\"/>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <circle fill=\"currentColor\" stroke-width=\"6\" stroke-dashoffset=\"5\" cx=\"60\" cy=\"60\" r=\"50\"/>
    <circle stroke-width=\"6\" stroke-dashoffset=\"5\" stroke-opacity=\"0\" cx=\"60\" cy=\"60\" r=\"50\"/>
    <circle stroke=\"currentColor\" stroke-width=\"6\" stroke-dashoffset=\"5\" stroke-opacity=\"0\" cx=\"60\" cy=\"60\" r=\"50\"/>
    <path d=\"M100,200 300,400 H100 V300 C100,100 250,100 250,200 S400,300 400,200 Q400,50 600,300 T1000,300 z\"/>
</svg>"#;

    let params = serde_json::from_str(r#"{
  "attrs": "(fill|stroke)",
  "preserveCurrentColor": true
}"#).ok();

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_attrs_04() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <circle fill=\"red\" stroke-width=\"6\" stroke-dashoffset=\"5\" cx=\"60\" cy=\"60\" r=\"50\"/>
    <circle fill=\"red\" stroke=\"#000\" stroke-width=\"6\" stroke-dashoffset=\"5\" stroke-opacity=\"0\" cx=\"60\" cy=\"60\" r=\"50\"/>
    <circle stroke=\"#000\" stroke-width=\"6\" stroke-dashoffset=\"5\" stroke-opacity=\"0\" cx=\"60\" cy=\"60\" r=\"50\"/>
    <circle stroke=\"#FFF\" stroke-width=\"6\" stroke-dashoffset=\"5\" stroke-opacity=\"0\" cx=\"60\" cy=\"60\" r=\"25\"/>
    <path fill=\"red\" stroke=\"red\" d=\"M100,200 300,400 H100 V300 C100,100 250,100 250,200 S400,300 400,200 Q400,50 600,300 T1000,300 z\"/>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <circle stroke-width=\"6\" stroke-dashoffset=\"5\" cx=\"60\" cy=\"60\" r=\"50\"/>
    <circle stroke=\"#000\" stroke-width=\"6\" stroke-dashoffset=\"5\" stroke-opacity=\"0\" cx=\"60\" cy=\"60\" r=\"50\"/>
    <circle stroke=\"#000\" stroke-width=\"6\" stroke-dashoffset=\"5\" stroke-opacity=\"0\" cx=\"60\" cy=\"60\" r=\"50\"/>
    <circle stroke=\"#FFF\" stroke-width=\"6\" stroke-dashoffset=\"5\" stroke-opacity=\"0\" cx=\"60\" cy=\"60\" r=\"25\"/>
    <path d=\"M100,200 300,400 H100 V300 C100,100 250,100 250,200 S400,300 400,200 Q400,50 600,300 T1000,300 z\"/>
</svg>"#;

    let params = serde_json::from_str(r#"{
  "attrs": "*:(stroke|fill):red"
}"#).ok();

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_attrs_05() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <circle fill=\"red\" stroke-width=\"6\" stroke-dashoffset=\"5\" cx=\"60\" cy=\"60\" r=\"50\"/>
    <circle fill=\"red\" stroke=\"#000\" stroke-width=\"6\" stroke-dashoffset=\"5\" stroke-opacity=\"0\" cx=\"60\" cy=\"60\" r=\"50\"/>
    <circle stroke=\"#000\" stroke-width=\"6\" stroke-dashoffset=\"5\" stroke-opacity=\"0\" cx=\"60\" cy=\"60\" r=\"50\"/>
    <circle stroke=\"#FFF\" stroke-width=\"6\" stroke-dashoffset=\"5\" stroke-opacity=\"0\" cx=\"60\" cy=\"60\" r=\"25\"/>
    <path fill=\"red\" stroke=\"red\" d=\"M100,200 300,400 H100 V300 C100,100 250,100 250,200 S400,300 400,200 Q400,50 600,300 T1000,300 z\"/>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <circle stroke-width=\"6\" stroke-dashoffset=\"5\" cx=\"60\" cy=\"60\" r=\"50\"/>
    <circle stroke-width=\"6\" stroke-dashoffset=\"5\" stroke-opacity=\"0\" cx=\"60\" cy=\"60\" r=\"50\"/>
    <circle stroke-width=\"6\" stroke-dashoffset=\"5\" stroke-opacity=\"0\" cx=\"60\" cy=\"60\" r=\"50\"/>
    <circle stroke=\"#FFF\" stroke-width=\"6\" stroke-dashoffset=\"5\" stroke-opacity=\"0\" cx=\"60\" cy=\"60\" r=\"25\"/>
    <path d=\"M100,200 300,400 H100 V300 C100,100 250,100 250,200 S400,300 400,200 Q400,50 600,300 T1000,300 z\"/>
</svg>"#;

    let params = serde_json::from_str(r#"{
  "attrs": "*:(stroke|fill):((?!^#FFF$).)*"
}"#).ok();

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_attrs_06() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <circle fill=\"red\" stroke-width=\"6\" stroke-dashoffset=\"5\" cx=\"60\" cy=\"60\" r=\"50\"/>
    <circle fill=\"red\" stroke=\"#000\" stroke-width=\"6\" stroke-dashoffset=\"5\" stroke-opacity=\"0\" cx=\"60\" cy=\"60\" r=\"50\"/>
    <circle stroke=\"#000\" stroke-width=\"6\" stroke-dashoffset=\"5\" stroke-opacity=\"0\" cx=\"60\" cy=\"60\" r=\"50\"/>
    <circle stroke=\"#FFF\" stroke-width=\"6\" stroke-dashoffset=\"5\" stroke-opacity=\"0\" cx=\"60\" cy=\"60\" r=\"25\"/>
    <path fill=\"red\" stroke=\"red\" d=\"M100,200 300,400 H100 V300 C100,100 250,100 250,200 S400,300 400,200 Q400,50 600,300 T1000,300 z\"/>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <circle fill=\"red\" stroke-width=\"6\" stroke-dashoffset=\"5\" cx=\"60\" cy=\"60\" r=\"50\"/>
    <circle fill=\"red\" stroke=\"#000\" stroke-width=\"6\" stroke-dashoffset=\"5\" stroke-opacity=\"0\" cx=\"60\" cy=\"60\" r=\"50\"/>
    <circle stroke=\"#000\" stroke-width=\"6\" stroke-dashoffset=\"5\" stroke-opacity=\"0\" cx=\"60\" cy=\"60\" r=\"50\"/>
    <circle stroke=\"#FFF\" stroke-width=\"6\" stroke-dashoffset=\"5\" stroke-opacity=\"0\" cx=\"60\" cy=\"60\" r=\"25\"/>
    <path fill=\"red\" stroke=\"red\" d=\"M100,200 300,400 H100 V300 C100,100 250,100 250,200 S400,300 400,200 Q400,50 600,300 T1000,300 z\"/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_attrs_07() {
    // The preserveCurrentColor param should be case-insensitive.
    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 150 150\">
    <linearGradient id=\"A\">
        <stop stop-color=\"ReD\" offset=\"5%\"/>
    </linearGradient>
    <text x=\"0\" y=\"32\" fill=\"currentColor\">uwu</text>
    <text x=\"0\" y=\"64\" fill=\"currentcolor\">owo</text>
    <text x=\"0\" y=\"96\" fill=\"url(#A)\">eue</text>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 150 150\">
    <linearGradient id=\"A\">
        <stop stop-color=\"ReD\" offset=\"5%\"/>
    </linearGradient>
    <text x=\"0\" y=\"32\" fill=\"currentColor\">uwu</text>
    <text x=\"0\" y=\"64\" fill=\"currentcolor\">owo</text>
    <text x=\"0\" y=\"96\">eue</text>
</svg>"#;

    let params = serde_json::from_str(r#"{
  "attrs": "fill",
  "preserveCurrentColor": true
}"#).ok();

    test_plugin(input, expected, params);
}
