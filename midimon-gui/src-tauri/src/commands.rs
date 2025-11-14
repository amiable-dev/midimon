// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! Tauri commands for interacting with the MIDIMon daemon

use serde::{Deserialize, Serialize};
use tauri::State;
use crate::state::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct DaemonStatus {
    pub running: bool,
    pub connected: bool,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MidiDevice {
    pub index: usize,
    pub name: String,
    pub connected: bool,
}

/// Get the current daemon status
#[tauri::command]
pub async fn get_daemon_status(_state: State<'_, AppState>) -> Result<DaemonStatus, String> {
    // TODO (AMI-162): Implement daemon status query via IPC
    Ok(DaemonStatus {
        running: false,
        connected: false,
        error: Some("Not implemented yet".to_string()),
    })
}

/// Reload the configuration
#[tauri::command]
pub async fn reload_config(_state: State<'_, AppState>) -> Result<(), String> {
    // TODO (AMI-162): Implement config reload via IPC
    Err("Not implemented yet".to_string())
}

/// List available MIDI devices
#[tauri::command]
pub async fn list_midi_devices(_state: State<'_, AppState>) -> Result<Vec<MidiDevice>, String> {
    // TODO (AMI-164): Implement MIDI device listing
    Ok(vec![])
}
