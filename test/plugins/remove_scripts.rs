// this_file: test/plugins/remove_scripts.rs

//! Tests for the removeScripts plugin
//! Auto-generated from SVGO test fixtures

use vexy_svgo::{optimize, OptimizeOptions, Config, PluginConfig};
use vexy_svgo::config::Js2SvgOptions;
use serde_json::json;

fn test_plugin(input: &str, expected: &str, params: Option<serde_json::Value>) {
    let config = Config {
        plugins: vec![PluginConfig {
            name: "removeScripts".to_string(),
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
        "\nPlugin: removeScripts\nInput:\n{}\nExpected:\n{}\nActual:\n{}\n", 
        input, expected, output);
}

#[test]
fn test_remove_scripts_01() {

    let input = r#"<?xml version=\"1.0\" encoding=\"utf-16\"?>
    <svg version=\"1.1\" id=\"Layer_1\" xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\" x=\"0px\" y=\"0px\" viewBox=\"0 0 100 100\" style=\"enable-background:new 0 0 100 100;\" xml:space=\"preserve\">
        <script></script>
        <circle class=\"st0\" cx=\"50\" cy=\"50\" r=\"50\" />
    </svg>"#;

    let expected = r#"<?xml version=\"1.0\" encoding=\"utf-16\"?>
<svg version=\"1.1\" id=\"Layer_1\" xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\" x=\"0px\" y=\"0px\" viewBox=\"0 0 100 100\" style=\"enable-background:new 0 0 100 100;\" xml:space=\"preserve\">
    <circle class=\"st0\" cx=\"50\" cy=\"50\" r=\"50\"/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_scripts_02() {
    // Collapses links to JavaScript functions, and removes event attributes from
    // nodes including children of a collapsed links.
    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 100 100\">
  <a href=\"javascript:(() => { alert('uwu') })();\">
    <text y=\"10\" onclick=\"alert('uwu')\">uwu</text>
  </a>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 100 100\">
    <text y=\"10\">uwu</text>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_scripts_03() {
    // Does not remove normal links, and does remove event attributes.
    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 100 100\">
  <a href=\"https://yewtu.be/watch?v=dQw4w9WgXcQ\">
    <text y=\"10\" onclick=\"alert('uwu')\">uwu</text>
  </a>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 100 100\">
    <a href=\"https://yewtu.be/watch?v=dQw4w9WgXcQ\">
    <text y=\"10\">uwu</text>
  </a>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_scripts_04() {
    // If making different modifications to two different nodes in the same parent,
    // drop attributes and collapse nodes appropriately without losing elements.
    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 100 100\" version=\"1.1\">
  <script>alert('uwu')</script>
  <g onclick=\"alert('uwu')\">
    <text y=\"10\">uwu</text>
  </g>
  <a href=\"javascript:(() => { alert('uwu') })();\">
    <text y=\"20\">uwu</text>
  </a>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 100 100\" version=\"1.1\">
    <g>
        <text y=\"10\">uwu</text>
    </g>
    <text y=\"20\">uwu</text>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_scripts_05() {
    // Removes hrefs to JavaScript URIs, including unconventional namespaces.
    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" xmlns:uwu=\"http://www.w3.org/1999/xlink\" viewBox=\"0 0 100 100\" version=\"1.1\">
  <a href=\"javascript:(() => { alert('uwu') })();\">
    <text y=\"20\">uwu</text>
  </a>
  <a uwu:href=\"javascript:(() => { alert('uwu') })();\">
    <text y=\"30\">uwu</text>
  </a>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" xmlns:uwu=\"http://www.w3.org/1999/xlink\" viewBox=\"0 0 100 100\" version=\"1.1\">
    <text y=\"20\">uwu</text>
    <text y=\"30\">uwu</text>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}
