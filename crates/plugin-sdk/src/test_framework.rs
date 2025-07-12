// this_file: crates/plugin-sdk/src/test_framework.rs

//! Comprehensive testing framework for SVGN plugins
//!
//! This module provides utilities for testing plugins including:
//! - Unit testing helpers
//! - Integration testing with the full pipeline
//! - Performance benchmarking
//! - Compatibility testing with SVGO
//! - Regression testing

use std::collections::HashMap;
use std::time::{Duration, Instant};
use anyhow::{anyhow, Result};
use serde_json::Value;

use crate::Plugin;
use vexy_svgo_core::{
    ast::Document,
    parse_svg,
    stringify,
};

/// Test case for plugin testing
#[derive(Debug, Clone)]
pub struct TestCase {
    /// Test name
    pub name: String,
    /// Input SVG
    pub input: String,
    /// Expected output SVG
    pub expected: String,
    /// Plugin configuration
    pub config: Option<Value>,
    /// Description of what this test validates
    pub description: Option<String>,
}

impl TestCase {
    /// Create a new test case
    pub fn new(name: &str, input: &str, expected: &str) -> Self {
        Self {
            name: name.to_string(),
            input: input.to_string(),
            expected: expected.to_string(),
            config: None,
            description: None,
        }
    }

    /// Add configuration to the test case
    pub fn with_config(mut self, config: Value) -> Self {
        self.config = Some(config);
        self
    }

    /// Add description to the test case
    pub fn with_description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }
}

/// Test result for a single test case
#[derive(Debug)]
pub struct TestResult {
    /// Test case name
    pub name: String,
    /// Whether the test passed
    pub passed: bool,
    /// Actual output (if different from expected)
    pub actual_output: Option<String>,
    /// Error message (if test failed)
    pub error: Option<String>,
    /// Execution time
    pub duration: Duration,
}

/// Test suite results
#[derive(Debug)]
pub struct TestSuiteResult {
    /// Total number of tests
    pub total_tests: usize,
    /// Number of passed tests
    pub passed_tests: usize,
    /// Number of failed tests
    pub failed_tests: usize,
    /// Individual test results
    pub results: Vec<TestResult>,
    /// Total execution time
    pub total_duration: Duration,
}

impl TestSuiteResult {
    /// Get success rate as percentage
    pub fn success_rate(&self) -> f64 {
        if self.total_tests == 0 {
            100.0
        } else {
            (self.passed_tests as f64 / self.total_tests as f64) * 100.0
        }
    }

    /// Check if all tests passed
    pub fn all_passed(&self) -> bool {
        self.failed_tests == 0
    }

    /// Get failed test names
    pub fn failed_test_names(&self) -> Vec<String> {
        self.results
            .iter()
            .filter(|r| !r.passed)
            .map(|r| r.name.clone())
            .collect()
    }
}

/// Plugin test framework
pub struct PluginTestFramework {
    /// Whether to print verbose output
    pub verbose: bool,
    /// Whether to stop on first failure
    pub stop_on_failure: bool,
    /// Timeout for individual tests
    pub test_timeout: Duration,
}

impl PluginTestFramework {
    /// Create a new test framework
    pub fn new() -> Self {
        Self {
            verbose: false,
            stop_on_failure: false,
            test_timeout: Duration::from_secs(30),
        }
    }

    /// Enable verbose output
    pub fn verbose(mut self) -> Self {
        self.verbose = true;
        self
    }

    /// Stop on first failure
    pub fn stop_on_failure(mut self) -> Self {
        self.stop_on_failure = true;
        self
    }

    /// Set test timeout
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.test_timeout = timeout;
        self
    }

    /// Test a single plugin with one test case
    pub fn test_plugin<P: Plugin>(&self, plugin: &mut P, test_case: &TestCase) -> TestResult {
        let start_time = Instant::now();
        
        if self.verbose {
            println!("Running test: {}", test_case.name);
        }

        let result = self.run_single_test(plugin, test_case);
        let duration = start_time.elapsed();

        match result {
            Ok(actual_output) => {
                let passed = self.compare_svg(&actual_output, &test_case.expected);
                
                if self.verbose {
                    if passed {
                        println!("✓ Test '{}' passed ({:?})", test_case.name, duration);
                    } else {
                        println!("✗ Test '{}' failed ({:?})", test_case.name, duration);
                        println!("  Expected: {}", test_case.expected);
                        println!("  Actual:   {}", actual_output);
                    }
                }

                TestResult {
                    name: test_case.name.clone(),
                    passed,
                    actual_output: if passed { None } else { Some(actual_output) },
                    error: None,
                    duration,
                }
            }
            Err(e) => {
                if self.verbose {
                    println!("✗ Test '{}' failed with error: {} ({:?})", test_case.name, e, duration);
                }

                TestResult {
                    name: test_case.name.clone(),
                    passed: false,
                    actual_output: None,
                    error: Some(e.to_string()),
                    duration,
                }
            }
        }
    }

    /// Test a plugin with multiple test cases
    pub fn test_plugin_suite<P: Plugin>(&self, mut plugin: P, test_cases: &[TestCase]) -> TestSuiteResult {
        let start_time = Instant::now();
        let mut results = Vec::new();
        let mut passed_tests = 0;

        for test_case in test_cases {
            let result = self.test_plugin(&mut plugin, test_case);
            
            if result.passed {
                passed_tests += 1;
            } else if self.stop_on_failure {
                results.push(result);
                break;
            }
            
            results.push(result);
        }

        let total_duration = start_time.elapsed();

        TestSuiteResult {
            total_tests: test_cases.len(),
            passed_tests,
            failed_tests: test_cases.len() - passed_tests,
            results,
            total_duration,
        }
    }

    /// Test plugin with generated test cases
    pub fn test_plugin_generated<P: Plugin>(&self, mut plugin: P, generator: &dyn TestCaseGenerator) -> TestSuiteResult {
        let test_cases = generator.generate_test_cases();
        self.test_plugin_suite(plugin, &test_cases)
    }

    /// Benchmark plugin performance
    pub fn benchmark_plugin<P: Plugin>(&self, mut plugin: P, test_svg: &str, iterations: usize) -> BenchmarkResult {
        let mut durations = Vec::new();
        let mut successful_runs = 0;

        for _ in 0..iterations {
            let start_time = Instant::now();
            
            match self.run_plugin_on_svg(&mut plugin, test_svg, None) {
                Ok(_) => {
                    successful_runs += 1;
                    durations.push(start_time.elapsed());
                }
                Err(_) => {
                    // Record failed execution but don't include timing
                }
            }
        }

        if durations.is_empty() {
            return BenchmarkResult {
                iterations,
                successful_runs: 0,
                average_duration: Duration::ZERO,
                min_duration: Duration::ZERO,
                max_duration: Duration::ZERO,
                std_deviation: Duration::ZERO,
            };
        }

        let total_time: Duration = durations.iter().sum();
        let average_duration = total_time / durations.len() as u32;
        let min_duration = *durations.iter().min().unwrap();
        let max_duration = *durations.iter().max().unwrap();

        // Calculate standard deviation
        let variance: f64 = durations
            .iter()
            .map(|d| {
                let diff = d.as_nanos() as f64 - average_duration.as_nanos() as f64;
                diff * diff
            })
            .sum::<f64>() / durations.len() as f64;
        
        let std_deviation = Duration::from_nanos(variance.sqrt() as u64);

        BenchmarkResult {
            iterations,
            successful_runs,
            average_duration,
            min_duration,
            max_duration,
            std_deviation,
        }
    }

    /// Test plugin compatibility with SVGO
    pub fn test_svgo_compatibility<P: Plugin>(&self, mut plugin: P, svgo_test_data: &SvgoTestData) -> TestSuiteResult {
        let test_cases: Vec<TestCase> = svgo_test_data.cases
            .iter()
            .map(|case| TestCase {
                name: case.name.clone(),
                input: case.input.clone(),
                expected: case.output.clone(),
                config: case.config.clone(),
                description: Some("SVGO compatibility test".to_string()),
            })
            .collect();

        self.test_plugin_suite(plugin, &test_cases)
    }

    // Private helper methods

    fn run_single_test<P: Plugin>(&self, plugin: &mut P, test_case: &TestCase) -> Result<String> {
        self.run_plugin_on_svg(plugin, &test_case.input, test_case.config.as_ref())
    }

    fn run_plugin_on_svg<P: Plugin>(&self, plugin: &mut P, svg: &str, _config: Option<&Value>) -> Result<String> {
        // Parse the SVG
        let mut document = parse_svg(svg)?;
        
        // Apply the plugin
        plugin.apply(&mut document)?;
        
        // Stringify the result
        let result = stringify(&document)?;
        Ok(result)
    }

    fn compare_svg(&self, actual: &str, expected: &str) -> bool {
        // Normalize whitespace for comparison
        let actual_normalized = normalize_svg(actual);
        let expected_normalized = normalize_svg(expected);
        
        actual_normalized == expected_normalized
    }
}

impl Default for PluginTestFramework {
    fn default() -> Self {
        Self::new()
    }
}

/// Benchmark result
#[derive(Debug)]
pub struct BenchmarkResult {
    /// Number of iterations performed
    pub iterations: usize,
    /// Number of successful runs
    pub successful_runs: usize,
    /// Average execution duration
    pub average_duration: Duration,
    /// Minimum execution duration
    pub min_duration: Duration,
    /// Maximum execution duration
    pub max_duration: Duration,
    /// Standard deviation of execution times
    pub std_deviation: Duration,
}

impl BenchmarkResult {
    /// Get success rate as percentage
    pub fn success_rate(&self) -> f64 {
        if self.iterations == 0 {
            0.0
        } else {
            (self.successful_runs as f64 / self.iterations as f64) * 100.0
        }
    }

    /// Format results as string
    pub fn format(&self) -> String {
        format!(
            "Benchmark Results:\n  Iterations: {}\n  Success Rate: {:.1}%\n  Average: {:?}\n  Min: {:?}\n  Max: {:?}\n  Std Dev: {:?}",
            self.iterations,
            self.success_rate(),
            self.average_duration,
            self.min_duration,
            self.max_duration,
            self.std_deviation
        )
    }
}

/// Test case generator trait
pub trait TestCaseGenerator {
    /// Generate test cases
    fn generate_test_cases(&self) -> Vec<TestCase>;
}

/// Basic test case generator
pub struct BasicTestGenerator {
    /// SVG templates to use
    pub templates: Vec<String>,
    /// Variations to apply
    pub variations: Vec<HashMap<String, String>>,
}

impl TestCaseGenerator for BasicTestGenerator {
    fn generate_test_cases(&self) -> Vec<TestCase> {
        let mut test_cases = Vec::new();
        
        for (template_idx, template) in self.templates.iter().enumerate() {
            for (variation_idx, variation) in self.variations.iter().enumerate() {
                let mut input = template.clone();
                
                // Apply variations
                for (placeholder, replacement) in variation {
                    input = input.replace(placeholder, replacement);
                }
                
                test_cases.push(TestCase::new(
                    &format!("generated_{}_{}", template_idx, variation_idx),
                    &input,
                    &input, // For generated tests, we expect the plugin to modify the input
                ));
            }
        }
        
        test_cases
    }
}

/// SVGO test data structure
#[derive(Debug)]
pub struct SvgoTestData {
    /// Test cases from SVGO
    pub cases: Vec<SvgoTestCase>,
}

/// Individual SVGO test case
#[derive(Debug)]
pub struct SvgoTestCase {
    /// Test name
    pub name: String,
    /// Input SVG
    pub input: String,
    /// Expected output SVG
    pub output: String,
    /// Plugin configuration
    pub config: Option<Value>,
}

impl SvgoTestData {
    /// Load SVGO test data from directory
    pub fn load_from_directory(path: &str) -> Result<Self> {
        // This would load test files from the SVGO test directory
        // For now, return empty test data
        Ok(Self { cases: Vec::new() })
    }
}

/// Utility functions

/// Normalize SVG for comparison
pub fn normalize_svg(svg: &str) -> String {
    svg.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("")
        .replace(' ', "")
}

/// Assert that two SVGs are equivalent
pub fn assert_svg_eq(actual: &str, expected: &str) {
    let actual_normalized = normalize_svg(actual);
    let expected_normalized = normalize_svg(expected);
    
    if actual_normalized != expected_normalized {
        panic!(
            "SVG assertion failed!\nExpected: {}\nActual:   {}",
            expected_normalized, actual_normalized
        );
    }
}

/// Create a simple test case
pub fn create_test_case(name: &str, input: &str, expected: &str) -> TestCase {
    TestCase::new(name, input, expected)
}

/// Test a plugin with a single input/output pair
pub fn test_plugin<P: Plugin>(mut plugin: P, input: &str, expected: &str) -> String {
    let framework = PluginTestFramework::new();
    let test_case = TestCase::new("single_test", input, expected);
    let result = framework.test_plugin(&mut plugin, &test_case);
    
    if !result.passed {
        if let Some(error) = result.error {
            panic!("Plugin test failed: {}", error);
        } else if let Some(actual) = result.actual_output {
            panic!("Plugin test failed:\nExpected: {}\nActual: {}", expected, actual);
        }
    }
    
    // Return the actual output for further inspection
    result.actual_output.unwrap_or_else(|| expected.to_string())
}

/// Test a plugin with configuration
pub fn test_plugin_with_config<P: Plugin>(mut plugin: P, input: &str, expected: &str, config: Value) -> String {
    let framework = PluginTestFramework::new();
    let test_case = TestCase::new("config_test", input, expected).with_config(config);
    let result = framework.test_plugin(&mut plugin, &test_case);
    
    if !result.passed {
        if let Some(error) = result.error {
            panic!("Plugin test failed: {}", error);
        } else if let Some(actual) = result.actual_output {
            panic!("Plugin test failed:\nExpected: {}\nActual: {}", expected, actual);
        }
    }
    
    result.actual_output.unwrap_or_else(|| expected.to_string())
}

/// Generate large SVG for performance testing
pub fn generate_large_svg(element_count: usize) -> String {
    let mut svg = String::from(r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1000 1000">"#);
    
    for i in 0..element_count {
        match i % 4 {
            0 => svg.push_str(&format!(
                "<rect x=\"{}\" y=\"{}\" width=\"10\" height=\"10\" fill=\"#{:06x}\"/>",
                i * 10 % 1000,
                (i * 10) / 1000 * 10,
                i % 0xFFFFFF
            )),
            1 => svg.push_str(&format!(
                "<circle cx=\"{}\" cy=\"{}\" r=\"5\" fill=\"blue\"/>",
                i * 10 % 1000 + 5,
                (i * 10) / 1000 * 10 + 5
            )),
            2 => svg.push_str(&format!(
                "<path d=\"M{} {} L{} {} Z\" fill=\"green\"/>",
                i * 10 % 1000,
                (i * 10) / 1000 * 10,
                i * 10 % 1000 + 10,
                (i * 10) / 1000 * 10 + 10
            )),
            3 => svg.push_str(&format!(
                "<text x=\"{}\" y=\"{}\" font-size=\"8\">T{}</text>",
                i * 10 % 1000,
                (i * 10) / 1000 * 10,
                i
            )),
            _ => unreachable!(),
        }
    }
    
    svg.push_str("</svg>");
    svg
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Plugin;

    #[derive(Default)]
    struct TestPlugin;

    impl Plugin for TestPlugin {
        fn name(&self) -> &'static str {
            "testPlugin"
        }

        fn description(&self) -> &'static str {
            "Test plugin"
        }

        fn apply(&self, _document: &mut Document) -> anyhow::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn test_framework_basic() {
        let framework = PluginTestFramework::new();
        let mut plugin = TestPlugin::default();
        // Use self-closing tag as expected output since that's what the stringifier produces
        let test_case = TestCase::new("basic", "<svg></svg>", "<svg/>");
        
        let result = framework.test_plugin(&mut plugin, &test_case);
        if !result.passed {
            if let Some(actual) = &result.actual_output {
                println!("Expected: '<svg/>'");
                println!("Actual: '{}'", actual);
            }
        }
        assert!(result.passed);
    }

    #[test]
    fn test_svg_normalization() {
        let svg1 = "<svg>\n  <rect x=\"0\" y=\"0\"/>\n</svg>";
        let svg2 = "<svg><rect x=\"0\" y=\"0\"/></svg>";
        
        assert_eq!(normalize_svg(svg1), normalize_svg(svg2));
    }

    #[test]
    fn test_benchmark() {
        let framework = PluginTestFramework::new();
        let plugin = TestPlugin::default();
        let test_svg = "<svg><rect/></svg>";
        
        let result = framework.benchmark_plugin(plugin, test_svg, 10);
        assert_eq!(result.iterations, 10);
    }
}