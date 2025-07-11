// this_file: test/plugins/collapse_groups.rs

//! Tests for the collapseGroups plugin
//! Auto-generated from SVGO test fixtures

use vexy_svgo::{optimize, OptimizeOptions, Config, PluginConfig};
use vexy_svgo::config::Js2SvgOptions;
use serde_json::json;

fn test_plugin(input: &str, expected: &str, params: Option<serde_json::Value>) {
    let config = Config {
        plugins: vec![PluginConfig {
            name: "collapseGroups".to_string(),
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
        "\nPlugin: collapseGroups\nInput:\n{}\nExpected:\n{}\nActual:\n{}\n", 
        input, expected, output);
}

#[test]
fn test_collapse_groups_01() {
    // Collapse groups without attributes
    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g>
        <g>
            <path d=\"...\"/>
        </g>
    </g>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <path d=\"...\"/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_collapse_groups_02() {
    // Inherit attributes to single child
    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g>
        <g attr1=\"val1\">
            <path d=\"...\"/>
        </g>
    </g>
    <g attr1=\"val1\">
        <g attr2=\"val2\">
            <path d=\"...\"/>
        </g>
    </g>
    <g attr1=\"val1\">
        <g>
            <path d=\"...\"/>
        </g>
        <path d=\"...\"/>
    </g>
    <g attr1=\"val1\">
        <g attr2=\"val2\">
            <path d=\"...\"/>
        </g>
        <path d=\"...\"/>
    </g>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <path d=\"...\" attr1=\"val1\"/>
    <path d=\"...\" attr2=\"val2\" attr1=\"val1\"/>
    <g attr1=\"val1\">
        <path d=\"...\"/>
        <path d=\"...\"/>
    </g>
    <g attr1=\"val1\">
        <path d=\"...\" attr2=\"val2\"/>
        <path d=\"...\"/>
    </g>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_collapse_groups_06() {
    // Remove inheritable overridden groups attributes
    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g attr1=\"val1\">
        <g fill=\"red\">
            <path fill=\"green\" d=\"...\"/>
        </g>
        <path d=\"...\"/>
    </g>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g attr1=\"val1\">
        <path fill=\"green\" d=\"...\"/>
        <path d=\"...\"/>
    </g>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_collapse_groups_07() {
    // Remove equal overridden groups attributes
    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g attr1=\"val1\">
        <g attr2=\"val2\">
            <path attr2=\"val2\" d=\"...\"/>
        </g>
        <g attr2=\"val2\">
            <path attr2=\"val3\" d=\"...\"/>
        </g>
    </g>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g attr1=\"val1\">
        <path attr2=\"val2\" d=\"...\"/>
        <g attr2=\"val2\">
            <path attr2=\"val3\" d=\"...\"/>
        </g>
    </g>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_collapse_groups_08() {
    // Combine own child transform and inherited
    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g attr1=\"val1\">
        <g transform=\"rotate(45)\">
            <path transform=\"scale(2)\" d=\"...\"/>
        </g>
        <path d=\"...\"/>
    </g>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g attr1=\"val1\">
        <path transform=\"rotate(45) scale(2)\" d=\"...\"/>
        <path d=\"...\"/>
    </g>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_collapse_groups_09() {
    // Preserve transform when group has clip-path
    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <clipPath id=\"a\">
       <path d=\"...\"/>
    </clipPath>
    <clipPath id=\"b\">
       <path d=\"...\"/>
    </clipPath>
    <g transform=\"matrix(0 -1.25 -1.25 0 100 100)\" clip-path=\"url(#a)\">
        <g transform=\"scale(.2)\">
            <path d=\"...\"/>
            <path d=\"...\"/>
        </g>
    </g>
    <g transform=\"matrix(0 -1.25 -1.25 0 100 100)\" clip-path=\"url(#a)\">
        <g transform=\"scale(.2)\">
            <g>
                <g clip-path=\"url(#b)\">
                    <path d=\"...\"/>
                    <path d=\"...\"/>
                </g>
            </g>
        </g>
    </g>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <clipPath id=\"a\">
        <path d=\"...\"/>
    </clipPath>
    <clipPath id=\"b\">
        <path d=\"...\"/>
    </clipPath>
    <g transform=\"matrix(0 -1.25 -1.25 0 100 100)\" clip-path=\"url(#a)\">
        <g transform=\"scale(.2)\">
            <path d=\"...\"/>
            <path d=\"...\"/>
        </g>
    </g>
    <g transform=\"matrix(0 -1.25 -1.25 0 100 100)\" clip-path=\"url(#a)\">
        <g clip-path=\"url(#b)\" transform=\"scale(.2)\">
            <path d=\"...\"/>
            <path d=\"...\"/>
        </g>
    </g>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_collapse_groups_11() {
    // Preserve groups when clip-path and mask are used without any other attributes
    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <clipPath id=\"a\">
       <path d=\"...\"/>
    </clipPath>
    <path d=\"...\"/>
    <g clip-path=\"url(#a)\">
        <path d=\"...\" transform=\"scale(.2)\"/>
    </g>
    <g mask=\"url(#a)\">
        <path d=\"...\" transform=\"scale(.2)\"/>
    </g>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <clipPath id=\"a\">
        <path d=\"...\"/>
    </clipPath>
    <path d=\"...\"/>
    <g clip-path=\"url(#a)\">
        <path d=\"...\" transform=\"scale(.2)\"/>
    </g>
    <g mask=\"url(#a)\">
        <path d=\"...\" transform=\"scale(.2)\"/>
    </g>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_collapse_groups_12() {
    // Preserve groups with id attribute or animation elements inside
    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g stroke=\"#000\">
        <g id=\"star\">
            <path id=\"bar\" d=\"...\"/>
        </g>
    </g>
    <g>
        <animate id=\"frame0\" attributeName=\"visibility\" values=\"visible\" dur=\"33ms\" begin=\"0s;frame27.end\"/>
        <path d=\"...\" fill=\"#272727\"/>
        <path d=\"...\" fill=\"#404040\"/>
        <path d=\"...\" fill=\"#2d2d2d\"/>
    </g>
    <g transform=\"rotate(-90 25 0)\">
        <circle stroke-dasharray=\"110\" r=\"20\" stroke=\"#10cfbd\" fill=\"none\" stroke-width=\"3\" stroke-linecap=\"round\">
            <animate attributeName=\"stroke-dashoffset\" values=\"360;140\" dur=\"2.2s\" keyTimes=\"0;1\" calcMode=\"spline\" fill=\"freeze\" keySplines=\"0.41,0.314,0.8,0.54\" repeatCount=\"indefinite\" begin=\"0\"/>
            <animateTransform attributeName=\"transform\" type=\"rotate\" values=\"0;274;360\" keyTimes=\"0;0.74;1\" calcMode=\"linear\" dur=\"2.2s\" repeatCount=\"indefinite\" begin=\"0\"/>
        </circle>
    </g>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g stroke=\"#000\">
        <g id=\"star\">
            <path id=\"bar\" d=\"...\"/>
        </g>
    </g>
    <g>
        <animate id=\"frame0\" attributeName=\"visibility\" values=\"visible\" dur=\"33ms\" begin=\"0s;frame27.end\"/>
        <path d=\"...\" fill=\"#272727\"/>
        <path d=\"...\" fill=\"#404040\"/>
        <path d=\"...\" fill=\"#2d2d2d\"/>
    </g>
    <g transform=\"rotate(-90 25 0)\">
        <circle stroke-dasharray=\"110\" r=\"20\" stroke=\"#10cfbd\" fill=\"none\" stroke-width=\"3\" stroke-linecap=\"round\">
            <animate attributeName=\"stroke-dashoffset\" values=\"360;140\" dur=\"2.2s\" keyTimes=\"0;1\" calcMode=\"spline\" fill=\"freeze\" keySplines=\"0.41,0.314,0.8,0.54\" repeatCount=\"indefinite\" begin=\"0\"/>
            <animateTransform attributeName=\"transform\" type=\"rotate\" values=\"0;274;360\" keyTimes=\"0;0.74;1\" calcMode=\"linear\" dur=\"2.2s\" repeatCount=\"indefinite\" begin=\"0\"/>
        </circle>
    </g>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_collapse_groups_13() {
    // Preserve groups with classes
    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <style>
        .n{display:none}
        .i{display:inline}
    </style>
    <g id=\"a\">
        <g class=\"i\"/>
    </g>
    <g id=\"b\" class=\"n\">
        <g class=\"i\"/>
    </g>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <style>
        .n{display:none}
        .i{display:inline}
    </style>
    <g class=\"i\" id=\"a\"/>
    <g id=\"b\" class=\"n\">
        <g class=\"i\"/>
    </g>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_collapse_groups_14() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <switch>
        <g id=\"a\">
            <g class=\"i\"/>
        </g>
        <g id=\"b\" class=\"n\">
            <g class=\"i\"/>
        </g>
        <g>
            <g/>
        </g>
    </switch>

</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <switch>
        <g id=\"a\">
            <g class=\"i\"/>
        </g>
        <g id=\"b\" class=\"n\">
            <g class=\"i\"/>
        </g>
        <g>
            <g/>
        </g>
    </switch>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_collapse_groups_15() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
	<g color=\"red\">
		<g color=\"inherit\" fill=\"none\" stroke=\"none\">
			<circle cx=\"130\" cy=\"80\" r=\"60\" fill=\"currentColor\"/>
			<circle cx=\"350\" cy=\"80\" r=\"60\" stroke=\"currentColor\" stroke-width=\"4\"/>
		</g>
	</g>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g color=\"red\" fill=\"none\" stroke=\"none\">
        <circle cx=\"130\" cy=\"80\" r=\"60\" fill=\"currentColor\"/>
        <circle cx=\"350\" cy=\"80\" r=\"60\" stroke=\"currentColor\" stroke-width=\"4\"/>
    </g>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_collapse_groups_16() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g filter=\"url(#...)\">
        <g>
            <path d=\"...\"/>
        </g>
    </g>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g filter=\"url(#...)\">
        <path d=\"...\"/>
    </g>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_collapse_groups_17() {
    // Don PROTECTED_33_ t collapse groups if outer group has filter (as style or attribute).
    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 100 100\">
    <clipPath id=\"a\">
        <circle cx=\"25\" cy=\"15\" r=\"10\"/>
    </clipPath>
    <filter id=\"b\">
        <feColorMatrix type=\"saturate\"/>
    </filter>
    <g filter=\"url(#b)\">
        <g clip-path=\"url(#a)\">
            <circle cx=\"30\" cy=\"10\" r=\"10\" fill=\"yellow\" id=\"c1\"/>
        </g>
    </g>
    <g style=\"filter:url(#b)\">
        <g clip-path=\"url(#a)\">
            <circle cx=\"20\" cy=\"10\" r=\"10\" fill=\"blue\" id=\"c2\"/>
        </g>
    </g>
    <circle cx=\"25\" cy=\"15\" r=\"10\" stroke=\"black\" stroke-width=\".1\" fill=\"none\"/>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 100 100\">
    <clipPath id=\"a\">
        <circle cx=\"25\" cy=\"15\" r=\"10\"/>
    </clipPath>
    <filter id=\"b\">
        <feColorMatrix type=\"saturate\"/>
    </filter>
    <g filter=\"url(#b)\">
        <g clip-path=\"url(#a)\">
            <circle cx=\"30\" cy=\"10\" r=\"10\" fill=\"yellow\" id=\"c1\"/>
        </g>
    </g>
    <g style=\"filter:url(#b)\">
        <g clip-path=\"url(#a)\">
            <circle cx=\"20\" cy=\"10\" r=\"10\" fill=\"blue\" id=\"c2\"/>
        </g>
    </g>
    <circle cx=\"25\" cy=\"15\" r=\"10\" stroke=\"black\" stroke-width=\".1\" fill=\"none\"/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}
    <circle cx=\"25\" cy=\"15\" r=\"10\" stroke=\"black\" stroke-width=\".1\" fill=\"none\"/>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 100 100\">
    <clipPath id=\"a\">
        <circle cx=\"25\" cy=\"15\" r=\"10\"/>
    </clipPath>
    <filter id=\"b\">
        <feColorMatrix type=\"saturate\"/>
    </filter>
    <g filter=\"url(#b)\">
        <g clip-path=\"url(#a)\">
            <circle cx=\"30\" cy=\"10\" r=\"10\" fill=\"yellow\" id=\"c1\"/>
        </g>
    </g>
    <g style=\"filter:url(#b)\">
        <g clip-path=\"url(#a)\">
            <circle cx=\"20\" cy=\"10\" r=\"10\" fill=\"blue\" id=\"c2\"/>
        </g>
    </g>
    <circle cx=\"25\" cy=\"15\" r=\"10\" stroke=\"black\" stroke-width=\".1\" fill=\"none\"/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}
