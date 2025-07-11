// this_file: test/plugins/move_elems_attrs_to_group.rs

//! Tests for the moveElemsAttrsToGroup plugin
//! Auto-generated from SVGO test fixtures

use vexy_svgo::{optimize, OptimizeOptions, Config, PluginConfig};
use vexy_svgo::config::Js2SvgOptions;
use serde_json::json;

fn test_plugin(input: &str, expected: &str, params: Option<serde_json::Value>) {
    let config = Config {
        plugins: vec![PluginConfig {
            name: "moveElemsAttrsToGroup".to_string(),
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
        "\nPlugin: moveElemsAttrsToGroup\nInput:\n{}\nExpected:\n{}\nActual:\n{}\n", 
        input, expected, output);
}

#[test]
fn test_move_elems_attrs_to_group_01() {
    // Move common children attributes to group
    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g attr1=\"val1\">
        <g fill=\"red\" color=\"#000\" stroke=\"blue\">
            text
        </g>
        <g>
          <rect fill=\"red\" color=\"#000\" />
          <ellipsis fill=\"red\" color=\"#000\" />
        </g>
        <circle fill=\"red\" color=\"#000\" attr3=\"val3\"/>
    </g>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g attr1=\"val1\" fill=\"red\" color=\"#000\">
        <g stroke=\"blue\">
            text
        </g>
        <g>
            <rect/>
            <ellipsis/>
        </g>
        <circle attr3=\"val3\"/>
    </g>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_move_elems_attrs_to_group_02() {
    // Override group attributes with children attributes
    let input = r#"<svg>
  <g fill=\"red\">
    <rect fill=\"blue\" />
    <circle fill=\"blue\" />
  </g>
</svg>"#;

    let expected = r#"<svg>
    <g fill=\"blue\">
        <rect/>
        <circle/>
    </g>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_move_elems_attrs_to_group_03() {
    // Move to group only inheritable attributes
    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g attr1=\"val1\">
        <g attr2=\"val2\">
            text
        </g>
        <circle attr2=\"val2\" attr3=\"val3\"/>
        <path d=\"...\"/>
    </g>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g attr1=\"val1\">
        <g attr2=\"val2\">
            text
        </g>
        <circle attr2=\"val2\" attr3=\"val3\"/>
        <path d=\"...\"/>
    </g>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_move_elems_attrs_to_group_04() {
    // Merge common group children transform attribute with the group transform
    // 
    // Preserve transform on children when group has clip-path or mask
    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <mask id=\"mask\">
        <path/>
    </mask>
    <g transform=\"rotate(45)\">
        <g transform=\"scale(2)\" fill=\"red\">
            <path d=\"...\"/>
        </g>
        <circle fill=\"red\" transform=\"scale(2)\"/>
    </g>
    <g clip-path=\"url(#clipPath)\">
        <g transform=\"translate(10 10)\"/>
        <g transform=\"translate(10 10)\"/>
    </g>
    <g mask=\"url(#mask)\">
        <g transform=\"translate(10 10)\"/>
        <g transform=\"translate(10 10)\"/>
    </g>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <mask id=\"mask\">
        <path/>
    </mask>
    <g transform=\"rotate(45) scale(2)\" fill=\"red\">
        <g>
            <path d=\"...\"/>
        </g>
        <circle/>
    </g>
    <g clip-path=\"url(#clipPath)\">
        <g transform=\"translate(10 10)\"/>
        <g transform=\"translate(10 10)\"/>
    </g>
    <g mask=\"url(#mask)\">
        <g transform=\"translate(10 10)\"/>
        <g transform=\"translate(10 10)\"/>
    </g>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_move_elems_attrs_to_group_06() {
    // Preserve transform when all children are paths
    // so the transform could be applied to path data by other plugins
    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g>
        <path transform=\"scale(2)\" d=\"M0,0 L10,20\"/>
        <path transform=\"scale(2)\" d=\"M0,10 L20,30\"/>
    </g>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g>
        <path transform=\"scale(2)\" d=\"M0,0 L10,20\"/>
        <path transform=\"scale(2)\" d=\"M0,10 L20,30\"/>
    </g>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_move_elems_attrs_to_group_07() {
    // Plugin is deoptimized when style element is present
    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <style id=\"current-color-scheme\">
        .ColorScheme-Highlight{color:#3daee9}
    </style>
    <g>
        <path transform=\"matrix(-1 0 0 1 72 51)\" class=\"ColorScheme-Highlight\" fill=\"currentColor\" d=\"M5-28h26v2H5z\"/>
        <path transform=\"matrix(-1 0 0 1 72 51)\" class=\"ColorScheme-Highlight\" fill=\"currentColor\" d=\"M5-29h26v1H5z\"/>
    </g>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <style id=\"current-color-scheme\">
        .ColorScheme-Highlight{color:#3daee9}
    </style>
    <g>
        <path transform=\"matrix(-1 0 0 1 72 51)\" class=\"ColorScheme-Highlight\" fill=\"currentColor\" d=\"M5-28h26v2H5z\"/>
        <path transform=\"matrix(-1 0 0 1 72 51)\" class=\"ColorScheme-Highlight\" fill=\"currentColor\" d=\"M5-29h26v1H5z\"/>
    </g>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_move_elems_attrs_to_group_08() {
    // Don't move transform if there is a filter attribute on group.
    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 32 32\">
    <defs>
        <filter id=\"a\" x=\"17\" y=\"13\" width=\"12\" height=\"10\" filterUnits=\"userSpaceOnUse\">
            <feGaussianBlur stdDeviation=\".01\"/>
        </filter>
    </defs>
    <g filter=\"url(#a)\">
        <rect x=\"19\" y=\"12\" width=\"14\" height=\"6\" rx=\"3\" transform=\"rotate(31 19 12.79)\"/>
        <rect x=\"19\" y=\"12\" width=\"14\" height=\"6\" rx=\"3\" transform=\"rotate(31 19 12.79)\"/>
    </g>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 32 32\">
    <defs>
        <filter id=\"a\" x=\"17\" y=\"13\" width=\"12\" height=\"10\" filterUnits=\"userSpaceOnUse\">
            <feGaussianBlur stdDeviation=\".01\"/>
        </filter>
    </defs>
    <g filter=\"url(#a)\">
        <rect x=\"19\" y=\"12\" width=\"14\" height=\"6\" rx=\"3\" transform=\"rotate(31 19 12.79)\"/>
        <rect x=\"19\" y=\"12\" width=\"14\" height=\"6\" rx=\"3\" transform=\"rotate(31 19 12.79)\"/>
    </g>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}
