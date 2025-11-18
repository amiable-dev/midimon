// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! OBS Studio integration plugin for MIDIMon
//!
//! This plugin provides actions for controlling OBS Studio through the OBS WebSocket protocol.
//! Supports scene switching, source visibility, recording, streaming, and more.

use midimon_core::plugin::{
    ActionPlugin, PluginCapability, PluginError, PluginInfo, PluginMetadata, PluginResult,
};
use obws::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

/// OBS action types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ObsAction {
    /// Switch to a specific scene
    SwitchScene { scene_name: String },
    /// Toggle scene visibility
    ToggleScene { scene_name: String },
    /// Start/Stop recording
    ToggleRecording,
    /// Start recording
    StartRecording,
    /// Stop recording
    StopRecording,
    /// Pause/Resume recording
    PauseRecording,
    /// Start/Stop streaming
    ToggleStreaming,
    /// Start streaming
    StartStreaming,
    /// Stop streaming
    StopStreaming,
    /// Toggle source visibility
    ToggleSource {
        scene_name: Option<String>,
        source_name: String,
    },
    /// Set source visibility
    SetSourceVisibility {
        scene_name: Option<String>,
        source_name: String,
        visible: bool,
    },
    /// Toggle mute for audio source
    ToggleMute { source_name: String },
    /// Set audio source volume (0.0 - 1.0)
    SetVolume { source_name: String, volume: f32 },
    /// Trigger a hotkey by name
    TriggerHotkey { hotkey_name: String },
    /// Start/Stop replay buffer
    ToggleReplayBuffer,
    /// Save replay buffer
    SaveReplayBuffer,
    /// Toggle Studio Mode
    ToggleStudioMode,
    /// Get current scene name
    GetCurrentScene,
    /// Get recording status
    GetRecordingStatus,
    /// Get streaming status
    GetStreamingStatus,
}

/// OBS plugin state
struct ObsPluginState {
    client: Option<Client>,
    host: String,
    port: u16,
    password: Option<String>,
}

/// OBS integration plugin
pub struct ObsPlugin {
    state: Arc<RwLock<ObsPluginState>>,
}

impl ObsPlugin {
    /// Create a new OBS plugin instance
    pub fn new() -> Self {
        let host = std::env::var("OBS_HOST").unwrap_or_else(|_| "localhost".to_string());
        let port = std::env::var("OBS_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(4455); // OBS WebSocket v5 default port
        let password = std::env::var("OBS_PASSWORD").ok();

        Self {
            state: Arc::new(RwLock::new(ObsPluginState {
                client: None,
                host,
                port,
                password,
            })),
        }
    }

    /// Ensure connection to OBS WebSocket
    async fn ensure_connected(&self) -> PluginResult<()> {
        let mut state = self.state.write().await;

        if state.client.is_some() {
            // TODO: Check if connection is still alive
            return Ok(());
        }

        // Connect to OBS WebSocket
        let client = Client::connect(&state.host, state.port, state.password.clone())
            .await
            .map_err(|e| {
                PluginError::Connection(format!(
                    "Failed to connect to OBS at {}:{}: {}",
                    state.host, state.port, e
                ))
            })?;

        info!(
            "Connected to OBS WebSocket at {}:{}",
            state.host, state.port
        );
        state.client = Some(client);
        Ok(())
    }

    /// Get a reference to the OBS client
    async fn client(&self) -> PluginResult<Arc<RwLock<Option<Client>>>> {
        self.ensure_connected().await?;
        Ok(Arc::clone(&self.state))
    }

    /// Execute an OBS action
    async fn execute_action(&self, action: &ObsAction) -> PluginResult<serde_json::Value> {
        self.ensure_connected().await?;

        let state = self.state.read().await;
        let client = state.client.as_ref().ok_or_else(|| {
            PluginError::Connection("Not connected to OBS".to_string())
        })?;

        match action {
            ObsAction::SwitchScene { scene_name } => {
                client
                    .scenes()
                    .set_current_program_scene(scene_name)
                    .await
                    .map_err(|e| {
                        PluginError::Execution(format!("Failed to switch scene: {}", e))
                    })?;
                Ok(serde_json::json!({"scene": scene_name}))
            }

            ObsAction::ToggleScene { scene_name } => {
                // Get current scene
                let current = client.scenes().current_program_scene().await.map_err(|e| {
                    PluginError::Execution(format!("Failed to get current scene: {}", e))
                })?;

                if current.as_str() == scene_name {
                    // If already on this scene, we can't really "toggle" it
                    // Could switch to previous scene, but that requires tracking state
                    Ok(serde_json::json!({
                        "scene": scene_name,
                        "already_active": true
                    }))
                } else {
                    client
                        .scenes()
                        .set_current_program_scene(scene_name)
                        .await
                        .map_err(|e| {
                            PluginError::Execution(format!("Failed to switch scene: {}", e))
                        })?;
                    Ok(serde_json::json!({"scene": scene_name}))
                }
            }

            ObsAction::ToggleRecording => {
                client.recording().toggle().await.map_err(|e| {
                    PluginError::Execution(format!("Failed to toggle recording: {}", e))
                })?;
                Ok(serde_json::json!({"action": "toggle_recording"}))
            }

            ObsAction::StartRecording => {
                client.recording().start().await.map_err(|e| {
                    PluginError::Execution(format!("Failed to start recording: {}", e))
                })?;
                Ok(serde_json::json!({"recording": true}))
            }

            ObsAction::StopRecording => {
                client.recording().stop().await.map_err(|e| {
                    PluginError::Execution(format!("Failed to stop recording: {}", e))
                })?;
                Ok(serde_json::json!({"recording": false}))
            }

            ObsAction::PauseRecording => {
                client.recording().toggle_pause().await.map_err(|e| {
                    PluginError::Execution(format!("Failed to pause/resume recording: {}", e))
                })?;
                Ok(serde_json::json!({"action": "pause_toggle"}))
            }

            ObsAction::ToggleStreaming => {
                client.streaming().toggle().await.map_err(|e| {
                    PluginError::Execution(format!("Failed to toggle streaming: {}", e))
                })?;
                Ok(serde_json::json!({"action": "toggle_streaming"}))
            }

            ObsAction::StartStreaming => {
                client.streaming().start().await.map_err(|e| {
                    PluginError::Execution(format!("Failed to start streaming: {}", e))
                })?;
                Ok(serde_json::json!({"streaming": true}))
            }

            ObsAction::StopStreaming => {
                client.streaming().stop().await.map_err(|e| {
                    PluginError::Execution(format!("Failed to stop streaming: {}", e))
                })?;
                Ok(serde_json::json!({"streaming": false}))
            }

            ObsAction::ToggleSource {
                scene_name,
                source_name,
            } => {
                let scene = scene_name.as_ref().map(|s| s.as_str());

                // Get current visibility
                let item_id = client
                    .scene_items()
                    .id(obws::requests::SceneItemId {
                        scene: scene,
                        source: source_name,
                    })
                    .await
                    .map_err(|e| {
                        PluginError::Execution(format!("Failed to get source item: {}", e))
                    })?;

                let enabled = client
                    .scene_items()
                    .enabled(item_id)
                    .await
                    .map_err(|e| {
                        PluginError::Execution(format!("Failed to get source visibility: {}", e))
                    })?;

                // Toggle visibility
                client
                    .scene_items()
                    .set_enabled(obws::requests::SetEnabled {
                        scene_item_id: item_id,
                        enabled: !enabled,
                    })
                    .await
                    .map_err(|e| {
                        PluginError::Execution(format!("Failed to toggle source: {}", e))
                    })?;

                Ok(serde_json::json!({
                    "source": source_name,
                    "visible": !enabled
                }))
            }

            ObsAction::SetSourceVisibility {
                scene_name,
                source_name,
                visible,
            } => {
                let scene = scene_name.as_ref().map(|s| s.as_str());

                let item_id = client
                    .scene_items()
                    .id(obws::requests::SceneItemId {
                        scene,
                        source: source_name,
                    })
                    .await
                    .map_err(|e| {
                        PluginError::Execution(format!("Failed to get source item: {}", e))
                    })?;

                client
                    .scene_items()
                    .set_enabled(obws::requests::SetEnabled {
                        scene_item_id: item_id,
                        enabled: *visible,
                    })
                    .await
                    .map_err(|e| {
                        PluginError::Execution(format!("Failed to set source visibility: {}", e))
                    })?;

                Ok(serde_json::json!({
                    "source": source_name,
                    "visible": visible
                }))
            }

            ObsAction::ToggleMute { source_name } => {
                client
                    .inputs()
                    .toggle_mute(source_name)
                    .await
                    .map_err(|e| PluginError::Execution(format!("Failed to toggle mute: {}", e)))?;
                Ok(serde_json::json!({
                    "source": source_name,
                    "action": "toggle_mute"
                }))
            }

            ObsAction::SetVolume {
                source_name,
                volume,
            } => {
                let volume_db = 20.0 * volume.log10(); // Convert to dB
                client
                    .inputs()
                    .set_volume(obws::requests::SetVolume {
                        input: source_name,
                        volume: obws::requests::Volume::Db(volume_db),
                    })
                    .await
                    .map_err(|e| {
                        PluginError::Execution(format!("Failed to set volume: {}", e))
                    })?;
                Ok(serde_json::json!({
                    "source": source_name,
                    "volume": volume
                }))
            }

            ObsAction::TriggerHotkey { hotkey_name } => {
                client
                    .hotkeys()
                    .trigger(hotkey_name)
                    .await
                    .map_err(|e| {
                        PluginError::Execution(format!("Failed to trigger hotkey: {}", e))
                    })?;
                Ok(serde_json::json!({"hotkey": hotkey_name}))
            }

            ObsAction::ToggleReplayBuffer => {
                client.replay_buffer().toggle().await.map_err(|e| {
                    PluginError::Execution(format!("Failed to toggle replay buffer: {}", e))
                })?;
                Ok(serde_json::json!({"action": "toggle_replay_buffer"}))
            }

            ObsAction::SaveReplayBuffer => {
                client.replay_buffer().save().await.map_err(|e| {
                    PluginError::Execution(format!("Failed to save replay buffer: {}", e))
                })?;
                Ok(serde_json::json!({"action": "save_replay_buffer"}))
            }

            ObsAction::ToggleStudioMode => {
                // Get current state
                let enabled = client.ui().studio_mode_enabled().await.map_err(|e| {
                    PluginError::Execution(format!("Failed to get studio mode state: {}", e))
                })?;

                // Toggle it
                client
                    .ui()
                    .set_studio_mode_enabled(!enabled)
                    .await
                    .map_err(|e| {
                        PluginError::Execution(format!("Failed to toggle studio mode: {}", e))
                    })?;

                Ok(serde_json::json!({"studio_mode": !enabled}))
            }

            ObsAction::GetCurrentScene => {
                let scene = client.scenes().current_program_scene().await.map_err(|e| {
                    PluginError::Execution(format!("Failed to get current scene: {}", e))
                })?;
                Ok(serde_json::json!({"current_scene": scene}))
            }

            ObsAction::GetRecordingStatus => {
                let status = client.recording().status().await.map_err(|e| {
                    PluginError::Execution(format!("Failed to get recording status: {}", e))
                })?;
                Ok(serde_json::json!({
                    "recording": status.active,
                    "paused": status.paused,
                }))
            }

            ObsAction::GetStreamingStatus => {
                let status = client.streaming().status().await.map_err(|e| {
                    PluginError::Execution(format!("Failed to get streaming status: {}", e))
                })?;
                Ok(serde_json::json!({
                    "streaming": status.active,
                }))
            }
        }
    }
}

impl Default for ObsPlugin {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl ActionPlugin for ObsPlugin {
    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            info: PluginInfo {
                name: "OBS Studio Control".to_string(),
                version: "0.1.0".to_string(),
                author: "Amiable Dev".to_string(),
                description: "Control OBS Studio through WebSocket protocol".to_string(),
                homepage: Some("https://github.com/amiable-dev/midimon-obs-plugin".to_string()),
            },
            capabilities: vec![PluginCapability::Network],
            action_schema: serde_json::json!({
                "type": "object",
                "oneOf": [
                    {
                        "type": "object",
                        "properties": {
                            "type": {"const": "switch_scene"},
                            "scene_name": {"type": "string"}
                        },
                        "required": ["scene_name"]
                    },
                    {"type": "object", "properties": {"type": {"const": "toggle_recording"}}},
                    {"type": "object", "properties": {"type": {"const": "start_recording"}}},
                    {"type": "object", "properties": {"type": {"const": "stop_recording"}}},
                    {"type": "object", "properties": {"type": {"const": "toggle_streaming"}}},
                    {"type": "object", "properties": {"type": {"const": "start_streaming"}}},
                    {"type": "object", "properties": {"type": {"const": "stop_streaming"}}},
                    {
                        "type": "object",
                        "properties": {
                            "type": {"const": "toggle_source"},
                            "scene_name": {"type": "string"},
                            "source_name": {"type": "string"}
                        },
                        "required": ["source_name"]
                    },
                    {
                        "type": "object",
                        "properties": {
                            "type": {"const": "toggle_mute"},
                            "source_name": {"type": "string"}
                        },
                        "required": ["source_name"]
                    },
                    {
                        "type": "object",
                        "properties": {
                            "type": {"const": "set_volume"},
                            "source_name": {"type": "string"},
                            "volume": {"type": "number", "minimum": 0.0, "maximum": 1.0}
                        },
                        "required": ["source_name", "volume"]
                    },
                    {"type": "object", "properties": {"type": {"const": "save_replay_buffer"}}}
                ]
            }),
        }
    }

    async fn execute(&self, action_data: serde_json::Value) -> PluginResult<serde_json::Value> {
        debug!("Executing OBS action: {:?}", action_data);

        // Parse action from JSON
        let action: ObsAction = serde_json::from_value(action_data)
            .map_err(|e| PluginError::InvalidAction(format!("Invalid action format: {}", e)))?;

        // Execute the action
        self.execute_action(&action).await
    }

    async fn initialize(&mut self) -> PluginResult<()> {
        info!("Initializing OBS plugin");
        // Try to connect on initialization
        let _ = self.ensure_connected().await;
        Ok(())
    }

    async fn shutdown(&mut self) -> PluginResult<()> {
        info!("Shutting down OBS plugin");
        let mut state = self.state.write().await;
        if let Some(client) = state.client.take() {
            // Disconnect gracefully
            drop(client);
        }
        Ok(())
    }
}

// Export the plugin constructor
#[no_mangle]
pub extern "C" fn create_plugin() -> *mut dyn ActionPlugin {
    Box::into_raw(Box::new(ObsPlugin::new()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata() {
        let plugin = ObsPlugin::new();
        let metadata = plugin.metadata();
        assert_eq!(metadata.info.name, "OBS Studio Control");
        assert_eq!(metadata.info.version, "0.1.0");
        assert!(metadata.capabilities.contains(&PluginCapability::Network));
    }

    #[test]
    fn test_action_serialization() {
        let action = ObsAction::StartRecording;
        let json = serde_json::to_value(&action).unwrap();
        assert_eq!(json["type"], "start_recording");

        let action = ObsAction::SwitchScene {
            scene_name: "Scene 1".to_string(),
        };
        let json = serde_json::to_value(&action).unwrap();
        assert_eq!(json["type"], "switch_scene");
        assert_eq!(json["scene_name"], "Scene 1");
    }
}
