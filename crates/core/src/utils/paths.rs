// this_file: crates/core/src/utils/paths.rs

//! Path data utilities for SVG processing
//!
//! Common functions for working with SVG path data across multiple plugins.

use crate::utils::numbers::NumberUtils;
use std::fmt;

/// Path data utilities
pub struct PathUtils;

impl PathUtils {
    /// Parse path data string into individual commands
    pub fn parse_path_data(data: &str) -> Vec<PathCommand> {
        let mut commands = Vec::new();
        let mut chars = data.chars().peekable();
        
        while let Some(&ch) = chars.peek() {
            if ch.is_whitespace() || ch == ',' {
                chars.next();
                continue;
            }
            
            if ch.is_alphabetic() {
                // Command letter
                let cmd = chars.next().unwrap();
                let mut params = Vec::new();
                
                // Parse parameters
                while let Some(&next_ch) = chars.peek() {
                    if next_ch.is_alphabetic() {
                        break;
                    }
                    
                    if next_ch.is_whitespace() || next_ch == ',' {
                        chars.next();
                        continue;
                    }
                    
                    // Parse number
                    let mut number_str = String::new();
                    while let Some(&digit_ch) = chars.peek() {
                        if digit_ch.is_ascii_digit() || digit_ch == '.' || digit_ch == '-' || digit_ch == '+' {
                            number_str.push(chars.next().unwrap());
                        } else {
                            break;
                        }
                    }
                    
                    if let Ok(num) = number_str.parse::<f64>() {
                        params.push(num);
                    }
                }
                
                commands.push(PathCommand { command: cmd, params });
            } else {
                chars.next();
            }
        }
        
        commands
    }
    
    /// Convert path commands back to string
    pub fn stringify_path_data(commands: &[PathCommand]) -> String {
        let mut result = String::new();
        
        for cmd in commands {
            result.push(cmd.command);
            
            for (i, &param) in cmd.params.iter().enumerate() {
                if i > 0 {
                    result.push(',');
                }
                result.push_str(&NumberUtils::format_number_minimal(param));
            }
        }
        
        result
    }
    
    /// Check if a path command is a move command
    pub fn is_move_command(cmd: char) -> bool {
        matches!(cmd, 'M' | 'm')
    }
    
    /// Check if a path command is a line command
    pub fn is_line_command(cmd: char) -> bool {
        matches!(cmd, 'L' | 'l' | 'H' | 'h' | 'V' | 'v')
    }
    
    /// Check if a path command is a curve command
    pub fn is_curve_command(cmd: char) -> bool {
        matches!(cmd, 'C' | 'c' | 'S' | 's' | 'Q' | 'q' | 'T' | 't')
    }
    
    /// Check if a path command is an arc command
    pub fn is_arc_command(cmd: char) -> bool {
        matches!(cmd, 'A' | 'a')
    }
    
    /// Check if a path command is a close command
    pub fn is_close_command(cmd: char) -> bool {
        matches!(cmd, 'Z' | 'z')
    }
    
    /// Convert relative command to absolute
    pub fn to_absolute_command(cmd: char) -> char {
        match cmd {
            'm' => 'M',
            'l' => 'L',
            'h' => 'H',
            'v' => 'V',
            'c' => 'C',
            's' => 'S',
            'q' => 'Q',
            't' => 'T',
            'a' => 'A',
            'z' => 'Z',
            _ => cmd, // Already absolute or unknown
        }
    }
    
    /// Convert absolute command to relative
    pub fn to_relative_command(cmd: char) -> char {
        match cmd {
            'M' => 'm',
            'L' => 'l',
            'H' => 'h',
            'V' => 'v',
            'C' => 'c',
            'S' => 's',
            'Q' => 'q',
            'T' => 't',
            'A' => 'a',
            'Z' => 'z',
            _ => cmd, // Already relative or unknown
        }
    }
    
    /// Check if a command is relative
    pub fn is_relative_command(cmd: char) -> bool {
        cmd.is_lowercase()
    }
    
    /// Get the number of parameters expected for a command
    pub fn get_param_count(cmd: char) -> usize {
        match cmd.to_ascii_uppercase() {
            'M' | 'L' => 2,
            'H' | 'V' => 1,
            'C' => 6,
            'S' | 'Q' => 4,
            'T' => 2,
            'A' => 7,
            'Z' => 0,
            _ => 0,
        }
    }
}

/// Represents a single path command
#[derive(Debug, Clone, PartialEq)]
pub struct PathCommand {
    pub command: char,
    pub params: Vec<f64>,
}

impl PathCommand {
    /// Create a new path command
    pub fn new(command: char, params: Vec<f64>) -> Self {
        Self { command, params }
    }
    
    /// Check if this command has the expected number of parameters
    pub fn is_valid(&self) -> bool {
        let expected = PathUtils::get_param_count(self.command);
        expected == 0 || self.params.len() % expected == 0
    }
    
}

impl fmt::Display for PathCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.command)?;
        
        for (i, &param) in self.params.iter().enumerate() {
            if i > 0 {
                write!(f, ",")?;
            }
            write!(f, "{}", NumberUtils::format_number_minimal(param))?;
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_path_data() {
        let commands = PathUtils::parse_path_data("M10,20 L30,40");
        assert_eq!(commands.len(), 2);
        assert_eq!(commands[0].command, 'M');
        assert_eq!(commands[0].params, vec![10.0, 20.0]);
        assert_eq!(commands[1].command, 'L');
        assert_eq!(commands[1].params, vec![30.0, 40.0]);
    }
    
    #[test]
    fn test_stringify_path_data() {
        let commands = vec![
            PathCommand::new('M', vec![10.0, 20.0]),
            PathCommand::new('L', vec![30.0, 40.0]),
        ];
        let result = PathUtils::stringify_path_data(&commands);
        assert_eq!(result, "M10,20L30,40");
    }
    
    #[test]
    fn test_command_types() {
        assert!(PathUtils::is_move_command('M'));
        assert!(PathUtils::is_move_command('m'));
        assert!(PathUtils::is_line_command('L'));
        assert!(PathUtils::is_line_command('l'));
        assert!(PathUtils::is_curve_command('C'));
        assert!(PathUtils::is_curve_command('c'));
        assert!(PathUtils::is_arc_command('A'));
        assert!(PathUtils::is_arc_command('a'));
        assert!(PathUtils::is_close_command('Z'));
        assert!(PathUtils::is_close_command('z'));
    }
    
    #[test]
    fn test_command_conversion() {
        assert_eq!(PathUtils::to_absolute_command('m'), 'M');
        assert_eq!(PathUtils::to_absolute_command('M'), 'M');
        assert_eq!(PathUtils::to_relative_command('M'), 'm');
        assert_eq!(PathUtils::to_relative_command('m'), 'm');
        assert!(PathUtils::is_relative_command('m'));
        assert!(!PathUtils::is_relative_command('M'));
    }
    
    #[test]
    fn test_param_count() {
        assert_eq!(PathUtils::get_param_count('M'), 2);
        assert_eq!(PathUtils::get_param_count('L'), 2);
        assert_eq!(PathUtils::get_param_count('C'), 6);
        assert_eq!(PathUtils::get_param_count('Z'), 0);
    }
    
    #[test]
    fn test_path_command_validity() {
        let cmd = PathCommand::new('M', vec![10.0, 20.0]);
        assert!(cmd.is_valid());
        
        let invalid_cmd = PathCommand::new('M', vec![10.0]);
        assert!(!invalid_cmd.is_valid());
    }
}