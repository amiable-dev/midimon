// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! MIDIMon GUI - Visual Configuration Interface
//!
//! Tauri v2-based desktop application for configuring MIDIMon mappings,
//! MIDI Learn functionality, and visual feedback.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app_detection;
mod commands;
mod config_helpers;
mod device_templates;
mod events;
mod midi_learn;
mod profile_manager;
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
            commands::start_midi_learn,
            commands::get_midi_learn_status,
            commands::get_midi_learn_remaining,
            commands::cancel_midi_learn,
            commands::get_midi_learn_result,
            commands::generate_trigger_config_toml,
            commands::trigger_suggestion_to_json,
            commands::get_frontmost_app,
            commands::start_app_monitoring,
            commands::stop_app_monitoring,
            commands::list_profiles,
            commands::register_profile,
            commands::switch_profile,
            commands::switch_profile_for_app,
            commands::get_active_profile,
            commands::scan_profiles,
            commands::clear_profile_cache,
            commands::export_profile_json,
            commands::import_profile_json,
            commands::export_profile_toml,
            commands::import_profile_toml,
            commands::list_device_templates,
            commands::get_device_template,
            commands::find_templates_by_midi,
            commands::get_template_categories,
            commands::list_templates_by_category,
            commands::create_config_from_template,
            commands::start_event_monitoring,
            commands::stop_event_monitoring,
            commands::is_event_monitoring_active,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
