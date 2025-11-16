// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! Action execution implementation for MIDIMon daemon.
//!
//! This module contains the ActionExecutor which is responsible for executing
//! actions (keyboard, mouse, shell commands, etc.) on the host system.
//!
//! This was moved from midimon-core to maintain architectural purity:
//! - Core: Pure data structures and logic (UI-independent)
//! - Daemon: System interaction (keyboard, mouse, shell, etc.)

use midimon_core::{Action, VolumeOperation};
use enigo::{Button, Coordinate, Direction, Enigo, Key, Keyboard, Mouse, Settings};
use std::process::Command;
use std::thread;
use std::time::Duration;

/// ActionExecutor handles the execution of actions on the host system.
///
/// This includes:
/// - Keyboard simulation via enigo
/// - Mouse simulation via enigo
/// - Shell command execution
/// - Application launching
/// - Volume control
///
/// # Architecture Note
/// This executor lives in the daemon layer (not core) because it interacts
/// with the operating system through UI libraries (enigo) and system commands.
pub struct ActionExecutor {
    enigo: Enigo,
}

impl Default for ActionExecutor {
    fn default() -> Self {
        Self::new()
    }
}

impl ActionExecutor {
    /// Create a new ActionExecutor with default settings
    pub fn new() -> Self {
        Self {
            enigo: Enigo::new(&Settings::default()).unwrap(),
        }
    }

    /// Execute an action
    ///
    /// # Arguments
    /// * `action` - The action to execute
    ///
    /// # Examples
    /// ```no_run
    /// use midimon_daemon::ActionExecutor;
    /// use midimon_core::Action;
    ///
    /// let mut executor = ActionExecutor::new();
    /// executor.execute(Action::Text("Hello, World!".to_string()));
    /// ```
    pub fn execute(&mut self, action: Action) {
        match action {
            Action::Keystroke { keys, modifiers } => {
                self.execute_keystroke(keys, modifiers);
            }
            Action::Text(text) => {
                self.enigo.text(&text).unwrap();
            }
            Action::Launch(app) => {
                self.launch_app(&app);
            }
            Action::Shell(cmd) => {
                self.execute_shell(&cmd);
            }
            Action::Sequence(actions) => {
                for act in actions {
                    self.execute(act);
                    thread::sleep(Duration::from_millis(50));
                }
            }
            Action::Delay(ms) => {
                thread::sleep(Duration::from_millis(ms));
            }
            Action::MouseClick { button, x, y } => {
                if let (Some(x), Some(y)) = (x, y) {
                    self.enigo.move_mouse(x, y, Coordinate::Abs).unwrap();
                }
                self.enigo.button(button, Direction::Click).unwrap();
            }
            Action::Repeat { action, count, delay_ms } => {
                for i in 0..count {
                    self.execute((*action).clone());

                    // Add delay between iterations (but not after the last one)
                    if i < count - 1 {
                        if let Some(delay) = delay_ms {
                            thread::sleep(Duration::from_millis(delay));
                        }
                    }
                }
            }
            Action::Conditional { condition, then_action, else_action } => {
                if evaluate_condition(&condition) {
                    self.execute((*then_action).clone());
                } else if let Some(else_act) = else_action {
                    self.execute((*else_act).clone());
                }
            }
            Action::VolumeControl { operation, value } => {
                execute_volume_control(&operation, &value);
            }
            Action::ModeChange { mode } => {
                // Mode changes are handled by the mapping engine
                // This action just serves as a marker in the config
                eprintln!("ModeChange to '{}' (handled by mapping engine)", mode);
            }
        }
    }

    /// Execute a keystroke with modifiers
    fn execute_keystroke(&mut self, keys: Vec<Key>, modifiers: Vec<Key>) {
        // Press modifiers
        for modifier in &modifiers {
            self.enigo.key(*modifier, Direction::Press).unwrap();
        }

        // Press keys
        for key in &keys {
            self.enigo.key(*key, Direction::Click).unwrap();
        }

        // Release modifiers
        for modifier in modifiers.iter().rev() {
            self.enigo.key(*modifier, Direction::Release).unwrap();
        }
    }

    /// Launch an application
    fn launch_app(&self, app: &str) {
        #[cfg(target_os = "macos")]
        {
            Command::new("open").arg("-a").arg(app).spawn().ok();
        }

        #[cfg(target_os = "linux")]
        {
            Command::new(app).spawn().ok();
        }

        #[cfg(target_os = "windows")]
        {
            Command::new("cmd").args(&["/C", "start", app]).spawn().ok();
        }
    }

    /// Execute a shell command
    ///
    /// # Security Note
    /// Shell commands should be validated before reaching this point
    /// (see `validate_shell_command()` in config loader).
    fn execute_shell(&self, cmd: &str) {
        #[cfg(unix)]
        {
            Command::new("sh").arg("-c").arg(cmd).spawn().ok();
        }

        #[cfg(windows)]
        {
            Command::new("cmd").args(&["/C", cmd]).spawn().ok();
        }
    }
}

/// Evaluates a condition string and returns true/false
///
/// Supported condition formats:
/// - "Always" - Always returns true
/// - "Never" - Always returns false
/// - "TimeRange:HH:MM-HH:MM" - Returns true if current time is within range (24-hour format)
fn evaluate_condition(condition: &str) -> bool {
    match condition {
        "Always" => true,
        "Never" => false,
        s if s.starts_with("TimeRange:") => {
            evaluate_time_range(&s[10..]) // Skip "TimeRange:" prefix
        }
        _ => {
            eprintln!("Warning: Unknown condition '{}', defaulting to false", condition);
            false
        }
    }
}

/// Evaluates a time range condition
///
/// Format: "HH:MM-HH:MM" (24-hour format)
/// Returns true if current time is within the range
fn evaluate_time_range(range: &str) -> bool {
    use chrono::{Local, Timelike};

    // Parse range "HH:MM-HH:MM"
    let parts: Vec<&str> = range.split('-').collect();
    if parts.len() != 2 {
        eprintln!("Invalid time range format: {}", range);
        return false;
    }

    let start_parts: Vec<&str> = parts[0].split(':').collect();
    let end_parts: Vec<&str> = parts[1].split(':').collect();

    if start_parts.len() != 2 || end_parts.len() != 2 {
        eprintln!("Invalid time format in range: {}", range);
        return false;
    }

    let start_hour: u32 = start_parts[0].parse().unwrap_or(0);
    let start_min: u32 = start_parts[1].parse().unwrap_or(0);
    let end_hour: u32 = end_parts[0].parse().unwrap_or(0);
    let end_min: u32 = end_parts[1].parse().unwrap_or(0);

    let now = Local::now();
    let current_hour = now.hour();
    let current_min = now.minute();

    let current_mins = current_hour * 60 + current_min;
    let start_mins = start_hour * 60 + start_min;
    let end_mins = end_hour * 60 + end_min;

    if start_mins <= end_mins {
        // Normal range (doesn't cross midnight)
        current_mins >= start_mins && current_mins <= end_mins
    } else {
        // Range crosses midnight
        current_mins >= start_mins || current_mins <= end_mins
    }
}

/// Execute volume control operations
///
/// # Platform Support
/// - macOS: Uses AppleScript
/// - Linux: Uses amixer (ALSA) or pactl (PulseAudio)
/// - Windows: Not implemented
fn execute_volume_control(operation: &VolumeOperation, value: &Option<u8>) {

    #[cfg(target_os = "macos")]
    {
        let script = match operation {
            VolumeOperation::Up => "set volume output volume ((output volume of (get volume settings)) + 10)",
            VolumeOperation::Down => "set volume output volume ((output volume of (get volume settings)) - 10)",
            VolumeOperation::Mute => "set volume output muted true",
            VolumeOperation::Unmute => "set volume output muted false",
            VolumeOperation::Set => {
                if let Some(vol) = value {
                    &format!("set volume output volume {}", vol)
                } else {
                    "set volume output volume 50"
                }
            }
        };

        Command::new("osascript")
            .arg("-e")
            .arg(script)
            .spawn()
            .ok();
    }

    #[cfg(target_os = "linux")]
    {
        // Try PulseAudio first, fall back to ALSA
        let cmd = match operation {
            VolumeOperation::Up => vec!["pactl", "set-sink-volume", "@DEFAULT_SINK@", "+10%"],
            VolumeOperation::Down => vec!["pactl", "set-sink-volume", "@DEFAULT_SINK@", "-10%"],
            VolumeOperation::Mute => vec!["pactl", "set-sink-mute", "@DEFAULT_SINK@", "1"],
            VolumeOperation::Unmute => vec!["pactl", "set-sink-mute", "@DEFAULT_SINK@", "0"],
            VolumeOperation::Set => {
                if let Some(vol) = value {
                    vec!["pactl", "set-sink-volume", "@DEFAULT_SINK@", &format!("{}%", vol)]
                } else {
                    vec!["pactl", "set-sink-volume", "@DEFAULT_SINK@", "50%"]
                }
            }
        };

        Command::new(cmd[0])
            .args(&cmd[1..])
            .spawn()
            .ok();
    }

    #[cfg(target_os = "windows")]
    {
        eprintln!("Volume control not implemented for Windows");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_condition_always() {
        assert_eq!(evaluate_condition("Always"), true);
    }

    #[test]
    fn test_evaluate_condition_never() {
        assert_eq!(evaluate_condition("Never"), false);
    }

    #[test]
    fn test_evaluate_condition_unknown() {
        assert_eq!(evaluate_condition("UnknownCondition"), false);
    }

    #[test]
    fn test_time_range_format() {
        // Just ensure it doesn't panic with valid format
        evaluate_time_range("09:00-17:00");
        evaluate_time_range("23:00-01:00"); // Crosses midnight
    }

    #[test]
    fn test_time_range_invalid_format() {
        assert_eq!(evaluate_time_range("invalid"), false);
        assert_eq!(evaluate_time_range("09:00"), false);
        assert_eq!(evaluate_time_range("09-17"), false);
    }
}
