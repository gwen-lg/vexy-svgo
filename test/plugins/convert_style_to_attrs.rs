// this_file: test/plugins/convert_style_to_attrs.rs

//! Tests for the convertStyleToAttrs plugin
//! Auto-generated from SVGO test fixtures

use vexy_svgo::{optimize, OptimizeOptions, Config, PluginConfig};
use vexy_svgo::config::Js2SvgOptions;
use serde_json::json;

fn test_plugin(input: &str, expected: &str, params: Option<serde_json::Value>) {
    let config = Config {
        plugins: vec![PluginConfig {
            name: "convertStyleToAttrs".to_string(),
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
        "\nPlugin: convertStyleToAttrs\nInput:\n{}\nExpected:\n{}\nActual:\n{}\n", 
        input, expected, output);
}

#[test]
fn test_convert_style_to_attrs_01() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g style=\"fill:#000;\"/>
    <g style=\"font-family:'Helvetica Neue'\"/>
    <g style=\"    fill:#000; color: #fff  ;  \"/>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g fill=\"#000\"/>
    <g font-family=\"Helvetica Neue\"/>
    <g fill=\"#000\" color=\"#fff\"/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_convert_style_to_attrs_02() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g style=\"    fill:#000; c\\olor: #fff; /**/illegal-'declaration/*'; -webkit-blah: 123  ; -webkit-trolo: 'lolo'; illegal2*/\"/>
    <g style=\"font:15px serif\"/>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g style=\"-webkit-blah:123;-webkit-trolo:'lolo'\" fill=\"#000\" color=\"#fff\"/>
    <g style=\"font:15px serif\"/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_convert_style_to_attrs_03() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g style=\"background/*-image*/:url(data:image/png;base64,iVBORw...)\"/>
    <g style=\"fill:url(data:image/png;base64,iVBORw...)\"/>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g style=\"background:url(data:image/png;base64,iVBORw...)\"/>
    <g fill=\"url(data:image/png;base64,iVBORw...)\"/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_convert_style_to_attrs_04() {

    let input = r#"<svg id=\"test\" xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 100 100\">
    <rect width=\"100\" height=\"100\" class=\"blue red\" style=\"fill:red!important\"/>
</svg>"#;

    let expected = r#"<svg id=\"test\" xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 100 100\">
    <rect width=\"100\" height=\"100\" class=\"blue red\" fill=\"red\"/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_convert_style_to_attrs_05() {

    let input = r#"<svg id=\"test\" xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 100 100\">
    <rect width=\"100\" height=\"100\" class=\"blue red\" style=\"fill:red!important\"/>
</svg>"#;

    let expected = r#"<svg id=\"test\" xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 100 100\">
    <rect width=\"100\" height=\"100\" class=\"blue red\" style=\"fill:red!important\"/>
</svg>"#;

    let params = serde_json::from_str(r#"{
  "keepImportant": true
}"#).ok();

    test_plugin(input, expected, params);
}
