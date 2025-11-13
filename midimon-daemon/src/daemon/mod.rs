// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! Daemon infrastructure for background service operation

pub mod config_watcher;
pub mod engine_manager;
pub mod error;
pub mod ipc;
pub mod service;
pub mod state;
pub mod types;

pub use config_watcher::ConfigWatcher;
pub use engine_manager::EngineManager;
pub use error::{DaemonError, IpcErrorCode, Result};
pub use ipc::{create_success_response, IpcClient, IpcServer};
pub use service::{run_daemon, run_daemon_with_config, DaemonService};
pub use state::{
    calculate_checksum, get_socket_path, get_state_dir, ConfigInfo, DaemonInfo, EngineInfo,
    PersistedState, StateManager,
};
pub use types::{
    DaemonCommand, DaemonStatistics, DeviceStatus, ErrorDetails, ErrorEntry, IpcCommand,
    IpcRequest, IpcResponse, LifecycleState, MenuBarAction, ReloadMetrics, ResponseStatus,
};
