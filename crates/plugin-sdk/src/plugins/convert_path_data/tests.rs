// this_file: crates/plugin-sdk/src/plugins/convert_path_data/tests.rs

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_plugin_info() {
        let plugin = ConvertPathDataPlugin::new();
        assert_eq!(plugin.name(), "convertPathData");
        assert_eq!(
            plugin.description(),
            "converts path data to relative or absolute, optimizes segments, simplifies curves"
        );
    }

    #[test]
    fn test_param_validation() {
        let plugin = ConvertPathDataPlugin::new();

        // Test null params
        assert!(plugin.validate_params(&Value::Null).is_ok());

        // Test valid params
        assert!(plugin
            .validate_params(&json!({
                "floatPrecision": 2,
                "removeUseless": false
            }))
            .is_ok());

        // Test invalid params
        assert!(plugin
            .validate_params(&json!({
                "invalidParam": true
            }))
            .is_err());
    }

    #[test]
    fn test_parse_simple_path() {
        let path = "M10 20 L30 40";
        let commands = parse_path_data(path).unwrap();
        assert_eq!(commands.len(), 2);
        assert_eq!(commands[0].cmd_type, CommandType::MoveTo);
        assert_eq!(commands[0].params, vec![10.0, 20.0]);
        assert_eq!(commands[1].cmd_type, CommandType::LineTo);
        assert_eq!(commands[1].params, vec![30.0, 40.0]);
    }

    #[test]
    fn test_parse_relative_path() {
        let path = "m10 20 l30 40";
        let commands = parse_path_data(path).unwrap();
        assert_eq!(commands.len(), 2);
        assert_eq!(commands[0].cmd_type, CommandType::MoveTo);
        assert!(!commands[0].is_absolute);
        assert_eq!(commands[1].cmd_type, CommandType::LineTo);
        assert!(!commands[1].is_absolute);
    }

    #[test]
    fn test_format_number() {
        assert_eq!(format_number(1.0, 3, true), "1");
        assert_eq!(format_number(1.234567, 3, true), "1.235");
        assert_eq!(format_number(0.5, 1, false), ".5");
        assert_eq!(format_number(-0.5, 1, false), "-.5");
    }

    #[test]
    fn test_optimize_removes_useless_lineto() {
        let path = "M10 10 L10 10 L20 20";
        let config = ConvertPathDataConfig {
            float_precision: 3,
            transform_precision: 5,
            remove_useless: true,
            collapse_repeated: true,
            utilize_absolute: true,
            leading_zero: true,
            negative_extra_space: true,
            make_arcs: false,
            straight_curves: false,
            convert_to_q: false,
            curve_tolerance: 0.1,
            arc_tolerance: 0.5,
        };
        let optimized = optimize_path_data(path, &config).unwrap();
        // Should remove the L10 10 as it's the same as current position
        assert!(!optimized.contains("L10 10"));
    }

    #[test]
    fn test_curve_straightening() {
        // Create a nearly straight cubic curve
        let curve = CubicBezierSegment {
            from: Point::new(0.0, 0.0),
            ctrl1: Point::new(1.0, 0.01), // Very slight deviation
            ctrl2: Point::new(2.0, -0.01),
            to: Point::new(3.0, 0.0),
        };
        
        assert!(is_curve_nearly_straight(&curve, 0.1));
        assert!(!is_curve_nearly_straight(&curve, 0.001));
    }

    #[test]
    fn test_quadratic_curve_straightening() {
        // Create a nearly straight quadratic curve
        let curve = QuadraticBezierSegment {
            from: Point::new(0.0, 0.0),
            ctrl: Point::new(1.5, 0.01), // Very slight deviation
            to: Point::new(3.0, 0.0),
        };
        
        assert!(is_quadratic_curve_nearly_straight(&curve, 0.1));
        assert!(!is_quadratic_curve_nearly_straight(&curve, 0.001));
    }

    #[test]
    fn test_circle_fitting() {
        // Create points on a circle
        let center = Point::new(10.0, 10.0);
        let radius = 5.0;
        let points = vec![
            Point::new(center.x + radius, center.y),
            Point::new(center.x, center.y + radius),
            Point::new(center.x - radius, center.y),
            Point::new(center.x, center.y - radius),
        ];
        
        if let Some((fitted_center, fitted_radius)) = fit_circle_to_points(&points, 0.1) {
            assert!((fitted_center.x - center.x).abs() < 0.1);
            assert!((fitted_center.y - center.y).abs() < 0.1);
            assert!((fitted_radius - radius).abs() < 0.1);
        } else {
            panic!("Should be able to fit circle to points on circle");
        }
    }

    #[test]
    fn test_advanced_config_validation() {
        let plugin = ConvertPathDataPlugin::new();

        // Test advanced features config
        assert!(plugin
            .validate_params(&json!({
                "floatPrecision": 2,
                "makeArcs": true,
                "straightCurves": true,
                "convertToQ": true,
                "curveTolerance": 0.05,
                "arcTolerance": 0.3
            }))
            .is_ok());
    }
}
