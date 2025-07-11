// this_file: test/plugins/remove_style_element.rs

//! Tests for the removeStyleElement plugin
//! Auto-generated from SVGO test fixtures

use vexy_svgo::{optimize, OptimizeOptions, Config, PluginConfig};
use vexy_svgo::config::Js2SvgOptions;
use serde_json::json;

fn test_plugin(input: &str, expected: &str, params: Option<serde_json::Value>) {
    let config = Config {
        plugins: vec![PluginConfig {
            name: "removeStyleElement".to_string(),
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
        "\nPlugin: removeStyleElement\nInput:\n{}\nExpected:\n{}\nActual:\n{}\n", 
        input, expected, output);
}

#[test]
fn test_remove_style_element_01() {

    let input = r#"<?xml version=\"1.0\" encoding=\"utf-16\"?>
    <svg version=\"1.1\" id=\"Layer_1\" xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\" x=\"0px\" y=\"0px\" viewBox=\"0 0 100 100\" style=\"enable-background:new 0 0 100 100;\" xml:space=\"preserve\">
        <style type=\"text/css\">
        .st0 {
            fill: #231F20;
        }
        </style>
        <circle class=\"st0\" cx=\"50\" cy=\"50\" r=\"50\" />
    </svg>"#;

    let expected = r#"<?xml version=\"1.0\" encoding=\"utf-16\"?>
<svg version=\"1.1\" id=\"Layer_1\" xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\" x=\"0px\" y=\"0px\" viewBox=\"0 0 100 100\" style=\"enable-background:new 0 0 100 100;\" xml:space=\"preserve\">
    <circle class=\"st0\" cx=\"50\" cy=\"50\" r=\"50\"/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}
