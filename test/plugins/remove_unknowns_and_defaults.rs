// this_file: test/plugins/remove_unknowns_and_defaults.rs

//! Tests for the removeUnknownsAndDefaults plugin
//! Auto-generated from SVGO test fixtures

use vexy_svgo::{optimize, OptimizeOptions, Config, PluginConfig};
use vexy_svgo::config::Js2SvgOptions;
use serde_json::json;

fn test_plugin(input: &str, expected: &str, params: Option<serde_json::Value>) {
    let config = Config {
        plugins: vec![PluginConfig {
            name: "removeUnknownsAndDefaults".to_string(),
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
        "\nPlugin: removeUnknownsAndDefaults\nInput:\n{}\nExpected:\n{}\nActual:\n{}\n", 
        input, expected, output);
}

#[test]
fn test_remove_unknowns_and_defaults_01() {

    let input = r#"<svg version=\"1.1\" xmlns=\"http://www.w3.org/2000/svg\" xmlns:test=\"http://\" attr=\"val\" x=\"0\" y=\"10\" test:attr=\"val\" xml:space=\"preserve\">
    <rect fill=\"#000\"/>
    <rect fill=\"#000\" id=\"black-rect\"/>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" xmlns:test=\"http://\" y=\"10\" test:attr=\"val\" xml:space=\"preserve\">
    <rect/>
    <rect fill=\"#000\" id=\"black-rect\"/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_unknowns_and_defaults_02() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" xmlns:test=\"http://\">
    <test>
        test
    </test>
    <test:test>
        test
    </test:test>
    <g>
        test
    </g>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" xmlns:test=\"http://\">
    <test:test>
        test
    </test:test>
    <g>
        test
    </g>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_unknowns_and_defaults_03() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g fill=\"red\">
        <path fill=\"#000\" d=\"M118.8 186.9l79.2\"/>
    </g>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g fill=\"red\">
        <path fill=\"#000\" d=\"M118.8 186.9l79.2\"/>
    </g>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_unknowns_and_defaults_04() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g fill=\"black\">
        <g fill=\"red\">
            <path fill=\"red\" d=\"M118.8 186.9l79.2\"/>
        </g>
    </g>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g fill=\"black\">
        <g fill=\"red\">
            <path d=\"M118.8 186.9l79.2\"/>
        </g>
    </g>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_unknowns_and_defaults_05() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g fill=\"red\">
        <g fill=\"red\">
            <g fill=\"green\">
                <g fill=\"green\">
                    <path fill=\"red\" d=\"M18.8 86.9l39.2\"/>
                </g>
            </g>
            <path fill=\"red\" d=\"M118.8 186.9l79.2\"/>
            <path id=\"red\" fill=\"red\" d=\"M118.8 186.9l79.2\"/>
        </g>
    </g>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g fill=\"red\">
        <g>
            <g fill=\"green\">
                <g>
                    <path fill=\"red\" d=\"M18.8 86.9l39.2\"/>
                </g>
            </g>
            <path d=\"M118.8 186.9l79.2\"/>
            <path id=\"red\" fill=\"red\" d=\"M118.8 186.9l79.2\"/>
        </g>
    </g>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_unknowns_and_defaults_06() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g fill=\"red\" data-foo=\"bar\">
        <path fill=\"#000\" d=\"M118.8 186.9l79.2\" data-bind=\"smth\"/>
    </g>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g fill=\"red\" data-foo=\"bar\">
        <path fill=\"#000\" d=\"M118.8 186.9l79.2\" data-bind=\"smth\"/>
    </g>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_unknowns_and_defaults_07() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" xmlns:test=\"http://\">
    <foreignObject>
        <div class=\"test\">
            fallback test
        </div>
    </foreignObject>

    <test>
        test
    </test>
    <test:test>
        test
    </test:test>
    <g>
        test
    </g>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" xmlns:test=\"http://\">
    <foreignObject>
        <div class=\"test\">
            fallback test
        </div>
    </foreignObject>
    <test:test>
        test
    </test:test>
    <g>
        test
    </g>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_unknowns_and_defaults_08() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" x=\"0\" y=\"0\">
    <svg x=\"10\" y=\"10\">
        <svg x=\"0\" y=\"0\">
            <path/>
        </svg>
        <svg x=\"0\" y=\"10\">
            <path/>
        </svg>
        <svg x=\"50\" y=\"0\">
            <path/>
        </svg>
    </svg>
    <svg x=\"100\" y=\"100\">
        <path/>
    </svg>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <svg x=\"10\" y=\"10\">
        <svg>
            <path/>
        </svg>
        <svg y=\"10\">
            <path/>
        </svg>
        <svg x=\"50\">
            <path/>
        </svg>
    </svg>
    <svg x=\"100\" y=\"100\">
        <path/>
    </svg>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_unknowns_and_defaults_09() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <metadata>
        <sfw>
            <slices></slices>
            <sliceSourceBounds height=\"67.3\" width=\"85.9\" y=\"-40.8\" x=\"-42.5\" bottomLeftOrigin=\"true\"></sliceSourceBounds>
        </sfw>
        <ellipse/>
    </metadata>
    <ellipse>
        <font-face/>
    </ellipse>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <metadata>
        <ellipse/>
    </metadata>
    <ellipse/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_unknowns_and_defaults_10() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g transform=\"translate(792)\">
        <g transform=\"translate(792)\">
            <path d=\"M118.8 186.9l79.2\"/>
        </g>
    </g>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g transform=\"translate(792)\">
        <g transform=\"translate(792)\">
            <path d=\"M118.8 186.9l79.2\"/>
        </g>
    </g>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_unknowns_and_defaults_11() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" aria-labelledby=\"title\">
    <title id=\"title\">
        Title
    </title>
    <g aria-label=\"foo\">
        test
    </g>
    <path id=\"t\" d=\"M10 10h10L10 20\"/>
    <use href=\"#t\"/>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" aria-labelledby=\"title\">
    <title id=\"title\">
        Title
    </title>
    <g aria-label=\"foo\">
        test
    </g>
    <path id=\"t\" d=\"M10 10h10L10 20\"/>
    <use href=\"#t\"/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_unknowns_and_defaults_12() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" aria-labelledby=\"title\">
    <title id=\"title\">
        Title
    </title>
    <g aria-label=\"foo\">
        test
    </g>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <title id=\"title\">
        Title
    </title>
    <g>
        test
    </g>
</svg>"#;

    let params = serde_json::from_str(r#"{
  "keepAriaAttrs": false
}"#).ok();

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_unknowns_and_defaults_13() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" role=\"img\">
    <g/>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_unknowns_and_defaults_14() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" role=\"img\">
    <g/>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" role=\"img\">
    <g/>
</svg>"#;

    let params = serde_json::from_str(r#"{
  "keepRoleAttr": true
}"#).ok();

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_unknowns_and_defaults_15() {

    let input = r#"<svg width=\"480\" height=\"360\" xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\">
  <text x=\"50\" y=\"50\">
    A <a xlink:href=\"#\"><tspan>link around tspan</tspan></a> for testing
  </text>
</svg>"#;

    let expected = r#"<svg width=\"480\" height=\"360\" xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\">
    <text x=\"50\" y=\"50\">
    A <a xlink:href=\"#\"><tspan>link around tspan</tspan></a> for testing
  </text>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_unknowns_and_defaults_16() {
    // Removes standalone= PROTECTED_49_  XML declaration.
    // 
    // See: https://github.com/svg/svgo/issues/836
    let input = r#"<?xml version=\"1.0\" standalone=\"no\"?>
<svg width=\"64\" height=\"18\" xmlns=\"http://www.w3.org/2000/svg\">
  <text x=\"4\" y=\"18\">uwu</text>
</svg>"#;

    let expected = r#"<?xml version=\"1.0\"?>
<svg width=\"64\" height=\"18\" xmlns=\"http://www.w3.org/2000/svg\">
    <text x=\"4\" y=\"18\">uwu</text>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_unknowns_and_defaults_17() {
    // Don't remove unknown attributes or attributes with default values if that
    // attribute is referenced in an attribute selector in CSS.
    // 
    // See: https://mastodon.social/@sir_pepe/114319751487861964
    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"-1 -1 202 40\">
  <style>
      [preserveAspectRatio] { fill: yellow; stroke: black; }
  </style>
  <svg viewBox=\"0 0 100 100\" preserveAspectRatio=\"xMidYMid meet\">
    <path d=\"M50,10 A40,40,1,1,1,50,90 A40,40,1,1,1,50,10 M30,40 Q36,35,42,40 M58,40 Q64,35,70,40 M30,60 Q50,75,70,60 Q50,75,30,60\"/>
  </svg>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"-1 -1 202 40\">
    <style>
        [preserveAspectRatio] { fill: yellow; stroke: black; }
    </style>
    <svg viewBox=\"0 0 100 100\" preserveAspectRatio=\"xMidYMid meet\">
        <path d=\"M50,10 A40,40,1,1,1,50,90 A40,40,1,1,1,50,10 M30,40 Q36,35,42,40 M58,40 Q64,35,70,40 M30,60 Q50,75,70,60 Q50,75,30,60\"/>
    </svg>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}
