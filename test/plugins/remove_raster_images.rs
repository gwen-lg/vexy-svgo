// this_file: test/plugins/remove_raster_images.rs

//! Tests for the removeRasterImages plugin
//! Auto-generated from SVGO test fixtures

use vexy_svgo::{optimize, OptimizeOptions, Config, PluginConfig};
use vexy_svgo::config::Js2SvgOptions;
use serde_json::json;

fn test_plugin(input: &str, expected: &str, params: Option<serde_json::Value>) {
    let config = Config {
        plugins: vec![PluginConfig {
            name: "removeRasterImages".to_string(),
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
        "\nPlugin: removeRasterImages\nInput:\n{}\nExpected:\n{}\nActual:\n{}\n", 
        input, expected, output);
}

#[test]
fn test_remove_raster_images_01() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\">
    <g>
        <image xlink:href=\"raster.jpg\" width=\"100\" height=\"100\"/>
        <image xlink:href=\"raster.png\" width=\"100\" height=\"100\"/>
        <image xlink:href=\"raster.gif\" width=\"100\" height=\"100\"/>
        <image xlink:href=\"raster.svg\" width=\"100\" height=\"100\"/>
    </g>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\">
    <g>
        <image xlink:href=\"raster.svg\" width=\"100\" height=\"100\"/>
    </g>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_raster_images_02() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\">
    <g>
        <image xlink:href=\"data:image/jpg;base64,...\" width=\"100\" height=\"100\"/>
        <image xlink:href=\"data:image/png;base64,...\" width=\"100\" height=\"100\"/>
        <image xlink:href=\"data:image/gif;base64,...\" width=\"100\" height=\"100\"/>
        <image xlink:href=\"data:image/svg+xml;base64,...\" width=\"100\" height=\"100\"/>
    </g>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\">
    <g>
        <image xlink:href=\"data:image/svg+xml;base64,...\" width=\"100\" height=\"100\"/>
    </g>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}
