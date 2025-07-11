// this_file: test/plugins/remove_non_inheritable_group_attrs.rs

//! Tests for the removeNonInheritableGroupAttrs plugin
//! Auto-generated from SVGO test fixtures

use vexy_svgo::{optimize, OptimizeOptions, Config, PluginConfig};
use vexy_svgo::config::Js2SvgOptions;
use serde_json::json;

fn test_plugin(input: &str, expected: &str, params: Option<serde_json::Value>) {
    let config = Config {
        plugins: vec![PluginConfig {
            name: "removeNonInheritableGroupAttrs".to_string(),
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
        "\nPlugin: removeNonInheritableGroupAttrs\nInput:\n{}\nExpected:\n{}\nActual:\n{}\n", 
        input, expected, output);
}

#[test]
fn test_remove_non_inheritable_group_attrs_01() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g class=\"test\" clip-path=\"url(#clip1)\" transform=\"rotate(45)\" display=\"none\" opacity=\"0.5\" visibility=\"visible\">
        <path d=\"M0 0 L 10 20\"/>
    </g>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g class=\"test\" clip-path=\"url(#clip1)\" transform=\"rotate(45)\" display=\"none\" opacity=\"0.5\" visibility=\"visible\">
        <path d=\"M0 0 L 10 20\"/>
    </g>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_non_inheritable_group_attrs_02() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g vector-effect=\"non-scaling-stroke\" stroke=\"blue\">
        <path d=\"M0 0 L 10 20\"/>
    </g>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g stroke=\"blue\">
        <path d=\"M0 0 L 10 20\"/>
    </g>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}
