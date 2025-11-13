// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! MIDIMon Daemon - Background service for MIDI controller mapping
//!
//! This crate provides the daemon infrastructure for running MIDIMon as a background service
//! with config hot-reload, IPC control, and system tray integration.

pub mod daemon;

// Re-export core types for convenience
pub use daemon::{
    calculate_checksum, create_success_response, get_socket_path, get_state_dir, run_daemon,
    run_daemon_with_config, ConfigWatcher, DaemonCommand, DaemonError, DaemonInfo, DaemonService,
    DaemonStatistics, DeviceStatus, EngineInfo, EngineManager, ErrorDetails, ErrorEntry,
    IpcClient, IpcCommand, IpcErrorCode, IpcRequest, IpcResponse, IpcServer, LifecycleState,
    MenuBarAction, PersistedState, ReloadMetrics, ResponseStatus, Result, StateManager,
};
