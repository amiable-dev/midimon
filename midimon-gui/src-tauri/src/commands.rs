// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! Tauri commands for interacting with the MIDIMon daemon

use serde::{Deserialize, Serialize};
use serde_json::json;
use tauri::State;
use uuid::Uuid;
use crate::state::AppState;
use crate::midi_learn::{MidiLearnSession, MidiLearnResult, LearnSessionState, TriggerSuggestion};
use crate::config_helpers::{suggestion_to_config, generate_mapping_toml, config_to_json};

// Import daemon types (we'll re-export these from daemon crate)
use midimon_daemon::daemon::{
    IpcClient, IpcCommand, IpcRequest, ResponseStatus,
};

/// Daemon status information for UI
#[derive(Debug, Serialize, Deserialize)]
pub struct DaemonStatus {
    pub running: bool,
    pub connected: bool,
    pub lifecycle_state: Option<String>,
    pub uptime_secs: Option<u64>,
    pub events_processed: Option<u64>,
    pub device: Option<DeviceInfo>,
    pub error: Option<String>,
}

/// Device information for UI
#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub connected: bool,
    pub name: Option<String>,
    pub port: Option<usize>,
}

/// MIDI device for device list
#[derive(Debug, Serialize, Deserialize)]
pub struct MidiDevice {
    pub index: usize,
    pub name: String,
    pub connected: bool,
}

/// Config validation result
#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigValidation {
    pub valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

/// Get the current daemon status
#[tauri::command]
pub async fn get_daemon_status(state: State<'_, AppState>) -> Result<DaemonStatus, String> {
    match IpcClient::connect().await {
        Ok(mut client) => {
            let request = IpcRequest {
                id: Uuid::new_v4().to_string(),
                command: IpcCommand::Status,
                args: json!({}),
            };

            match client.send_request(request).await {
                Ok(response) => {
                    if matches!(response.status, ResponseStatus::Success) {
                        state.set_daemon_connected(true).await;

                        // Parse daemon info from response
                        if let Some(data) = response.data {
                            let lifecycle_state = data.get("daemon")
                                .and_then(|d| d.get("lifecycle_state"))
                                .and_then(|s| s.as_str())
                                .map(String::from);

                            let uptime_secs = data.get("daemon")
                                .and_then(|d| d.get("uptime_seconds"))
                                .and_then(|u| u.as_u64());

                            let events_processed = data.get("statistics")
                                .and_then(|s| s.get("events_processed"))
                                .and_then(|e| e.as_u64());

                            let device = data.get("device").and_then(|d| {
                                Some(DeviceInfo {
                                    connected: d.get("connected")?.as_bool()?,
                                    name: d.get("name").and_then(|n| n.as_str()).map(String::from),
                                    port: d.get("port").and_then(|p| p.as_u64()).map(|p| p as usize),
                                })
                            });

                            Ok(DaemonStatus {
                                running: true,
                                connected: true,
                                lifecycle_state,
                                uptime_secs,
                                events_processed,
                                device,
                                error: None,
                            })
                        } else {
                            Ok(DaemonStatus {
                                running: true,
                                connected: true,
                                lifecycle_state: None,
                                uptime_secs: None,
                                events_processed: None,
                                device: None,
                                error: None,
                            })
                        }
                    } else {
                        let error_msg = response.error
                            .map(|e| e.message)
                            .unwrap_or_else(|| "Unknown error".to_string());

                        Ok(DaemonStatus {
                            running: true,
                            connected: true,
                            lifecycle_state: None,
                            uptime_secs: None,
                            events_processed: None,
                            device: None,
                            error: Some(error_msg),
                        })
                    }
                }
                Err(e) => {
                    state.set_daemon_connected(false).await;
                    Err(format!("Failed to get daemon status: {}", e))
                }
            }
        }
        Err(_) => {
            state.set_daemon_connected(false).await;
            Ok(DaemonStatus {
                running: false,
                connected: false,
                lifecycle_state: None,
                uptime_secs: None,
                events_processed: None,
                device: None,
                error: Some("Daemon not running".to_string()),
            })
        }
    }
}

/// Reload the configuration
#[tauri::command]
pub async fn reload_config(state: State<'_, AppState>) -> Result<(), String> {
    let mut client = IpcClient::connect()
        .await
        .map_err(|e| format!("Failed to connect to daemon: {}", e))?;

    let request = IpcRequest {
        id: Uuid::new_v4().to_string(),
        command: IpcCommand::Reload,
        args: json!({}),
    };

    let response = client
        .send_request(request)
        .await
        .map_err(|e| format!("Failed to send reload request: {}", e))?;

    match response.status {
        ResponseStatus::Success => {
            state.set_daemon_connected(true).await;
            Ok(())
        }
        ResponseStatus::Error => {
            let error_msg = response
                .error
                .map(|e| e.message)
                .unwrap_or_else(|| "Unknown error".to_string());
            Err(format!("Reload failed: {}", error_msg))
        }
    }
}

/// Stop the daemon
#[tauri::command]
pub async fn stop_daemon(state: State<'_, AppState>) -> Result<(), String> {
    let mut client = IpcClient::connect()
        .await
        .map_err(|e| format!("Failed to connect to daemon: {}", e))?;

    let request = IpcRequest {
        id: Uuid::new_v4().to_string(),
        command: IpcCommand::Stop,
        args: json!({}),
    };

    let response = client
        .send_request(request)
        .await
        .map_err(|e| format!("Failed to send stop request: {}", e))?;

    match response.status {
        ResponseStatus::Success => {
            state.set_daemon_connected(false).await;
            Ok(())
        }
        ResponseStatus::Error => {
            let error_msg = response
                .error
                .map(|e| e.message)
                .unwrap_or_else(|| "Unknown error".to_string());
            Err(format!("Stop failed: {}", error_msg))
        }
    }
}

/// Validate the configuration file
#[tauri::command]
pub async fn validate_config(_state: State<'_, AppState>) -> Result<ConfigValidation, String> {
    let mut client = IpcClient::connect()
        .await
        .map_err(|e| format!("Failed to connect to daemon: {}", e))?;

    let request = IpcRequest {
        id: Uuid::new_v4().to_string(),
        command: IpcCommand::ValidateConfig,
        args: json!({}),
    };

    let response = client
        .send_request(request)
        .await
        .map_err(|e| format!("Failed to send validation request: {}", e))?;

    match response.status {
        ResponseStatus::Success => {
            // Parse validation result from response data
            if let Some(data) = response.data {
                let valid = data.get("valid")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(true);

                let errors = data.get("errors")
                    .and_then(|e| e.as_array())
                    .map(|arr| arr.iter()
                        .filter_map(|v| v.as_str().map(String::from))
                        .collect())
                    .unwrap_or_default();

                let warnings = data.get("warnings")
                    .and_then(|w| w.as_array())
                    .map(|arr| arr.iter()
                        .filter_map(|v| v.as_str().map(String::from))
                        .collect())
                    .unwrap_or_default();

                Ok(ConfigValidation {
                    valid,
                    errors,
                    warnings,
                })
            } else {
                Ok(ConfigValidation {
                    valid: true,
                    errors: vec![],
                    warnings: vec![],
                })
            }
        }
        ResponseStatus::Error => {
            let error_msg = response
                .error
                .map(|e| e.message)
                .unwrap_or_else(|| "Unknown error".to_string());

            Ok(ConfigValidation {
                valid: false,
                errors: vec![error_msg],
                warnings: vec![],
            })
        }
    }
}

/// Ping the daemon to check if it's responsive
#[tauri::command]
pub async fn ping_daemon(state: State<'_, AppState>) -> Result<u64, String> {
    let start = std::time::Instant::now();

    let mut client = IpcClient::connect()
        .await
        .map_err(|e| format!("Failed to connect to daemon: {}", e))?;

    let request = IpcRequest {
        id: Uuid::new_v4().to_string(),
        command: IpcCommand::Ping,
        args: json!({}),
    };

    let response = client
        .send_request(request)
        .await
        .map_err(|e| format!("Failed to send ping request: {}", e))?;

    let latency_ms = start.elapsed().as_millis() as u64;

    match response.status {
        ResponseStatus::Success => {
            state.set_daemon_connected(true).await;
            Ok(latency_ms)
        }
        ResponseStatus::Error => {
            let error_msg = response
                .error
                .map(|e| e.message)
                .unwrap_or_else(|| "Unknown error".to_string());
            Err(format!("Ping failed: {}", error_msg))
        }
    }
}

/// List available MIDI devices
#[tauri::command]
pub async fn list_midi_devices(_state: State<'_, AppState>) -> Result<Vec<MidiDevice>, String> {
    use midir::MidiInput;

    let midi_in = MidiInput::new("MIDIMon Device Scanner")
        .map_err(|e| format!("Failed to create MIDI input: {}", e))?;

    let ports = midi_in.ports();
    let mut devices = Vec::new();

    for (index, port) in ports.iter().enumerate() {
        let name = midi_in
            .port_name(port)
            .unwrap_or_else(|_| format!("Unknown Device {}", index));

        devices.push(MidiDevice {
            index,
            name,
            connected: false, // Will be determined by checking daemon status
        });
    }

    Ok(devices)
}

/// Get the current configuration as JSON
#[tauri::command]
pub async fn get_config(_state: State<'_, AppState>) -> Result<serde_json::Value, String> {
    // TODO (AMI-167): Implement config retrieval
    // This will require either:
    // 1. Reading the config file directly from the standard location
    // 2. Adding a new IPC command to get config from daemon
    Err("Not implemented yet".to_string())
}

/// Save the configuration
#[tauri::command]
pub async fn save_config(
    _state: State<'_, AppState>,
    _config: serde_json::Value,
) -> Result<(), String> {
    // TODO (AMI-167): Implement config saving
    // This will require either:
    // 1. Writing the config file directly to the standard location
    // 2. Adding a new IPC command to update config via daemon
    Err("Not implemented yet".to_string())
}

/// Get the config file path
#[tauri::command]
pub async fn get_config_path() -> Result<String, String> {
    // Use the same logic as daemon to find config path
    let config_dir = dirs::config_dir()
        .ok_or_else(|| "Failed to determine config directory".to_string())?;

    let config_path = config_dir.join("midimon").join("config.toml");

    Ok(config_path.to_string_lossy().to_string())
}

/// Start a MIDI Learn session
#[tauri::command]
pub async fn start_midi_learn(
    timeout_secs: u64,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let session = MidiLearnSession::new(timeout_secs);
    let session_id = session.id.clone();

    session.start().await;
    state.set_learn_session(session).await;

    Ok(session_id)
}

/// Get the status of the current MIDI Learn session
#[tauri::command]
pub async fn get_midi_learn_status(state: State<'_, AppState>) -> Result<LearnSessionState, String> {
    let session = state.get_learn_session().await;
    match session {
        Some(s) => Ok(s.get_state().await),
        None => Ok(LearnSessionState::Idle),
    }
}

/// Get remaining time for MIDI Learn session
#[tauri::command]
pub async fn get_midi_learn_remaining(state: State<'_, AppState>) -> Result<u64, String> {
    let session = state.get_learn_session().await;
    match session {
        Some(s) => Ok(s.remaining_secs().await),
        None => Ok(0),
    }
}

/// Cancel the current MIDI Learn session
#[tauri::command]
pub async fn cancel_midi_learn(state: State<'_, AppState>) -> Result<(), String> {
    let session = state.get_learn_session().await;
    if let Some(s) = session {
        s.cancel().await;
    }
    Ok(())
}

/// Get the result of the MIDI Learn session
#[tauri::command]
pub async fn get_midi_learn_result(state: State<'_, AppState>) -> Result<Option<MidiLearnResult>, String> {
    let session = state.get_learn_session().await;
    match session {
        Some(s) => {
            // Check if timed out
            if s.is_timed_out().await {
                s.set_timed_out().await;
            }
            Ok(s.get_result().await)
        },
        None => Ok(None),
    }
}

/// Generate TOML config snippet from trigger suggestion
#[tauri::command]
pub fn generate_trigger_config_toml(
    suggestion: TriggerSuggestion,
    mode_name: String,
) -> Result<String, String> {
    let config = suggestion_to_config(&suggestion);
    Ok(generate_mapping_toml(&config, &mode_name))
}

/// Convert trigger suggestion to JSON config
#[tauri::command]
pub fn trigger_suggestion_to_json(suggestion: TriggerSuggestion) -> Result<serde_json::Value, String> {
    let config = suggestion_to_config(&suggestion);
    Ok(config_to_json(&config))
}

/// Get the current frontmost application
#[tauri::command]
pub async fn get_frontmost_app(state: State<'_, AppState>) -> Result<Option<crate::app_detection::AppInfo>, String> {
    Ok(state.get_current_app().await)
}

/// Start monitoring for app changes
#[tauri::command]
pub async fn start_app_monitoring(state: State<'_, AppState>) -> Result<(), String> {
    state.start_app_detection().await;
    Ok(())
}

/// Stop monitoring for app changes
#[tauri::command]
pub async fn stop_app_monitoring(state: State<'_, AppState>) -> Result<(), String> {
    state.stop_app_detection().await;
    Ok(())
}
