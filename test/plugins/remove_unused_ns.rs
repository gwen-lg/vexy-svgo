// this_file: test/plugins/remove_unused_ns.rs

//! Tests for the removeUnusedNS plugin
//! Auto-generated from SVGO test fixtures

use vexy_svgo::{optimize, OptimizeOptions, Config, PluginConfig};
use vexy_svgo::config::Js2SvgOptions;
use serde_json::json;

fn test_plugin(input: &str, expected: &str, params: Option<serde_json::Value>) {
    let config = Config {
        plugins: vec![PluginConfig {
            name: "removeUnusedNS".to_string(),
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
        "\nPlugin: removeUnusedNS\nInput:\n{}\nExpected:\n{}\nActual:\n{}\n", 
        input, expected, output);
}

#[test]
fn test_remove_unused_ns_01() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" xmlns:test=\"http://trololololololololololo.com/\">
    <g>
        test
    </g>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g>
        test
    </g>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_unused_ns_02() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" xmlns:test=\"http://trololololololololololo.com/\">
    <g test:attr=\"val\">
        test
    </g>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" xmlns:test=\"http://trololololololololololo.com/\">
    <g test:attr=\"val\">
        test
    </g>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_unused_ns_03() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" xmlns:test=\"http://trololololololololololo.com/\" xmlns:test2=\"http://trololololololololololo.com/\">
    <g test:attr=\"val\">
        <g>
            test
        </g>
    </g>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" xmlns:test=\"http://trololololololololololo.com/\">
    <g test:attr=\"val\">
        <g>
            test
        </g>
    </g>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_unused_ns_04() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" xmlns:test=\"http://trololololololololololo.com/\" xmlns:test2=\"http://trololololololololololo.com/\">
    <g test:attr=\"val\">
        <g test2:attr=\"val\">
            test
        </g>
    </g>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" xmlns:test=\"http://trololololololololololo.com/\" xmlns:test2=\"http://trololololololololololo.com/\">
    <g test:attr=\"val\">
        <g test2:attr=\"val\">
            test
        </g>
    </g>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_unused_ns_05() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" xmlns:test=\"http://trololololololololololo.com/\" xmlns:test2=\"http://trololololololololololo.com/\">
    <g>
        <test:elem>
            test
        </test:elem>
    </g>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" xmlns:test=\"http://trololololololololololo.com/\">
    <g>
        <test:elem>
            test
        </test:elem>
    </g>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_unused_ns_06() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" xmlns:test=\"http://trololololololololololo.com/\" xmlns:test2=\"http://trololololololololololo.com/\">
    <test:elem>
        <test2:elem>
            test
        </test2:elem>
    </test:elem>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" xmlns:test=\"http://trololololololololololo.com/\" xmlns:test2=\"http://trololololololololololo.com/\">
    <test:elem>
        <test2:elem>
            test
        </test2:elem>
    </test:elem>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_unused_ns_07() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" inkscape:version=\"0.92.2 (5c3e80d, 2017-08-06)\" sodipodi:docname=\"test.svg\" xmlns:inkscape=\"http://www.inkscape.org/namespaces/inkscape\" xmlns:sodipodi=\"http://sodipodi.sourceforge.net/DTD/sodipodi-0.dtd\">
    test
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" inkscape:version=\"0.92.2 (5c3e80d, 2017-08-06)\" sodipodi:docname=\"test.svg\" xmlns:inkscape=\"http://www.inkscape.org/namespaces/inkscape\" xmlns:sodipodi=\"http://sodipodi.sourceforge.net/DTD/sodipodi-0.dtd\">
    test
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}
