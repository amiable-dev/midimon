// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! MIDIMon GUI - Visual Configuration Interface
//!
//! Tauri v2-based desktop application for configuring MIDIMon mappings,
//! MIDI Learn functionality, and visual feedback.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod events;
mod state;

use state::AppState;

fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("midimon_gui=debug".parse().unwrap()),
        )
        .init();

    tauri::Builder::default()
        .manage(AppState::new())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::get_daemon_status,
            commands::reload_config,
            commands::stop_daemon,
            commands::validate_config,
            commands::ping_daemon,
            commands::list_midi_devices,
            commands::get_config,
            commands::save_config,
            commands::get_config_path,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
