// this_file: crates/core/src/error.rs

//! Centralized error types for the Vexy SVGO core library
//!
//! This module provides typed error enums for better error handling
//! throughout the codebase.

use thiserror::Error;

/// Main error type for the Vexy SVGO core library
#[derive(Debug, Error)]
pub enum VexySvgoError {
    /// Parse errors
    #[error("Parse error: {0}")]
    Parse(#[from] crate::parser::error::ParseError),
    
    /// Stringify errors
    #[error("Stringify error: {0}")]
    Stringify(#[from] crate::stringifier::StringifyError),
    
    /// Plugin errors
    #[error("Plugin error: {0}")]
    Plugin(#[from] PluginError),
    
    /// Configuration errors
    #[error("Configuration error: {0}")]
    Config(#[from] ConfigError),
    
    /// CLI errors
    #[error("CLI error: {0}")]
    Cli(#[from] CliError),
    
    /// IO errors
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    /// Regex errors
    #[error("Regex error: {0}")]
    Regex(String),
    
    /// General errors
    #[error("{0}")]
    General(String),
}

/// Plugin-specific errors
#[derive(Debug, Error)]
pub enum PluginError {
    /// Plugin not found
    #[error("Plugin not found: {0}")]
    NotFound(String),
    
    /// Invalid plugin parameters
    #[error("Invalid plugin parameters for {plugin}: {message}")]
    InvalidParams { plugin: String, message: String },
    
    /// Plugin execution failed
    #[error("Plugin {plugin} failed: {message}")]
    ExecutionFailed { plugin: String, message: String },
    
    /// Plugin validation failed
    #[error("Plugin validation failed: {0}")]
    ValidationFailed(String),
}

/// Configuration-specific errors
#[derive(Debug, Error)]
pub enum ConfigError {
    /// Invalid configuration format
    #[error("Invalid configuration format: {0}")]
    InvalidFormat(String),
    
    /// Missing required field
    #[error("Missing required configuration field: {0}")]
    MissingField(String),
    
    /// Invalid value
    #[error("Invalid value for {field}: {message}")]
    InvalidValue { field: String, message: String },
    
    /// File not found
    #[error("Configuration file not found: {0}")]
    FileNotFound(String),
}

/// CLI-specific errors
#[derive(Debug, Error)]
pub enum CliError {
    /// Invalid directory path
    #[error("'{path}' is not a directory")]
    InvalidDirectory { path: String },
    
    /// No files found
    #[error("No SVG files found in '{directory}'")]
    NoFilesFound { directory: String },
    
    /// Invalid file path
    #[error("Invalid file path: {0}")]
    InvalidFilePath(String),
    
    /// Feature not found
    #[error("Unknown feature: {0}")]
    UnknownFeature(String),
}

/// Result type alias for Vexy SVGO operations
pub type Result<T> = std::result::Result<T, VexySvgoError>;

impl From<String> for VexySvgoError {
    fn from(s: String) -> Self {
        VexySvgoError::General(s)
    }
}

impl From<&str> for VexySvgoError {
    fn from(s: &str) -> Self {
        VexySvgoError::General(s.to_string())
    }
}

impl From<anyhow::Error> for VexySvgoError {
    fn from(err: anyhow::Error) -> Self {
        VexySvgoError::General(err.to_string())
    }
}

impl From<regex::Error> for VexySvgoError {
    fn from(err: regex::Error) -> Self {
        VexySvgoError::Regex(err.to_string())
    }
}