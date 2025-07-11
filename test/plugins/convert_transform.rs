// this_file: test/plugins/convert_transform.rs

//! Tests for the convertTransform plugin
//! Auto-generated from SVGO test fixtures

use vexy_svgo::{optimize, OptimizeOptions, Config, PluginConfig};
use vexy_svgo::config::Js2SvgOptions;
use serde_json::json;

fn test_plugin(input: &str, expected: &str, params: Option<serde_json::Value>) {
    let config = Config {
        plugins: vec![PluginConfig {
            name: "convertTransform".to_string(),
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
        "\nPlugin: convertTransform\nInput:\n{}\nExpected:\n{}\nActual:\n{}\n", 
        input, expected, output);
}

#[test]
fn test_convert_transform_01() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"300\" height=\"300\">
    <rect width=\"10\" height=\"20\" transform=\"matrix(0.707 -0.707 0.707 0.707 255.03 111.21)\"/>
    <rect width=\"10\" height=\"20\" transform=\"matrix(1 0 0 1 50 90),matrix(0.707 -0.707 0.707 0.707 0 0) ,matrix(1 0 0 1 130 160)\"/>
    <rect width=\"10\" height=\"20\" transform=\"translate(50 90) , rotate(-45)   translate(130 160)\"/>
    <rect width=\"10\" height=\"20\" transform=\"matrix(0.707 -0.707 0.707 0.707 255.03 111.21) scale(2)\"/>
    <rect width=\"10\" height=\"20\" transform=\"matrix(0.707 -0.707 0.707 0.707 255.03 111.21) skewX(45)\"/>
    <rect width=\"10\" height=\"20\" transform=\"matrix( 0.707 -0.707 0.707 0.707 255.03 111.21 ) skewY( 45 )\"/>
    <rect width=\"10\" height=\"20\" transform=\"matrix(1 0 1 1 0 0)\"/>
    <rect width=\"10\" height=\"20\" transform=\"matrix(1.25,0,0,-1.25,0,56.26) scale(1,-1)\"/>
    <rect width=\"10\" height=\"20\" transform=\"matrix(1.25,0,0,-1.25,0,56.26) matrix(0.1325312,0,0,-0.1325312,-31.207631,89.011662)\"/>
    <rect width=\"10\" height=\"20\" transform=\"matrix(1 0 0 -1 0 0)\"/>
    <rect width=\"10\" height=\"20\" transform=\"matrix(-1 0 0 1 0 0)\"/>
    <rect width=\"10\" height=\"20\" transform=\"matrix(0 1-1 0 0 0)\"/>
    <rect width=\"10\" height=\"20\" transform=\"matrix(0-1 1 0 0 0)\"/>
    <rect width=\"10\" height=\"20\" transform=\"matrix(0.707 -0.707 -0.707 -0.707 0 0)\"/>
    <rect width=\"10\" height=\"20\" transform=\"matrix(-0.707 0.707 0.707 0.707 0 0)\"/>
    <rect width=\"10\" height=\"20\" transform=\"matrix(-0.707 0.707 -0.707 -0.707 0 0)\"/>
    <rect width=\"10\" height=\"20\" transform=\"matrix(0.707 0.707 -0.707 0.707 0 0)\"/>
    <rect width=\"10\" height=\"20\" transform=\"matrix(.647 -.647 -.6443 -.6443 0 0)\"/>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"300\" height=\"300\">
    <rect width=\"10\" height=\"20\" transform=\"rotate(-45 261.757 -252.243)\"/>
    <rect width=\"10\" height=\"20\" transform=\"rotate(-45 261.757 -252.243)\"/>
    <rect width=\"10\" height=\"20\" transform=\"rotate(-45 261.777 -252.28)\"/>
    <rect width=\"10\" height=\"20\" transform=\"rotate(-45 261.757 -252.243)scale(2)\"/>
    <rect width=\"10\" height=\"20\" transform=\"rotate(-45 261.757 -252.243)skewX(45)\"/>
    <rect width=\"10\" height=\"20\" transform=\"rotate(-45 261.757 -252.243)skewY(45)\"/>
    <rect width=\"10\" height=\"20\" transform=\"skewX(45)\"/>
    <rect width=\"10\" height=\"20\" transform=\"translate(0 56.26)scale(1.25)\"/>
    <rect width=\"10\" height=\"20\" transform=\"translate(-39.01 -55.005)scale(.16566)\"/>
    <rect width=\"10\" height=\"20\" transform=\"scale(1 -1)\"/>
    <rect width=\"10\" height=\"20\" transform=\"scale(-1 1)\"/>
    <rect width=\"10\" height=\"20\" transform=\"rotate(90)\"/>
    <rect width=\"10\" height=\"20\" transform=\"rotate(-90)\"/>
    <rect width=\"10\" height=\"20\" transform=\"rotate(-45)scale(1 -1)\"/>
    <rect width=\"10\" height=\"20\" transform=\"rotate(135)scale(1 -1)\"/>
    <rect width=\"10\" height=\"20\" transform=\"rotate(135)\"/>
    <rect width=\"10\" height=\"20\" transform=\"rotate(45)\"/>
    <rect width=\"10\" height=\"20\" transform=\"rotate(-45)scale(.915 -.9112)\"/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_convert_transform_02() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g transform=\"translate(50 0) scale(2 2)\"/>
    <g transform=\"translate(50) scale(2 2)\"/>
    <g transform=\"translate(10 20) rotate(45) translate(-10-20)\"/>
    <g transform=\"scale(2) translate(10 20) rotate(45) translate(-10-20)\"/>
    <g transform=\"rotate(15) scale(2 1)\"/>
    <g transform=\"scale(2 1) rotate(15)\"/>
    <g transform=\"translate(10 20) rotate(45) translate(-10-20) scale(2)\"/>
    <g transform=\"translate(15, 3) translate(13) rotate(47 39.885486 39.782373)\"/>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g transform=\"matrix(2 0 0 2 50 0)\"/>
    <g transform=\"matrix(2 0 0 2 50 0)\"/>
    <g transform=\"rotate(45 10 20)\"/>
    <g transform=\"rotate(45 20 40)scale(2)\"/>
    <g transform=\"rotate(15)scale(2 1)\"/>
    <g transform=\"matrix(1.93185 .25882 -.51764 .96593 0 0)\"/>
    <g transform=\"rotate(45 10 20)scale(2)\"/>
    <g transform=\"rotate(47 50.436 73.48)\"/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_convert_transform_03() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g transform=\"matrix(1 0 0 1 50 100)\"/>
    <g transform=\"matrix(0.5 0 0 2 0 0)\"/>
    <g transform=\"matrix(.707-.707.707.707 0 0)\"/>
    <g transform=\"matrix(1 0 0.466 1 0 0)\"/>
    <g transform=\"matrix(1 0.466 0 1 0 0)\"/>
    <g transform=\"matrix(1 0 0 1 50 90) matrix(1 0 0 1 60 20) matrix(1 0 0 1 20 40)\"/>
    <g transform=\"matrix(-0.10443115234375 0 0 -0.10443115234375 182.15 61.15)\"/>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g transform=\"translate(50 100)\"/>
    <g transform=\"scale(.5 2)\"/>
    <g transform=\"rotate(-45)\"/>
    <g transform=\"skewX(24.99)\"/>
    <g transform=\"skewY(24.99)\"/>
    <g transform=\"translate(130 150)\"/>
    <g transform=\"translate(182.15 61.15)scale(-.10443)\"/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_convert_transform_04() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g transform=\"\"/>
    <g transform=\"translate(0)\"/>
    <g transform=\"translate(0 0)\"/>
    <g transform=\"translate(0 50)\"/>
    <g transform=\"scale(1)\"/>
    <g transform=\"scale(1 2)\"/>
    <g transform=\"rotate(0)\"/>
    <g transform=\"rotate(0 100 100)\"/>
    <g transform=\"skewX(0)\"/>
    <g transform=\"skewY(0)\"/>
    <g transform=\"translate(0,-100) translate(0,100)\"/>
    <g transform=\"rotate(45, 34, 34\"/>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <g/>
    <g/>
    <g/>
    <g transform=\"translate(0 50)\"/>
    <g/>
    <g transform=\"scale(1 2)\"/>
    <g/>
    <g/>
    <g/>
    <g/>
    <g/>
    <g/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_convert_transform_05() {
    // Correctly optimize transform with same sign non-zero shears and.
    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 128 128\">
  <rect x=\"-45\" y=\"-77\" height=\"3\" width=\"8\" transform=\"matrix(0,-1,-1,0,0,0)\" />
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 128 128\">
    <rect x=\"-45\" y=\"-77\" height=\"3\" width=\"8\" transform=\"rotate(90)scale(-1 1)\"/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_convert_transform_06() {
    // Test matrices which are identities after rounding.
    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 256 256\">
  <text y=\"32\" transform=\"matrix(1.0000002 0 0 1 0 0)\">uwu</text>
  <text y=\"64\" transform=\"matrix(1 0 0 1 0.00002 0)\">uwu</text>
  <text y=\"96\" transform=\"matrix(0.9999999847691 1.745329243133368e-4 -1.745329243133368e-4 0.9999999847691 0 0)\">uwu</text>
  <text y=\"128\" transform=\"matrix(1.0000002 0 0 1 0.00002 0)\">uwu</text>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 256 256\">
    <text y=\"32\">uwu</text>
    <text y=\"64\">uwu</text>
    <text y=\"96\">uwu</text>
    <text y=\"128\">uwu</text>
</svg>"#;

    let params = serde_json::from_str(r#"{
  "degPrecision": 1
}"#).ok();

    test_plugin(input, expected, params);
}

#[test]
fn test_convert_transform_07() {
    // Test with skewX and sx != sy
    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 64 64\">
  <text x=\"-32\" y=\"32\" transform=\"matrix(-1,0,-0.3,0.9,0,0)\">uwu</text>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 64 64\">
    <text x=\"-32\" y=\"32\" transform=\"scale(-1 .9)skewX(16.7)\">uwu</text>
</svg>"#;

    let params = serde_json::from_str(r#"{
  "degPrecision": 3
}"#).ok();

    test_plugin(input, expected, params);
}

#[test]
fn test_convert_transform_10() {
    // Make sure translate(n,n) and translate(n) work.
    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"500\" height=\"500\" viewBox=\"-100 -100 100 100\">
    <rect x=\"0\" y=\"0\" width=\"10\" height=\"20\" transform=\"matrix(1,0,0,1,3,0)\"/>
    <rect x=\"0\" y=\"0\" width=\"10\" height=\"20\" transform=\"matrix(1,0,0,1,3,3)\"/>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"500\" height=\"500\" viewBox=\"-100 -100 100 100\">
    <rect x=\"0\" y=\"0\" width=\"10\" height=\"20\" transform=\"translate(3)\"/>
    <rect x=\"0\" y=\"0\" width=\"10\" height=\"20\" transform=\"translate(3 3)\"/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_convert_transform_11() {
    // Test with 180 degree rotation, translation, and no scaling in matrix. Matrix not changed,
    // since it is shorter than translate(5,7)scale(-1).
    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"500\" height=\"500\" viewBox=\"-100 -100 100 100\">
    <rect x=\"0\" y=\"0\" width=\"10\" height=\"20\" transform=\"matrix(-1,0,0,-1,5,7)\"/>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"500\" height=\"500\" viewBox=\"-100 -100 100 100\">
    <rect x=\"0\" y=\"0\" width=\"10\" height=\"20\" transform=\"matrix(-1 0 0 -1 5 7)\"/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_convert_transform_12() {
    // Test with 180 degree rotation and no scaling in matrix.
    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"500\" height=\"500\" viewBox=\"-100 -100 100 100\">
    <rect x=\"0\" y=\"0\" width=\"10\" height=\"20\" transform=\"matrix(-1,0,0,-1,0 0)\"/>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"500\" height=\"500\" viewBox=\"-100 -100 100 100\">
    <rect x=\"0\" y=\"0\" width=\"10\" height=\"20\" transform=\"scale(-1)\"/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_convert_transform_13() {
    // Test rotate()scale(), rotate()skewX() when starting with matrix.
    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"-50 -50 100 100\">
    <rect x=\"0\" y=\"0\" width=\"10\" height=\"20\" transform=\"matrix(1.93185,0.51764,-0.25882,0.96593,0,0)\"/>
    <rect x=\"-20\" y=\"-20\" width=\"10\" height=\"20\" transform=\"matrix(0.85606,0.66883,-0.25882,0.96593,0,0)\"/>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"-50 -50 100 100\">
    <rect x=\"0\" y=\"0\" width=\"10\" height=\"20\" transform=\"rotate(15)scale(2 1)\"/>
    <rect x=\"-20\" y=\"-20\" width=\"10\" height=\"20\" transform=\"rotate(15)skewY(23)\"/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_convert_transform_14() {
    // Test to make sure rotate(180) inverts scale(1).
    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 200 200\">
    <rect x=\"20\" y=\"30\" width=\"40\" height=\"50\" transform=\"matrix(-1,-4.371139e-8,4.371139e-8,-1,139.2007,136.8)\"/>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 200 200\">
    <rect x=\"20\" y=\"30\" width=\"40\" height=\"50\" transform=\"translate(139.2007 136.8)scale(-1)\"/>
</svg>"#;

    let params = serde_json::from_str(r#"{
  "degPrecision": 4,
  "floatPrecision": 6,
  "transformPrecision": 8
}"#).ok();

    test_plugin(input, expected, params);
}

#[test]
fn test_convert_transform_15() {
    // Make sure scale(n,0) is handled correctly.
    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"-10 -10 100 150\">
    <rect x=\"0\" y=\"10\" width=\"5\" height=\"8\" fill=\"red\" transform=\"translate(5,70) scale(.4 0)\"/>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"-10 -10 100 150\">
    <rect x=\"0\" y=\"10\" width=\"5\" height=\"8\" fill=\"red\" transform=\"matrix(.4 0 0 0 5 70)\"/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}
