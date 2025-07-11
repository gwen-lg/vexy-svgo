// this_file: test/plugins/sort_defs_children.rs

//! Tests for the sortDefsChildren plugin
//! Auto-generated from SVGO test fixtures

use vexy_svgo::{optimize, OptimizeOptions, Config, PluginConfig};
use vexy_svgo::config::Js2SvgOptions;
use serde_json::json;

fn test_plugin(input: &str, expected: &str, params: Option<serde_json::Value>) {
    let config = Config {
        plugins: vec![PluginConfig {
            name: "sortDefsChildren".to_string(),
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
        "\nPlugin: sortDefsChildren\nInput:\n{}\nExpected:\n{}\nActual:\n{}\n", 
        input, expected, output);
}

#[test]
fn test_sort_defs_children_01() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <defs>
        <text id=\"a\">
            referenced text
        </text>
        <path id=\"b\" d=\"M0 0zM10 10zM20 20l10 10M30 0c10 0 20 10 20 20M30 30z\"/>
        <text id=\"c\">
            referenced text
        </text>
        <path id=\"d\" d=\"M 30,30 z\"/>
        <circle id=\"e\" fill=\"none\" fill-rule=\"evenodd\" cx=\"60\" cy=\"60\" r=\"50\"/>
        <circle id=\"f\" fill=\"none\" fill-rule=\"evenodd\" cx=\"60\" cy=\"60\" r=\"50\"/>
    </defs>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <defs>
        <circle id=\"e\" fill=\"none\" fill-rule=\"evenodd\" cx=\"60\" cy=\"60\" r=\"50\"/>
        <circle id=\"f\" fill=\"none\" fill-rule=\"evenodd\" cx=\"60\" cy=\"60\" r=\"50\"/>
        <text id=\"a\">
            referenced text
        </text>
        <text id=\"c\">
            referenced text
        </text>
        <path id=\"b\" d=\"M0 0zM10 10zM20 20l10 10M30 0c10 0 20 10 20 20M30 30z\"/>
        <path id=\"d\" d=\"M 30,30 z\"/>
    </defs>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}
