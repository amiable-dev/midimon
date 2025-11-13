// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! Configuration types for MIDIMon.
//!
//! This module defines the data structures used to represent MIDI mappings,
//! triggers, and actions in the configuration file.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Top-level configuration structure
///
/// Contains device settings, mode definitions, global mappings, and logging configuration.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    /// Device configuration (name, connection settings)
    pub device: DeviceConfig,
    /// List of mapping modes (each with its own set of mappings)
    pub modes: Vec<Mode>,
    /// Global mappings that work in all modes (applied before mode-specific mappings)
    #[serde(default)]
    pub global_mappings: Vec<Mapping>,
    /// Logging configuration
    #[serde(default)]
    pub logging: Option<LoggingConfig>,
    /// Advanced settings for event processing
    #[serde(default)]
    pub advanced_settings: AdvancedSettings,
}

/// Logging configuration
///
/// Defines how the application should log diagnostic information.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LoggingConfig {
    /// Log level: "off", "error", "warn", "info", "debug", "trace"
    #[serde(default = "default_log_level")]
    pub level: String,
    /// Enable file logging
    #[serde(default)]
    pub file: Option<String>,
}

fn default_log_level() -> String {
    "info".to_string()
}

/// Advanced settings for event processing and timing
///
/// Fine-tunes behavior of event detection algorithms.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdvancedSettings {
    /// Time window in milliseconds for chord detection (default: 50ms)
    #[serde(default = "default_chord_timeout_ms")]
    pub chord_timeout_ms: u64,
    /// Time window in milliseconds for double-tap detection (default: 300ms)
    #[serde(default = "default_double_tap_timeout_ms")]
    pub double_tap_timeout_ms: u64,
    /// Hold threshold in milliseconds for long press detection (default: 2000ms)
    #[serde(default = "default_hold_threshold_ms")]
    pub hold_threshold_ms: u64,
}

fn default_chord_timeout_ms() -> u64 {
    50
}

fn default_double_tap_timeout_ms() -> u64 {
    300
}

fn default_hold_threshold_ms() -> u64 {
    2000
}

/// Device-specific configuration
///
/// Defines the MIDI device to connect to and connection preferences.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DeviceConfig {
    /// Human-readable device name
    pub name: String,
    /// Whether to automatically connect to the first available device matching this name
    pub auto_connect: bool,
}

/// A mode defines a set of mappings that can be switched between at runtime
///
/// Each mode has its own mapping set and optional visual identifier (color).
/// Users can switch between modes using special triggers (e.g., encoder rotation).
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Mode {
    /// Mode name (used for mode switching triggers)
    pub name: String,
    /// Optional color for visual identification (e.g., "blue", "green", "#FF0000")
    pub color: Option<String>,
    /// Mappings active only in this mode
    #[serde(default)]
    pub mappings: Vec<Mapping>,
}

/// A mapping connects a MIDI trigger to an action
///
/// When a trigger is detected, the associated action is executed.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Mapping {
    /// The MIDI trigger that activates this mapping
    pub trigger: Trigger,
    /// The action to execute when the trigger is detected
    pub action: ActionConfig,
    /// Optional human-readable description of this mapping
    pub description: Option<String>,
}

/// MIDI trigger types
///
/// Defines different ways a MIDI message can activate a mapping.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum Trigger {
    /// Basic note trigger with optional velocity threshold
    ///
    /// # Examples
    /// ```toml
    /// [trigger]
    /// type = "Note"
    /// note = 60
    /// velocity_min = 1
    /// ```
    Note {
        /// MIDI note number (0-127)
        note: u8,
        /// Minimum velocity to trigger (0-127), None = any velocity
        velocity_min: Option<u8>,
    },

    /// Velocity-sensitive trigger with different actions per velocity level
    ///
    /// Classifies note presses into soft, medium, and hard based on velocity thresholds.
    /// Used with `VelocityRange` action type for velocity-dependent behavior.
    VelocityRange {
        /// MIDI note number (0-127)
        note: u8,
        /// Maximum velocity for soft (default 40), velocities below this are soft
        soft_max: Option<u8>,
        /// Maximum velocity for medium (default 80), velocities below this are medium (after soft_max)
        medium_max: Option<u8>,
    },

    /// Long press detection (hold threshold in ms)
    ///
    /// Triggers when a note is held for longer than the specified duration.
    LongPress {
        /// MIDI note number (0-127)
        note: u8,
        /// Duration in milliseconds to trigger long press (default 2000ms)
        duration_ms: Option<u64>,
    },

    /// Double-tap detection
    ///
    /// Triggers when a note is pressed and released quickly twice within a time window.
    DoubleTap {
        /// MIDI note number (0-127)
        note: u8,
        /// Time window in milliseconds for detecting double-tap (default 300ms)
        timeout_ms: Option<u64>,
    },

    /// Chord detection (multiple notes pressed simultaneously)
    ///
    /// Triggers when all specified notes are pressed within a narrow time window.
    NoteChord {
        /// List of MIDI note numbers that form this chord
        notes: Vec<u8>,
        /// Time window in milliseconds for detecting simultaneous presses (default 50ms)
        timeout_ms: Option<u64>,
    },

    /// Encoder turn with direction
    ///
    /// Triggers on continuous controller (CC) messages from encoder/knob rotation.
    /// Can filter by direction (clockwise/counter-clockwise) or respond to both.
    EncoderTurn {
        /// Control Change number (0-127)
        cc: u8,
        /// Direction filter: "Clockwise", "CounterClockwise", or None for either
        direction: Option<String>,
    },

    /// Aftertouch/pressure sensitivity
    ///
    /// Triggers based on channel pressure (aftertouch) values.
    Aftertouch {
        /// Minimum pressure value to trigger (0-127)
        pressure_min: Option<u8>,
    },

    /// Pitch bend
    ///
    /// Triggers based on pitch bend messages from touch strips or pitch bend wheels.
    PitchBend {
        /// Minimum value range (0-16383)
        value_min: Option<u16>,
        /// Maximum value range (0-16383)
        value_max: Option<u16>,
    },

    /// Control Change (generic CC)
    ///
    /// Triggers on any control change message matching the specified CC number.
    CC {
        /// Control Change number (0-127)
        cc: u8,
        /// Minimum value to trigger (0-127)
        value_min: Option<u8>,
    },
}

/// Action configuration types
///
/// Defines different actions that can be executed when a trigger is detected.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum ActionConfig {
    /// Simulate keyboard keystroke(s) with optional modifiers
    ///
    /// # Examples
    /// ```toml
    /// [action]
    /// type = "Keystroke"
    /// keys = "space"
    /// modifiers = ["cmd"]
    /// ```
    Keystroke {
        /// Key name or sequence (e.g., "space", "Return", "Escape")
        keys: String,
        /// Modifier keys (e.g., "cmd", "shift", "alt", "ctrl")
        #[serde(default)]
        modifiers: Vec<String>,
    },

    /// Type a text string
    ///
    /// Simulates typing the provided text character by character.
    Text {
        /// Text to type
        text: String,
    },

    /// Launch an application
    ///
    /// Attempts to open the specified application by name or path.
    Launch {
        /// Application name or path to executable
        app: String,
    },

    /// Execute a shell command
    ///
    /// Runs an arbitrary shell command. Be cautious with untrusted config files.
    Shell {
        /// Shell command to execute
        command: String,
    },

    /// Execute a sequence of actions in order
    ///
    /// Executes multiple actions sequentially (useful for complex behaviors).
    Sequence {
        /// List of actions to execute in order
        actions: Vec<ActionConfig>,
    },

    /// Delay for a specified duration (in milliseconds)
    ///
    /// Pauses execution for the given duration. Useful in sequences.
    Delay {
        /// Delay duration in milliseconds
        ms: u64,
    },

    /// Simulate mouse click
    ///
    /// Clicks at the current or specified location with the specified button.
    MouseClick {
        /// Mouse button: "left", "right", "middle"
        button: String,
        /// X coordinate (optional, uses current mouse position if not specified)
        x: Option<i32>,
        /// Y coordinate (optional, uses current mouse position if not specified)
        y: Option<i32>,
    },

    /// Control system volume
    ///
    /// Adjusts or sets the system volume.
    VolumeControl {
        /// Operation: "Up", "Down", "Mute", "Unmute", "Set"
        operation: String,
        /// Volume level (0-100) for "Set" operation
        #[serde(default)]
        value: Option<u8>,
    },

    /// Switch to a different mode
    ///
    /// Changes the active mapping mode by name.
    ModeChange {
        /// Name of the mode to switch to
        mode: String,
    },

    /// Repeat an action multiple times
    ///
    /// Executes the specified action the given number of times.
    Repeat {
        /// Action to repeat
        action: Box<ActionConfig>,
        /// Number of times to repeat
        count: usize,
    },

    /// Conditional action execution
    ///
    /// Executes different actions based on a condition.
    Conditional {
        /// Condition to evaluate (format depends on condition type)
        condition: String,
        /// Action to execute if condition is true
        then_action: Box<ActionConfig>,
        /// Optional action to execute if condition is false
        #[serde(default)]
        else_action: Option<Box<ActionConfig>>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_deserialize() {
        let toml_str = r#"
[device]
name = "Test Device"
auto_connect = true

[[modes]]
name = "Default"
color = "blue"

[[modes.mappings]]
description = "Test mapping"

[modes.mappings.trigger]
type = "Note"
note = 60
velocity_min = 1

[modes.mappings.action]
type = "Keystroke"
keys = "space"
modifiers = ["cmd"]
"#;

        let config: Config = toml::from_str(toml_str).expect("Failed to parse config");
        assert_eq!(config.device.name, "Test Device");
        assert_eq!(config.modes.len(), 1);
        assert_eq!(config.modes[0].name, "Default");
    }

    #[test]
    fn test_trigger_note() {
        let trigger = Trigger::Note {
            note: 60,
            velocity_min: Some(1),
        };
        assert!(matches!(trigger, Trigger::Note { note: 60, .. }));
    }

    #[test]
    fn test_action_keystroke() {
        let action = ActionConfig::Keystroke {
            keys: "space".to_string(),
            modifiers: vec!["cmd".to_string()],
        };
        assert!(matches!(action, ActionConfig::Keystroke { .. }));
    }
}
