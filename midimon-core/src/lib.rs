// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! MIDIMon Core Engine
//!
//! Pure Rust MIDI mapping engine with zero UI dependencies.
//! This library provides the core functionality for processing MIDI events,
//! mapping them to actions, and executing those actions.

#![allow(dead_code, unused_variables, unused_imports)]

// Public modules
pub mod config;
pub mod engine;
pub mod events;
pub mod actions;
pub mod mapping;
pub mod feedback;
pub mod device;
pub mod error;

// Private modules (implementation details)
mod event_processor;
mod mikro_leds;
mod midi_feedback;

// Re-exports for convenience
pub use engine::MidiMonEngine;
pub use config::Config;
pub use events::{MidiEvent, ProcessedEvent, VelocityLevel, EncoderDirection};
pub use error::{EngineError, ConfigError, ActionError, FeedbackError, ProfileError};

// Note: actions and feedback re-exports will be added once we verify the types exist
// pub use actions::{Action, VolumeAction};
// pub use feedback::{FeedbackController, RGB, LightingScheme};
// pub use device::{DeviceProfile, PadPageMapping};
