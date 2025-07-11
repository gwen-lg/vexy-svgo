// this_file: test/plugins/apply_transforms.rs
use vexy_svgo::optimize;

#[test]
fn test_apply_transforms_translate() {
    let input = r#"<svg xmlns='http://www.w3.org/2000/svg'><path transform='translate(10, 20)' d='M0,0 L10,10'/></svg>"#;
    let config = serde_json::json!({
        "plugins": [
            {"name": "applyTransforms"}
        ]
    });
    let result = optimize(input, Some(&config.to_string())).unwrap().data;
    // Transform should be applied and removed
    assert!(!result.contains("transform="), "Transform attribute should be removed");
    assert!(result.contains("M10 20L20 30"), "Coordinates should be transformed");
}

#[test]
fn test_apply_transforms_scale() {
    let input = r#"<svg xmlns='http://www.w3.org/2000/svg'><path transform='scale(2)' d='M5,5 L10,10'/></svg>"#;
    let config = serde_json::json!({
        "plugins": [
            {"name": "applyTransforms"}
        ]
    });
    let result = optimize(input, Some(&config.to_string())).unwrap().data;
    // Transform should be applied and removed
    assert!(!result.contains("transform="), "Transform attribute should be removed");
    assert!(result.contains("M10 10L20 20"), "Coordinates should be scaled");
}

#[test]
fn test_apply_transforms_rotate() {
    let input = r#"<svg xmlns='http://www.w3.org/2000/svg'><path transform='rotate(90)' d='M10,0 L10,10'/></svg>"#;
    let config = serde_json::json!({
        "plugins": [
            {"name": "applyTransforms"}
        ]
    });
    let result = optimize(input, Some(&config.to_string())).unwrap().data;
    // Transform should be applied and removed
    assert!(!result.contains("transform="), "Transform attribute should be removed");
    // After 90 degree rotation, (10,0) becomes (0,10) and (10,10) becomes (-10,10)
    assert!(result.contains("M0 10") || result.contains("M0.0 10.0"), "First point should be rotated");
}

#[test]
fn test_apply_transforms_stroked_path_config() {
    let input = r#"<svg xmlns='http://www.w3.org/2000/svg'><path transform='scale(2)' stroke='black' d='M5,5 L10,10'/></svg>"#;
    let config = serde_json::json!({
        "plugins": [
            {
                "name": "applyTransforms",
                "params": {
                    "applyToStroked": false
                }
            }
        ]
    });
    let result = optimize(input, Some(&config.to_string())).unwrap().data;
    // Transform should NOT be applied to stroked paths when applyToStroked is false
    assert!(result.contains("transform="), "Transform attribute should be preserved for stroked paths");
}

#[test]
fn test_apply_transforms_precision() {
    let input = r#"<svg xmlns='http://www.w3.org/2000/svg'><path transform='translate(0.123456, 0.987654)' d='M0,0 L10,10'/></svg>"#;
    let config = serde_json::json!({
        "plugins": [
            {
                "name": "applyTransforms",
                "params": {
                    "floatPrecision": 2
                }
            }
        ]
    });
    let result = optimize(input, Some(&config.to_string())).unwrap().data;
    // Should round to 2 decimal places
    assert!(result.contains("M0.12 0.99"), "Coordinates should be rounded to specified precision");
}