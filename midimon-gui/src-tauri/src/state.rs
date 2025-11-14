// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! Application state management

use std::sync::Arc;
use tokio::sync::RwLock;
use crate::midi_learn::MidiLearnSession;

/// Global application state
pub struct AppState {
    /// Inner state behind Arc<RwLock<>> for thread-safe access
    inner: Arc<RwLock<AppStateInner>>,
}

struct AppStateInner {
    /// Whether the daemon is currently connected
    daemon_connected: bool,
    /// Current MIDI Learn session (if active)
    learn_session: Option<MidiLearnSession>,
}

impl AppState {
    /// Create a new application state
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(AppStateInner {
                daemon_connected: false,
                learn_session: None,
            })),
        }
    }

    /// Check if the daemon is connected
    pub async fn is_daemon_connected(&self) -> bool {
        self.inner.read().await.daemon_connected
    }

    /// Set the daemon connection status
    pub async fn set_daemon_connected(&self, connected: bool) {
        self.inner.write().await.daemon_connected = connected;
    }

    /// Set the current MIDI Learn session
    pub async fn set_learn_session(&self, session: MidiLearnSession) {
        self.inner.write().await.learn_session = Some(session);
    }

    /// Get the current MIDI Learn session
    pub async fn get_learn_session(&self) -> Option<MidiLearnSession> {
        self.inner.read().await.learn_session.clone()
    }

    /// Clear the MIDI Learn session
    pub async fn clear_learn_session(&self) {
        self.inner.write().await.learn_session = None;
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
