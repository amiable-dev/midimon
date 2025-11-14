// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! Event types for communication between backend and frontend

use serde::{Deserialize, Serialize};

/// Events emitted from backend to frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum AppEvent {
    /// Daemon status changed
    DaemonStatusChanged {
        running: bool,
        connected: bool,
    },

    /// MIDI event received (for MIDI Learn mode)
    MidiEventReceived {
        note: u8,
        velocity: u8,
        channel: u8,
    },

    /// Configuration reloaded
    ConfigReloaded {
        success: bool,
        error: Option<String>,
    },

    /// Error occurred
    Error {
        message: String,
    },
}
