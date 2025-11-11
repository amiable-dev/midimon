// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! MIDIMon Library
//!
//! This library exposes core types and functionality for testing
//! and future modularization into midimon-core.

pub mod config;

// Re-export commonly used types for convenience
pub use config::{ActionConfig, Config, DeviceConfig, Mapping, Mode, Trigger};
