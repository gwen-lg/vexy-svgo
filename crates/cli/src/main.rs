// this_file: crates/cli/src/main.rs

//! Vexy SVGO command-line interface
//!
//! This is the CLI binary for Vexy SVGO, providing SVGO-compatible command-line
//! options for SVG optimization.

use clap::{Arg, ArgAction, ArgMatches, Command};
use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use vexy_svgo_core::{optimize_with_config, Config, VERSION, error::VexySvgoError};

#[derive(Debug)]
enum InputMode {
    Stdin,
    Files(Vec<String>),
    String(String),
    Folder(String),
}

#[derive(Debug)]
enum OutputMode {
    Stdout,
    File(String),
    Directory(String),
    InPlace,
}

fn main() {
    let matches = Command::new("vexy_svgo")
        .version(VERSION)
        .about("A high-performance Rust port of SVGO (SVG Optimizer)")
        // Allow positional arguments for input files
        .arg(
            Arg::new("INPUT")
                .help("Input files, \"-\" for STDIN")
                .value_name("INPUT")
                .num_args(1..)
                .conflicts_with_all(["input", "string", "folder"]),
        )
        .arg(
            Arg::new("input")
                .help("Input files, \"-\" for STDIN")
                .short('i')
                .long("input")
                .value_name("INPUT")
                .num_args(1..)
                .conflicts_with_all(["INPUT", "string", "folder"]),
        )
        .arg(
            Arg::new("string")
                .help("Input SVG data string")
                .short('s')
                .long("string")
                .value_name("STRING")
                .conflicts_with_all(["INPUT", "input", "folder"]),
        )
        .arg(
            Arg::new("folder")
                .help("Input folder, optimize and rewrite all *.svg files")
                .short('f')
                .long("folder")
                .value_name("FOLDER")
                .conflicts_with_all(["INPUT", "input", "string"]),
        )
        .arg(
            Arg::new("output")
                .help("Output file or folder (by default the same as the input), \"-\" for STDOUT")
                .short('o')
                .long("output")
                .value_name("OUTPUT")
                .num_args(1..),
        )
        .arg(
            Arg::new("recursive")
                .help("Use with '–folder'. Recursively traverse folders")
                .short('r')
                .long("recursive")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("exclude")
                .help("Use with '–folder'. Exclude files matching regex pattern")
                .long("exclude")
                .value_name("PATTERN")
                .action(ArgAction::Append),
        )
        .arg(
            Arg::new("config")
                .help("Config file or JSON string to customize optimization")
                .long("config")
                .value_name("CONFIG"),
        )
        .arg(
            Arg::new("disable")
                .help("Disable plugin by name, separator: ','")
                .long("disable")
                .value_name("PLUGIN")
                .num_args(1..)
                .value_delimiter(','),
        )
        .arg(
            Arg::new("enable")
                .help("Enable plugin by name, separator: ','")
                .long("enable")
                .value_name("PLUGIN")
                .num_args(1..)
                .value_delimiter(','),
        )
        .arg(
            Arg::new("datauri")
                .help("Output as Data URI (base64 or URI encoded)")
                .long("datauri")
                .value_name("TYPE")
                .value_parser(["base64", "enc", "unenc"])
                .default_missing_value("base64"),
        )
        .arg(
            Arg::new("multipass")
                .help("Apply optimizations multiple times")
                .long("multipass")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("pretty")
                .help("Add line breaks and indentation to output")
                .long("pretty")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("indent")
                .help("Number of spaces for indentation with '–pretty'")
                .long("indent")
                .value_name("NUM")
                .default_value("4")
                .value_parser(clap::value_parser!(usize)),
        )
        .arg(
            Arg::new("eol")
                .help("Line break to use when formatting SVG")
                .long("eol")
                .value_name("TYPE")
                .value_parser(["lf", "crlf"])
                .default_value("lf"),
        )
        .arg(
            Arg::new("final-newline")
                .help("Add final newline to output")
                .long("final-newline")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("precision")
                .help("Number of significant digits for numbers")
                .short('p')
                .long("precision")
                .value_name("NUM")
                .value_parser(clap::value_parser!(u8)),
        )
        .arg(
            Arg::new("show-plugins")
                .help("Show available plugins")
                .long("show-plugins")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("quiet")
                .help("Suppress non-error output")
                .short('q')
                .long("quiet")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("parallel")
                .help("Number of threads for parallel processing (0 = auto)")
                .long("parallel")
                .value_name("NUM")
                .value_parser(clap::value_parser!(usize)),
        )
        .arg(
            Arg::new("no-color")
                .help("Disable colored output")
                .long("no-color")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    // Check if we should show plugins and exit
    if matches.get_flag("show-plugins") {
        show_plugins();
        std::process::exit(0);
    }

    let result = run_cli(matches);

    if let Err(e) = result {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

fn run_cli(matches: clap::ArgMatches) -> Result<(), VexySvgoError> {
    let quiet = matches.get_flag("quiet");
    let no_color = matches.get_flag("no-color") || std::env::var("NO_COLOR").is_ok();

    // Load configuration
    let mut config = if let Some(config_path) = matches.get_one::<String>("config") {
        Config::from_file(config_path)?
    } else {
        // Try to load from current directory
        vexy_svgo_core::load_config_from_directory(".")
            .unwrap_or_else(|_| Config::with_default_preset())
    };

    // Apply CLI overrides
    if matches.get_flag("pretty") {
        config.js2svg.pretty = true;
    }

    if let Some(indent) = matches.get_one::<usize>("indent") {
        config.js2svg.indent = indent.to_string();
    }

    if let Some(threads) = matches.get_one::<usize>("parallel") {
        config.parallel = Some(*threads);
    }

    if let Some(eol) = matches.get_one::<String>("eol") {
        use vexy_svgo_core::LineEnding;
        config.js2svg.eol = match eol.as_str() {
            "lf" => LineEnding::Lf,
            "crlf" => LineEnding::Crlf,
            _ => unreachable!(), // Clap validates this
        };
    }

    if matches.get_flag("final-newline") {
        config.js2svg.final_newline = true;
    }

    if matches.get_flag("multipass") {
        config.multipass = true;
    }

    // Apply precision override
    if let Some(precision) = matches.get_one::<u8>("precision") {
        apply_precision_override(&mut config, *precision);
    }

    if let Some(datauri_format) = matches.get_one::<String>("datauri") {
        use vexy_svgo_core::DataUriFormat;
        config.datauri = Some(match datauri_format.as_str() {
            "base64" => DataUriFormat::Base64,
            "enc" => DataUriFormat::Enc,
            "unenc" => DataUriFormat::Unenc,
            _ => unreachable!(), // Clap validates this
        });
    }

    // Handle plugin enable/disable
    if let Some(disabled_plugins) = matches.get_many::<String>("disable") {
        for plugin_name in disabled_plugins {
            config.set_plugin_enabled(plugin_name, false);
        }
    }

    if let Some(enabled_plugins) = matches.get_many::<String>("enable") {
        for plugin_name in enabled_plugins {
            config.set_plugin_enabled(plugin_name, true);
        }
    }

    // Parse CLI arguments
    let (input_mode, output_mode) = parse_cli_args(&matches)?;

    // Process based on input mode
    match input_mode {
        InputMode::Stdin => {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer)?;
            process_string(&buffer, output_mode, &config, quiet)?;
        }
        InputMode::String(content) => {
            process_string(&content, output_mode, &config, quiet)?;
        }
        InputMode::Files(files) => {
            process_files(&files, output_mode, &config, quiet, no_color)?;
        }
        InputMode::Folder(folder) => {
            let recursive = matches.get_flag("recursive");
            let exclude_patterns: Vec<&str> = matches
                .get_many::<String>("exclude")
                .map(|v| v.map(|s| s.as_str()).collect())
                .unwrap_or_default();
            process_folder(&folder, &config, quiet, recursive, &exclude_patterns)?;
        }
    }

    Ok(())
}

fn parse_cli_args(matches: &ArgMatches) -> Result<(InputMode, OutputMode), VexySvgoError> {
    // Determine input mode
    let input_mode = if matches.contains_id("string") {
        InputMode::String(matches.get_one::<String>("string").unwrap().clone())
    } else if matches.contains_id("folder") {
        InputMode::Folder(matches.get_one::<String>("folder").unwrap().clone())
    } else if let Some(inputs) = matches.get_many::<String>("INPUT") {
        let files: Vec<String> = inputs.cloned().collect();
        if files.len() == 1 && files[0] == "-" {
            InputMode::Stdin
        } else {
            InputMode::Files(files)
        }
    } else if let Some(inputs) = matches.get_many::<String>("input") {
        let files: Vec<String> = inputs.cloned().collect();
        if files.len() == 1 && files[0] == "-" {
            InputMode::Stdin
        } else {
            InputMode::Files(files)
        }
    } else {
        InputMode::Stdin
    };

    // Determine output mode
    let output_mode = if let Some(outputs) = matches.get_many::<String>("output") {
        let output_vec: Vec<String> = outputs.cloned().collect();
        if output_vec.len() == 1 {
            let output = &output_vec[0];
            if output == "-" {
                OutputMode::Stdout
            } else if std::path::Path::new(output).is_dir() {
                OutputMode::Directory(output.clone())
            } else {
                OutputMode::File(output.clone())
            }
        } else {
            OutputMode::Directory(output_vec[0].clone())
        }
    } else {
        match &input_mode {
            InputMode::Folder(_) => OutputMode::InPlace,
            InputMode::Files(_) => OutputMode::InPlace,
            _ => OutputMode::Stdout,
        }
    };

    Ok((input_mode, output_mode))
}

fn process_string(
    content: &str,
    output_mode: OutputMode,
    config: &Config,
    quiet: bool,
) -> Result<(), VexySvgoError> {
    let result = optimize_with_config(content, config.clone())?;

    match output_mode {
        OutputMode::Stdout => {
            print!("{}", result.data);
            io::stdout().flush()?;
        }
        OutputMode::File(path) => {
            fs::write(&path, &result.data)?;
            if !quiet {
                let original_size = content.len();
                let optimized_size = result.data.len();
                let saved = original_size.saturating_sub(optimized_size);
                let percent = if original_size > 0 {
                    (saved as f64 / original_size as f64) * 100.0
                } else {
                    0.0
                };
                println!(
                    "{}: {} → {} ({:.1}%)",
                    path,
                    format_bytes(original_size),
                    format_bytes(optimized_size),
                    percent
                );
            }
        }
        _ => return Err("Invalid output mode for string input".into()),
    }

    Ok(())
}

fn process_files(
    files: &[String],
    output_mode: OutputMode,
    config: &Config,
    quiet: bool,
    _no_color: bool,
) -> Result<(), VexySvgoError> {
    match output_mode {
        OutputMode::Stdout => {
            if files.len() > 1 {
                return Err("Cannot output multiple files to stdout".into());
            }
            let content = fs::read_to_string(&files[0])?;
            let result = optimize_with_config(&content, config.clone())?;
            print!("{}", result.data);
            io::stdout().flush()?;
        }
        OutputMode::File(output_path) => {
            if files.len() > 1 {
                return Err("Cannot output multiple files to a single file".into());
            }
            let content = fs::read_to_string(&files[0])?;
            let result = optimize_with_config(&content, config.clone())?;
            fs::write(&output_path, &result.data)?;
            if !quiet {
                let original_size = content.len();
                let optimized_size = result.data.len();
                let saved = original_size.saturating_sub(optimized_size);
                let percent = if original_size > 0 {
                    (saved as f64 / original_size as f64) * 100.0
                } else {
                    0.0
                };
                println!(
                    "{} → {}: {} → {} ({:.1}%)",
                    files[0],
                    output_path,
                    format_bytes(original_size),
                    format_bytes(optimized_size),
                    percent
                );
            }
        }
        OutputMode::Directory(output_dir) => {
            let output_path = Path::new(&output_dir);
            if !output_path.exists() {
                fs::create_dir_all(output_path)?;
            }

            for file in files {
                let content = fs::read_to_string(file)?;
                let result = optimize_with_config(&content, config.clone())?;

                let file_path = Path::new(file);
                let file_name = file_path
                    .file_name()
                    .ok_or("Invalid file name")?;
                let output_file = output_path.join(file_name);

                fs::write(&output_file, &result.data)?;
                if !quiet {
                    let original_size = content.len();
                    let optimized_size = result.data.len();
                    let saved = original_size.saturating_sub(optimized_size);
                    let percent = if original_size > 0 {
                        (saved as f64 / original_size as f64) * 100.0
                    } else {
                        0.0
                    };
                    println!(
                        "{} → {}: {} → {} ({:.1}%)",
                        file,
                        output_file.display(),
                        format_bytes(original_size),
                        format_bytes(optimized_size),
                        percent
                    );
                }
            }
        }
        OutputMode::InPlace => {
            for file in files {
                let content = fs::read_to_string(file)?;
                let result = optimize_with_config(&content, config.clone())?;
                fs::write(file, &result.data)?;
                if !quiet {
                    let original_size = content.len();
                    let optimized_size = result.data.len();
                    let saved = original_size.saturating_sub(optimized_size);
                    let percent = if original_size > 0 {
                        (saved as f64 / original_size as f64) * 100.0
                    } else {
                        0.0
                    };
                    println!(
                        "{}: {} → {} ({:.1}%)",
                        file,
                        format_bytes(original_size),
                        format_bytes(optimized_size),
                        percent
                    );
                }
            }
        }
    }

    Ok(())
}

fn process_folder(
    folder: &str,
    config: &Config,
    quiet: bool,
    recursive: bool,
    exclude_patterns: &[&str],
) -> Result<(), VexySvgoError> {
    let folder_path = Path::new(folder);
    if !folder_path.is_dir() {
        return Err(vexy_svgo_core::error::CliError::InvalidDirectory { 
            path: folder.to_string() 
        }.into());
    }

    let svg_files = if recursive {
        find_svg_files_recursive(folder_path, exclude_patterns)?
    } else {
        find_svg_files(folder_path, exclude_patterns)?
    };

    if svg_files.is_empty() {
        if !quiet {
            println!("No SVG files found in '{folder}'");
        }
        return Ok(());
    }

    if !quiet {
        println!("Processing {} SVG files...", svg_files.len());
    }

    let mut total_original = 0;
    let mut total_optimized = 0;

    for svg_file in &svg_files {
        match fs::read_to_string(svg_file) {
            Ok(content) => {
                match optimize_with_config(&content, config.clone()) {
                    Ok(result) => {
                        fs::write(svg_file, &result.data)?;
                        let original_size = content.len();
                        let optimized_size = result.data.len();
                        total_original += original_size;
                        total_optimized += optimized_size;

                        if !quiet {
                            let saved = original_size.saturating_sub(optimized_size);
                            let percent = if original_size > 0 {
                                (saved as f64 / original_size as f64) * 100.0
                            } else {
                                0.0
                            };
                            println!(
                                "{}: {} → {} ({:.1}%)",
                                svg_file.display(),
                                format_bytes(original_size),
                                format_bytes(optimized_size),
                                percent
                            );
                        }
                    }
                    Err(e) => {
                        eprintln!("Error optimizing '{}': {}", svg_file.display(), e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading '{}': {}", svg_file.display(), e);
            }
        }
    }

    if !quiet && svg_files.len() > 1 {
        let total_saved = total_original.saturating_sub(total_optimized);
        let total_percent = if total_original > 0 {
            (total_saved as f64 / total_original as f64) * 100.0
        } else {
            0.0
        };
        println!(
            "\nTotal: {} → {} ({:.1}%)",
            format_bytes(total_original),
            format_bytes(total_optimized),
            total_percent
        );
    }

    Ok(())
}

fn find_svg_files(
    dir: &Path,
    exclude_patterns: &[&str],
) -> Result<Vec<PathBuf>, VexySvgoError> {
    let mut svg_files = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && is_svg_file(&path) && !is_excluded(&path, exclude_patterns)? {
            svg_files.push(path);
        }
    }

    svg_files.sort();
    Ok(svg_files)
}

fn find_svg_files_recursive(
    dir: &Path,
    exclude_patterns: &[&str],
) -> Result<Vec<PathBuf>, VexySvgoError> {
    let mut svg_files = Vec::new();
    let mut dirs_to_process = vec![dir.to_path_buf()];

    while let Some(current_dir) = dirs_to_process.pop() {
        for entry in fs::read_dir(&current_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                dirs_to_process.push(path);
            } else if path.is_file() && is_svg_file(&path) && !is_excluded(&path, exclude_patterns)?
            {
                svg_files.push(path);
            }
        }
    }

    svg_files.sort();
    Ok(svg_files)
}

fn is_svg_file(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.eq_ignore_ascii_case("svg"))
        .unwrap_or(false)
}

fn is_excluded(path: &Path, patterns: &[&str]) -> Result<bool, VexySvgoError> {
    if patterns.is_empty() {
        return Ok(false);
    }

    let path_str = path.to_str().ok_or(VexySvgoError::General("Invalid path".to_string()))?;
    
    for pattern in patterns {
        let regex = regex::Regex::new(pattern)?;
        if regex.is_match(path_str) {
            return Ok(true);
        }
    }
    
    Ok(false)
}

fn apply_precision_override(config: &mut Config, precision: u8) {
    use vexy_svgo_core::PluginConfig;
    
    for plugin in &mut config.plugins {
        let plugin_name = plugin.name();
        match plugin_name {
            "cleanupNumericValues" | "convertPathData" | "convertTransform" | "cleanupListOfValues" => {
                match plugin {
                    PluginConfig::Name(name) => {
                        // Convert to WithParams variant
                        *plugin = PluginConfig::WithParams {
                            name: name.clone(),
                            params: serde_json::json!({"floatPrecision": precision}),
                        };
                    }
                    PluginConfig::WithParams { params, .. } => {
                        if let Some(obj) = params.as_object_mut() {
                            obj.insert("floatPrecision".to_string(), serde_json::json!(precision));
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

fn show_plugins() {
    use vexy_svgo_core::PluginConfig;
    
    let config = Config::default();
    println!("Available plugins:");
    for plugin in &config.plugins {
        let name = plugin.name();
        let enabled = match plugin {
            PluginConfig::Name(_) => true,
            PluginConfig::WithParams { params, .. } => {
                params.get("enabled")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(true)
            }
        };
        let status = if enabled { "enabled" } else { "disabled" };
        println!("  {name} ({status})");
    }
}

fn format_bytes(bytes: usize) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    if unit_index == 0 {
        format!("{} {}", size as usize, UNITS[unit_index])
    } else {
        format!("{:.2} {}", size, UNITS[unit_index])
    }
}