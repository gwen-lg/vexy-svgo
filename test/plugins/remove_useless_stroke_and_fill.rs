// this_file: test/plugins/remove_useless_stroke_and_fill.rs

//! Tests for the removeUselessStrokeAndFill plugin
//! Auto-generated from SVGO test fixtures

use vexy_svgo::{optimize, OptimizeOptions, Config, PluginConfig};
use vexy_svgo::config::Js2SvgOptions;
use serde_json::json;

fn test_plugin(input: &str, expected: &str, params: Option<serde_json::Value>) {
    let config = Config {
        plugins: vec![PluginConfig {
            name: "removeUselessStrokeAndFill".to_string(),
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
        "\nPlugin: removeUselessStrokeAndFill\nInput:\n{}\nExpected:\n{}\nActual:\n{}\n", 
        input, expected, output);
}

#[test]
fn test_remove_useless_stroke_and_fill_01() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <defs>
        <g id=\"test\">
            <rect stroke-dashoffset=\"5\" width=\"100\" height=\"100\"/>
        </g>
    </defs>
    <circle fill=\"red\" stroke-width=\"6\" stroke-dashoffset=\"5\" cx=\"60\" cy=\"60\" r=\"50\"/>
    <circle fill=\"red\" stroke=\"#000\" stroke-width=\"6\" stroke-dashoffset=\"5\" stroke-opacity=\"0\" cx=\"60\" cy=\"60\" r=\"50\"/>
    <circle fill=\"red\" stroke=\"#000\" stroke-width=\"0\" stroke-dashoffset=\"5\" cx=\"60\" cy=\"60\" r=\"50\"/>
    <circle fill=\"red\" stroke=\"#000\" stroke-width=\"6\" stroke-dashoffset=\"5\" cx=\"60\" cy=\"60\" r=\"50\"/>
    <g stroke=\"#000\" stroke-width=\"6\">
        <circle fill=\"red\" stroke=\"red\" stroke-width=\"0\" stroke-dashoffset=\"5\" cx=\"60\" cy=\"60\" r=\"50\"/>
        <circle fill=\"red\" stroke-dashoffset=\"5\" cx=\"60\" cy=\"60\" r=\"50\"/>
    </g>
    <g stroke=\"#000\">
        <circle fill=\"red\" stroke-width=\"0\" stroke-dashoffset=\"5\" cx=\"60\" cy=\"60\" r=\"50\"/>
        <circle fill=\"red\" stroke=\"none\" stroke-dashoffset=\"5\" cx=\"60\" cy=\"60\" r=\"50\"/>
    </g>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <defs>
        <g id=\"test\">
            <rect stroke-dashoffset=\"5\" width=\"100\" height=\"100\"/>
        </g>
    </defs>
    <circle fill=\"red\" cx=\"60\" cy=\"60\" r=\"50\"/>
    <circle fill=\"red\" cx=\"60\" cy=\"60\" r=\"50\"/>
    <circle fill=\"red\" cx=\"60\" cy=\"60\" r=\"50\"/>
    <circle fill=\"red\" stroke=\"#000\" stroke-width=\"6\" stroke-dashoffset=\"5\" cx=\"60\" cy=\"60\" r=\"50\"/>
    <g stroke=\"#000\" stroke-width=\"6\">
        <circle fill=\"red\" cx=\"60\" cy=\"60\" r=\"50\" stroke=\"none\"/>
        <circle fill=\"red\" stroke-dashoffset=\"5\" cx=\"60\" cy=\"60\" r=\"50\"/>
    </g>
    <g stroke=\"#000\">
        <circle fill=\"red\" cx=\"60\" cy=\"60\" r=\"50\" stroke=\"none\"/>
        <circle fill=\"red\" cx=\"60\" cy=\"60\" r=\"50\" stroke=\"none\"/>
    </g>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_useless_stroke_and_fill_02() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <defs>
        <g id=\"test\">
            <rect fill-opacity=\".5\" width=\"100\" height=\"100\"/>
        </g>
    </defs>
    <circle fill=\"none\" fill-rule=\"evenodd\" cx=\"60\" cy=\"60\" r=\"50\"/>
    <circle fill=\"red\" fill-opacity=\"0\" cx=\"90\" cy=\"90\" r=\"50\"/>
    <circle fill-opacity=\"0\" fill-rule=\"evenodd\" cx=\"90\" cy=\"60\" r=\"50\"/>
    <circle fill=\"red\" fill-opacity=\".5\" cx=\"60\" cy=\"60\" r=\"50\"/>
    <g fill=\"none\">
        <circle fill-opacity=\".5\" cx=\"60\" cy=\"60\" r=\"50\"/>
    </g>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <defs>
        <g id=\"test\">
            <rect fill-opacity=\".5\" width=\"100\" height=\"100\"/>
        </g>
    </defs>
    <circle fill=\"none\" cx=\"60\" cy=\"60\" r=\"50\"/>
    <circle fill=\"none\" cx=\"90\" cy=\"90\" r=\"50\"/>
    <circle cx=\"90\" cy=\"60\" r=\"50\" fill=\"none\"/>
    <circle fill=\"red\" fill-opacity=\".5\" cx=\"60\" cy=\"60\" r=\"50\"/>
    <g fill=\"none\">
        <circle cx=\"60\" cy=\"60\" r=\"50\"/>
    </g>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_useless_stroke_and_fill_03() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <style>
        …
    </style>
    <circle fill=\"none\" fill-rule=\"evenodd\" cx=\"60\" cy=\"60\" r=\"50\"/>
    <circle fill-opacity=\"0\" fill-rule=\"evenodd\" cx=\"90\" cy=\"60\" r=\"50\"/>
    <circle fill=\"red\" stroke-width=\"6\" stroke-dashoffset=\"5\" cx=\"60\" cy=\"60\" r=\"50\"/>
    <circle fill=\"red\" stroke=\"#000\" stroke-width=\"6\" stroke-dashoffset=\"5\" stroke-opacity=\"0\" cx=\"60\" cy=\"60\" r=\"50\"/>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <style>
        …
    </style>
    <circle fill=\"none\" fill-rule=\"evenodd\" cx=\"60\" cy=\"60\" r=\"50\"/>
    <circle fill-opacity=\"0\" fill-rule=\"evenodd\" cx=\"90\" cy=\"60\" r=\"50\"/>
    <circle fill=\"red\" stroke-width=\"6\" stroke-dashoffset=\"5\" cx=\"60\" cy=\"60\" r=\"50\"/>
    <circle fill=\"red\" stroke=\"#000\" stroke-width=\"6\" stroke-dashoffset=\"5\" stroke-opacity=\"0\" cx=\"60\" cy=\"60\" r=\"50\"/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_useless_stroke_and_fill_04() {

    let input = r#"<svg width=\"480\" height=\"360\" xmlns=\"http://www.w3.org/2000/svg\">
  <defs>
    <marker id=\"testMarker\">
      <rect width=\"100\" height=\"100\" fill=\"blue\" />
    </marker>
  </defs>
  <line x1=\"150\" y1=\"150\" x2=\"165\" y2=\"150\" stroke=\"red\" stroke-width=\"25\" marker-end=\"url(#testMarker)\" />
  <line x1=\"250\" y1=\"250\" x2=\"265\" y2=\"250\" stroke=\"red\" stroke-width=\"0\" marker-end=\"url(#testMarker)\" />
</svg>"#;

    let expected = r#"<svg width=\"480\" height=\"360\" xmlns=\"http://www.w3.org/2000/svg\">
    <defs>
        <marker id=\"testMarker\">
            <rect width=\"100\" height=\"100\" fill=\"blue\"/>
        </marker>
    </defs>
    <line x1=\"150\" y1=\"150\" x2=\"165\" y2=\"150\" stroke=\"red\" stroke-width=\"25\" marker-end=\"url(#testMarker)\"/>
    <line x1=\"250\" y1=\"250\" x2=\"265\" y2=\"250\" marker-end=\"url(#testMarker)\"/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_useless_stroke_and_fill_05() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <defs>
        <g id=\"test\">
            <rect fill-opacity=\".5\" width=\"100\" height=\"100\"/>
        </g>
    </defs>
    <circle fill=\"none\" fill-rule=\"evenodd\" cx=\"60\" cy=\"60\" r=\"50\"/>
    <circle fill=\"red\" fill-opacity=\"0\" cx=\"90\" cy=\"90\" r=\"50\"/>
    <circle fill-opacity=\"0\" fill-rule=\"evenodd\" cx=\"90\" cy=\"60\" r=\"50\"/>
    <circle fill=\"red\" fill-opacity=\".5\" cx=\"60\" cy=\"60\" r=\"50\"/>
    <g fill=\"none\">
        <circle fill-opacity=\".5\" cx=\"60\" cy=\"60\" r=\"50\"/>
    </g>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <defs>
        <g id=\"test\">
            <rect fill-opacity=\".5\" width=\"100\" height=\"100\"/>
        </g>
    </defs>
    <circle fill=\"red\" fill-opacity=\".5\" cx=\"60\" cy=\"60\" r=\"50\"/>
    <g fill=\"none\"/>
</svg>"#;

    let params = serde_json::from_str(r#"{
  "removeNone": true
}"#).ok();

    test_plugin(input, expected, params);
}
