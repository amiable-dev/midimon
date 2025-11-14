// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! Application state management

use std::sync::Arc;
use tokio::sync::RwLock;
use crate::midi_learn::MidiLearnSession;
use crate::app_detection::{AppDetector, AppInfo};
use crate::profile_manager::ProfileManager;

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
    /// App detector for frontmost app monitoring
    app_detector: Arc<AppDetector>,
    /// Profile manager for per-app configs
    profile_manager: Arc<ProfileManager>,
}

impl AppState {
    /// Create a new application state
    pub fn new() -> Self {
        let profile_manager = ProfileManager::new()
            .expect("Failed to create profile manager");

        Self {
            inner: Arc::new(RwLock::new(AppStateInner {
                daemon_connected: false,
                learn_session: None,
                app_detector: Arc::new(AppDetector::new()),
                profile_manager: Arc::new(profile_manager),
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

    /// Get the current frontmost app
    pub async fn get_current_app(&self) -> Option<AppInfo> {
        let inner = self.inner.read().await;
        inner.app_detector.get_current_app().await
    }

    /// Start app detection with change notifications
    pub async fn start_app_detection(&self) {
        let inner = self.inner.read().await;
        let detector = Arc::clone(&inner.app_detector);

        detector.start_detection(move |_app_info| {
            // App change callback
            // Could emit Tauri events here for frontend notification
            // For now, just store in detector state
        }).await;
    }

    /// Stop app detection
    pub async fn stop_app_detection(&self) {
        let inner = self.inner.read().await;
        inner.app_detector.stop_detection().await;
    }

    /// Get profile manager
    pub async fn get_profile_manager(&self) -> Arc<ProfileManager> {
        let inner = self.inner.read().await;
        Arc::clone(&inner.profile_manager)
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
