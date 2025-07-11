// this_file: test/plugins/prefix_ids.rs

//! Tests for the prefixIds plugin
//! Auto-generated from SVGO test fixtures

use vexy_svgo::{optimize, OptimizeOptions, Config, PluginConfig};
use vexy_svgo::config::Js2SvgOptions;
use serde_json::json;

fn test_plugin(input: &str, expected: &str, params: Option<serde_json::Value>) {
    let config = Config {
        plugins: vec![PluginConfig {
            name: "prefixIds".to_string(),
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
        "\nPlugin: prefixIds\nInput:\n{}\nExpected:\n{}\nActual:\n{}\n", 
        input, expected, output);
}

#[test]
fn test_prefix_ids_01() {

    let input = r#"<svg width=\"120\" height=\"120\" xmlns=\"http://www.w3.org/2000/svg\">
    <style>
        .test {
            color: blue;
        }
        #test {
            color: red;
        }

    </style>
    <rect class=\"test\" x=\"10\" y=\"10\" width=\"100\" height=\"100\"/>
    <rect class=\"\" id=\"test\" x=\"10\" y=\"10\" width=\"100\" height=\"100\"/>
</svg>"#;

    let expected = r#"<svg width=\"120\" height=\"120\" xmlns=\"http://www.w3.org/2000/svg\">
    <style>
        .prefixIds_01_svg_txt__test{color:blue}#prefixIds_01_svg_txt__test{color:red}
    </style>
    <rect class=\"prefixIds_01_svg_txt__test\" x=\"10\" y=\"10\" width=\"100\" height=\"100\"/>
    <rect class=\"\" id=\"prefixIds_01_svg_txt__test\" x=\"10\" y=\"10\" width=\"100\" height=\"100\"/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_prefix_ids_02() {

    let input = r#"<svg width=\"120\" height=\"120\" xmlns=\"http://www.w3.org/2000/svg\">
    <defs>
        <linearGradient id=\"MyGradient\">
            <stop offset=\"5%\" stop-color=\"green\"/>
            <stop offset=\"95%\" stop-color=\"gold\"/>
        </linearGradient>
    </defs>
    <rect fill=\"url(#MyGradient)\" x=\"10\" y=\"10\" width=\"100\" height=\"100\"/>
</svg>"#;

    let expected = r#"<svg width=\"120\" height=\"120\" xmlns=\"http://www.w3.org/2000/svg\">
    <defs>
        <linearGradient id=\"prefixIds_02_svg_txt__MyGradient\">
            <stop offset=\"5%\" stop-color=\"green\"/>
            <stop offset=\"95%\" stop-color=\"gold\"/>
        </linearGradient>
    </defs>
    <rect fill=\"url(#prefixIds_02_svg_txt__MyGradient)\" x=\"10\" y=\"10\" width=\"100\" height=\"100\"/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_prefix_ids_03() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\">
    <use xlink:href=\"#Port\"/>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\">
    <use xlink:href=\"#prefixIds_03_svg_txt__Port\"/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_prefix_ids_04() {

    let input = r#"<svg width=\"120\" height=\"120\" xmlns=\"http://www.w3.org/2000/svg\">
    <style>
        rect {
            cursor: pointer;
            shape-rendering: crispEdges;
            fill:url(\"#MyGradient\");
        }

    </style>
    <rect x=\"10\" y=\"10\" width=\"100\" height=\"100\"/>
    <rect x=\"10\" y=\"10\" width=\"100\" height=\"100\"/>
</svg>"#;

    let expected = r#"<svg width=\"120\" height=\"120\" xmlns=\"http://www.w3.org/2000/svg\">
    <style>
        rect{cursor:pointer;shape-rendering:crispEdges;fill:url(#prefixIds_04_svg_txt__MyGradient)}
    </style>
    <rect x=\"10\" y=\"10\" width=\"100\" height=\"100\"/>
    <rect x=\"10\" y=\"10\" width=\"100\" height=\"100\"/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_prefix_ids_05() {

    let input = r#"<svg width=\"340\" height=\"120\" xmlns=\"http://www.w3.org/2000/svg\">
    <defs>
        <linearGradient id=\"gradient_1\">
            <stop offset=\"5%\" stop-color=\"green\"/>
            <stop offset=\"95%\" stop-color=\"gold\"/>
        </linearGradient>
        <linearGradient id=\"gradient_2\">
            <stop offset=\"5%\" stop-color=\"red\"/>
            <stop offset=\"95%\" stop-color=\"black\"/>
        </linearGradient>
        <linearGradient id=\"gradient_3\">
            <stop offset=\"5%\" stop-color=\"blue\"/>
            <stop offset=\"95%\" stop-color=\"orange\"/>
        </linearGradient>
    </defs>
    <rect fill=\"url(#gradient_1)\" x=\"10\" y=\"10\" width=\"100\" height=\"100\"/>
    <rect fill=\"url(#gradient_2)\" x=\"120\" y=\"10\" width=\"100\" height=\"100\"/>
    <rect fill=\"url(#gradient_3)\" x=\"230\" y=\"10\" width=\"100\" height=\"100\"/>
</svg>"#;

    let expected = r#"<svg width=\"340\" height=\"120\" xmlns=\"http://www.w3.org/2000/svg\">
    <defs>
        <linearGradient id=\"prefixIds_05_svg_txt__gradient_1\">
            <stop offset=\"5%\" stop-color=\"green\"/>
            <stop offset=\"95%\" stop-color=\"gold\"/>
        </linearGradient>
        <linearGradient id=\"prefixIds_05_svg_txt__gradient_2\">
            <stop offset=\"5%\" stop-color=\"red\"/>
            <stop offset=\"95%\" stop-color=\"black\"/>
        </linearGradient>
        <linearGradient id=\"prefixIds_05_svg_txt__gradient_3\">
            <stop offset=\"5%\" stop-color=\"blue\"/>
            <stop offset=\"95%\" stop-color=\"orange\"/>
        </linearGradient>
    </defs>
    <rect fill=\"url(#prefixIds_05_svg_txt__gradient_1)\" x=\"10\" y=\"10\" width=\"100\" height=\"100\"/>
    <rect fill=\"url(#prefixIds_05_svg_txt__gradient_2)\" x=\"120\" y=\"10\" width=\"100\" height=\"100\"/>
    <rect fill=\"url(#prefixIds_05_svg_txt__gradient_3)\" x=\"230\" y=\"10\" width=\"100\" height=\"100\"/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_prefix_ids_06() {

    let input = r#"<svg width=\"120\" height=\"120\" xmlns=\"http://www.w3.org/2000/svg\">
    <style>
        .test {
            color: blue;
        }
        .test2 {
            color: green;
        }
        #test {
            color: red;
        }
        .test3 .test4 {
            color: black;
        }
        .test5.test6 {
            color: brown;
        }
        .test5.test6 #test7 {
            color: yellow;
        }
    </style>
    <rect class=\"test\" x=\"10\" y=\"10\" width=\"100\" height=\"100\"/>
    <rect class=\"test test2\" x=\"10\" y=\"10\" width=\"100\" height=\"100\"/>
    <rect class=\"test  test2\" x=\"10\" y=\"10\" width=\"100\" height=\"100\"/>
    <rect class=\"\" id=\"test\" x=\"10\" y=\"10\" width=\"100\" height=\"100\"/>
</svg>"#;

    let expected = r#"<svg width=\"120\" height=\"120\" xmlns=\"http://www.w3.org/2000/svg\">
    <style>
        .prefixIds_06_svg_txt__test{color:blue}.prefixIds_06_svg_txt__test2{color:green}#prefixIds_06_svg_txt__test{color:red}.prefixIds_06_svg_txt__test3 .prefixIds_06_svg_txt__test4{color:black}.prefixIds_06_svg_txt__test5.prefixIds_06_svg_txt__test6{color:brown}.prefixIds_06_svg_txt__test5.prefixIds_06_svg_txt__test6 #prefixIds_06_svg_txt__test7{color:yellow}
    </style>
    <rect class=\"prefixIds_06_svg_txt__test\" x=\"10\" y=\"10\" width=\"100\" height=\"100\"/>
    <rect class=\"prefixIds_06_svg_txt__test prefixIds_06_svg_txt__test2\" x=\"10\" y=\"10\" width=\"100\" height=\"100\"/>
    <rect class=\"prefixIds_06_svg_txt__test prefixIds_06_svg_txt__test2\" x=\"10\" y=\"10\" width=\"100\" height=\"100\"/>
    <rect class=\"\" id=\"prefixIds_06_svg_txt__test\" x=\"10\" y=\"10\" width=\"100\" height=\"100\"/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_prefix_ids_07() {

    let input = r#"<svg width=\"120\" height=\"120\" xmlns=\"http://www.w3.org/2000/svg\">
    <style>
        .test {
            color: blue;
        }
        #test {
            color: red;
        }

    </style>
    <rect class=\"test\" x=\"10\" y=\"10\" width=\"100\" height=\"100\"/>
    <rect class=\"\" id=\"test\" x=\"10\" y=\"10\" width=\"100\" height=\"100\"/>
</svg>"#;

    let expected = r#"<svg width=\"120\" height=\"120\" xmlns=\"http://www.w3.org/2000/svg\">
    <style>
        .prefixIds_07_svg_txt__test{color:blue}#test{color:red}
    </style>
    <rect class=\"prefixIds_07_svg_txt__test\" x=\"10\" y=\"10\" width=\"100\" height=\"100\"/>
    <rect class=\"\" id=\"test\" x=\"10\" y=\"10\" width=\"100\" height=\"100\"/>
</svg>"#;

    let params = serde_json::from_str(r#"{
  "prefixIds": false
}"#).ok();

    test_plugin(input, expected, params);
}

#[test]
fn test_prefix_ids_08() {

    let input = r#"<svg width=\"120\" height=\"120\" xmlns=\"http://www.w3.org/2000/svg\">
    <style>
        .test {
            color: blue;
        }
        #test {
            color: red;
        }

    </style>
    <rect class=\"test\" x=\"10\" y=\"10\" width=\"100\" height=\"100\"/>
    <rect class=\"\" id=\"test\" x=\"10\" y=\"10\" width=\"100\" height=\"100\"/>
</svg>"#;

    let expected = r#"<svg width=\"120\" height=\"120\" xmlns=\"http://www.w3.org/2000/svg\">
    <style>
        .test{color:blue}#prefixIds_08_svg_txt__test{color:red}
    </style>
    <rect class=\"test\" x=\"10\" y=\"10\" width=\"100\" height=\"100\"/>
    <rect class=\"\" id=\"prefixIds_08_svg_txt__test\" x=\"10\" y=\"10\" width=\"100\" height=\"100\"/>
</svg>"#;

    let params = serde_json::from_str(r#"{
  "prefixClassNames": false
}"#).ok();

    test_plugin(input, expected, params);
}

#[test]
fn test_prefix_ids_09() {

    let input = r#"<svg width=\"120\" height=\"120\" xmlns=\"http://www.w3.org/2000/svg\">
    <style>
        .test {
            color: blue;
        }
        #test {
            color: red;
        }

    </style>
    <rect class=\"test\" x=\"10\" y=\"10\" width=\"100\" height=\"100\"/>
    <rect class=\"\" id=\"test\" x=\"10\" y=\"10\" width=\"100\" height=\"100\"/>
</svg>"#;

    let expected = r#"<svg width=\"120\" height=\"120\" xmlns=\"http://www.w3.org/2000/svg\">
    <style>
        .test{color:blue}#test{color:red}
    </style>
    <rect class=\"test\" x=\"10\" y=\"10\" width=\"100\" height=\"100\"/>
    <rect class=\"\" id=\"test\" x=\"10\" y=\"10\" width=\"100\" height=\"100\"/>
</svg>"#;

    let params = serde_json::from_str(r#"{
  "prefixIds": false,
  "prefixClassNames": false
}"#).ok();

    test_plugin(input, expected, params);
}

#[test]
fn test_prefix_ids_10() {

    let input = r#"<g xmlns=\"http://www.w3.org/2000/svg\" transform=\"translate(130, 112)\">
    <path class=\"st1\" d=\"M27,0h-37v64C-10,64,27,64.2,27,0z\" transform=\"scale(0.811377 1)\">
    <animateTransform id=\"t_1s\" attributeName=\"transform\" type=\"scale\" from=\"1 1\" to=\"-1 1\" begin=\"0s; t_2s.end\" dur=\"0.5s\" repeatCount=\"0\"/>
    <animateTransform id=\"t_2s\" attributeName=\"transform\" type=\"scale\" from=\"-1 1\" to=\"1 1\" begin=\"t_1s.end\" dur=\"0.5s\" repeatCount=\"0\"/>
    </path>
</g>"#;

    let expected = r#"<g xmlns=\"http://www.w3.org/2000/svg\" transform=\"translate(130, 112)\">
    <path class=\"prefixIds_10_svg_txt__st1\" d=\"M27,0h-37v64C-10,64,27,64.2,27,0z\" transform=\"scale(0.811377 1)\">
        <animateTransform id=\"prefixIds_10_svg_txt__t_1s\" attributeName=\"transform\" type=\"scale\" from=\"1 1\" to=\"-1 1\" begin=\"0s; prefixIds_10_svg_txt__t_2s.end\" dur=\"0.5s\" repeatCount=\"0\"/>
        <animateTransform id=\"prefixIds_10_svg_txt__t_2s\" attributeName=\"transform\" type=\"scale\" from=\"-1 1\" to=\"1 1\" begin=\"prefixIds_10_svg_txt__t_1s.end\" dur=\"0.5s\" repeatCount=\"0\"/>
    </path>
</g>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_prefix_ids_11() {
    // prefixIds should correctly handle url()s in style attribute, including multiple ones
    let input = r#"<svg width=\"120\" height=\"120\" xmlns=\"http://www.w3.org/2000/svg\">
    <defs>
        <linearGradient id=\"fill\"/>
        <linearGradient id=\"stroke\"/>
    </defs>
    <rect style=\"fill:url(#fill); stroke: url(#stroke)\" x=\"10\" y=\"10\" width=\"100\" height=\"100\"/>
</svg>"#;

    let expected = r#"<svg width=\"120\" height=\"120\" xmlns=\"http://www.w3.org/2000/svg\">
    <defs>
        <linearGradient id=\"prefixIds_11_svg_txt__fill\"/>
        <linearGradient id=\"prefixIds_11_svg_txt__stroke\"/>
    </defs>
    <rect style=\"fill:url(#prefixIds_11_svg_txt__fill); stroke: url(#prefixIds_11_svg_txt__stroke)\" x=\"10\" y=\"10\" width=\"100\" height=\"100\"/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_prefix_ids_12() {
    // Prefix IDs should apply to all nodes in styles, namely when styles are split
    // into multiple nodes due to XML comments.
    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 1 1\">
  <style>
    <!-- uwu -->
    #a {}
  </style>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 1 1\">
    <style>
<!--uwu-->
        #prefixIds_12_svg_txt__a{}
    </style>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_prefix_ids_13() {
    // Prefix IDs should apply to all nodes in styles, namely when styles are split
    // into multiple nodes due to XML comments.
    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 1 1\">
  <style>
    <!-- uwu -->
    #a13 {} <!-- xyz -->
    #b13 {}
  </style>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 1 1\">
    <style>
<!--uwu-->
        #prefixIds_13_svg_txt__a13{}
<!--xyz-->
        #prefixIds_13_svg_txt__b13{}
    </style>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}
