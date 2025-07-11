// this_file: test/plugins/remove_off_canvas_paths.rs

//! Tests for the removeOffCanvasPaths plugin
//! Auto-generated from SVGO test fixtures

use vexy_svgo::{optimize, OptimizeOptions, Config, PluginConfig};
use vexy_svgo::config::Js2SvgOptions;
use serde_json::json;

fn test_plugin(input: &str, expected: &str, params: Option<serde_json::Value>) {
    let config = Config {
        plugins: vec![PluginConfig {
            name: "removeOffCanvasPaths".to_string(),
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
        "\nPlugin: removeOffCanvasPaths\nInput:\n{}\nExpected:\n{}\nActual:\n{}\n", 
        input, expected, output);
}

#[test]
fn test_remove_off_canvas_paths_01() {

    let input = r#"<svg viewBox=\"0 0 100 100\" xmlns=\"http://www.w3.org/2000/svg\">
    <path d=\"M10 10 h 80 v 80 h -80 z\"/>
    <path d=\"M10 -90 h 80 v 80 h -80 z\"/>
    <path d=\"M110 10 h 80 v 80 h -80 z\"/>
    <path d=\"M10 110 h 80 v 80 h -80 z\"/>
    <path d=\"M-90 10 h 80 v 80 h -80 z\"/>
</svg>"#;

    let expected = r#"<svg viewBox=\"0 0 100 100\" xmlns=\"http://www.w3.org/2000/svg\">
    <path d=\"M10 10 h 80 v 80 h -80 z\"/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_off_canvas_paths_02() {

    let input = r#"<svg height=\"1000\" width=\"1000\" xmlns=\"http://www.w3.org/2000/svg\">
    <path d=\"M10 10 h 80 v 80 h -80 z\"/>
    <path d=\"M10 -90 h 80 v 80 h -80 z\"/>
    <path d=\"M110 10 h 80 v 80 h -80 z\"/>
    <path d=\"M10 110 h 80 v 80 h -80 z\"/>
    <path d=\"M-90 10 h 80 v 80 h -80 z\"/>
</svg>"#;

    let expected = r#"<svg height=\"1000\" width=\"1000\" xmlns=\"http://www.w3.org/2000/svg\">
    <path d=\"M10 10 h 80 v 80 h -80 z\"/>
    <path d=\"M110 10 h 80 v 80 h -80 z\"/>
    <path d=\"M10 110 h 80 v 80 h -80 z\"/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_off_canvas_paths_03() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 128 128\">
    <path d=\"M0 0h128v128H0z\" fill=\"none\" stroke=\"red\"/>
    <path d=\"M10.14 51.5c4.07 1.56 7.52 4.47 7.37 11.16\" fill=\"none\" stroke=\"#00f\"/>
    <path d=\"M100 200c4.07 1.56 7.52 4.47 7.37 11.16\" fill=\"none\" stroke=\"#00f\"/>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 128 128\">
    <path d=\"M0 0h128v128H0z\" fill=\"none\" stroke=\"red\"/>
    <path d=\"M10.14 51.5c4.07 1.56 7.52 4.47 7.37 11.16\" fill=\"none\" stroke=\"#00f\"/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_off_canvas_paths_04() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 128 128\">
    <path d=\"M20.16 107.3l13.18-12.18m-1.6-5.41l-16.32 6.51M13 84.5h18m77 22.8L94.83 95.12m1.6-5.41l16.32 6.51M115 84.5H98\" fill=\"none\" stroke=\"#444\" stroke-width=\"3\"/>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 128 128\">
    <path d=\"M20.16 107.3l13.18-12.18m-1.6-5.41l-16.32 6.51M13 84.5h18m77 22.8L94.83 95.12m1.6-5.41l16.32 6.51M115 84.5H98\" fill=\"none\" stroke=\"#444\" stroke-width=\"3\"/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_off_canvas_paths_05() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 100 100\">
    <path d=\"M-100-100h50v50h-50z\" fill=\"red\" transform=\"translate(100 100)\"/>
    <g transform=\"translate(150 150)\">
        <path d=\"M-100-100h50v50h-50z\" fill=\"blue\"/>
    </g>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 100 100\">
    <path d=\"M-100-100h50v50h-50z\" fill=\"red\" transform=\"translate(100 100)\"/>
    <g transform=\"translate(150 150)\">
        <path d=\"M-100-100h50v50h-50z\" fill=\"blue\"/>
    </g>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_off_canvas_paths_06() {
    // Should not throw when viewBox isn't present
    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <path d=\"M10 10 h 80 v 80 h -80 z\"/>
    <path d=\"M10 -90 h 80 v 80 h -80 z\"/>
    <path d=\"M110 10 h 80 v 80 h -80 z\"/>
    <path d=\"M10 110 h 80 v 80 h -80 z\"/>
    <path d=\"M-90 10 h 80 v 80 h -80 z\"/>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <path d=\"M10 10 h 80 v 80 h -80 z\"/>
    <path d=\"M10 -90 h 80 v 80 h -80 z\"/>
    <path d=\"M110 10 h 80 v 80 h -80 z\"/>
    <path d=\"M10 110 h 80 v 80 h -80 z\"/>
    <path d=\"M-90 10 h 80 v 80 h -80 z\"/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}
