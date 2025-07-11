// this_file: test/plugins/remove_xmlproc_inst.rs

//! Tests for the removeXMLProcInst plugin
//! Auto-generated from SVGO test fixtures

use vexy_svgo::{optimize, OptimizeOptions, Config, PluginConfig};
use vexy_svgo::config::Js2SvgOptions;
use serde_json::json;

fn test_plugin(input: &str, expected: &str, params: Option<serde_json::Value>) {
    let config = Config {
        plugins: vec![PluginConfig {
            name: "removeXMLProcInst".to_string(),
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
        "\nPlugin: removeXMLProcInst\nInput:\n{}\nExpected:\n{}\nActual:\n{}\n", 
        input, expected, output);
}

#[test]
fn test_remove_xmlproc_inst_01() {

    let input = r#"<?xml version=\"1.0\" standalone=\"no\"?>
<svg xmlns=\"http://www.w3.org/2000/svg\">
    test
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    test
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_xmlproc_inst_02() {

    let input = r#"<?xml-stylesheet href=\"style.css\" type=\"text/css\"?>
<svg xmlns=\"http://www.w3.org/2000/svg\">
    test
</svg>"#;

    let expected = r#"<?xml-stylesheet href=\"style.css\" type=\"text/css\"?>
<svg xmlns=\"http://www.w3.org/2000/svg\">
    test
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}
