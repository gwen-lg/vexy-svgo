// this_file: test/plugins/merge_paths.rs

//! Tests for the mergePaths plugin
//! Auto-generated from SVGO test fixtures

use vexy_svgo::{optimize, OptimizeOptions, Config, PluginConfig};
use vexy_svgo::config::Js2SvgOptions;
use serde_json::json;

fn test_plugin(input: &str, expected: &str, params: Option<serde_json::Value>) {
    let config = Config {
        plugins: vec![PluginConfig {
            name: "mergePaths".to_string(),
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
        "\nPlugin: mergePaths\nInput:\n{}\nExpected:\n{}\nActual:\n{}\n", 
        input, expected, output);
}

#[test]
fn test_merge_paths_01() {
    // Merge sequences of paths without attributes
    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <path d=\"M 0,0 z\"/>
    <path d=\"M 10,10 z\"/>
    <path d=\"M 20,20 l 10,10 M 30,0 c 10,0 20,10 20,20\"/>
    <path d=\"M 30,30 z\"/>
    <path d=\"M 30,30 z\" fill=\"#f00\"/>
    <path d=\"M 40,40 z\"/>
    <path d=\"m 50,50 0,10 20,30 40,0\"/>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <path d=\"M0 0zM10 10zM20 20l10 10M30 0c10 0 20 10 20 20M30 30z\"/>
    <path d=\"M 30,30 z\" fill=\"#f00\"/>
    <path d=\"M40 40zM50 50l0 10 20 30 40 0\"/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_merge_paths_02() {
    // Merge sequences of paths with the same attributes
    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <path d=\"M 0,0 z\" fill=\"#fff\" stroke=\"#333\"/>
    <path d=\"M 10,10 z\" fill=\"#fff\" stroke=\"#333\"/>
    <path d=\"M 20,20\" fill=\"#fff\" stroke=\"#333\"/>
    <path d=\"M 30,30 z\" fill=\"#fff\" stroke=\"#333\"/>
    <path d=\"M 30,30 z\" fill=\"#f00\"/>
    <path d=\"M 40,40 z\"/>
    <path d=\"m 50,50 z\"/>
    <path d=\"M 40,40\"/>
    <path d=\"m 50,50\"/>
    <path d=\"M 40,40 z\" fill=\"#fff\" stroke=\"#333\"/>
    <path d=\"m 50,50 z\" fill=\"#fff\" stroke=\"#333\"/>
    <path d=\"M 40,40\" fill=\"#fff\" stroke=\"#333\"/>
    <path d=\"m 50,50\" fill=\"#fff\" stroke=\"#333\"/>
    <path d=\"m 50,50 z\" fill=\"#fff\" stroke=\"#333\"/>
    <path d=\"M0 0v100h100V0z\" fill=\"red\"/>
    <path d=\"M200 0v100h100V0z\" fill=\"red\"/>
    <path d=\"M0 0v100h100V0z\" fill=\"blue\"/>
    <path d=\"M200 0v100h100V0zM0 200h100v100H0z\" fill=\"blue\"/>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <path d=\"M0 0zM10 10zM30 30z\" fill=\"#fff\" stroke=\"#333\"/>
    <path d=\"M 30,30 z\" fill=\"#f00\"/>
    <path d=\"M40 40zM50 50zM50 50\"/>
    <path d=\"M40 40zM50 50zM50 50z\" fill=\"#fff\" stroke=\"#333\"/>
    <path d=\"M0 0v100h100V0zM200 0v100h100V0z\" fill=\"red\"/>
    <path d=\"M0 0v100h100V0zM200 0v100h100V0zM0 200h100v100H0z\" fill=\"blue\"/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_merge_paths_03() {
    // Merge only intersected paths
    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <path d=\"M30 0L0 40H60z\"/>
    <path d=\"M0 10H60L30 50z\"/>
    <path d=\"M0 0V50L50 0\"/>
    <path d=\"M0 60L50 10V60\"/>
    <g>
        <path d=\"M100 0a50 50 0 0 1 0 100\"/>
        <path d=\"M25 25H75V75H25z\"/>
        <path d=\"M135 85H185V135H135z\"/>
    </g>
    <g>
        <path d=\"M10 14H7v1h3v-1z\"/>
        <path d=\"M9 21H8v1h1v-1z\"/>
    </g>
    <g>
        <path d=\"M30 32.705V40h10.42L30 32.705z\"/>
        <path d=\"M46.25 34.928V30h-7.04l7.04 4.928z\"/>
    </g>
    <g>
        <path d=\"M20 20H60L100 30\"/>
        <path d=\"M20 20L50 30H100\"/>
    </g>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <path d=\"M30 0L0 40H60z\"/>
    <path d=\"M0 10H60L30 50z\"/>
    <path d=\"M0 0V50L50 0M0 60 50 10V60\"/>
    <g>
        <path d=\"M100 0a50 50 0 0 1 0 100M25 25H75V75H25z\"/>
        <path d=\"M135 85H185V135H135z\"/>
    </g>
    <g>
        <path d=\"M10 14H7v1h3v-1zM9 21H8v1h1v-1z\"/>
    </g>
    <g>
        <path d=\"M30 32.705V40h10.42L30 32.705zM46.25 34.928V30h-7.04l7.04 4.928z\"/>
    </g>
    <g>
        <path d=\"M20 20H60L100 30M20 20 50 30H100\"/>
    </g>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_merge_paths_04() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <path d=\"M320 60c17.466-8.733 33.76-12.78 46.593-12.484 12.856.297 22.254 4.936 26.612 12.484 4.358 7.548 3.676 18.007-2.494 29.29-6.16 11.26-17.812 23.348-34.107 34.107-16.26 10.735-37.164 20.14-60.72 26.613C272.356 156.473 246.178 160 220 160c-26.18 0-52.357-3.527-75.882-9.99-23.557-6.472-44.462-15.878-60.72-26.613-16.296-10.76-27.95-22.846-34.11-34.108-6.17-11.283-6.85-21.742-2.493-29.29 4.358-7.548 13.756-12.187 26.612-12.484C86.24 47.22 102.535 51.266 120 60c17.426 8.713 36.024 22.114 53.407 39.28C190.767 116.42 206.91 137.33 220 160c13.09 22.67 23.124 47.106 29.29 70.71 6.173 23.638 8.48 46.445 7.313 65.893-1.17 19.49-5.812 35.627-12.485 46.592C237.432 354.18 228.716 360 220 360s-17.432-5.82-24.118-16.805c-6.673-10.965-11.315-27.1-12.485-46.592-1.167-19.448 1.14-42.255 7.314-65.892 6.166-23.604 16.2-48.04 29.29-70.71 13.09-22.67 29.233-43.58 46.593-60.72C283.976 82.113 302.573 68.712 320 60z\"/>
    <path d=\"M280 320l100-173.2h200l100 173.2-100 173.2h-200\"/>
    <g>
        <path d=\"M706.69 299.29c-.764-11.43-6.036-56.734-16.338-71.32 0 0 9.997 14.14 11.095 76.806l5.243-5.486z\"/>
        <path d=\"M705.16 292.54c-5.615-35.752-25.082-67.015-25.082-67.015 7.35 15.128 20.257 53.835 23.64 77.45l2.33-2.24-.888-8.195z\"/>
    </g>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\">
    <path d=\"M320 60c17.466-8.733 33.76-12.78 46.593-12.484 12.856.297 22.254 4.936 26.612 12.484 4.358 7.548 3.676 18.007-2.494 29.29-6.16 11.26-17.812 23.348-34.107 34.107-16.26 10.735-37.164 20.14-60.72 26.613C272.356 156.473 246.178 160 220 160c-26.18 0-52.357-3.527-75.882-9.99-23.557-6.472-44.462-15.878-60.72-26.613-16.296-10.76-27.95-22.846-34.11-34.108-6.17-11.283-6.85-21.742-2.493-29.29 4.358-7.548 13.756-12.187 26.612-12.484C86.24 47.22 102.535 51.266 120 60c17.426 8.713 36.024 22.114 53.407 39.28C190.767 116.42 206.91 137.33 220 160c13.09 22.67 23.124 47.106 29.29 70.71 6.173 23.638 8.48 46.445 7.313 65.893-1.17 19.49-5.812 35.627-12.485 46.592C237.432 354.18 228.716 360 220 360s-17.432-5.82-24.118-16.805c-6.673-10.965-11.315-27.1-12.485-46.592-1.167-19.448 1.14-42.255 7.314-65.892 6.166-23.604 16.2-48.04 29.29-70.71 13.09-22.67 29.233-43.58 46.593-60.72C283.976 82.113 302.573 68.712 320 60zM280 320l100-173.2h200l100 173.2-100 173.2h-200\"/>
    <g>
        <path d=\"M706.69 299.29c-.764-11.43-6.036-56.734-16.338-71.32 0 0 9.997 14.14 11.095 76.806l5.243-5.486z\"/>
        <path d=\"M705.16 292.54c-5.615-35.752-25.082-67.015-25.082-67.015 7.35 15.128 20.257 53.835 23.64 77.45l2.33-2.24-.888-8.195z\"/>
    </g>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_merge_paths_05() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"499.25\" height=\"732.44\">
    <g fill=\"#ffe900\" fill-rule=\"evenodd\" stroke=\"#1b1918\">
        <g stroke-width=\"2.52\">
            <path d=\"M373.27 534.98c-8.092-54.74-4.391-98.636 56.127-90.287 77.894 55.595-9.147 98.206-5.311 151.74 21.027 45.08 17.096 66.495-7.512 68.302-17.258 10.998-32.537 13.238-46.236 8.48-.246-1.867-.69-3.845-1.368-5.94l-19.752-40.751c44.709 19.982 82.483-.171 51.564-24.28zm32.16-40.207c-5.449-9.977 3.342-14.397 8.048-3.55 12.4 31.857 6.043 40.206-16.136 72.254l-1.911-2.463c11.558-13.292 20.249-27.75 21.334-39.194.899-9.481-5.973-16.736-11.335-27.048z\"/>
            <path d=\"M407.72 580.04c40.745 49.516-3.991 92.385-40.977 82.64\"/>
        </g>
    </g>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"499.25\" height=\"732.44\">
    <g fill=\"#ffe900\" fill-rule=\"evenodd\" stroke=\"#1b1918\">
        <g stroke-width=\"2.52\">
            <path d=\"M373.27 534.98c-8.092-54.74-4.391-98.636 56.127-90.287 77.894 55.595-9.147 98.206-5.311 151.74 21.027 45.08 17.096 66.495-7.512 68.302-17.258 10.998-32.537 13.238-46.236 8.48-.246-1.867-.69-3.845-1.368-5.94l-19.752-40.751c44.709 19.982 82.483-.171 51.564-24.28zm32.16-40.207c-5.449-9.977 3.342-14.397 8.048-3.55 12.4 31.857 6.043 40.206-16.136 72.254l-1.911-2.463c11.558-13.292 20.249-27.75 21.334-39.194.899-9.481-5.973-16.736-11.335-27.048z\"/>
            <path d=\"M407.72 580.04c40.745 49.516-3.991 92.385-40.977 82.64\"/>
        </g>
    </g>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_merge_paths_06() {

    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"1221.3\" height=\"1297.3\" viewBox=\"0 0 1145 1216.2\">
    <g stroke=\"gray\" stroke-width=\"1.46\">
        <path d=\"M2236.1 787.25c6.625.191 11.52.01 11.828-2.044-8.189-9.2 8.854-46.86-11.828-48.722-17.83 3.99-6.438 26.66-11.828 48.722-.133 2.352 7.537 2.028 11.828 2.044z\" transform=\"matrix(-.02646 -1.4538 -1.2888 .02985 1465.1 3284.4)\"/>
        <path d=\"M2243.9 787.13c-7.561-19.76 6.33-43.05-7.817-50.642\" transform=\"matrix(-.02646 -1.4538 -1.2888 .02985 1465.1 3284.4)\"/>
        <path d=\"M2238.8 787.31c-4.873-19.48 2.772-37.1-2.667-50.82\" transform=\"matrix(-.02646 -1.4538 -1.2888 .02985 1465.1 3284.4)\"/>
        <path d=\"M2228.3 787.13c4.104-21.9-3.13-44.68 7.817-50.642\" transform=\"matrix(-.02646 -1.4538 -1.2888 .02985 1465.1 3284.4)\"/>
        <path d=\"M2233.4 787.31c-.692-5.383-1.098-39.17 2.667-50.82\" transform=\"matrix(-.02646 -1.4538 -1.2888 .02985 1465.1 3284.4)\"/>
    </g>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"1221.3\" height=\"1297.3\" viewBox=\"0 0 1145 1216.2\">
    <g stroke=\"gray\" stroke-width=\"1.46\">
        <path d=\"M2236.1 787.25c6.625.191 11.52.01 11.828-2.044-8.189-9.2 8.854-46.86-11.828-48.722-17.83 3.99-6.438 26.66-11.828 48.722-.133 2.352 7.537 2.028 11.828 2.044z\" transform=\"matrix(-.02646 -1.4538 -1.2888 .02985 1465.1 3284.4)\"/>
        <path d=\"M2243.9 787.13c-7.561-19.76 6.33-43.05-7.817-50.642\" transform=\"matrix(-.02646 -1.4538 -1.2888 .02985 1465.1 3284.4)\"/>
        <path d=\"M2238.8 787.31c-4.873-19.48 2.772-37.1-2.667-50.82M2228.3 787.13c4.104-21.9-3.13-44.68 7.817-50.642\" transform=\"matrix(-.02646 -1.4538 -1.2888 .02985 1465.1 3284.4)\"/>
        <path d=\"M2233.4 787.31c-.692-5.383-1.098-39.17 2.667-50.82\" transform=\"matrix(-.02646 -1.4538 -1.2888 .02985 1465.1 3284.4)\"/>
    </g>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_merge_paths_07() {
    // Merged paths lose their ends and markers are rendered incorrectly
    let input = r#"<svg width=\"100\" height=\"100\">
  <defs>
    <style>
      .a {marker-end: url(#arrowhead_end);}
    </style>
    <marker id=\"arrowhead_end\" markerWidth=\"10\" markerHeight=\"10\" refX=\"6\" refY=\"3\">
      <path d=\"M 0,0 l 6,3 l -6,3\" stroke=\"black\" />
    </marker>
  </defs>
  <path d=\"M 10,10 h50\" stroke=\"black\" marker-end=\"url(#arrowhead_end)\" />
  <path d=\"M 10,50 h50\" stroke=\"black\" marker-end=\"url(#arrowhead_end)\" />
  <path d=\"M 10,60 h60\" stroke=\"black\" class=\"a\" />
  <path d=\"M 10,70 h60\" stroke=\"black\" class=\"a\"/>
</svg>"#;

    let expected = r#"<svg width=\"100\" height=\"100\">
    <defs>
        <style>
            .a {marker-end: url(#arrowhead_end);}
        </style>
        <marker id=\"arrowhead_end\" markerWidth=\"10\" markerHeight=\"10\" refX=\"6\" refY=\"3\">
            <path d=\"M 0,0 l 6,3 l -6,3\" stroke=\"black\"/>
        </marker>
    </defs>
    <path d=\"M 10,10 h50\" stroke=\"black\" marker-end=\"url(#arrowhead_end)\"/>
    <path d=\"M 10,50 h50\" stroke=\"black\" marker-end=\"url(#arrowhead_end)\"/>
    <path d=\"M 10,60 h60\" stroke=\"black\" class=\"a\"/>
    <path d=\"M 10,70 h60\" stroke=\"black\" class=\"a\"/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_merge_paths_08() {
    // Don PROTECTED_37_ t merge paths with a linearGradient stroke (issue #1267).
    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"-5 -5 300 300\">
    <style>
        path.lg{stroke:url(#gradient);}
    </style>
    <linearGradient id=\"gradient\">
        <stop offset=\"0\" stop-color=\"#ff0000\"/>
        <stop offset=\"1\" stop-color=\"#0000ff\"/>
    </linearGradient>
    <path stroke=\"url(#gradient)\" stroke-width=\"10\" d=\"M 0 0 H 100 V 80 H 0 z\"/>
    <path stroke=\"url(#gradient)\" stroke-width=\"10\" d=\"M 200 0 H 300 V 80 H 200 z\"/>
    <path style=\"stroke:url(#gradient)\" stroke-width=\"10\" d=\"M 0 100 h 100 v 80 H 0 z\"/>
    <path style=\"stroke:url(#gradient)\" stroke-width=\"10\" d=\"M 200 100 H 300 v 80 H 200 z\"/>
    <path class=\"lg\" stroke-width=\"10\" d=\"M 0 200 h 100 v 80 H 0 z\"/>
    <path class=\"lg\" stroke-width=\"10\" d=\"M 200 200 H 300 v 80 H 200 z\"/>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"-5 -5 300 300\">
    <style>
        path.lg{stroke:url(#gradient);}
    </style>
    <linearGradient id=\"gradient\">
        <stop offset=\"0\" stop-color=\"#ff0000\"/>
        <stop offset=\"1\" stop-color=\"#0000ff\"/>
    </linearGradient>
    <path stroke=\"url(#gradient)\" stroke-width=\"10\" d=\"M 0 0 H 100 V 80 H 0 z\"/>
    <path stroke=\"url(#gradient)\" stroke-width=\"10\" d=\"M 200 0 H 300 V 80 H 200 z\"/>
    <path style=\"stroke:url(#gradient)\" stroke-width=\"10\" d=\"M 0 100 h 100 v 80 H 0 z\"/>
    <path style=\"stroke:url(#gradient)\" stroke-width=\"10\" d=\"M 200 100 H 300 v 80 H 200 z\"/>
    <path class=\"lg\" stroke-width=\"10\" d=\"M 0 200 h 100 v 80 H 0 z\"/>
    <path class=\"lg\" stroke-width=\"10\" d=\"M 200 200 H 300 v 80 H 200 z\"/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_merge_paths_10() {
    // Don PROTECTED_38_ t merge paths with a clip-path (issue #1267).
    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"-5 -5 400 400\">
    <style>
        path.lg{clip-path:url(#myClip);}
    </style>
    <clipPath id=\"myClip\" clipPathUnits=\"objectBoundingBox\">
        <circle cx=\".5\" cy=\".5\" r=\".5\"/>
    </clipPath>
    <path clip-path=\"url(#myClip)\" fill=\"red\" d=\"M 0 0 H 100 V 80 H 0 z\"/>
    <path clip-path=\"url(#myClip)\" fill=\"red\" d=\"M 200 0 H 300 V 80 H 200 z\"/>
    <path style=\"clip-path:url(#myClip)\" fill=\"red\" d=\"M 0 100 h 100 v 80 H 0 z\"/>
    <path style=\"clip-path:url(#myClip)\" fill=\"red\" d=\"M 200 100 H 300 v 80 H 200 z\"/>
    <path class=\"lg\" fill=\"red\" d=\"M 0 200 h 100 v 80 H 0 z\"/>
    <path class=\"lg\" fill=\"red\" d=\"M 200 200 H 300 v 80 H 200 z\"/>
    <path style=\"clip-path:circle(25%)\" fill=\"red\" d=\"M 0 300 h 100 v 80 H 0 z\"/>
    <path style=\"clip-path:circle(25%)\" fill=\"red\" d=\"M 200 300 H 300 v 80 H 200 z\"/>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"-5 -5 400 400\">
    <style>
        path.lg{clip-path:url(#myClip);}
    </style>
    <clipPath id=\"myClip\" clipPathUnits=\"objectBoundingBox\">
        <circle cx=\".5\" cy=\".5\" r=\".5\"/>
    </clipPath>
    <path clip-path=\"url(#myClip)\" fill=\"red\" d=\"M 0 0 H 100 V 80 H 0 z\"/>
    <path clip-path=\"url(#myClip)\" fill=\"red\" d=\"M 200 0 H 300 V 80 H 200 z\"/>
    <path style=\"clip-path:url(#myClip)\" fill=\"red\" d=\"M 0 100 h 100 v 80 H 0 z\"/>
    <path style=\"clip-path:url(#myClip)\" fill=\"red\" d=\"M 200 100 H 300 v 80 H 200 z\"/>
    <path class=\"lg\" fill=\"red\" d=\"M 0 200 h 100 v 80 H 0 z\"/>
    <path class=\"lg\" fill=\"red\" d=\"M 200 200 H 300 v 80 H 200 z\"/>
    <path style=\"clip-path:circle(25%)\" fill=\"red\" d=\"M 0 300 h 100 v 80 H 0 z\"/>
    <path style=\"clip-path:circle(25%)\" fill=\"red\" d=\"M 200 300 H 300 v 80 H 200 z\"/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_merge_paths_12() {
    // Don PROTECTED_35_ t merge paths with a mask (issue #1267).
    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"-5 -5 400 400\">
    <style>
        path.lg{mask:url(#mask);}
    </style>
    <mask id=\"mask\" maskContentUnits=\"objectBoundingBox\">
        <rect fill=\"white\" x=\"0\" y=\"0\" width=\"100%\" height=\"100%\"/>
        <circle fill=\"black\" cx=\".5\" cy=\".5\" r=\".5\"/>
    </mask>
    <path mask=\"url(#mask)\" fill=\"red\" d=\"M 0 0 H 100 V 80 H 0 z\"/>
    <path mask=\"url(#mask)\" fill=\"red\" d=\"M 200 0 H 300 V 80 H 200 z\"/>
    <path style=\"mask:url(#mask)\" fill=\"red\" d=\"M 0 100 h 100 v 80 H 0 z\"/>
    <path style=\"mask:url(#mask)\" fill=\"red\" d=\"M 200 100 H 300 v 80 H 200 z\"/>
    <path class=\"lg\" fill=\"red\" d=\"M 0 200 h 100 v 80 H 0 z\"/>
    <path class=\"lg\" fill=\"red\" d=\"M 200 200 H 300 v 80 H 200 z\"/>
    <path style=\"mask-image: linear-gradient(to left top,black, transparent)\" fill=\"red\" d=\"M 0 300 h 100 v 80 H 0 z\"/>
    <path style=\"mask-image: linear-gradient(to left top,black, transparent)\" fill=\"red\" d=\"M 200 300 H 300 v 80 H 200 z\"/>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"-5 -5 400 400\">
    <style>
        path.lg{mask:url(#mask);}
    </style>
    <mask id=\"mask\" maskContentUnits=\"objectBoundingBox\">
        <rect fill=\"white\" x=\"0\" y=\"0\" width=\"100%\" height=\"100%\"/>
        <circle fill=\"black\" cx=\".5\" cy=\".5\" r=\".5\"/>
    </mask>
    <path mask=\"url(#mask)\" fill=\"red\" d=\"M 0 0 H 100 V 80 H 0 z\"/>
    <path mask=\"url(#mask)\" fill=\"red\" d=\"M 200 0 H 300 V 80 H 200 z\"/>
    <path style=\"mask:url(#mask)\" fill=\"red\" d=\"M 0 100 h 100 v 80 H 0 z\"/>
    <path style=\"mask:url(#mask)\" fill=\"red\" d=\"M 200 100 H 300 v 80 H 200 z\"/>
    <path class=\"lg\" fill=\"red\" d=\"M 0 200 h 100 v 80 H 0 z\"/>
    <path class=\"lg\" fill=\"red\" d=\"M 200 200 H 300 v 80 H 200 z\"/>
    <path style=\"mask-image: linear-gradient(to left top,black, transparent)\" fill=\"red\" d=\"M 0 300 h 100 v 80 H 0 z\"/>
    <path style=\"mask-image: linear-gradient(to left top,black, transparent)\" fill=\"red\" d=\"M 200 300 H 300 v 80 H 200 z\"/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}
    <clipPath id=\"myClip\" clipPathUnits=\"objectBoundingBox\">
        <circle cx=\".5\" cy=\".5\" r=\".5\"/>
    </clipPath>
    <path clip-path=\"url(#myClip)\" fill=\"red\" d=\"M 0 0 H 100 V 80 H 0 z\"/>
    <path clip-path=\"url(#myClip)\" fill=\"red\" d=\"M 200 0 H 300 V 80 H 200 z\"/>
    <path style=\"clip-path:url(#myClip)\" fill=\"red\" d=\"M 0 100 h 100 v 80 H 0 z\"/>
    <path style=\"clip-path:url(#myClip)\" fill=\"red\" d=\"M 200 100 H 300 v 80 H 200 z\"/>
    <path class=\"lg\" fill=\"red\" d=\"M 0 200 h 100 v 80 H 0 z\"/>
    <path class=\"lg\" fill=\"red\" d=\"M 200 200 H 300 v 80 H 200 z\"/>
    <path style=\"clip-path:circle(25%)\" fill=\"red\" d=\"M 0 300 h 100 v 80 H 0 z\"/>
    <path style=\"clip-path:circle(25%)\" fill=\"red\" d=\"M 200 300 H 300 v 80 H 200 z\"/>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"-5 -5 400 400\">
    <style>
        path.lg{clip-path:url(#myClip);}
    </style>
    <clipPath id=\"myClip\" clipPathUnits=\"objectBoundingBox\">
        <circle cx=\".5\" cy=\".5\" r=\".5\"/>
    </clipPath>
    <path clip-path=\"url(#myClip)\" fill=\"red\" d=\"M 0 0 H 100 V 80 H 0 z\"/>
    <path clip-path=\"url(#myClip)\" fill=\"red\" d=\"M 200 0 H 300 V 80 H 200 z\"/>
    <path style=\"clip-path:url(#myClip)\" fill=\"red\" d=\"M 0 100 h 100 v 80 H 0 z\"/>
    <path style=\"clip-path:url(#myClip)\" fill=\"red\" d=\"M 200 100 H 300 v 80 H 200 z\"/>
    <path class=\"lg\" fill=\"red\" d=\"M 0 200 h 100 v 80 H 0 z\"/>
    <path class=\"lg\" fill=\"red\" d=\"M 200 200 H 300 v 80 H 200 z\"/>
    <path style=\"clip-path:circle(25%)\" fill=\"red\" d=\"M 0 300 h 100 v 80 H 0 z\"/>
    <path style=\"clip-path:circle(25%)\" fill=\"red\" d=\"M 200 300 H 300 v 80 H 200 z\"/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}

#[test]
fn test_merge_paths_12() {
    // Don't merge paths with a mask (issue #1267).
    let input = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"-5 -5 400 400\">
    <style>
        path.lg{mask:url(#mask);}
    </style>
    <mask id=\"mask\" maskContentUnits=\"objectBoundingBox\">
        <rect fill=\"white\" x=\"0\" y=\"0\" width=\"100%\" height=\"100%\"/>
        <circle fill=\"black\" cx=\".5\" cy=\".5\" r=\".5\"/>
    </mask>
    <path mask=\"url(#mask)\" fill=\"red\" d=\"M 0 0 H 100 V 80 H 0 z\"/>
    <path mask=\"url(#mask)\" fill=\"red\" d=\"M 200 0 H 300 V 80 H 200 z\"/>
    <path style=\"mask:url(#mask)\" fill=\"red\" d=\"M 0 100 h 100 v 80 H 0 z\"/>
    <path style=\"mask:url(#mask)\" fill=\"red\" d=\"M 200 100 H 300 v 80 H 200 z\"/>
    <path class=\"lg\" fill=\"red\" d=\"M 0 200 h 100 v 80 H 0 z\"/>
    <path class=\"lg\" fill=\"red\" d=\"M 200 200 H 300 v 80 H 200 z\"/>
    <path style=\"mask-image: linear-gradient(to left top,black, transparent)\" fill=\"red\" d=\"M 0 300 h 100 v 80 H 0 z\"/>
    <path style=\"mask-image: linear-gradient(to left top,black, transparent)\" fill=\"red\" d=\"M 200 300 H 300 v 80 H 200 z\"/>
</svg>"#;

    let expected = r#"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"-5 -5 400 400\">
    <style>
        path.lg{mask:url(#mask);}
    </style>
    <mask id=\"mask\" maskContentUnits=\"objectBoundingBox\">
        <rect fill=\"white\" x=\"0\" y=\"0\" width=\"100%\" height=\"100%\"/>
        <circle fill=\"black\" cx=\".5\" cy=\".5\" r=\".5\"/>
    </mask>
    <path mask=\"url(#mask)\" fill=\"red\" d=\"M 0 0 H 100 V 80 H 0 z\"/>
    <path mask=\"url(#mask)\" fill=\"red\" d=\"M 200 0 H 300 V 80 H 200 z\"/>
    <path style=\"mask:url(#mask)\" fill=\"red\" d=\"M 0 100 h 100 v 80 H 0 z\"/>
    <path style=\"mask:url(#mask)\" fill=\"red\" d=\"M 200 100 H 300 v 80 H 200 z\"/>
    <path class=\"lg\" fill=\"red\" d=\"M 0 200 h 100 v 80 H 0 z\"/>
    <path class=\"lg\" fill=\"red\" d=\"M 200 200 H 300 v 80 H 200 z\"/>
    <path style=\"mask-image: linear-gradient(to left top,black, transparent)\" fill=\"red\" d=\"M 0 300 h 100 v 80 H 0 z\"/>
    <path style=\"mask-image: linear-gradient(to left top,black, transparent)\" fill=\"red\" d=\"M 200 300 H 300 v 80 H 200 z\"/>
</svg>"#;

    let params = None;

    test_plugin(input, expected, params);
}
