// this_file: crates/plugin-sdk/src/plugins/convert_path_data/parser.rs

use anyhow::Result;

/// Path command types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CommandType {
    MoveTo,
    LineTo,
    HorizontalLineTo,
    VerticalLineTo,
    CurveTo,
    SmoothCurveTo,
    QuadraticBezier,
    SmoothQuadraticBezier,
    Arc,
    ClosePath,
}

/// Represents a path command with its parameters
#[derive(Debug, Clone)]
pub struct PathCommand {
    pub cmd_type: CommandType,
    pub is_absolute: bool,
    pub params: Vec<f64>,
}

impl PathCommand {
    /// Get the command character
    pub fn get_char(&self) -> char {
        match (self.cmd_type, self.is_absolute) {
            (CommandType::MoveTo, true) => 'M',
            (CommandType::MoveTo, false) => 'm',
            (CommandType::LineTo, true) => 'L',
            (CommandType::LineTo, false) => 'l',
            (CommandType::HorizontalLineTo, true) => 'H',
            (CommandType::HorizontalLineTo, false) => 'h',
            (CommandType::VerticalLineTo, true) => 'V',
            (CommandType::VerticalLineTo, false) => 'v',
            (CommandType::CurveTo, true) => 'C',
            (CommandType::CurveTo, false) => 'c',
            (CommandType::SmoothCurveTo, true) => 'S',
            (CommandType::SmoothCurveTo, false) => 's',
            (CommandType::QuadraticBezier, true) => 'Q',
            (CommandType::QuadraticBezier, false) => 'q',
            (CommandType::SmoothQuadraticBezier, true) => 'T',
            (CommandType::SmoothQuadraticBezier, false) => 't',
            (CommandType::Arc, true) => 'A',
            (CommandType::Arc, false) => 'a',
            (CommandType::ClosePath, _) => 'z',
        }
    }
}

/// Parse a path data string into commands
pub fn parse_path_data(path_data: &str) -> Result<Vec<PathCommand>> {
    let mut commands = Vec::new();
    let mut chars = path_data.chars().peekable();
    let mut current_nums = Vec::new();
    let mut current_num = String::new();
    let mut last_cmd_type = None;
    let mut in_number = false;

    for ch in chars.by_ref() {
        match ch {
            'M' | 'm' | 'L' | 'l' | 'H' | 'h' | 'V' | 'v' | 'C' | 'c' | 'S' | 's' | 'Q' | 'q'
            | 'T' | 't' | 'A' | 'a' | 'Z' | 'z' => {
                // Finish current number if any
                if !current_num.is_empty() {
                    if let Ok(num) = current_num.parse::<f64>() {
                        current_nums.push(num);
                    }
                    current_num.clear();
                    in_number = false;
                }

                // Process accumulated numbers with previous command
                if let Some(cmd_type) = last_cmd_type {
                    process_accumulated_params(&mut commands, cmd_type, &mut current_nums)?;
                }

                // Parse new command
                let (cmd_type, is_absolute) = parse_command_char(ch)?;

                if cmd_type == CommandType::ClosePath {
                    commands.push(PathCommand {
                        cmd_type,
                        is_absolute: true,
                        params: vec![],
                    });
                    last_cmd_type = None;
                } else {
                    last_cmd_type = Some((cmd_type, is_absolute));
                }
            }
            '0'..='9' | '.' | '-' | '+' | 'e' | 'E' => {
                if ch == '-' || ch == '+' {
                    // Start new number if not at beginning of current number
                    if !current_num.is_empty() && in_number {
                        if let Ok(num) = current_num.parse::<f64>() {
                            current_nums.push(num);
                        }
                        current_num.clear();
                    }
                }
                current_num.push(ch);
                in_number = true;
            }
            ' ' | ',' | '\t' | '\n' | '\r' => {
                // Number separator
                if !current_num.is_empty() {
                    if let Ok(num) = current_num.parse::<f64>() {
                        current_nums.push(num);
                    }
                    current_num.clear();
                    in_number = false;
                }
            }
            _ => {
                // Ignore other characters
            }
        }
    }

    // Finish last number
    if !current_num.is_empty() {
        if let Ok(num) = current_num.parse::<f64>() {
            current_nums.push(num);
        }
    }

    // Process final accumulated numbers
    if let Some(cmd_type) = last_cmd_type {
        process_accumulated_params(&mut commands, cmd_type, &mut current_nums)?;
    }

    Ok(commands)
}

/// Parse a command character into its type and whether it's absolute
fn parse_command_char(ch: char) -> Result<(CommandType, bool)> {
    match ch {
        'M' => Ok((CommandType::MoveTo, true)),
        'm' => Ok((CommandType::MoveTo, false)),
        'L' => Ok((CommandType::LineTo, true)),
        'l' => Ok((CommandType::LineTo, false)),
        'H' => Ok((CommandType::HorizontalLineTo, true)),
        'h' => Ok((CommandType::HorizontalLineTo, false)),
        'V' => Ok((CommandType::VerticalLineTo, true)),
        'v' => Ok((CommandType::VerticalLineTo, false)),
        'C' => Ok((CommandType::CurveTo, true)),
        'c' => Ok((CommandType::CurveTo, false)),
        'S' => Ok((CommandType::SmoothCurveTo, true)),
        's' => Ok((CommandType::SmoothCurveTo, false)),
        'Q' => Ok((CommandType::QuadraticBezier, true)),
        'q' => Ok((CommandType::QuadraticBezier, false)),
        'T' => Ok((CommandType::SmoothQuadraticBezier, true)),
        't' => Ok((CommandType::SmoothQuadraticBezier, false)),
        'A' => Ok((CommandType::Arc, true)),
        'a' => Ok((CommandType::Arc, false)),
        'Z' | 'z' => Ok((CommandType::ClosePath, true)),
        _ => Err(anyhow::anyhow!("Unknown command character: {}", ch)),
    }
}

/// Process accumulated parameters for a command type
fn process_accumulated_params(
    commands: &mut Vec<PathCommand>,
    (cmd_type, is_absolute): (CommandType, bool),
    params: &mut Vec<f64>,
) -> Result<()> {
    let expected = match cmd_type {
        CommandType::MoveTo | CommandType::LineTo => 2,
        CommandType::HorizontalLineTo | CommandType::VerticalLineTo => 1,
        CommandType::CurveTo => 6,
        CommandType::SmoothCurveTo | CommandType::QuadraticBezier => 4,
        CommandType::SmoothQuadraticBezier => 2,
        CommandType::Arc => 7,
        CommandType::ClosePath => 0,
    };

    if expected == 0 {
        return Ok(());
    }

    // Process params in chunks
    while params.len() >= expected {
        let chunk: Vec<f64> = params.drain(..expected).collect();

        // Special case: MoveTo followed by implicit LineTo
        let actual_cmd_type = if cmd_type == CommandType::MoveTo && !commands.is_empty() {
            CommandType::LineTo
        } else {
            cmd_type
        };

        commands.push(PathCommand {
            cmd_type: actual_cmd_type,
            is_absolute,
            params: chunk,
        });
    }

    if !params.is_empty() {
        // Leftover params - this is technically an error but we'll ignore them
        params.clear();
    }

    Ok(())
}