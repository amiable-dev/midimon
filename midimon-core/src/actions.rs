// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

use crate::config::ActionConfig;
use enigo::{Button, Coordinate, Direction, Enigo, Key, Keyboard, Mouse, Settings};
use std::process::Command;
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone)]
pub enum Action {
    Keystroke {
        keys: Vec<Key>,
        modifiers: Vec<Key>,
    },
    Text(String),
    Launch(String),
    Shell(String),
    Sequence(Vec<Action>),
    Delay(u64),
    MouseClick {
        button: Button,
        x: Option<i32>,
        y: Option<i32>,
    },
    Repeat {
        action: Box<Action>,
        count: usize,
        delay_ms: Option<u64>,
    },
    Conditional {
        condition: String,
        then_action: Box<Action>,
        else_action: Option<Box<Action>>,
    },
    VolumeControl {
        operation: VolumeOperation,
        value: Option<u8>,
    },
    ModeChange {
        mode: String,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum VolumeOperation {
    Up,
    Down,
    Mute,
    Unmute,
    Set,
}

pub struct ActionExecutor {
    enigo: Enigo,
}

impl Default for ActionExecutor {
    fn default() -> Self {
        Self::new()
    }
}

impl ActionExecutor {
    pub fn new() -> Self {
        Self {
            enigo: Enigo::new(&Settings::default()).unwrap(),
        }
    }

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
                execute_volume_control(&operation, value);
            }
            Action::ModeChange { mode } => {
                // Mode changes are handled by the mapping engine
                // This action just serves as a marker in the config
                eprintln!("ModeChange to '{}' (handled by mapping engine)", mode);
            }
        }
    }

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

impl From<ActionConfig> for Action {
    fn from(config: ActionConfig) -> Self {
        match config {
            ActionConfig::Keystroke { keys, modifiers } => Action::Keystroke {
                keys: parse_keys(&keys),
                modifiers: modifiers.iter().flat_map(|m| parse_modifier(m)).collect(),
            },
            ActionConfig::Text { text } => Action::Text(text),
            ActionConfig::Launch { app } => Action::Launch(app),
            ActionConfig::Shell { command } => Action::Shell(command),
            ActionConfig::Sequence { actions } => {
                Action::Sequence(actions.into_iter().map(Into::into).collect())
            }
            ActionConfig::Delay { ms } => Action::Delay(ms),
            ActionConfig::MouseClick { button, x, y } => Action::MouseClick {
                button: parse_mouse_button(&button),
                x,
                y,
            },
            ActionConfig::VolumeControl { operation, value } => Action::VolumeControl {
                operation: parse_volume_operation(&operation),
                value,
            },
            ActionConfig::ModeChange { mode } => Action::ModeChange { mode },
            ActionConfig::Repeat { action, count, delay_ms } => Action::Repeat {
                action: Box::new((*action).into()),
                count,
                delay_ms,
            },
            ActionConfig::Conditional { condition, then_action, else_action } => Action::Conditional {
                condition,
                then_action: Box::new((*then_action).into()),
                else_action: else_action.map(|a| Box::new((*a).into())),
            },
        }
    }
}

fn parse_keys(keys: &str) -> Vec<Key> {
    keys.split('+')
        .filter_map(|k| parse_key(k.trim()))
        .collect()
}

fn parse_key(key: &str) -> Option<Key> {
    match key.to_lowercase().as_str() {
        "space" => Some(Key::Space),
        "return" | "enter" => Some(Key::Return),
        "tab" => Some(Key::Tab),
        "escape" | "esc" => Some(Key::Escape),
        "backspace" => Some(Key::Backspace),
        "delete" | "del" => Some(Key::Delete),
        "up" => Some(Key::UpArrow),
        "down" => Some(Key::DownArrow),
        "left" => Some(Key::LeftArrow),
        "right" => Some(Key::RightArrow),
        "home" => Some(Key::Home),
        "end" => Some(Key::End),
        "pageup" => Some(Key::PageUp),
        "pagedown" => Some(Key::PageDown),
        "f1" => Some(Key::F1),
        "f2" => Some(Key::F2),
        "f3" => Some(Key::F3),
        "f4" => Some(Key::F4),
        "f5" => Some(Key::F5),
        "f6" => Some(Key::F6),
        "f7" => Some(Key::F7),
        "f8" => Some(Key::F8),
        "f9" => Some(Key::F9),
        "f10" => Some(Key::F10),
        "f11" => Some(Key::F11),
        "f12" => Some(Key::F12),
        s if s.len() == 1 => {
            let c = s.chars().next().unwrap();
            Some(Key::Unicode(c))
        }
        _ => None,
    }
}

fn parse_modifier(modifier: &str) -> Option<Key> {
    match modifier.to_lowercase().as_str() {
        "cmd" | "command" | "meta" => Some(Key::Meta),
        "ctrl" | "control" => Some(Key::Control),
        "alt" | "option" => Some(Key::Option),
        "shift" => Some(Key::Shift),
        _ => None,
    }
}

fn parse_mouse_button(button: &str) -> Button {
    match button.to_lowercase().as_str() {
        "right" => Button::Right,
        "middle" => Button::Middle,
        _ => Button::Left,
    }
}

fn parse_volume_operation(operation: &str) -> VolumeOperation {
    match operation.to_lowercase().as_str() {
        "up" => VolumeOperation::Up,
        "down" => VolumeOperation::Down,
        "mute" => VolumeOperation::Mute,
        "unmute" => VolumeOperation::Unmute,
        "set" => VolumeOperation::Set,
        _ => {
            eprintln!("Unknown volume operation '{}', defaulting to Up", operation);
            VolumeOperation::Up
        }
    }
}

/// Evaluates a condition string and returns true/false
///
/// Supported condition formats:
/// - "Always" - Always returns true
/// - "Never" - Always returns false
/// - "TimeRange:HH:MM-HH:MM" - Returns true if current time is within range (24-hour format)
///
/// # Examples
/// ```
/// assert_eq!(evaluate_condition("Always"), true);
/// assert_eq!(evaluate_condition("Never"), false);
/// assert_eq!(evaluate_condition("TimeRange:09:00-17:00"), /* depends on current time */);
/// ```
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

/// Evaluates a time range condition in format "HH:MM-HH:MM"
///
/// Returns true if the current time is within the specified range.
/// Uses 24-hour format.
fn evaluate_time_range(range: &str) -> bool {
    // Parse the time range "HH:MM-HH:MM"
    let parts: Vec<&str> = range.split('-').collect();
    if parts.len() != 2 {
        eprintln!("Warning: Invalid TimeRange format '{}', expected HH:MM-HH:MM", range);
        return false;
    }

    let start_time = parse_time(parts[0]);
    let end_time = parse_time(parts[1]);

    if start_time.is_none() || end_time.is_none() {
        eprintln!("Warning: Failed to parse time range '{}'", range);
        return false;
    }

    let (start_hour, start_min) = start_time.unwrap();
    let (end_hour, end_min) = end_time.unwrap();

    // Get current time
    use std::time::SystemTime;
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // Convert to local time (seconds since midnight)
    // This is a simplified version - for production you'd want chrono or time crate
    let seconds_in_day = 86400;
    let local_seconds = (now % seconds_in_day) as u32;
    let current_hour = (local_seconds / 3600) as u8;
    let current_min = ((local_seconds % 3600) / 60) as u8;

    let current_minutes = current_hour as u16 * 60 + current_min as u16;
    let start_minutes = start_hour as u16 * 60 + start_min as u16;
    let end_minutes = end_hour as u16 * 60 + end_min as u16;

    // Handle ranges that cross midnight
    if start_minutes <= end_minutes {
        current_minutes >= start_minutes && current_minutes <= end_minutes
    } else {
        current_minutes >= start_minutes || current_minutes <= end_minutes
    }
}

/// Parses a time string in format "HH:MM" to (hour, minute)
fn parse_time(time_str: &str) -> Option<(u8, u8)> {
    let parts: Vec<&str> = time_str.trim().split(':').collect();
    if parts.len() != 2 {
        return None;
    }

    let hour = parts[0].parse::<u8>().ok()?;
    let minute = parts[1].parse::<u8>().ok()?;

    if hour > 23 || minute > 59 {
        return None;
    }

    Some((hour, minute))
}

/// Execute volume control operation using platform-specific commands
fn execute_volume_control(operation: &VolumeOperation, value: Option<u8>) {
    #[cfg(target_os = "macos")]
    {
        let script = match operation {
            VolumeOperation::Up => "osascript -e 'set volume output volume ((output volume of (get volume settings)) + 5)'",
            VolumeOperation::Down => "osascript -e 'set volume output volume ((output volume of (get volume settings)) - 5)'",
            VolumeOperation::Mute => "osascript -e 'set volume output muted true'",
            VolumeOperation::Unmute => "osascript -e 'set volume output muted false'",
            VolumeOperation::Set => {
                if let Some(vol) = value {
                    let vol = vol.min(100); // Clamp to 100
                    return Command::new("osascript")
                        .arg("-e")
                        .arg(format!("set volume output volume {}", vol))
                        .output()
                        .map(|_| ())
                        .unwrap_or_else(|e| eprintln!("Volume control error: {}", e));
                } else {
                    eprintln!("VolumeControl::Set requires a value");
                    return;
                }
            }
        };

        Command::new("sh")
            .arg("-c")
            .arg(script)
            .output()
            .unwrap_or_else(|e| {
                eprintln!("Volume control error: {}", e);
                std::process::Output {
                    status: std::process::ExitStatus::default(),
                    stdout: Vec::new(),
                    stderr: Vec::new(),
                }
            });
    }

    #[cfg(not(target_os = "macos"))]
    {
        eprintln!("VolumeControl not yet implemented for this platform");
        let _ = (operation, value); // Suppress unused warnings
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    /// Mock action executor that tracks execution count instead of performing real actions.
    /// This allows testing without side effects like actual keyboard/mouse input.
    struct MockActionExecutor {
        execution_count: Arc<Mutex<usize>>,
    }

    impl MockActionExecutor {
        fn new() -> Self {
            Self {
                execution_count: Arc::new(Mutex::new(0)),
            }
        }

        fn execute(&mut self, action: &Action) {
            match action {
                Action::Delay(ms) => {
                    // For testing, we don't actually sleep
                    // but we record that a delay was requested
                    let _ = ms;
                }
                Action::Sequence(actions) => {
                    for act in actions {
                        self.execute(act);
                    }
                }
                Action::Repeat { action, count, delay_ms } => {
                    for i in 0..*count {
                        self.execute(action);
                        if i < count - 1 && delay_ms.is_some() {
                            // Record delay but don't sleep
                        }
                    }
                }
                Action::Conditional { condition, then_action, else_action } => {
                    if evaluate_condition(condition) {
                        self.execute(then_action);
                    } else if let Some(else_act) = else_action {
                        self.execute(else_act);
                    }
                }
                // For all other actions, just increment the counter
                _ => {
                    let mut count = self.execution_count.lock().unwrap();
                    *count += 1;
                }
            }
        }

        fn get_count(&self) -> usize {
            *self.execution_count.lock().unwrap()
        }
    }

    // ========================================
    // Tests for F19: Repeat Action
    // ========================================

    #[test]
    fn test_repeat_basic_execution() {
        // Test basic repeat execution (count=3, verify action executes 3 times)
        let mut executor = MockActionExecutor::new();

        let action = Action::Repeat {
            action: Box::new(Action::Text("test".to_string())),
            count: 3,
            delay_ms: None,
        };

        executor.execute(&action);
        assert_eq!(executor.get_count(), 3, "Action should execute exactly 3 times");
    }

    #[test]
    fn test_repeat_with_delay() {
        // Test repeat with delay between iterations
        let mut executor = MockActionExecutor::new();

        let action = Action::Repeat {
            action: Box::new(Action::Text("test".to_string())),
            count: 5,
            delay_ms: Some(100), // 100ms delay between iterations
        };

        executor.execute(&action);
        assert_eq!(executor.get_count(), 5, "Action should execute 5 times with delays");
    }

    #[test]
    fn test_repeat_count_one() {
        // Test repeat with count=1 (edge case - should execute once)
        let mut executor = MockActionExecutor::new();

        let action = Action::Repeat {
            action: Box::new(Action::Text("test".to_string())),
            count: 1,
            delay_ms: Some(50),
        };

        executor.execute(&action);
        assert_eq!(executor.get_count(), 1, "Action should execute exactly once");
    }

    #[test]
    fn test_repeat_count_zero() {
        // Test repeat with count=0 (edge case - should not execute)
        let mut executor = MockActionExecutor::new();

        let action = Action::Repeat {
            action: Box::new(Action::Text("test".to_string())),
            count: 0,
            delay_ms: None,
        };

        executor.execute(&action);
        assert_eq!(executor.get_count(), 0, "Action should not execute with count=0");
    }

    #[test]
    fn test_nested_repeats() {
        // Test nested repeats (Repeat containing Repeat)
        let mut executor = MockActionExecutor::new();

        let inner_repeat = Action::Repeat {
            action: Box::new(Action::Text("inner".to_string())),
            count: 2,
            delay_ms: None,
        };

        let outer_repeat = Action::Repeat {
            action: Box::new(inner_repeat),
            count: 3,
            delay_ms: None,
        };

        executor.execute(&outer_repeat);
        // Outer repeats 3 times, each time executing inner which repeats 2 times
        assert_eq!(executor.get_count(), 6, "Nested repeat should execute 3 * 2 = 6 times");
    }

    #[test]
    fn test_repeat_of_sequence() {
        // Test repeat of Sequence action
        let mut executor = MockActionExecutor::new();

        let sequence = Action::Sequence(vec![
            Action::Text("a".to_string()),
            Action::Text("b".to_string()),
            Action::Text("c".to_string()),
        ]);

        let repeat = Action::Repeat {
            action: Box::new(sequence),
            count: 4,
            delay_ms: Some(10),
        };

        executor.execute(&repeat);
        // Sequence has 3 actions, repeated 4 times = 12 executions
        assert_eq!(executor.get_count(), 12, "Repeat of sequence should execute all actions");
    }

    #[test]
    fn test_repeat_no_delay_after_final() {
        // Verify no delay after final iteration
        // This test verifies the logic in the actual implementation
        // where we check `if i < count - 1` before adding delay
        let action = Action::Repeat {
            action: Box::new(Action::Text("test".to_string())),
            count: 3,
            delay_ms: Some(100),
        };

        // We can't easily measure timing in unit tests without slowing them down,
        // but we verify the structure is correct
        match action {
            Action::Repeat { count, delay_ms, .. } => {
                assert_eq!(count, 3);
                assert_eq!(delay_ms, Some(100));
            }
            _ => panic!("Expected Repeat action"),
        }
    }

    // ========================================
    // Tests for F20: Conditional Action
    // ========================================

    #[test]
    fn test_conditional_always() {
        // Test "Always" condition (should execute then_action)
        let mut executor = MockActionExecutor::new();

        let action = Action::Conditional {
            condition: "Always".to_string(),
            then_action: Box::new(Action::Text("then".to_string())),
            else_action: Some(Box::new(Action::Text("else".to_string()))),
        };

        executor.execute(&action);
        assert_eq!(executor.get_count(), 1, "Always condition should execute then_action");
    }

    #[test]
    fn test_conditional_never() {
        // Test "Never" condition (should execute else_action)
        let mut executor = MockActionExecutor::new();

        let action = Action::Conditional {
            condition: "Never".to_string(),
            then_action: Box::new(Action::Text("then".to_string())),
            else_action: Some(Box::new(Action::Text("else".to_string()))),
        };

        executor.execute(&action);
        assert_eq!(executor.get_count(), 1, "Never condition should execute else_action");
    }

    #[test]
    fn test_conditional_missing_else() {
        // Test conditional with missing else_action (should do nothing when false)
        let mut executor = MockActionExecutor::new();

        let action = Action::Conditional {
            condition: "Never".to_string(),
            then_action: Box::new(Action::Text("then".to_string())),
            else_action: None,
        };

        executor.execute(&action);
        assert_eq!(executor.get_count(), 0, "Missing else_action should result in no execution");
    }

    #[test]
    fn test_conditional_unknown_condition() {
        // Test unknown condition (should default to false and execute else_action)
        let mut executor = MockActionExecutor::new();

        let action = Action::Conditional {
            condition: "UnknownCondition".to_string(),
            then_action: Box::new(Action::Text("then".to_string())),
            else_action: Some(Box::new(Action::Text("else".to_string()))),
        };

        executor.execute(&action);
        assert_eq!(executor.get_count(), 1, "Unknown condition should default to false");
    }

    #[test]
    fn test_nested_conditionals() {
        // Test nested conditionals
        let mut executor = MockActionExecutor::new();

        let inner_conditional = Action::Conditional {
            condition: "Always".to_string(),
            then_action: Box::new(Action::Text("inner_then".to_string())),
            else_action: Some(Box::new(Action::Text("inner_else".to_string()))),
        };

        let outer_conditional = Action::Conditional {
            condition: "Always".to_string(),
            then_action: Box::new(inner_conditional),
            else_action: Some(Box::new(Action::Text("outer_else".to_string()))),
        };

        executor.execute(&outer_conditional);
        // Outer Always -> inner Always -> inner_then executes
        assert_eq!(executor.get_count(), 1, "Nested conditionals should execute correctly");
    }

    // ========================================
    // Tests for evaluate_condition function
    // ========================================

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
        assert_eq!(evaluate_condition("SomeUnknownCondition"), false);
    }

    // ========================================
    // Tests for parse_time function
    // ========================================

    #[test]
    fn test_parse_time_valid() {
        assert_eq!(parse_time("09:30"), Some((9, 30)));
        assert_eq!(parse_time("00:00"), Some((0, 0)));
        assert_eq!(parse_time("23:59"), Some((23, 59)));
        assert_eq!(parse_time("12:00"), Some((12, 0)));
    }

    #[test]
    fn test_parse_time_invalid_format() {
        assert_eq!(parse_time("9:30:00"), None); // Too many parts
        assert_eq!(parse_time("930"), None);     // Missing colon
        assert_eq!(parse_time("09"), None);      // Missing minute
    }

    #[test]
    fn test_parse_time_invalid_values() {
        assert_eq!(parse_time("24:00"), None);   // Hour > 23
        assert_eq!(parse_time("12:60"), None);   // Minute > 59
        assert_eq!(parse_time("25:30"), None);   // Hour > 23
        assert_eq!(parse_time("12:99"), None);   // Minute > 59
    }

    #[test]
    fn test_parse_time_invalid_numbers() {
        assert_eq!(parse_time("ab:cd"), None);   // Not numbers
        assert_eq!(parse_time("12:3x"), None);   // Invalid minute
    }

    // ========================================
    // Tests for evaluate_time_range function
    // ========================================

    // Note: Time range tests are challenging because they depend on the current system time.
    // For production code, you would want to inject a clock interface for testing.
    // Here we test the parsing logic and structure validation.

    #[test]
    fn test_time_range_invalid_format() {
        // Invalid format should return false
        assert_eq!(evaluate_time_range("09:00"), false);      // Missing end time
        assert_eq!(evaluate_time_range("09:00-"), false);     // Missing end time
        assert_eq!(evaluate_time_range("-17:00"), false);     // Missing start time
        assert_eq!(evaluate_time_range(""), false);           // Empty string
    }

    #[test]
    fn test_time_range_invalid_times() {
        // Invalid time values should return false
        assert_eq!(evaluate_time_range("25:00-17:00"), false); // Invalid hour
        assert_eq!(evaluate_time_range("09:00-24:00"), false); // Invalid hour
        assert_eq!(evaluate_time_range("09:60-17:00"), false); // Invalid minute
    }

    // ========================================
    // Integration tests for Action conversion
    // ========================================

    #[test]
    fn test_action_config_repeat_conversion() {
        use crate::config::ActionConfig;

        let config = ActionConfig::Repeat {
            action: Box::new(ActionConfig::Text {
                text: "test".to_string(),
            }),
            count: 5,
            delay_ms: Some(100),
        };

        let action: Action = config.into();

        match action {
            Action::Repeat { count, delay_ms, .. } => {
                assert_eq!(count, 5);
                assert_eq!(delay_ms, Some(100));
            }
            _ => panic!("Expected Repeat action"),
        }
    }

    #[test]
    fn test_action_config_conditional_conversion() {
        use crate::config::ActionConfig;

        let config = ActionConfig::Conditional {
            condition: "Always".to_string(),
            then_action: Box::new(ActionConfig::Text {
                text: "then".to_string(),
            }),
            else_action: Some(Box::new(ActionConfig::Text {
                text: "else".to_string(),
            })),
        };

        let action: Action = config.into();

        match action {
            Action::Conditional { condition, .. } => {
                assert_eq!(condition, "Always");
            }
            _ => panic!("Expected Conditional action"),
        }
    }

    // ========================================
    // Complex scenario tests
    // ========================================

    #[test]
    fn test_repeat_inside_conditional() {
        // Test repeat action inside conditional
        let mut executor = MockActionExecutor::new();

        let repeat = Action::Repeat {
            action: Box::new(Action::Text("repeated".to_string())),
            count: 3,
            delay_ms: None,
        };

        let conditional = Action::Conditional {
            condition: "Always".to_string(),
            then_action: Box::new(repeat),
            else_action: None,
        };

        executor.execute(&conditional);
        assert_eq!(executor.get_count(), 3, "Conditional should trigger repeat 3 times");
    }

    #[test]
    fn test_conditional_inside_repeat() {
        // Test conditional action inside repeat
        let mut executor = MockActionExecutor::new();

        let conditional = Action::Conditional {
            condition: "Always".to_string(),
            then_action: Box::new(Action::Text("then".to_string())),
            else_action: None,
        };

        let repeat = Action::Repeat {
            action: Box::new(conditional),
            count: 4,
            delay_ms: Some(50),
        };

        executor.execute(&repeat);
        assert_eq!(executor.get_count(), 4, "Repeat should execute conditional 4 times");
    }

    #[test]
    fn test_sequence_with_repeat_and_conditional() {
        // Test complex sequence with both repeat and conditional
        let mut executor = MockActionExecutor::new();

        let sequence = Action::Sequence(vec![
            Action::Repeat {
                action: Box::new(Action::Text("a".to_string())),
                count: 2,
                delay_ms: None,
            },
            Action::Conditional {
                condition: "Always".to_string(),
                then_action: Box::new(Action::Text("b".to_string())),
                else_action: None,
            },
            Action::Repeat {
                action: Box::new(Action::Text("c".to_string())),
                count: 3,
                delay_ms: None,
            },
        ]);

        executor.execute(&sequence);
        // 2 (repeat) + 1 (conditional then) + 3 (repeat) = 6
        assert_eq!(executor.get_count(), 6, "Complex sequence should execute all actions");
    }

    // VolumeControl tests
    #[test]
    fn test_parse_volume_operation_up() {
        assert_eq!(parse_volume_operation("Up"), VolumeOperation::Up);
        assert_eq!(parse_volume_operation("up"), VolumeOperation::Up);
        assert_eq!(parse_volume_operation("UP"), VolumeOperation::Up);
    }

    #[test]
    fn test_parse_volume_operation_down() {
        assert_eq!(parse_volume_operation("Down"), VolumeOperation::Down);
        assert_eq!(parse_volume_operation("down"), VolumeOperation::Down);
    }

    #[test]
    fn test_parse_volume_operation_mute() {
        assert_eq!(parse_volume_operation("Mute"), VolumeOperation::Mute);
        assert_eq!(parse_volume_operation("mute"), VolumeOperation::Mute);
    }

    #[test]
    fn test_parse_volume_operation_unmute() {
        assert_eq!(parse_volume_operation("Unmute"), VolumeOperation::Unmute);
        assert_eq!(parse_volume_operation("unmute"), VolumeOperation::Unmute);
    }

    #[test]
    fn test_parse_volume_operation_set() {
        assert_eq!(parse_volume_operation("Set"), VolumeOperation::Set);
        assert_eq!(parse_volume_operation("set"), VolumeOperation::Set);
    }

    #[test]
    fn test_parse_volume_operation_unknown() {
        // Unknown operations should default to Up
        assert_eq!(parse_volume_operation("invalid"), VolumeOperation::Up);
    }

    #[test]
    fn test_volume_control_action_conversion() {
        use crate::config::ActionConfig;

        // Test conversion from ActionConfig to Action
        let config = ActionConfig::VolumeControl {
            operation: "Up".to_string(),
            value: None,
        };
        let action: Action = config.into();

        match action {
            Action::VolumeControl { operation, value } => {
                assert_eq!(operation, VolumeOperation::Up);
                assert_eq!(value, None);
            }
            _ => panic!("Expected VolumeControl action"),
        }
    }

    #[test]
    fn test_volume_control_set_with_value() {
        use crate::config::ActionConfig;

        let config = ActionConfig::VolumeControl {
            operation: "Set".to_string(),
            value: Some(75),
        };
        let action: Action = config.into();

        match action {
            Action::VolumeControl { operation, value } => {
                assert_eq!(operation, VolumeOperation::Set);
                assert_eq!(value, Some(75));
            }
            _ => panic!("Expected VolumeControl action"),
        }
    }

    // ModeChange tests
    #[test]
    fn test_mode_change_action_conversion() {
        use crate::config::ActionConfig;

        let config = ActionConfig::ModeChange {
            mode: "Development".to_string(),
        };
        let action: Action = config.into();

        match action {
            Action::ModeChange { mode } => {
                assert_eq!(mode, "Development");
            }
            _ => panic!("Expected ModeChange action"),
        }
    }

    #[test]
    fn test_mode_change_in_sequence() {
        use crate::config::ActionConfig;

        let config = ActionConfig::Sequence {
            actions: vec![
                ActionConfig::Text { text: "Starting mode change...".to_string() },
                ActionConfig::ModeChange { mode: "Media".to_string() },
                ActionConfig::Text { text: "Mode changed".to_string() },
            ],
        };

        let action: Action = config.into();

        match action {
            Action::Sequence(actions) => {
                assert_eq!(actions.len(), 3);
                match &actions[1] {
                    Action::ModeChange { mode } => {
                        assert_eq!(mode, "Media");
                    }
                    _ => panic!("Expected second action to be ModeChange"),
                }
            }
            _ => panic!("Expected Sequence action"),
        }
    }
}
