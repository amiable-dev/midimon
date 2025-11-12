// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! Event type definitions
//!
//! NOTE: Currently these types are still defined in event_processor.rs
//! They will be moved here during the refactoring phase.
//! For now, this module re-exports them from event_processor for the public API.

pub use crate::event_processor::{EncoderDirection, MidiEvent, ProcessedEvent, VelocityLevel};
