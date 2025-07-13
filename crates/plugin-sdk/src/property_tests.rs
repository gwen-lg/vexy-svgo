// this_file: crates/plugin-sdk/src/property_tests.rs

//! Property-based testing utilities for SVGN plugins
//!
//! This module provides property-based testing infrastructure using proptest
//! to generate random valid SVGs and test optimization invariants.

use vexy_svgo_core::Plugin;
use anyhow::Result;
use proptest::prelude::*;
use proptest::strategy::ValueTree;
use vexy_svgo_core::{parse_svg, stringify};

/// Strategy for generating valid SVG tag names
fn svg_tag_names() -> impl Strategy<Value = &'static str> {
    prop_oneof![
        Just("svg"),
        Just("g"),
        Just("rect"),
        Just("circle"),
        Just("ellipse"),
        Just("line"),
        Just("path"),
        Just("polygon"),
        Just("polyline"),
        Just("text"),
        Just("defs"),
        Just("use"),
        Just("image"),
        Just("clipPath"),
        Just("mask"),
        Just("pattern"),
        Just("marker"),
        Just("linearGradient"),
        Just("radialGradient"),
        Just("stop"),
        Just("filter"),
        Just("feGaussianBlur"),
        Just("feColorMatrix"),
        Just("feOffset"),
        Just("feMorphology"),
        Just("feFlood"),
        Just("feComposite"),
        Just("feMerge"),
        Just("feMergeNode"),
        Just("feImage"),
        Just("feTurbulence"),
        Just("feDisplacementMap"),
        Just("feConvolveMatrix"),
        Just("feDiffuseLighting"),
        Just("feSpecularLighting"),
        Just("feDistantLight"),
        Just("fePointLight"),
        Just("feSpotLight"),
        Just("style"),
        Just("title"),
        Just("desc"),
        Just("metadata"),
    ]
}

/// Strategy for generating valid attribute names
fn attr_names() -> impl Strategy<Value = &'static str> {
    prop_oneof![
        Just("id"),
        Just("class"),
        Just("style"),
        Just("fill"),
        Just("stroke"),
        Just("stroke-width"),
        Just("stroke-dasharray"),
        Just("stroke-linecap"),
        Just("stroke-linejoin"),
        Just("opacity"),
        Just("fill-opacity"),
        Just("stroke-opacity"),
        Just("transform"),
        Just("x"),
        Just("y"),
        Just("width"),
        Just("height"),
        Just("cx"),
        Just("cy"),
        Just("r"),
        Just("rx"),
        Just("ry"),
        Just("x1"),
        Just("y1"),
        Just("x2"),
        Just("y2"),
        Just("d"),
        Just("points"),
        Just("href"),
        Just("xlink:href"),
        Just("viewBox"),
        Just("preserveAspectRatio"),
        Just("xmlns"),
        Just("version"),
        Just("baseProfile"),
    ]
}

/// Strategy for generating simple attribute values
fn attr_values() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("none".to_string()),
        Just("currentColor".to_string()),
        Just("#ff0000".to_string()),
        Just("rgb(255, 0, 0)".to_string()),
        Just("100".to_string()),
        Just("100px".to_string()),
        Just("50%".to_string()),
        Just("translate(10, 20)".to_string()),
        Just("scale(1.5)".to_string()),
        Just("rotate(45)".to_string()),
    ]
}

/// Strategy for generating simple SVG elements
fn simple_element() -> impl Strategy<Value = String> {
    (svg_tag_names(), prop::collection::vec((attr_names(), attr_values()), 0..5))
        .prop_map(|(tag, attrs)| {
            if attrs.is_empty() {
                format!("<{}/>", tag)
            } else {
                let attr_str = attrs
                    .iter()
                    .map(|(name, value)| format!("{}=\"{}\"", name, value))
                    .collect::<Vec<_>>()
                    .join(" ");
                format!("<{} {}/>", tag, attr_str)
            }
        })
}

/// Strategy for generating simple SVG documents
pub fn simple_svg_document() -> impl Strategy<Value = String> {
    prop::collection::vec(simple_element(), 1..10).prop_map(|elements| {
        format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\">{}</svg>",
            elements.join("")
        )
    })
}

/// Property test: Plugin should not crash on valid SVG input
pub fn prop_plugin_no_crash<P: Plugin + Clone>(plugin: P) -> impl Strategy<Value = ()> {
    simple_svg_document().prop_map(move |svg| {
        let mut plugin_clone = plugin.clone();
        if let Ok(mut doc) = parse_svg(&svg) {
            // Plugin should not panic
            let _ = plugin_clone.apply(&mut doc);
        }
    })
}

/// Property test: Plugin output should be valid SVG
pub fn prop_plugin_output_valid<P: Plugin + Clone>(plugin: P) -> impl Strategy<Value = ()> {
    simple_svg_document().prop_map(move |svg| {
        let mut plugin_clone = plugin.clone();
        if let Ok(mut doc) = parse_svg(&svg) {
            if plugin_clone.apply(&mut doc).is_ok() {
                // Output should be valid SVG (parseable)
                if let Ok(output) = stringify(&doc) {
                    assert!(parse_svg(&output).is_ok(), "Plugin output is not valid SVG");
                }
            }
        }
    })
}

/// Property test: Plugin should be idempotent (applying twice gives same result)
pub fn prop_plugin_idempotent<P: Plugin + Clone>(plugin: P) -> impl Strategy<Value = ()> {
    simple_svg_document().prop_map(move |svg| {
        let mut plugin1 = plugin.clone();
        let mut plugin2 = plugin.clone();
        
        if let Ok(mut doc1) = parse_svg(&svg) {
            if plugin1.apply(&mut doc1).is_ok() {
                if let Ok(output1) = stringify(&doc1) {
                    if let Ok(mut doc2) = parse_svg(&output1) {
                        if plugin2.apply(&mut doc2).is_ok() {
                            if let Ok(output2) = stringify(&doc2) {
                                assert_eq!(output1, output2, "Plugin is not idempotent");
                            }
                        }
                    }
                }
            }
        }
    })
}

/// Test invariants for optimization plugins
pub fn test_optimization_invariants<P: Plugin + Clone>(plugin: P, test_cases: u32) -> Result<()> {
    use proptest::test_runner::TestRunner;
    
    let mut runner = TestRunner::default();
    
    // Test 1: Plugin should not crash
    runner.run(&prop_plugin_no_crash(plugin.clone()), |_| Ok(()))?;
    
    // Test 2: Output should be valid SVG
    runner.run(&prop_plugin_output_valid(plugin.clone()), |_| Ok(()))?;
    
    // Test 3: Plugin should be idempotent
    runner.run(&prop_plugin_idempotent(plugin.clone()), |_| Ok(()))?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plugins::RemoveCommentsPlugin;

    #[test]
    fn test_simple_svg_generation() {
        let strategy = simple_svg_document();
        let mut runner = proptest::test_runner::TestRunner::default();
        
        for _ in 0..10 {
            let svg = strategy.new_tree(&mut runner).unwrap().current();
            assert!(svg.starts_with("<svg"));
            assert!(svg.ends_with("</svg>"));
            
            // Should be parseable
            assert!(parse_svg(&svg).is_ok());
        }
    }

    #[test]
    fn test_remove_comments_invariants() {
        let plugin = RemoveCommentsPlugin::new();
        test_optimization_invariants(plugin, 100).unwrap();
    }
}