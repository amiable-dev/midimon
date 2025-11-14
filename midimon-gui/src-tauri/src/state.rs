// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! Application state management

use std::sync::Arc;
use tokio::sync::RwLock;

/// Global application state
pub struct AppState {
    /// Inner state behind Arc<RwLock<>> for thread-safe access
    inner: Arc<RwLock<AppStateInner>>,
}

struct AppStateInner {
    /// Whether the daemon is currently connected
    daemon_connected: bool,
}

impl AppState {
    /// Create a new application state
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(AppStateInner {
                daemon_connected: false,
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
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
