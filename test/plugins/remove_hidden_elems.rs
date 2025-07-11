// this_file: test/plugins/remove_hidden_elems.rs

//! Tests for the removeHiddenElems plugin
//! Auto-generated from SVGO test fixtures

use vexy_svgo::{optimize, OptimizeOptions, Config, PluginConfig};
use vexy_svgo::config::Js2SvgOptions;
use serde_json::json;

fn test_plugin(input: &str, expected: &str, params: Option<serde_json::Value>) {
    let config = Config {
        plugins: vec![PluginConfig {
            name: "removeHiddenElems".to_string(),
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
        "\nPlugin: removeHiddenElems\nInput:\n{}\nExpected:\n{}\nActual:\n{}\n", 
        input, expected, output);
}

#[test]
fn test_remove_hidden_elems_01() {
    // Remove elements with display=none
    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <style>
      .a { display: block; }
    </style>
    <g>
        <rect display=\"none\" x=\"0\" y=\"0\" width=\"20\" height=\"20\" />
        <rect display=\"none\" class=\"a\" x=\"0\" y=\"0\" width=\"20\" height=\"20\" />
    </g>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <style>
        .a { display: block; }
    </style>
    <g>
        <rect display=\"none\" class=\"a\" x=\"0\" y=\"0\" width=\"20\" height=\"20\"/>
    </g>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_hidden_elems_02() {
    // Remove elements with zero opacity
    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <style>
      .a { opacity: 0.5; }
    </style>
    <g>
        <rect opacity=\"0\" x=\"0\" y=\"0\" width=\"20\" height=\"20\" />
        <rect opacity=\"0\" class=\"a\" x=\"0\" y=\"0\" width=\"20\" height=\"20\" />
    </g>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <style>
        .a { opacity: 0.5; }
    </style>
    <g>
        <rect opacity=\"0\" class=\"a\" x=\"0\" y=\"0\" width=\"20\" height=\"20\"/>
    </g>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_hidden_elems_03() {
    // Remove circle element with zero radius but preserve when animation element is inside
    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g>
        <circle r=\"0\"/>
    </g>
    <circle cx=\"16\" cy=\"3\" r=\"0\">
        <animate attributeName=\"r\" values=\"0;3;0;0\" dur=\"1s\" repeatCount=\"indefinite\" begin=\"0\" keySplines=\"0.2 0.2 0.4 0.8;0.2 0.2 0.4 0.8;0.2 0.2 0.4 0.8\" calcMode=\"spline\"/>
    </circle>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g/>
    <circle cx=\"16\" cy=\"3\" r=\"0\">
        <animate attributeName=\"r\" values=\"0;3;0;0\" dur=\"1s\" repeatCount=\"indefinite\" begin=\"0\" keySplines=\"0.2 0.2 0.4 0.8;0.2 0.2 0.4 0.8;0.2 0.2 0.4 0.8\" calcMode=\"spline\"/>
    </circle>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_hidden_elems_04() {
    // Remove ellipse element with zero radius
    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g>
        <ellipse rx=\"0\"/>
        <ellipse ry=\"0\"/>
    </g>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_hidden_elems_05() {
    // Remove rect element with zero size
    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g>
        <rect width=\"0\"/>
        <rect height=\"0\"/>
    </g>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_hidden_elems_06() {
    // Remove pattern element with zero size
    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g>
        <pattern width=\"0\"/>
        <pattern height=\"0\"/>
    </g>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_hidden_elems_07() {
    // Remove image element with zero size
    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g>
        <image width=\"0\"/>
        <image height=\"0\"/>
    </g>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_hidden_elems_08() {
    // Remove empty or single point paths without markers
    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g>
        <path/>
        <path d=\"z\"/>
        <path d=\"M 50 50\"/>
        <path d=\"M 50 50 L 0\"/>
        <path d=\"M1.25.75\"/>
        <path d=\"M 50 50 20 20\"/>
        <path d=\"M 50,50 20,20\"/>
        <path d=\"M 50 50 H 10\"/>
        <path d=\"M4.1.5.5.1\"/>
        <path d=\"M10.77.45c-.19-.2-.51-.2-.7 0\"/>
        <path d=\"M 6.39441613e-11,8.00287799 C2.85816855e-11,3.58301052 3.5797863,0 8.00005106,0\"/>
        <path d=\"\" marker-start=\"url(#id)\"/>
        <path d=\"\" marker-end=\"url(#id)\"/>
        <path d=\"M 50 50\" marker-start=\"url(#id)\"/>
        <path d=\"M 50 50\" marker-end=\"url(#id)\"/>
    </g>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g>
        <path d=\"M 50 50 20 20\"/>
        <path d=\"M 50,50 20,20\"/>
        <path d=\"M 50 50 H 10\"/>
        <path d=\"M4.1.5.5.1\"/>
        <path d=\"M10.77.45c-.19-.2-.51-.2-.7 0\"/>
        <path d=\"M 6.39441613e-11,8.00287799 C2.85816855e-11,3.58301052 3.5797863,0 8.00005106,0\"/>
        <path d=\"M 50 50\" marker-start=\"url(#id)\"/>
        <path d=\"M 50 50\" marker-end=\"url(#id)\"/>
    </g>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_hidden_elems_09() {
    // Remove polyline without points
    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g>
        <polyline/>
    </g>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_hidden_elems_10() {
    // Remove polygon without points
    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g>
        <polygon/>
    </g>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_hidden_elems_11() {
    // Preserve transparent rect inside clipPath element
    let input = r#"<svg width=\"480\" height=\"360\" xmlns=\"http://www.w3.org/2000/svg\">
  <clipPath id=\"opacityclip\">
    <rect width=\"100\" height=\"100\" opacity=\"0\"/>
  </clipPath>
  <rect x=\"0.5\" y=\"0.5\" width=\"99\" height=\"99\" fill=\"red\"/>
  <rect width=\"100\" height=\"100\" fill=\"lime\" clip-path=\"url(#opacityclip)\"/>
</svg>"#;

    let expected = r#"<svg width=\"480\" height=\"360\" xmlns=\"http://www.w3.org/2000/svg\">
    <clipPath id=\"opacityclip\">
        <rect width=\"100\" height=\"100\" opacity=\"0\"/>
    </clipPath>
    <rect x=\"0.5\" y=\"0.5\" width=\"99\" height=\"99\" fill=\"red\"/>
    <rect width=\"100\" height=\"100\" fill=\"lime\" clip-path=\"url(#opacityclip)\"/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_hidden_elems_12() {
    // Keep invisible elements which have visible ones inside
    // and resolve styles
    let input = r#"<svg width=\"480\" height=\"360\" xmlns=\"http://www.w3.org/2000/svg\">
  <style>
    .a { visibility: visible; }
  </style>
  <rect x=\"96\" y=\"96\" width=\"96\" height=\"96\" fill=\"lime\" />
  <g visibility=\"hidden\">
    <rect x=\"96\" y=\"96\" width=\"96\" height=\"96\" fill=\"red\" />
  </g>
  <rect x=\"196.5\" y=\"196.5\" width=\"95\" height=\"95\" fill=\"red\"/>
  <g visibility=\"hidden\">
    <rect x=\"196\" y=\"196\" width=\"96\" height=\"96\" fill=\"lime\" visibility=\"visible\" />
  </g>
  <rect x=\"96\" y=\"96\" width=\"96\" height=\"96\" visibility=\"hidden\" class=\"a\" />
</svg>"#;

    let expected = r#"<svg width=\"480\" height=\"360\" xmlns=\"http://www.w3.org/2000/svg\">
    <style>
        .a { visibility: visible; }
    </style>
    <rect x=\"96\" y=\"96\" width=\"96\" height=\"96\" fill=\"lime\"/>
    <rect x=\"196.5\" y=\"196.5\" width=\"95\" height=\"95\" fill=\"red\"/>
    <g visibility=\"hidden\">
        <rect x=\"196\" y=\"196\" width=\"96\" height=\"96\" fill=\"lime\" visibility=\"visible\"/>
    </g>
    <rect x=\"96\" y=\"96\" width=\"96\" height=\"96\" visibility=\"hidden\" class=\"a\"/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_hidden_elems_13() {
    // When removing a useless definition, remove references to that definition.
    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 64 64\">
  <defs>
    <path d=\"M15.852 62.452\" id=\"a\"/>
  </defs>
  <use href=\"#a\"/>
  <use opacity=\".35\" href=\"#a\"/>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 64 64\"/>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_hidden_elems_14() {
    // Remove unused defs
    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <defs>
        <linearGradient id=\"a\">
        </linearGradient>
    </defs>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\"/>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_hidden_elems_15() {
    // Don PROTECTED_46_ t remove non-rendering elements if children have IDs.
    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <rect fill=\"url(#a)\" width=\"64\" height=\"64\"/>
    <symbol>
        <linearGradient id=\"a\">
            <stop offset=\"5%\" stop-color=\"gold\" />
        </linearGradient>
    </symbol>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <rect fill=\"url(#a)\" width=\"64\" height=\"64\"/>
    <symbol>
        <linearGradient id=\"a\">
            <stop offset=\"5%\" stop-color=\"gold\"/>
        </linearGradient>
    </symbol>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_hidden_elems_17() {
    // Don't remove nodes that have children with referenced IDs.
    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <rect fill=\"url(#a)\" width=\"64\" height=\"64\"/>
    <g>
        <linearGradient id=\"a\">
            <stop offset=\"5%\" stop-color=\"gold\" />
        </linearGradient>
    </g>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <rect fill=\"url(#a)\" width=\"64\" height=\"64\"/>
    <g>
        <linearGradient id=\"a\">
            <stop offset=\"5%\" stop-color=\"gold\"/>
        </linearGradient>
    </g>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_hidden_elems_18() {
    // Preserve <defs> with referenced path.
    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\">
    <g id=\"test-body-content\">
        <defs>
            <path id=\"reference\" d=\"M240 1h239v358H240z\"/>
        </defs>
        <use xlink:href=\"#reference\" id=\"use\" fill=\"gray\" onclick=\"test(evt)\"/>
    </g>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\">
    <g id=\"test-body-content\">
        <defs>
            <path id=\"reference\" d=\"M240 1h239v358H240z\"/>
        </defs>
        <use xlink:href=\"#reference\" id=\"use\" fill=\"gray\" onclick=\"test(evt)\"/>
    </g>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_remove_hidden_elems_19() {
    // Preserve referenced path, even when path has opacity=0.
    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\">
    <defs>
        <path id=\"path2\" d=\"M200 200 l50 -300\" style=\"opacity:0\"/>
    </defs>
    <text style=\"font-size:24px;\">
        <textPath xlink:href=\"#path2\">
        this is path 2
        </textPath>
    </text>
    <path id=\"path1\" d=\"M200 200 l50 -300\" style=\"opacity:0\"/>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\">
    <defs>
        <path id=\"path2\" d=\"M200 200 l50 -300\" style=\"opacity:0\"/>
    </defs>
    <text style=\"font-size:24px;\">
        <textPath xlink:href=\"#path2\">
        this is path 2
        </textPath>
    </text>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}
    <text style=\"font-size:24px;\">
        <textPath xlink:href=\"#path2\">
        this is path 2
        </textPath>
    </text>
    <path id=\"path1\" d=\"M200 200 l50 -300\" style=\"opacity:0\"/>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\">
    <defs>
        <path id=\"path2\" d=\"M200 200 l50 -300\" style=\"opacity:0\"/>
    </defs>
    <text style=\"font-size:24px;\">
        <textPath xlink:href=\"#path2\">
        this is path 2
        </textPath>
    </text>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}
