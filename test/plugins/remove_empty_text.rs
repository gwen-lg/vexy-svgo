// this_file: test/plugins/remove_empty_text.rs

//! Tests for the removeEmptyText plugin
//! Auto-generated from SVGO test fixtures

use vexy_svgo::{optimize, OptimizeOptions, Config, PluginConfig};
use vexy_svgo::config::Js2SvgOptions;
use serde_json::json;

fn test_plugin(input: &str, expected: &str, params: Option<serde_json::Value>) {
    let config = Config {
        plugins: vec![PluginConfig {
            name: "removeEmptyText".to_string(),
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
        "\nPlugin: removeEmptyText\nInput:\n{}\nExpected:\n{}\nActual:\n{}\n", 
        input, expected, output);
}

#[test]
fn test_remove_empty_text_01() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g>
        <text></text>
    </g>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_empty_text_02() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g>
        <tspan></tspan>
    </g>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_empty_text_03() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g>
        <tref>...</tref>
    </g>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}
