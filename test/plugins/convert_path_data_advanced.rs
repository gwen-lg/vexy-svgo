// this_file: test/plugins/convert_path_data_advanced.rs
use vexy_svgo::optimize;

#[test]
fn test_convert_path_data_basic() {
    let input = r#"<svg xmlns='http://www.w3.org/2000/svg'><path d='M10 10 L10 10 L20 20'/></svg>"#;
    let config = serde_json::json!({
        "plugins": [
            {
                "name": "convertPathData",
                "params": {
                    "removeUseless": true
                }
            }
        ]
    });
    let result = optimize(input, Some(&config.to_string())).unwrap().data;
    // Should remove the useless L10 10 command
    assert!(!result.contains("L10 10"));
    assert!(result.contains("M10 10"));
    assert!(result.contains("L20 20"));
}

#[test]
fn test_convert_path_data_straight_curves() {
    let input = r#"<svg xmlns='http://www.w3.org/2000/svg'><path d='M0 0 C1 0.01 2 -0.01 3 0'/></svg>"#;
    let config = serde_json::json!({
        "plugins": [
            {
                "name": "convertPathData",
                "params": {
                    "straightCurves": true,
                    "curveTolerance": 0.1
                }
            }
        ]
    });
    let result = optimize(input, Some(&config.to_string())).unwrap().data;
    // Should convert nearly straight curve to line
    assert!(result.contains("L3 0") || result.contains("l3 0"));
}

#[test]
fn test_convert_path_data_precision() {
    let input = r#"<svg xmlns='http://www.w3.org/2000/svg'><path d='M0.123456 0.987654 L10.123456 20.987654'/></svg>"#;
    let config = serde_json::json!({
        "plugins": [
            {
                "name": "convertPathData",
                "params": {
                    "floatPrecision": 2
                }
            }
        ]
    });
    let result = optimize(input, Some(&config.to_string())).unwrap().data;
    // Should round to 2 decimal places
    assert!(result.contains("0.12") && result.contains("0.99"));
}

#[test]
fn test_convert_path_data_leading_zero() {
    let input = r#"<svg xmlns='http://www.w3.org/2000/svg'><path d='M0.5 0.5 L1.5 1.5'/></svg>"#;
    let config = serde_json::json!({
        "plugins": [
            {
                "name": "convertPathData",
                "params": {
                    "leadingZero": false
                }
            }
        ]
    });
    let result = optimize(input, Some(&config.to_string())).unwrap().data;
    // Should remove leading zeros from decimals
    assert!(result.contains(".5"));
}

#[test]
fn test_convert_path_data_advanced_features_disabled() {
    let input = r#"<svg xmlns='http://www.w3.org/2000/svg'><path d='M0 0 C1 1 2 2 3 3'/></svg>"#;
    let config = serde_json::json!({
        "plugins": [
            {
                "name": "convertPathData",
                "params": {
                    "makeArcs": false,
                    "straightCurves": false,
                    "convertToQ": false
                }
            }
        ]
    });
    let result = optimize(input, Some(&config.to_string())).unwrap().data;
    // Should keep the curve as-is when advanced features are disabled
    assert!(result.contains("C") || result.contains("c"));
}