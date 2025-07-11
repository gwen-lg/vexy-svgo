// this_file: test/plugins/remove_editors_nsdata.rs

//! Tests for the removeEditorsNSData plugin
//! Auto-generated from SVGO test fixtures

use vexy_svgo::{optimize, OptimizeOptions, Config, PluginConfig};
use vexy_svgo::config::Js2SvgOptions;
use serde_json::json;

fn test_plugin(input: &str, expected: &str, params: Option<serde_json::Value>) {
    let config = Config {
        plugins: vec![PluginConfig {
            name: "removeEditorsNSData".to_string(),
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
        "\nPlugin: removeEditorsNSData\nInput:\n{}\nExpected:\n{}\nActual:\n{}\n", 
        input, expected, output);
}

#[test]
fn test_remove_editors_nsdata_01() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" xmlns:sodipodi=\"http://sodipodi.sourceforge.net/DTD/sodipodi-0.dtd\">
    <sodipodi:namedview>
        ...
    </sodipodi:namedview>

    <path d=\"...\" sodipodi:nodetypes=\"cccc\"/>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <path d=\"...\"/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_editors_nsdata_02() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" xmlns:sodipodi=\"http://inkscape.sourceforge.net/DTD/sodipodi-0.dtd\">
    <sodipodi:namedview>
        ...
    </sodipodi:namedview>

    <path d=\"...\" sodipodi:nodetypes=\"cccc\"/>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <path d=\"...\"/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}
