// this_file: crates/core/src/parser/mod.rs

pub mod main;
pub mod attributes;
pub mod elements;
pub mod util;
pub mod tests;
pub mod config;
pub mod entities;
pub mod error;
pub mod streaming;
pub mod utils;

// Re-export main types
pub use main::{parse_svg, parse_svg_file, parse_svg_streaming, Parser};
pub use config::load_config_from_directory;
