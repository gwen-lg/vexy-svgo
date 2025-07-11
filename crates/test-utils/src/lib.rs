// this_file: crates/test-utils/src/lib.rs

//! # vexy_svgo-test-utils
//!
//! Shared testing utilities for vexy_svgo.

use std::fs;
use std::path::Path;
use tempfile::TempDir;

/// Test fixture structure
#[derive(Debug)]
pub struct TestFixture {
    pub name: String,
    pub input: String,
    pub expected: String,
    pub params: Option<serde_json::Value>,
}

/// Load test fixtures from a directory
pub fn load_fixtures(dir: &Path) -> Result<Vec<TestFixture>, Box<dyn std::error::Error>> {
    let mut fixtures = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("txt") {
            let content = fs::read_to_string(&path)?;
            if let Some(fixture) = parse_fixture(&content) {
                fixtures.push(fixture);
            }
        }
    }

    Ok(fixtures)
}

/// Parse a fixture file in SVGO format
fn parse_fixture(content: &str) -> Option<TestFixture> {
    let parts: Vec<&str> = content.split("@@@").collect();
    if parts.len() < 2 {
        return None;
    }

    let input = parts[0].trim().to_string();
    let expected = parts[1].trim().to_string();
    let params = parts
        .get(2)
        .and_then(|p| serde_json::from_str(p.trim()).ok());

    Some(TestFixture {
        name: String::new(), // Will be set from filename
        input,
        expected,
        params,
    })
}

/// Create a temporary directory with test files
pub fn create_test_dir() -> Result<TempDir, Box<dyn std::error::Error>> {
    Ok(TempDir::new()?)
}
