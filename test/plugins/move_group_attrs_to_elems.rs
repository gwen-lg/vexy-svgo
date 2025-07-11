// this_file: test/plugins/move_group_attrs_to_elems.rs

//! Tests for the moveGroupAttrsToElems plugin
//! Auto-generated from SVGO test fixtures

use vexy_svgo::{optimize, OptimizeOptions, Config, PluginConfig};
use vexy_svgo::config::Js2SvgOptions;
use serde_json::json;

fn test_plugin(input: &str, expected: &str, params: Option<serde_json::Value>) {
    let config = Config {
        plugins: vec![PluginConfig {
            name: "moveGroupAttrsToElems".to_string(),
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
        "\nPlugin: moveGroupAttrsToElems\nInput:\n{}\nExpected:\n{}\nActual:\n{}\n", 
        input, expected, output);
}

#[test]
fn test_move_group_attrs_to_elems_01() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g transform=\"scale(2)\">
        <path transform=\"rotate(45)\" d=\"M0,0 L10,20\"/>
        <path transform=\"translate(10, 20)\" d=\"M0,10 L20,30\"/>
    </g>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g>
        <path transform=\"scale(2) rotate(45)\" d=\"M0,0 L10,20\"/>
        <path transform=\"scale(2) translate(10, 20)\" d=\"M0,10 L20,30\"/>
    </g>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_move_group_attrs_to_elems_02() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g transform=\"scale(2)\">
        <path d=\"M0,0 L10,20\"/>
        <path d=\"M0,10 L20,30\"/>
    </g>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g>
        <path d=\"M0,0 L10,20\" transform=\"scale(2)\"/>
        <path d=\"M0,10 L20,30\" transform=\"scale(2)\"/>
    </g>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_move_group_attrs_to_elems_03() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g transform=\"rotate(30)\">
        <g transform=\"scale(2)\">
            <path d=\"M0,0 L10,20\"/>
            <path d=\"M0,10 L20,30\"/>
        </g>
    </g>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g>
        <g>
            <path d=\"M0,0 L10,20\" transform=\"rotate(30) scale(2)\"/>
            <path d=\"M0,10 L20,30\" transform=\"rotate(30) scale(2)\"/>
        </g>
    </g>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_move_group_attrs_to_elems_04() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g transform=\"rotate(30)\">
        <g>
            <g transform=\"scale(2)\">
                <path d=\"M0,0 L10,20\"/>
                <path d=\"M0,10 L20,30\"/>
            </g>
        </g>
        <path d=\"M0,10 L20,30\"/>
    </g>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g>
        <g>
            <g>
                <path d=\"M0,0 L10,20\" transform=\"rotate(30) scale(2)\"/>
                <path d=\"M0,10 L20,30\" transform=\"rotate(30) scale(2)\"/>
            </g>
        </g>
        <path d=\"M0,10 L20,30\" transform=\"rotate(30)\"/>
    </g>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_move_group_attrs_to_elems_05() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g transform=\"scale(2)\" clip-path=\"url(#a)\">
        <path d=\"M0,0 L10,20\"/>
        <path d=\"M0,10 L20,30\"/>
    </g>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g transform=\"scale(2)\" clip-path=\"url(#a)\">
        <path d=\"M0,0 L10,20\"/>
        <path d=\"M0,10 L20,30\"/>
    </g>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_move_group_attrs_to_elems_06() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\">
    <g transform=\"translate(0 -140)\">
        <path id=\"c\" transform=\"scale(.5)\" d=\"M0,0 L10,20\"/>
    </g>
    <use xlink:href=\"#c\" transform=\"translate(-140)\"/>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\">
    <g transform=\"translate(0 -140)\">
        <path id=\"c\" transform=\"scale(.5)\" d=\"M0,0 L10,20\"/>
    </g>
    <use xlink:href=\"#c\" transform=\"translate(-140)\"/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}
