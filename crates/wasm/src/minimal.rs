// this_file: crates/wasm/src/minimal.rs

//! Minimal WASM API for the smallest possible bundle size
//!
//! This module provides a bare-bones API that trades features for size.

use wasm_bindgen::prelude::*;
use vexy_svgo_core::{Config, PluginConfig, optimize_with_config};

#[cfg(feature = "size-optimization")]
#[wasm_bindgen]
pub struct MinimalOptimizer {
    // Store config to avoid repeated parsing
    multipass: bool,
}

#[cfg(feature = "size-optimization")]
#[wasm_bindgen]
impl MinimalOptimizer {
    /// Create a new optimizer with minimal config
    #[wasm_bindgen(constructor)]
    pub fn new(multipass: bool) -> Self {
        Self { multipass }
    }

    /// Optimize SVG with minimal overhead
    /// Returns the optimized SVG or the original on error
    #[wasm_bindgen]
    pub fn optimize(&self, svg: &str) -> String {
        use vexy_svgo_core::{Config, optimize_with_config};
        
        let mut config = Config::default();
        config.multipass = self.multipass;
        
        // Use the most common plugins only
        config.plugins = vec![
            PluginConfig::Name("removeComments".to_string()),
            PluginConfig::Name("removeEmptyAttrs".to_string()),
            PluginConfig::Name("removeEmptyContainers".to_string()),
            PluginConfig::Name("collapseGroups".to_string()),
            PluginConfig::Name("convertColors".to_string()),
            PluginConfig::Name("removeUselessDefs".to_string()),
        ];
        
        match optimize_with_config(svg, config) {
            Ok(result) => result.data,
            Err(_) => svg.to_string(), // Return original on error
        }
    }
    
    /// Get the compression ratio for the last optimization
    #[wasm_bindgen]
    pub fn compress_ratio(&self, original: &str, optimized: &str) -> f32 {
        if original.is_empty() {
            1.0
        } else {
            optimized.len() as f32 / original.len() as f32
        }
    }
}

/// Ultra-minimal optimize function - just the essentials
#[cfg(feature = "size-optimization")]
#[wasm_bindgen]
pub fn optimize_minimal(svg: &str) -> String {
    use vexy_svgo_core::{Config, PluginConfig, optimize_with_config};
    
    // Use only the most impactful plugins
    let mut config = Config::default();
    config.plugins = vec![
        PluginConfig::Name("removeComments".to_string()),
        PluginConfig::Name("collapseGroups".to_string()),
    ];
    
    match optimize_with_config(svg, config) {
        Ok(result) => result.data,
        Err(_) => svg.to_string(),
    }
}