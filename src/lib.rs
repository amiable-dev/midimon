// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! MIDIMon Library
//!
//! This library exposes core types and functionality for testing
//! and future modularization into midimon-core.

pub mod config;
pub mod event_processor;
pub mod actions;
pub mod mappings;
pub mod feedback;
pub mod device_profile;
pub mod mikro_leds;
pub mod midi_feedback;

// Re-export commonly used types for convenience
pub use config::{ActionConfig, Config, DeviceConfig, Mapping, Mode, Trigger};
pub use event_processor::{
    EncoderDirection, EventProcessor, MidiEvent, ProcessedEvent, VelocityLevel,
};
