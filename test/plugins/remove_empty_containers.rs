// this_file: test/plugins/remove_empty_containers.rs

//! Tests for the removeEmptyContainers plugin
//! Auto-generated from SVGO test fixtures

use vexy_svgo::{optimize, OptimizeOptions, Config, PluginConfig};
use vexy_svgo::config::Js2SvgOptions;
use serde_json::json;

fn test_plugin(input: &str, expected: &str, params: Option<serde_json::Value>) {
    let config = Config {
        plugins: vec![PluginConfig {
            name: "removeEmptyContainers".to_string(),
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
        "\nPlugin: removeEmptyContainers\nInput:\n{}\nExpected:\n{}\nActual:\n{}\n", 
        input, expected, output);
}

#[test]
fn test_remove_empty_containers_01() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <pattern/>
    <g>
        <marker>
            <a/>
        </marker>
    </g>
    <path d=\"...\"/>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <path d=\"...\"/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_empty_containers_02() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\">
    <defs>
        <pattern id=\"a\">
            <rect/>
        </pattern>
        <pattern xlink:href=\"url(#a)\" id=\"b\"/>
    </defs>
    <g>
        <marker>
            <a/>
        </marker>
        <path d=\"...\"/>
    </g>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\">
    <defs>
        <pattern id=\"a\">
            <rect/>
        </pattern>
        <pattern xlink:href=\"url(#a)\" id=\"b\"/>
    </defs>
    <g>
        <path d=\"...\"/>
    </g>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_empty_containers_03() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" xmlns:x=\"http://www.w3.org/1999/xlink\">
    <defs>
        <pattern id=\"a\">
            <rect/>
        </pattern>
        <pattern x:href=\"url(#a)\" id=\"b\"/>
    </defs>
    <g>
        <marker>
            <a/>
        </marker>
        <path d=\"...\"/>
    </g>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" xmlns:x=\"http://www.w3.org/1999/xlink\">
    <defs>
        <pattern id=\"a\">
            <rect/>
        </pattern>
        <pattern x:href=\"url(#a)\" id=\"b\"/>
    </defs>
    <g>
        <path d=\"...\"/>
    </g>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_empty_containers_04() {

    let input = r#"<svg>
  <defs>
    <filter id=\"feTileFilter\" filterUnits=\"userSpaceOnUse\" primitiveUnits=\"userSpaceOnUse\" x=\"115\" y=\"40\" width=\"250\" height=\"250\">
      <feFlood x=\"115\" y=\"40\" width=\"54\" height=\"19\" flood-color=\"lime\"/>
      <feOffset x=\"115\" y=\"40\" width=\"50\" height=\"25\" dx=\"6\" dy=\"6\" result=\"offset\"/>
      <feTile/>
    </filter>
  </defs>
  <g filter=\"url(#feTileFilter)\"/>
</svg>"#;

    let expected = r#"<svg>
    <defs>
        <filter id=\"feTileFilter\" filterUnits=\"userSpaceOnUse\" primitiveUnits=\"userSpaceOnUse\" x=\"115\" y=\"40\" width=\"250\" height=\"250\">
            <feFlood x=\"115\" y=\"40\" width=\"54\" height=\"19\" flood-color=\"lime\"/>
            <feOffset x=\"115\" y=\"40\" width=\"50\" height=\"25\" dx=\"6\" dy=\"6\" result=\"offset\"/>
            <feTile/>
        </filter>
    </defs>
    <g filter=\"url(#feTileFilter)\"/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_empty_containers_05() {

    let input = r#"<svg width=\"480\" height=\"360\" xmlns=\"http://www.w3.org/2000/svg\">
  <mask id=\"testMask\" />
  <rect x=\"100\" y=\"100\" width=\"250\" height=\"150\" fill=\"green\" />
  <rect x=\"100\" y=\"100\" width=\"250\" height=\"150\" fill=\"red\" mask=\"url(#testMask)\" />
</svg>"#;

    let expected = r#"<svg width=\"480\" height=\"360\" xmlns=\"http://www.w3.org/2000/svg\">
    <mask id=\"testMask\"/>
    <rect x=\"100\" y=\"100\" width=\"250\" height=\"150\" fill=\"green\"/>
    <rect x=\"100\" y=\"100\" width=\"250\" height=\"150\" fill=\"red\" mask=\"url(#testMask)\"/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_empty_containers_06() {
    // In switch elements, don't remove non-rendering children that contain
    // conditional attributes like requiredFeatures, requiredExtensions, or
    // systemLanguage.
    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 462 352\">
  <switch>
    <g requiredFeatures=\"http://www.w3.org/TR/SVG11/feature#Extensibility\"/>
    <a transform=\"translate(0,-5)\" href=\"https://www.diagrams.net/doc/faq/svg-export-text-problems\" target=\"_blank\">
      <text text-anchor=\"middle\" font-size=\"10px\" x=\"50%\" y=\"100%\">Viewer does not support full SVG 1.1</text>
    </a>
  </switch>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 462 352\">
    <switch>
        <g requiredFeatures=\"http://www.w3.org/TR/SVG11/feature#Extensibility\"/>
        <a transform=\"translate(0,-5)\" href=\"https://www.diagrams.net/doc/faq/svg-export-text-problems\" target=\"_blank\">
      <text text-anchor=\"middle\" font-size=\"10px\" x=\"50%\" y=\"100%\">Viewer does not support full SVG 1.1</text>
    </a>
    </switch>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_empty_containers_07() {
    // Empty <g> nodes should not be removed if they contain a filter, including
    // filters applied via CSS.
    let input = r#"<svg viewBox=\"0 0 50 50\" xmlns=\"http://www.w3.org/2000/svg\">
    <filter id=\"a\" x=\"0\" y=\"0\" width=\"50\" height=\"50\" filterUnits=\"userSpaceOnUse\">
        <feFlood flood-color=\"#aaa\"/>
    </filter>
    <mask id=\"b\" x=\"0\" y=\"0\" width=\"50\" height=\"50\">
        <g style=\"filter: url(#a)\"/>
    </mask>
    <text x=\"16\" y=\"16\" style=\"mask: url(#b)\">•ᴗ•</text>
</svg>"#;

    let expected = r#"<svg viewBox=\"0 0 50 50\" xmlns=\"http://www.w3.org/2000/svg\">
    <filter id=\"a\" x=\"0\" y=\"0\" width=\"50\" height=\"50\" filterUnits=\"userSpaceOnUse\">
        <feFlood flood-color=\"#aaa\"/>
    </filter>
    <mask id=\"b\" x=\"0\" y=\"0\" width=\"50\" height=\"50\">
        <g style=\"filter: url(#a)\"/>
    </mask>
    <text x=\"16\" y=\"16\" style=\"mask: url(#b)\">•ᴗ•</text>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}
