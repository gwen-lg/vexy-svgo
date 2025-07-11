// this_file: crates/plugin-sdk/src/test_utils.rs

//! Test utilities for plugin development
//!
//! This module provides macros and utilities for parameterized testing of plugins
//! using SVGO-compatible test fixtures.

use crate::Plugin;
use anyhow::Result;
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use vexy_svgo_core::ast::Document;
use vexy_svgo_core::parse_svg;
use vexy_svgo_core::stringify;

/// Test fixture data parsed from SVGO format
#[derive(Debug, Clone)]
pub struct TestFixture {
    pub name: String,
    pub input: String,
    pub expected: String,
    pub params: Option<Value>,
}

/// Parse a test fixture file in SVGO format
pub fn parse_fixture(content: &str, name: &str) -> Result<TestFixture> {
    let parts: Vec<&str> = content.split("@@@").collect();

    if parts.len() < 2 {
        return Err(anyhow::anyhow!(
            "Invalid fixture format: missing @@@ separator"
        ));
    }

    let input = parts[0].trim().to_string();
    let expected = parts[1].trim().to_string();

    let params = if parts.len() > 2 {
        let params_str = parts[2].trim();
        if params_str.is_empty() {
            None
        } else {
            Some(serde_json::from_str(params_str)?)
        }
    } else {
        None
    };

    Ok(TestFixture {
        name: name.to_string(),
        input,
        expected,
        params,
    })
}

/// Load all test fixtures for a plugin from testdata directory
pub fn load_plugin_fixtures(plugin_name: &str) -> Result<Vec<TestFixture>> {
    let testdata_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("testdata")
        .join("plugins")
        .join(plugin_name);

    if !testdata_dir.exists() {
        return Ok(Vec::new());
    }

    let mut fixtures = Vec::new();

    for entry in fs::read_dir(&testdata_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().map_or(false, |ext| ext == "txt") {
            let content = fs::read_to_string(&path)?;
            let name = path.file_stem().unwrap().to_str().unwrap();
            let fixture = parse_fixture(&content, name)?;
            fixtures.push(fixture);
        }
    }

    // Sort by name for consistent test ordering
    fixtures.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(fixtures)
}

/// Apply a plugin to SVG content and return the result
pub fn apply_plugin_to_svg(plugin: &dyn Plugin, svg_content: &str, _params: Option<&Value>) -> Result<String> {
    let mut doc = parse_svg(svg_content)?;
    // PluginInfo is no longer needed
    
    // Apply the plugin directly
    plugin.apply(&mut doc)?;
    
    Ok(stringify(&doc)?)
}

/// Normalize SVG content for comparison (removes insignificant whitespace)
pub fn normalize_svg(svg: &str) -> String {
    // Parse and re-stringify to ensure consistent formatting
    if let Ok(doc) = parse_svg(svg) {
        if let Ok(normalized) = stringify(&doc) {
            return normalized;
        }
    }
    
    // Fallback: just trim whitespace
    svg.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

/// Compare two SVG strings with normalized whitespace
pub fn compare_svg(actual: &str, expected: &str) -> bool {
    normalize_svg(actual) == normalize_svg(expected)
}

/// Create a plugin instance with optional parameters
pub fn create_plugin_with_params<P: Plugin + Default>(params: Option<&Value>) -> Result<P> {
    // Try to validate parameters first
    let plugin = P::default();
    if let Some(params_value) = params {
        plugin.validate_params(params_value)?;
    }
    
    // For now, return default plugin since we donPROTECTED_46_t read files at compile time
        // For now, wePROTECTED_52_t read files at compile time
        // For now, we'll use the single test function approach
        #[test]
        fn test_all_fixtures() {
            let fixtures = load_plugin_fixtures($plugin_name).unwrap();

            for fixture in fixtures {
                test_single_fixture::<$plugin_type>(&fixture);
            }
        }

        fn test_single_fixture<P: Plugin + Default>(fixture: &TestFixture) {
            let mut plugin = if let Some(ref params) = fixture.params {
                create_plugin_with_params::<P>(Some(params)).unwrap_or_else(|e| {
                    panic!(
                        "Failed to create plugin with params for fixture {}: {}",
                        fixture.name, e
                    )
                })
            } else {
                P::default()
            };

            let result = apply_plugin_to_svg(&mut plugin, &fixture.input, fixture.params.as_ref()).unwrap_or_else(|e| {
                panic!("Failed to apply plugin to fixture {}: {}", fixture.name, e)
            });

            if !compare_svg(&result, &fixture.expected) {
                panic!(
                    "Fixture {} failed\nInput:\n{}\nExpected:\n{}\nActual:\n{}",
                    fixture.name, fixture.input, fixture.expected, result
                );
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_parse_fixture_simple() {
        let content = r#"<svg><g/></svg>
@@@
<svg><g/></svg>"#;

        let fixture = parse_fixture(content, "test").unwrap();
        assert_eq!(fixture.name, "test");
        assert_eq!(fixture.input, "<svg><g/></svg>");
        assert_eq!(fixture.expected, "<svg><g/></svg>");
        assert!(fixture.params.is_none());
    }

    #[test]
    fn test_parse_fixture_with_params() {
        let content = r#"<svg><g/></svg>
@@@
<svg><g/></svg>
@@@
{"test": true}"#;

        let fixture = parse_fixture(content, "test").unwrap();
        assert_eq!(fixture.name, "test");
        assert_eq!(fixture.input, "<svg><g/></svg>");
        assert_eq!(fixture.expected, "<svg><g/></svg>");
        assert_eq!(fixture.params, Some(json!({"test": true})));
    }

    #[test]
    fn test_normalize_svg() {
        let svg1 = r#"<svg>
    <g>
        <rect/>
    </g>
</svg>"#;

        let svg2 = "<svg>\n<g>\n<rect/>\n</g>\n</svg>";

        assert_eq!(normalize_svg(svg1), normalize_svg(svg2));
    }

    #[test]
    fn test_compare_svg() {
        let svg1 = "<svg><g><rect/></g></svg>";
        let svg2 = r#"<svg>
    <g>
        <rect/>
    </g>
</svg>"#;

        assert!(compare_svg(svg1, svg2));
    }
}
                        panic!("Failed to apply plugin to fixture {}: {}", fixture.name, e)
                    });

                    // Compare result with expected output
                    if !compare_svg(&result, &fixture.expected) {
                        panic!(
                            "Fixture {} failed\nInput:\n{}\nExpected:\n{}\nActual:\n{}",
                            fixture.name, fixture.input, fixture.expected, result
                        );
                    }
                }

                println!(
                    "All {} fixtures passed for plugin: {}",
                    fixture_count, $plugin_name
                );
            }
        }
    };
}

/// Macro for generating parameterized plugin tests from fixtures
#[macro_export]
macro_rules! plugin_fixture_tests {
    ($plugin_type:ty, $plugin_name:literal) => {
        #[cfg(test)]
        mod fixture_tests {
            use super::*;
            use $crate::test_utils::*;

            #[test]
            fn test_plugin_with_fixtures() {
                let fixtures = load_plugin_fixtures($plugin_name).unwrap();

                if fixtures.is_empty() {
                    println!("No fixtures found for plugin: {}", $plugin_name);
                    return;
                }

                let fixture_count = fixtures.len();
                for fixture in fixtures {
                    println!("Testing fixture: {}", fixture.name);

                    // Create plugin instance
                    let mut plugin = if let Some(ref params) = fixture.params {
                        create_plugin_with_params::<$plugin_type>(Some(params)).unwrap_or_else(
                            |e| {
                                panic!(
                                    "Failed to create plugin with params for fixture {}: {}",
                                    fixture.name, e
                                )
                            },
                        )
                    } else {
                        <$plugin_type>::default()
                    };

                    // Apply plugin to input
                    let result = apply_plugin_to_svg(&mut plugin, &fixture.input, fixture.params.as_ref()).unwrap_or_else(|e| {
                        panic!("Failed to apply plugin to fixture {}: {}", fixture.name, e)
                    });

                    // Compare result with expected output
                    if !compare_svg(&result, &fixture.expected) {
                        panic!(
                            "Fixture {} failed\nInput:\n{}\nExpected:\n{}\nActual:\n{}",
                            fixture.name, fixture.input, fixture.expected, result
                        );
                    }
                }

                println!(
                    "All {} fixtures passed for plugin: {}",
                    fixture_count, $plugin_name
                );
            }
        }
    };
}

/// Macro for generating individual test functions for each fixture
#[macro_export]
macro_rules! plugin_fixture_test_each {
    ($plugin_type:ty, $plugin_name:literal) => {
        #[cfg(test)]
        mod fixture_tests {
            use super::*;
            use $crate::test_utils::*;

            // Generate test functions at compile time
            $crate::generate_fixture_test_functions!($plugin_type, $plugin_name);
        }
    };
}

/// Internal macro for generating individual test functions
#[macro_export]
macro_rules! generate_fixture_test_functions {
    ($plugin_type:ty, $plugin_name:literal) => {
        // This would ideally be generated at compile time, but Rust macros can't read files at compile time
        // For now, we'll use the single test function approach
        #[test]
        fn test_all_fixtures() {
            let fixtures = load_plugin_fixtures($plugin_name).unwrap();

            for fixture in fixtures {
                test_single_fixture::<$plugin_type>(&fixture);
            }
        }

        fn test_single_fixture<P: Plugin + Default>(fixture: &TestFixture) {
            let mut plugin = if let Some(ref params) = fixture.params {
                create_plugin_with_params::<P>(Some(params)).unwrap_or_else(|e| {
                    panic!(
                        "Failed to create plugin with params for fixture {}: {}",
                        fixture.name, e
                    )
                })
            } else {
                P::default()
            };

            let result = apply_plugin_to_svg(&mut plugin, &fixture.input, fixture.params.as_ref()).unwrap_or_else(|e| {
                panic!("Failed to apply plugin to fixture {}: {}", fixture.name, e)
            });

            if !compare_svg(&result, &fixture.expected) {
                panic!(
                    "Fixture {} failed\nInput:\n{}\nExpected:\n{}\nActual:\n{}",
                    fixture.name, fixture.input, fixture.expected, result
                );
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_parse_fixture_simple() {
        let content = r#"<svg><g/></svg>
@@@
<svg><g/></svg>"#;

        let fixture = parse_fixture(content, "test").unwrap();
        assert_eq!(fixture.name, "test");
        assert_eq!(fixture.input, "<svg><g/></svg>");
        assert_eq!(fixture.expected, "<svg><g/></svg>");
        assert!(fixture.params.is_none());
    }

    #[test]
    fn test_parse_fixture_with_params() {
        let content = r#"<svg><g/></svg>
@@@
<svg><g/></svg>
@@@
{"test": true}"#;

        let fixture = parse_fixture(content, "test").unwrap();
        assert_eq!(fixture.name, "test");
        assert_eq!(fixture.input, "<svg><g/></svg>");
        assert_eq!(fixture.expected, "<svg><g/></svg>");
        assert_eq!(fixture.params, Some(json!({"test": true})));
    }

    #[test]
    fn test_normalize_svg() {
        let svg1 = r#"<svg>
    <g>
        <rect/>
    </g>
</svg>"#;

        let svg2 = "<svg>\n<g>\n<rect/>\n</g>\n</svg>";

        assert_eq!(normalize_svg(svg1), normalize_svg(svg2));
    }

    #[test]
    fn test_compare_svg() {
        let svg1 = "<svg><g><rect/></g></svg>";
        let svg2 = r#"<svg>
    <g>
        <rect/>
    </g>
</svg>"#;

        assert!(compare_svg(svg1, svg2));
    }
}
