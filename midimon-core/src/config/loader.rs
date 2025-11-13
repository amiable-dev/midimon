// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! Configuration loading, saving, and validation.
//!
//! This module provides functionality to load configuration from files,
//! save configuration to files, and validate configuration correctness.

use crate::error::ConfigError;
use std::collections::HashSet;
use std::path::Path;

use super::types::{ActionConfig, Config, DeviceConfig, Mapping, Mode, Trigger};

impl Config {
    /// Load configuration from a TOML file
    ///
    /// If the file doesn't exist, creates a default configuration and saves it to the specified path.
    ///
    /// # Arguments
    /// * `path` - Path to the configuration file
    ///
    /// # Returns
    /// * `Ok(Config)` - Successfully loaded or created configuration
    /// * `Err(ConfigError)` - IO, parsing, or validation error
    ///
    /// # Example
    /// ```no_run
    /// use midimon_core::Config;
    ///
    /// let config = Config::load("config.toml")?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        if Path::new(path).exists() {
            let contents = std::fs::read_to_string(path)?;
            let config: Config = toml::from_str(&contents)?;
            config.validate()?;
            Ok(config)
        } else {
            println!("Config file not found, creating default config...");
            let config = Self::default_config();
            config.save(path)?;
            Ok(config)
        }
    }

    /// Save configuration to a TOML file
    ///
    /// Writes the configuration as formatted TOML.
    ///
    /// # Arguments
    /// * `path` - Path where the configuration file will be written
    ///
    /// # Returns
    /// * `Ok(())` - Successfully saved
    /// * `Err(Box<dyn std::error::Error>)` - IO or serialization error
    ///
    /// # Example
    /// ```no_run
    /// use midimon_core::Config;
    ///
    /// let config = Config::load("config.toml")?;
    /// config.save("backup.toml")?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn save(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let contents = toml::to_string_pretty(self)?;
        std::fs::write(path, contents)?;
        Ok(())
    }

    /// Create a default configuration
    ///
    /// Generates a default configuration with sample modes and mappings.
    /// This is used when no configuration file exists.
    ///
    /// # Returns
    /// Default configuration with:
    /// - Device name: "Mikro"
    /// - Two modes: "Default" and "Development"
    /// - Sample mappings for each mode
    pub fn default_config() -> Self {
        Config {
            device: DeviceConfig {
                name: "Mikro".to_string(),
                auto_connect: true,
            },
            modes: vec![
                Mode {
                    name: "Default".to_string(),
                    color: Some("blue".to_string()),
                    mappings: vec![Mapping {
                        trigger: Trigger::Note {
                            note: 60,
                            velocity_min: Some(1),
                        },
                        action: ActionConfig::Keystroke {
                            keys: "space".to_string(),
                            modifiers: vec!["cmd".to_string()],
                        },
                        description: Some("Spotlight Search".to_string()),
                    }],
                },
                Mode {
                    name: "Development".to_string(),
                    color: Some("green".to_string()),
                    mappings: vec![Mapping {
                        trigger: Trigger::Note {
                            note: 60,
                            velocity_min: None,
                        },
                        action: ActionConfig::Shell {
                            command: "git status".to_string(),
                        },
                        description: Some("Git status".to_string()),
                    }],
                },
            ],
            global_mappings: vec![],
            logging: None,
            advanced_settings: Default::default(),
        }
    }

    /// Validate the configuration for correctness
    ///
    /// Checks for:
    /// - Unique mode names (required for mode switching)
    /// - Valid trigger types (structural validation)
    /// - Valid action types (structural validation)
    ///
    /// Note: This validation ensures basic correctness. More detailed validation
    /// (e.g., valid key names, valid command syntax) happens at execution time.
    ///
    /// # Returns
    /// * `Ok(())` - Configuration is valid
    /// * `Err(ConfigError::ValidationError)` - Configuration has issues
    pub fn validate(&self) -> Result<(), ConfigError> {
        // Validate unique mode names
        let mut mode_names = HashSet::new();
        for mode in &self.modes {
            if !mode_names.insert(&mode.name) {
                return Err(ConfigError::ValidationError(format!(
                    "Duplicate mode name: '{}'",
                    mode.name
                )));
            }
        }

        // Validate all mappings (both global and mode-specific)
        for mapping in &self.global_mappings {
            validate_mapping(mapping)?;
        }

        for mode in &self.modes {
            for mapping in &mode.mappings {
                validate_mapping(mapping)?;
            }
        }

        Ok(())
    }
}

/// Validate a single mapping
fn validate_mapping(mapping: &Mapping) -> Result<(), ConfigError> {
    validate_trigger(&mapping.trigger)?;
    validate_action(&mapping.action)?;
    Ok(())
}

/// Validate a trigger configuration
fn validate_trigger(trigger: &Trigger) -> Result<(), ConfigError> {
    match trigger {
        Trigger::Note { note, .. } => {
            if *note > 127 {
                return Err(ConfigError::InvalidTrigger(format!(
                    "Note number out of range: {} (must be 0-127)",
                    note
                )));
            }
        }
        Trigger::VelocityRange { note, .. } => {
            if *note > 127 {
                return Err(ConfigError::InvalidTrigger(format!(
                    "Note number out of range: {} (must be 0-127)",
                    note
                )));
            }
        }
        Trigger::LongPress { note, .. } => {
            if *note > 127 {
                return Err(ConfigError::InvalidTrigger(format!(
                    "Note number out of range: {} (must be 0-127)",
                    note
                )));
            }
        }
        Trigger::DoubleTap { note, .. } => {
            if *note > 127 {
                return Err(ConfigError::InvalidTrigger(format!(
                    "Note number out of range: {} (must be 0-127)",
                    note
                )));
            }
        }
        Trigger::NoteChord { notes, .. } => {
            for note in notes {
                if *note > 127 {
                    return Err(ConfigError::InvalidTrigger(format!(
                        "Note number out of range: {} (must be 0-127)",
                        note
                    )));
                }
            }
            if notes.is_empty() {
                return Err(ConfigError::InvalidTrigger(
                    "NoteChord must have at least one note".to_string(),
                ));
            }
        }
        Trigger::EncoderTurn { cc, direction } => {
            if *cc > 127 {
                return Err(ConfigError::InvalidTrigger(format!(
                    "CC number out of range: {} (must be 0-127)",
                    cc
                )));
            }
            if let Some(dir) = direction
                && dir != "Clockwise"
                && dir != "CounterClockwise"
            {
                return Err(ConfigError::InvalidTrigger(format!(
                    "Invalid direction: '{}' (must be 'Clockwise' or 'CounterClockwise')",
                    dir
                )));
            }
        }
        Trigger::CC { cc, .. } => {
            if *cc > 127 {
                return Err(ConfigError::InvalidTrigger(format!(
                    "CC number out of range: {} (must be 0-127)",
                    cc
                )));
            }
        }
        Trigger::Aftertouch { .. } => {
            // Valid trigger, no specific validation needed
        }
        Trigger::PitchBend { .. } => {
            // Valid trigger, range validation is optional as values can be None
        }
    }
    Ok(())
}

/// Validate an action configuration
fn validate_action(action: &ActionConfig) -> Result<(), ConfigError> {
    match action {
        ActionConfig::Keystroke { keys, modifiers } => {
            if keys.is_empty() {
                return Err(ConfigError::InvalidAction(
                    "Keystroke requires keys".to_string(),
                ));
            }
            // Validate modifiers are known
            let valid_modifiers = ["cmd", "shift", "alt", "ctrl", "fn"];
            for modifier in modifiers {
                if !valid_modifiers.contains(&modifier.as_str()) {
                    return Err(ConfigError::InvalidAction(format!(
                        "Unknown modifier: '{}'. Valid modifiers: {}",
                        modifier,
                        valid_modifiers.join(", ")
                    )));
                }
            }
        }
        ActionConfig::Text { text } => {
            if text.is_empty() {
                return Err(ConfigError::InvalidAction(
                    "Text action requires text".to_string(),
                ));
            }
        }
        ActionConfig::Launch { app } => {
            if app.is_empty() {
                return Err(ConfigError::InvalidAction(
                    "Launch action requires app name".to_string(),
                ));
            }
        }
        ActionConfig::Shell { command } => {
            if command.is_empty() {
                return Err(ConfigError::InvalidAction(
                    "Shell action requires command".to_string(),
                ));
            }
        }
        ActionConfig::Sequence { actions } => {
            if actions.is_empty() {
                return Err(ConfigError::InvalidAction(
                    "Sequence requires at least one action".to_string(),
                ));
            }
            for action in actions {
                validate_action(action)?;
            }
        }
        ActionConfig::Delay { ms } => {
            if *ms == 0 {
                return Err(ConfigError::InvalidAction(
                    "Delay must be > 0 ms".to_string(),
                ));
            }
        }
        ActionConfig::MouseClick { button, .. } => {
            let valid_buttons = ["left", "right", "middle"];
            if !valid_buttons.contains(&button.as_str()) {
                return Err(ConfigError::InvalidAction(format!(
                    "Invalid mouse button: '{}'. Valid buttons: {}",
                    button,
                    valid_buttons.join(", ")
                )));
            }
        }
        ActionConfig::VolumeControl { operation, value } => {
            let valid_ops = ["Up", "Down", "Mute", "Unmute", "Set"];
            if !valid_ops.contains(&operation.as_str()) {
                return Err(ConfigError::InvalidAction(format!(
                    "Invalid volume operation: '{}'. Valid operations: {}",
                    operation,
                    valid_ops.join(", ")
                )));
            }
            if operation == "Set" && value.is_none() {
                return Err(ConfigError::InvalidAction(
                    "VolumeControl Set operation requires value".to_string(),
                ));
            }
        }
        ActionConfig::ModeChange { mode } => {
            if mode.is_empty() {
                return Err(ConfigError::InvalidAction(
                    "ModeChange requires mode name".to_string(),
                ));
            }
        }
        ActionConfig::Repeat { action, count } => {
            if *count == 0 {
                return Err(ConfigError::InvalidAction(
                    "Repeat count must be > 0".to_string(),
                ));
            }
            validate_action(action)?;
        }
        ActionConfig::Conditional {
            then_action,
            else_action,
            ..
        } => {
            validate_action(then_action)?;
            if let Some(else_act) = else_action {
                validate_action(else_act)?;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = Config::default_config();
        assert_eq!(config.device.name, "Mikro");
        assert_eq!(config.modes.len(), 2);
        assert_eq!(config.modes[0].name, "Default");
        assert_eq!(config.modes[1].name, "Development");
    }

    #[test]
    fn test_validate_duplicate_mode_names() {
        let mut config = Config::default_config();
        config.modes.push(Mode {
            name: "Default".to_string(),
            color: None,
            mappings: vec![],
        });

        let result = config.validate();
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Duplicate mode name")
        );
    }

    #[test]
    fn test_validate_invalid_note_number() {
        let mut config = Config::default_config();
        config.modes[0].mappings[0].trigger = Trigger::Note {
            note: 128,
            velocity_min: None,
        };

        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("out of range"));
    }

    #[test]
    fn test_validate_invalid_modifier() {
        let mut config = Config::default_config();
        config.modes[0].mappings[0].action = ActionConfig::Keystroke {
            keys: "a".to_string(),
            modifiers: vec!["invalid_mod".to_string()],
        };

        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Unknown modifier"));
    }

    #[test]
    fn test_validate_invalid_direction() {
        let mut config = Config::default_config();
        config.modes[0].mappings[0].trigger = Trigger::EncoderTurn {
            cc: 1,
            direction: Some("Invalid".to_string()),
        };

        let result = config.validate();
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Invalid direction")
        );
    }

    #[test]
    fn test_validate_empty_keystroke_keys() {
        let mut config = Config::default_config();
        config.modes[0].mappings[0].action = ActionConfig::Keystroke {
            keys: String::new(),
            modifiers: vec![],
        };

        let result = config.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_sequence_with_empty_actions() {
        let mut config = Config::default_config();
        config.modes[0].mappings[0].action = ActionConfig::Sequence { actions: vec![] };

        let result = config.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_valid_config() {
        let config = Config::default_config();
        let result = config.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_encoder_direction_clockwise() {
        let mut config = Config::default_config();
        config.modes[0].mappings[0].trigger = Trigger::EncoderTurn {
            cc: 1,
            direction: Some("Clockwise".to_string()),
        };

        let result = config.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_encoder_direction_counter_clockwise() {
        let mut config = Config::default_config();
        config.modes[0].mappings[0].trigger = Trigger::EncoderTurn {
            cc: 1,
            direction: Some("CounterClockwise".to_string()),
        };

        let result = config.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_note_chord_with_empty_notes() {
        let mut config = Config::default_config();
        config.modes[0].mappings[0].trigger = Trigger::NoteChord {
            notes: vec![],
            timeout_ms: None,
        };

        let result = config.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_invalid_mouse_button() {
        let mut config = Config::default_config();
        config.modes[0].mappings[0].action = ActionConfig::MouseClick {
            button: "invalid".to_string(),
            x: None,
            y: None,
        };

        let result = config.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_volume_control_set_without_value() {
        let mut config = Config::default_config();
        config.modes[0].mappings[0].action = ActionConfig::VolumeControl {
            operation: "Set".to_string(),
            value: None,
        };

        let result = config.validate();
        assert!(result.is_err());
    }
}
