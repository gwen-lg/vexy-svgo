// this_file: crates/cli/src/features_cmd.rs

//! Feature management commands for the CLI

use clap::{Args, Subcommand};
use vexy_svgo_core::features::{Feature, enable_feature, disable_feature, enabled_features};
use vexy_svgo_core::error::VexyError;

#[derive(Debug, Args)]
pub struct FeaturesCommand {
    #[command(subcommand)]
    pub command: FeaturesSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum FeaturesSubcommand {
    /// List all available features and their status
    List,
    /// Enable a feature
    Enable {
        /// Feature to enable
        feature: String,
    },
    /// Disable a feature
    Disable {
        /// Feature to disable
        feature: String,
    },
    /// Show current feature configuration
    Show,
}

impl FeaturesCommand {
    pub fn execute(&self) -> Result<(), VexyError> {
        match &self.command {
            FeaturesSubcommand::List => list_features(),
            FeaturesSubcommand::Enable { feature } => enable_feature_cmd(feature),
            FeaturesSubcommand::Disable { feature } => disable_feature_cmd(feature),
            FeaturesSubcommand::Show => show_features(),
        }
    }
}

fn list_features() -> Result<(), VexyError> {
    println!("Available features:");
    println!();
    
    let features = [
        (Feature::ParallelProcessing, "parallel", "Enable parallel processing for large files"),
        (Feature::StreamingParser, "streaming", "Enable streaming parser for memory efficiency"),
        (Feature::GeometricOptimizations, "geometric", "Enable advanced geometric optimizations (requires lyon)"),
        (Feature::SimdOptimizations, "simd", "Enable SIMD optimizations (requires target support)"),
        (Feature::ExperimentalPlugins, "experimental", "Enable experimental plugins"),
        (Feature::DebugMode, "debug", "Enable debug assertions and logging"),
        (Feature::WasmOptimizations, "wasm", "Enable WebAssembly-specific optimizations"),
        (Feature::MemoryProfiling, "memory-profiling", "Enable memory profiling"),
    ];
    
    let enabled = enabled_features();
    
    for (feature, name, desc) in features {
        let status = if enabled.contains(&feature) { "✓" } else { " " };
        println!("[{}] {:<20} {}", status, name, desc);
    }
    
    Ok(())
}

fn enable_feature_cmd(feature_name: &str) -> Result<(), VexyError> {
    let feature = parse_feature(feature_name)?;
    
    match enable_feature(feature) {
        Ok(()) => {
            println!("Feature '{}' enabled successfully", feature_name);
            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to enable feature '{}': {}", feature_name, e);
            Err(VexySvgoError::from(e))
        }
    }
}

fn disable_feature_cmd(feature_name: &str) -> Result<(), VexyError> {
    let feature = parse_feature(feature_name)?;
    disable_feature(feature);
    println!("Feature '{}' disabled", feature_name);
    Ok(())
}

fn show_features() -> Result<(), VexyError> {
    let enabled = enabled_features();
    
    if enabled.is_empty() {
        println!("No features enabled");
    } else {
        println!("Enabled features:");
        for feature in enabled {
            println!("  - {:?}", feature);
        }
    }
    
    // Show compile-time configuration
    println!("\nCompile-time configuration:");
    println!("  Target arch: {}", std::env::consts::ARCH);
    println!("  Target OS: {}", std::env::consts::OS);
    println!("  Debug assertions: {}", cfg!(debug_assertions));
    
    #[cfg(feature = "parallel")]
    println!("  Parallel support: enabled");
    #[cfg(not(feature = "parallel"))]
    println!("  Parallel support: disabled");
    
    #[cfg(feature = "lyon")]
    println!("  Lyon geometric: enabled");
    #[cfg(not(feature = "lyon"))]
    println!("  Lyon geometric: disabled");
    
    Ok(())
}

fn parse_feature(name: &str) -> Result<Feature, VexyError> {
    match name.to_lowercase().as_str() {
        "parallel" => Ok(Feature::ParallelProcessing),
        "streaming" => Ok(Feature::StreamingParser),
        "geometric" => Ok(Feature::GeometricOptimizations),
        "simd" => Ok(Feature::SimdOptimizations),
        "experimental" => Ok(Feature::ExperimentalPlugins),
        "debug" => Ok(Feature::DebugMode),
        "wasm" => Ok(Feature::WasmOptimizations),
        "memory-profiling" => Ok(Feature::MemoryProfiling),
        _ => Err(vexy_svgo_core::error::CliError::UnknownFeature(name.to_string()).into()),
    }
}