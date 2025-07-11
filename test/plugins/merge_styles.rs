// this_file: test/plugins/merge_styles.rs

//! Tests for the mergeStyles plugin
//! Auto-generated from SVGO test fixtures

use vexy_svgo::{optimize, OptimizeOptions, Config, PluginConfig};
use vexy_svgo::config::Js2SvgOptions;
use serde_json::json;

fn test_plugin(input: &str, expected: &str, params: Option<serde_json::Value>) {
    let config = Config {
        plugins: vec![PluginConfig {
            name: "mergeStyles".to_string(),
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
        "\nPlugin: mergeStyles\nInput:\n{}\nExpected:\n{}\nActual:\n{}\n", 
        input, expected, output);
}

#[test]
fn test_merge_styles_01() {
    // Check whether plugin works with only one style element (no further merging needed, noop).
    let input = r#"<svg id=\"test\" xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 100 100\">
    <style>
        .st0{ fill:red; padding-top: 1em; padding-right: 1em; padding-bottom: 1em; padding-left: 1em; }
    </style>
    <rect width=\"100\" height=\"100\" class=\"st0\" style=\"stroke-width:3;margin-top:1em;margin-right:1em;margin-bottom:1em;margin-left:1em\"/>
</svg>"#;

    let expected = r#"<svg id=\"test\" xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 100 100\">
    <style>
        .st0{ fill:red; padding-top: 1em; padding-right: 1em; padding-bottom: 1em; padding-left: 1em; }
    </style>
    <rect width=\"100\" height=\"100\" class=\"st0\" style=\"stroke-width:3;margin-top:1em;margin-right:1em;margin-bottom:1em;margin-left:1em\"/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_merge_styles_02() {
    // Check whether plugin works with only one style element (no further merging needed, noop) and a media query.
    let input = r#"<svg id=\"test\" xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 100 100\">
    <style>.st0{ fill:red; padding-top: 1em; padding-right: 1em; padding-bottom: 1em; padding-left: 1em; }</style>
    <style>
        @media screen and (max-width: 200px) { .st0 { display: none; } }
    </style>
    <rect width=\"100\" height=\"100\" class=\"st0\" style=\"stroke-width:3;margin-top:1em;margin-right:1em;margin-bottom:1em;margin-left:1em\"/>
</svg>"#;

    let expected = r#"<svg id=\"test\" xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 100 100\">
    <style>
        .st0{ fill:red; padding-top: 1em; padding-right: 1em; padding-bottom: 1em; padding-left: 1em; }@media screen and (max-width: 200px) { .st0 { display: none; } }
    </style>
    <rect width=\"100\" height=\"100\" class=\"st0\" style=\"stroke-width:3;margin-top:1em;margin-right:1em;margin-bottom:1em;margin-left:1em\"/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_merge_styles_03() {
    // Check whether plugin works with merging styles of two style elements (no media queries).
    let input = r#"<svg id=\"test\" xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 100 100\">
    <style media=\"print\">.st0{ fill:red; padding-top: 1em; padding-right: 1em; padding-bottom: 1em; padding-left: 1em; }</style>
    <style>.test { background: red; }</style>
    <rect width=\"100\" height=\"100\" class=\"st0\" style=\"stroke-width:3;margin-top:1em;margin-right:1em;margin-bottom:1em;margin-left:1em\"/>
</svg>"#;

    let expected = r#"<svg id=\"test\" xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 100 100\">
    <style>
        @media print{.st0{ fill:red; padding-top: 1em; padding-right: 1em; padding-bottom: 1em; padding-left: 1em; }}.test { background: red; }
    </style>
    <rect width=\"100\" height=\"100\" class=\"st0\" style=\"stroke-width:3;margin-top:1em;margin-right:1em;margin-bottom:1em;margin-left:1em\"/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_merge_styles_04() {
    // Check whether plugin works with two style elements that contain styles that also uses media queries.
    let input = r#"<svg id=\"test\" xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 100 100\">
    <style media=\"print\">.st0{ fill:red; padding-top: 1em; padding-right: 1em; padding-bottom: 1em; padding-left: 1em; }</style>
    <style>.test { background: red; }</style>
    <rect width=\"100\" height=\"100\" class=\"st0\" style=\"stroke-width:3;margin-top:1em;margin-right:1em;margin-bottom:1em;margin-left:1em\"/>
    <style media=\"only screen and (min-width: 600px)\">.wrapper { color: blue; }</style>
</svg>"#;

    let expected = r#"<svg id=\"test\" xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 100 100\">
    <style>
        @media print{.st0{ fill:red; padding-top: 1em; padding-right: 1em; padding-bottom: 1em; padding-left: 1em; }}.test { background: red; }@media only screen and (min-width: 600px){.wrapper { color: blue; }}
    </style>
    <rect width=\"100\" height=\"100\" class=\"st0\" style=\"stroke-width:3;margin-top:1em;margin-right:1em;margin-bottom:1em;margin-left:1em\"/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_merge_styles_05() {
    // Check whether plugin works with no style elements at all (no merging needed, noop).
    let input = r#"<svg id=\"test\" xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 100 100\">
    <rect width=\"100\" height=\"100\" class=\"st0\"/>
</svg>"#;

    let expected = r#"<svg id=\"test\" xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 100 100\">
    <rect width=\"100\" height=\"100\" class=\"st0\"/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_merge_styles_06() {
    // Check whether plugin removes empty <style> elements
    let input = r#"<svg id=\"test\" xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 100 100\">
	<style></style>
    <style>
        .st0{ fill:red; padding-top: 1em; padding-right: 1em; padding-bottom: 1em; padding-left: 1em; }
    </style>
    <rect width=\"100\" height=\"100\" class=\"st0\" style=\"stroke-width:3;margin-top:1em;margin-right:1em;margin-bottom:1em;margin-left:1em\"/>
</svg>"#;

    let expected = r#"<svg id=\"test\" xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 100 100\">
    <style>
        .st0{ fill:red; padding-top: 1em; padding-right: 1em; padding-bottom: 1em; padding-left: 1em; }
    </style>
    <rect width=\"100\" height=\"100\" class=\"st0\" style=\"stroke-width:3;margin-top:1em;margin-right:1em;margin-bottom:1em;margin-left:1em\"/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_merge_styles_07() {
    // Check whether plugin removes empty <style> elements with only empty <style> elements
    let input = r#"<svg id=\"test\" xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 100 100\">
	<style></style>
    <style>
    </style>
    <rect width=\"100\" height=\"100\" class=\"st0\" style=\"stroke-width:3;margin-top:1em;margin-right:1em;margin-bottom:1em;margin-left:1em\"/>
</svg>"#;

    let expected = r#"<svg id=\"test\" xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 100 100\">
    <rect width=\"100\" height=\"100\" class=\"st0\" style=\"stroke-width:3;margin-top:1em;margin-right:1em;margin-bottom:1em;margin-left:1em\"/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_merge_styles_08() {
    // Check whether plugin removes empty <style> elements mixed with non-empty <style> elements
    let input = r#"<svg id=\"test\" xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 100 100\">
    <style></style>
    <style></style>
    <style>
        .test { color: red; }
    </style>
    <style></style>
    <style></style>
    <rect width=\"100\" height=\"100\" class=\"st0\" style=\"stroke-width:3;margin-top:1em;margin-right:1em;margin-bottom:1em;margin-left:1em\"/>
</svg>"#;

    let expected = r#"<svg id=\"test\" xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 100 100\">
    <style>
        .test { color: red; }
    </style>
    <rect width=\"100\" height=\"100\" class=\"st0\" style=\"stroke-width:3;margin-top:1em;margin-right:1em;margin-bottom:1em;margin-left:1em\"/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_merge_styles_09() {
    // Check whether plugin removes empty <style> elements mixed with non-empty <style> elements
    let input = r#"<svg id=\"test\" xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 100 100\">
    <style>
	  .a { fill: blue; }
	</style>
	<style type=\"\">
	  .b { fill: green; }
	</style>
    <style type=\"text/css\">
	  .c { fill: red; }
	</style>
	<style type=\"text/invalid\">
	  .d { fill: blue; }
	</style>
    <rect width=\"100\" height=\"100\" class=\"st0\" style=\"stroke-width:3;margin-top:1em;margin-right:1em;margin-bottom:1em;margin-left:1em\"/>
</svg>"#;

    let expected = r#"<svg id=\"test\" xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 100 100\">
    <style>
        .a { fill: blue; }.b { fill: green; }.c { fill: red; }
    </style>
    <style type=\"text/invalid\">
        .d { fill: blue; }
    </style>
    <rect width=\"100\" height=\"100\" class=\"st0\" style=\"stroke-width:3;margin-top:1em;margin-right:1em;margin-bottom:1em;margin-left:1em\"/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_merge_styles_10() {
    // Check whether plugin removes one empty <style> element that is also the only <style> element.
    let input = r#"<svg id=\"test\" xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 100 100\">
    <style>
	  </style>
    <rect width=\"100\" height=\"100\" class=\"st0\" style=\"stroke-width:3;margin-top:1em;margin-right:1em;margin-bottom:1em;margin-left:1em\"/>
</svg>"#;

    let expected = r#"<svg id=\"test\" xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 100 100\">
    <rect width=\"100\" height=\"100\" class=\"st0\" style=\"stroke-width:3;margin-top:1em;margin-right:1em;margin-bottom:1em;margin-left:1em\"/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_merge_styles_11() {
    // Convert content to cdata if any style element contains cdata
    let input = r#"<svg id=\"test\" xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 100 100\">
    <style>
        .st0 { fill: yellow; }
    </style>
    <style>
        <![CDATA[
            .st1 { fill: red; }
        ]]>
    </style>
</svg>"#;

    let expected = r#"<svg id=\"test\" xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 100 100\">
    <style>
        <![CDATA[.st0 { fill: yellow; }
            .st1 { fill: red; }
        ]]>
    </style>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_merge_styles_12() {
    // Skip styles inside foreignObject element
    let input = r#"<svg id=\"test\" xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 100 100\">
  <foreignObject>
    <style>
      .st0 { fill: yellow; }
    </style>
  </foreignObject>
  <style>
    .st1 { fill: red; }
  </style>
</svg>"#;

    let expected = r#"<svg id=\"test\" xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 100 100\">
    <foreignObject>
        <style>
            .st0 { fill: yellow; }
        </style>
    </foreignObject>
    <style>
        .st1 { fill: red; }
    </style>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}
