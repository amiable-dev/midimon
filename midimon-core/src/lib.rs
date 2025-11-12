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
pub mod event_processor;  // Public for advanced event processing

// Private modules (implementation details)
mod mikro_leds;
mod midi_feedback;

// Re-exports for convenience

// Engine
pub use engine::MidiMonEngine;

// Configuration
pub use config::{Config, DeviceConfig, Mode, Mapping, Trigger, ActionConfig};

// Events
pub use events::{MidiEvent, ProcessedEvent, VelocityLevel, EncoderDirection};
pub use event_processor::EventProcessor;

// Actions
pub use actions::{Action, ActionExecutor};

// Feedback
pub use feedback::{PadFeedback, LightingScheme};

// Device Profiles
pub use device::{DeviceProfile, PadPageMapping};

// Errors
pub use error::{EngineError, ConfigError, ActionError, FeedbackError, ProfileError};

// Mapping
pub use mapping::MappingEngine;
