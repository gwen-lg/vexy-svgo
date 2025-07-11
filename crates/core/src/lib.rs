// this_file: crates/core/src/lib.rs

//! # vexy_svgo-core
//!
//! Core SVG processing engine for vexy_svgo, providing parsing, AST manipulation,
//! optimization, and stringification capabilities.

pub mod ast;
pub mod collections;
pub mod features;
pub mod error;

pub mod optimizer;
pub mod parser;
pub mod plugin_registry;
pub mod stringifier;
pub mod utils;
pub mod visitor;

/// Version string for vexy_svgo
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

// Re-export main types
pub use ast::{Document, DocumentMetadata, Element, Node};

pub use optimizer::{
    optimize, optimize_default, optimize_with_config, OptimizationInfo, OptimizationResult,
    OptimizeOptions, parallel,
};
pub use parser::{parse_svg, parse_svg_file, parse_svg_streaming, Parser, load_config_from_directory};
pub use parser::error::ParseError;
pub use error::VexySvgoError;
pub use parser::config::{Config, StreamingConfig, PluginConfig, DataUriFormat, LineEnding};

// Re-export config module
pub use parser::config;
pub use plugin_registry::{create_default_registry, Plugin, PluginRegistry};
pub use stringifier::{stringify, stringify_with_config, StringifyConfig, StringifyError, StreamingStringifier};
