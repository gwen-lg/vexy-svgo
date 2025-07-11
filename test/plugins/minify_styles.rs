// this_file: test/plugins/minify_styles.rs

//! Tests for the minifyStyles plugin
//! Auto-generated from SVGO test fixtures

use vexy_svgo::{optimize, OptimizeOptions, Config, PluginConfig};
use vexy_svgo::config::Js2SvgOptions;
use serde_json::json;

fn test_plugin(input: &str, expected: &str, params: Option<serde_json::Value>) {
    let config = Config {
        plugins: vec![PluginConfig {
            name: "minifyStyles".to_string(),
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
        "\nPlugin: minifyStyles\nInput:\n{}\nExpected:\n{}\nActual:\n{}\n", 
        input, expected, output);
}

#[test]
fn test_minify_styles_01() {

    let input = r#"<svg id=\"test\" xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 100 100\">
    <style>
        .st0{ fill:red; padding-top: 1em; padding-right: 1em; padding-bottom: 1em; padding-left: 1em; } @media screen and (max-width: 200px) { .st0 { display: none; } }
    </style>
    <rect width=\"100\" height=\"100\" class=\"st0\" style=\"stroke-width:3; margin-top: 1em; margin-right: 1em; margin-bottom: 1em; margin-left: 1em;\"/>
</svg>"#;

    let expected = r#"<svg id=\"test\" xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 100 100\">
    <style>
        .st0{fill:red;padding:1em}@media screen and (max-width:200px){.st0{display:none}}
    </style>
    <rect width=\"100\" height=\"100\" class=\"st0\" style=\"stroke-width:3;margin:1em\"/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_minify_styles_02() {

    let input = r#"<svg id=\"test\" xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 100 100\">
    <style>
        <![CDATA[
            .st0{ fill:red; padding-top: 1em; padding-right: 1em; padding-bottom: 1em; padding-left: 1em; } @media screen and (max-width: 200px) { .st0 { display: none; } }
        ]]>
    </style>
    <style></style>
    <rect width=\"100\" height=\"100\" class=\"st0\" style=\"stroke-width:3; margin-top: 1em; margin-right: 1em; margin-bottom: 1em; margin-left: 1em;\"/>
</svg>"#;

    let expected = r#"<svg id=\"test\" xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 100 100\">
    <style>
        .st0{fill:red;padding:1em}@media screen and (max-width:200px){.st0{display:none}}
    </style>
    <style/>
    <rect width=\"100\" height=\"100\" class=\"st0\" style=\"stroke-width:3;margin:1em\"/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_minify_styles_03() {

    let input = r#"<svg id=\"test\" xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 100 100\">
    <style>
        <![CDATA[
            .st0{ fill:red; padding-top: 1em; padding-right: 1em; padding-bottom: 1em; padding-left: 1em; background-image: url('data:image/svg,<svg width=\"16\" height=\"16\"/>') } @media screen and (max-width: 200px) { .st0 { display: none; } }
        ]]>
    </style>
    <rect width=\"100\" height=\"100\" class=\"st0\" style=\"stroke-width:3; margin-top: 1em; margin-right: 1em; margin-bottom: 1em; margin-left: 1em;\"/>
</svg>"#;

    let expected = r#"<svg id=\"test\" xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 100 100\">
    <style>
        <![CDATA[.st0{fill:red;padding:1em;background-image:url('data:image/svg,<svg width=\"16\" height=\"16\"/>')}@media screen and (max-width:200px){.st0{display:none}}]]>
    </style>
    <rect width=\"100\" height=\"100\" class=\"st0\" style=\"stroke-width:3;margin:1em\"/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_minify_styles_04() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <style>
        .used { p: 1 }
        .unused { p: 2 }
        #used { p: 3 }
        #unused { p: 4 }
        g { p: 5 }
        unused { p: 6 }
    </style>
    <g id=\"used\" class=\"used\">
        test
    </g>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <style>
        .used{p:1}#used{p:3}g{p:5}
    </style>
    <g id=\"used\" class=\"used\">
        test
    </g>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_minify_styles_05() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <style>
        .used { p: 1 }
        .unused { p: 2 }
        #used { p: 3 }
        #unused { p: 4 }
        g { p: 5 }
        unused { p: 6 }
    </style>
    <g id=\"used\" class=\"used\">
        test
    </g>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <style>
        .used{p:1}#used{p:3}#unused{p:4}g{p:5}unused{p:6}
    </style>
    <g id=\"used\" class=\"used\">
        test
    </g>
</svg>"#;

    let params = serde_json::from_str(r#"{
  "usage": {
    "ids": false,
    "tags": false
  }
}"#).ok();

    test_plugin(input, expected, params);
}

#[test]
fn test_minify_styles_06() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <style>
        .used { p: 1 }
        .unused { p: 2 }
        #used { p: 3 }
        #unused { p: 4 }
        g { p: 5 }
        unused { p: 6 }
    </style>
    <g id=\"used\" class=\"used\">
        test
    </g>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <style>
        .used{p:1}.unused{p:2}#used{p:3}#unused{p:4}g{p:5}unused{p:6}
    </style>
    <g id=\"used\" class=\"used\">
        test
    </g>
</svg>"#;

    let params = serde_json::from_str(r#"{
  "usage": false
}"#).ok();

    test_plugin(input, expected, params);
}

#[test]
fn test_minify_styles_07() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <style>
        .used { p: 1 }
        .unused { p: 2 }
    </style>
    <script>
        /* script element prevents removing unused styles */
    </script>
    <g class=\"used\">
        test
    </g>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <style>
        .used{p:1}.unused{p:2}
    </style>
    <script>
        /* script element prevents removing unused styles */
    </script>
    <g class=\"used\">
        test
    </g>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_minify_styles_08() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <style>
        .used { p: 1 }
        .unused { p: 2 }
    </style>
    <g class=\"used\" onclick=\"/* on* attributes prevents removing unused styles */\">
        test
    </g>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <style>
        .used{p:1}.unused{p:2}
    </style>
    <g class=\"used\" onclick=\"/* on* attributes prevents removing unused styles */\">
        test
    </g>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_minify_styles_09() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <style>
        .used { p: 1 }
        .unused { p: 2 }
    </style>
    <script>
        /* with usage.force=true script element does not prevent removing unused styles */
    </script>
    <g class=\"used\" onclick=\"/* with usage.force=true on* attributes doesn't prevent removing unused styles */\">
        test
    </g>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <style>
        .used{p:1}
    </style>
    <script>
        /* with usage.force=true script element does not prevent removing unused styles */
    </script>
    <g class=\"used\" onclick=\"/* with usage.force=true on* attributes doesn't prevent removing unused styles */\">
        test
    </g>
</svg>"#;

    let params = serde_json::from_str(r#"{
  "usage": {
    "force": true
  }
}"#).ok();

    test_plugin(input, expected, params);
}

#[test]
fn test_minify_styles_10() {

    let input = r#"<svg viewBox=\"0 0 2203 1777\" xmlns=\"http://www.w3.org/2000/svg\">
    <style type=\"text/css\">
        .st6{font-family:Helvetica LT Std, Helvetica, Arial; font-size:118px;; stroke-opacity:0; fill-opacity:0;}
    </style>
    <text class=\"st6\" transform=\"translate(353.67 1514)\">
        tell stories in 250 characters
    </text>
</svg>"#;

    let expected = r#"<svg viewBox=\"0 0 2203 1777\" xmlns=\"http://www.w3.org/2000/svg\">
    <style type=\"text/css\">
        .st6{font-family:Helvetica LT Std,Helvetica,Arial;font-size:118px;stroke-opacity:0;fill-opacity:0}
    </style>
    <text class=\"st6\" transform=\"translate(353.67 1514)\">
        tell stories in 250 characters
    </text>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_minify_styles_11() {
    // Ensure all unused styles are removed, even if no there are no classes in
    // the document.
    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 113.9 130.4\">
  <style>
  .st1{fill:#453624;stroke:#453624;stroke-width:0.7495;stroke-miterlimit:10;}
  .st2{fill:#FFFFFF;}
  .st3{fill:#FCBF2A;}
  </style>
  <path d=\"\"/>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 113.9 130.4\">
    <path d=\"\"/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}
