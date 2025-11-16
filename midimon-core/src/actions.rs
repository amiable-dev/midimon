// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! Action types and parsing for MIDIMon core engine.
//!
//! This module defines the Action enum and parsing logic for converting ActionConfig
//! to Action. Action execution has been moved to midimon-daemon (Phase 2 refactor).

use crate::config::ActionConfig;
use enigo::{Button, Key};

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

// ActionExecutor has been moved to midimon-daemon (Phase 2 refactor)
// Only pure Action types and parsing remain in core

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

// Condition evaluation and volume control moved to midimon-daemon/action_executor.rs

#[cfg(test)]
mod tests {
    use super::*;

    // NOTE: Action execution tests have been moved to midimon-daemon/action_executor.rs
    // These tests now only cover parsing and conversion from ActionConfig to Action

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

    #[test]
    fn test_parse_volume_operation() {
        assert_eq!(parse_volume_operation("Up"), VolumeOperation::Up);
        assert_eq!(parse_volume_operation("up"), VolumeOperation::Up);
        assert_eq!(parse_volume_operation("Down"), VolumeOperation::Down);
        assert_eq!(parse_volume_operation("Mute"), VolumeOperation::Mute);
        assert_eq!(parse_volume_operation("Unmute"), VolumeOperation::Unmute);
        assert_eq!(parse_volume_operation("Set"), VolumeOperation::Set);
        // Unknown operations default to Up
        assert_eq!(parse_volume_operation("invalid"), VolumeOperation::Up);
    }

    #[test]
    fn test_volume_control_action_conversion() {
        use crate::config::ActionConfig;

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
}
