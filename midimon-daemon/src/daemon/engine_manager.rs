// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! Engine manager with atomic config reloading and device reconnection

use crate::daemon::error::{DaemonError, IpcErrorCode, Result};
use crate::daemon::ipc::create_success_response;
use crate::daemon::state::{calculate_checksum, ConfigInfo, EngineInfo};
use crate::daemon::types::{
    DaemonCommand, DaemonStatistics, DeviceStatus, ErrorDetails, ErrorEntry, IpcCommand,
    IpcResponse, LifecycleState, ReloadMetrics, ResponseStatus,
};
use midimon_core::{ActionExecutor, Config, EventProcessor, MappingEngine};
use serde_json::json;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use tokio::sync::{broadcast, mpsc, Mutex, RwLock};
use tracing::{debug, error, info, warn};

/// Engine manager coordinating MIDIMon engine with daemon lifecycle
pub struct EngineManager {
    /// Current config (atomic swap support)
    config: Arc<RwLock<Config>>,
    config_path: PathBuf,

    /// Engine components
    #[allow(dead_code)] // Reserved for future direct event processing integration
    event_processor: Arc<RwLock<EventProcessor>>,
    mapping_engine: Arc<RwLock<MappingEngine>>,
    #[allow(dead_code)] // Reserved for future direct action execution integration (uses Mutex as ActionExecutor is not Sync)
    action_executor: Arc<Mutex<ActionExecutor>>,

    /// Lifecycle state
    state: Arc<RwLock<LifecycleState>>,

    /// Device status
    device_status: Arc<RwLock<DeviceStatus>>,

    /// Statistics
    statistics: Arc<RwLock<DaemonStatistics>>,
    start_time: Instant,

    /// Error log (keep last 10 errors)
    error_log: Arc<RwLock<Vec<ErrorEntry>>>,

    /// Command receiver
    command_rx: mpsc::Receiver<DaemonCommand>,

    /// Shutdown broadcaster
    shutdown_tx: broadcast::Sender<()>,
}

impl EngineManager {
    /// Create a new engine manager
    pub fn new(
        config: Config,
        config_path: PathBuf,
        command_rx: mpsc::Receiver<DaemonCommand>,
        shutdown_tx: broadcast::Sender<()>,
    ) -> Result<Self> {
        let event_processor = EventProcessor::new();
        let mut mapping_engine = MappingEngine::new();
        mapping_engine.load_from_config(&config);
        let action_executor = ActionExecutor::new();

        Ok(Self {
            config: Arc::new(RwLock::new(config)),
            config_path,
            event_processor: Arc::new(RwLock::new(event_processor)),
            mapping_engine: Arc::new(RwLock::new(mapping_engine)),
            action_executor: Arc::new(Mutex::new(action_executor)),
            state: Arc::new(RwLock::new(LifecycleState::Init)),
            device_status: Arc::new(RwLock::new(DeviceStatus::default())),
            statistics: Arc::new(RwLock::new(DaemonStatistics::default())),
            start_time: Instant::now(),
            error_log: Arc::new(RwLock::new(Vec::new())),
            command_rx,
            shutdown_tx,
        })
    }

    /// Run the engine manager loop
    pub async fn run(&mut self) -> Result<()> {
        info!("Engine manager starting");

        // Transition to Starting state
        self.transition_state(LifecycleState::Starting).await?;

        // TODO: Initialize MIDI device connection
        // For now, just transition to Running
        self.transition_state(LifecycleState::Running).await?;

        info!("Engine manager running");

        // Main command loop
        while let Some(command) = self.command_rx.recv().await {
            match command {
                DaemonCommand::IpcRequest {
                    request,
                    response_tx,
                } => {
                    let response = self.handle_ipc_request(request).await;
                    let _ = response_tx.send(response);
                }

                DaemonCommand::ConfigFileChanged(path) => {
                    info!("Config file changed: {:?}", path);
                    if let Err(e) = self.reload_config().await {
                        error!("Config reload failed: {}", e);
                        self.log_error("ConfigReloadFailed", e.to_string())
                            .await;
                    }
                }

                DaemonCommand::DeviceDisconnected => {
                    warn!("Device disconnected");
                    self.transition_state(LifecycleState::Degraded).await?;
                    self.update_device_status(false, None, None).await;
                    self.log_error("DeviceDisconnected", "MIDI device unplugged")
                        .await;
                }

                DaemonCommand::DeviceReconnected => {
                    info!("Device reconnected");
                    self.transition_state(LifecycleState::Running).await?;
                    // TODO: Update device status with actual device info
                    self.update_device_status(true, Some("Maschine Mikro MK3".to_string()), Some(0))
                        .await;
                }

                DaemonCommand::DeviceReconnectionFailed => {
                    error!("Device reconnection failed after max attempts");
                    self.log_error(
                        "DeviceReconnectionFailed",
                        "Failed to reconnect after multiple attempts",
                    )
                    .await;
                }

                DaemonCommand::FatalError(msg) => {
                    error!("Fatal error: {}", msg);
                    self.log_error("FatalError", msg).await;
                    self.transition_state(LifecycleState::Stopping).await?;
                    break;
                }

                DaemonCommand::Shutdown => {
                    info!("Shutdown requested");
                    self.transition_state(LifecycleState::Stopping).await?;
                    break;
                }

                DaemonCommand::MenuBarAction(_) => {
                    // TODO: Handle menu bar actions
                    debug!("Menu bar action received (not yet implemented)");
                }
            }
        }

        // Final state transition
        self.transition_state(LifecycleState::Stopped).await?;

        info!("Engine manager stopped");
        Ok(())
    }

    /// Handle IPC request
    async fn handle_ipc_request(
        &mut self,
        request: crate::daemon::types::IpcRequest,
    ) -> IpcResponse {
        let id = request.id.clone();

        match request.command {
            IpcCommand::Ping => {
                let timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();

                create_success_response(
                    &id,
                    Some(json!({
                        "message": "pong",
                        "timestamp": timestamp
                    })),
                )
            }

            IpcCommand::Status => {
                let state = *self.state.read().await;
                let config = self.config.read().await;
                let device_status = self.device_status.read().await.clone();
                let stats = self.statistics.read().await.clone();
                let uptime_secs = self.start_time.elapsed().as_secs();

                create_success_response(
                    &id,
                    Some(json!({
                        "state": format!("{}", state),
                        "current_mode": config.modes.first().map(|m| &m.name).unwrap_or(&"None".to_string()),
                        "config_path": self.config_path,
                        "config_loaded_at": stats.uptime_secs, // Placeholder
                        "device_status": device_status,
                        "uptime_secs": uptime_secs,
                        "events_processed": stats.events_processed,
                        "errors_since_start": stats.errors_since_start,
                        "config_reloads": stats.config_reloads,
                        "reload_stats": {
                            "last_reload_ms": stats.last_reload_duration_ms,
                            "fastest_reload_ms": stats.fastest_reload_ms,
                            "slowest_reload_ms": stats.slowest_reload_ms,
                            "avg_reload_ms": stats.avg_reload_ms,
                        },
                    })),
                )
            }

            IpcCommand::Reload => match self.reload_config().await {
                Ok(metrics) => create_success_response(
                    &id,
                    Some(json!({
                        "message": "Config reloaded successfully",
                        "config_path": self.config_path,
                        "reload_duration_ms": metrics.duration_ms,
                        "modes_loaded": metrics.modes_loaded,
                        "mappings_loaded": metrics.mappings_loaded,
                        "config_load_ms": metrics.config_load_ms,
                        "mapping_compile_ms": metrics.mapping_compile_ms,
                        "swap_ms": metrics.swap_ms,
                        "performance_grade": metrics.performance_grade(),
                        "met_target": metrics.met_target(),
                    })),
                ),
                Err(e) => IpcResponse {
                    id,
                    status: ResponseStatus::Error,
                    data: None,
                    error: Some(ErrorDetails {
                        code: IpcErrorCode::ConfigValidationFailed.as_u16(),
                        message: e.to_string(),
                        details: None,
                    }),
                },
            },

            IpcCommand::Stop => {
                // Send shutdown command to self
                let _ = self.shutdown_tx.send(());

                create_success_response(
                    &id,
                    Some(json!({
                        "message": "Daemon stopping",
                        "state_saved": true
                    })),
                )
            }

            IpcCommand::ValidateConfig => {
                // Extract path from args
                let path = request
                    .args
                    .get("path")
                    .and_then(|v| v.as_str())
                    .map(PathBuf::from)
                    .unwrap_or_else(|| self.config_path.clone());

                match Config::load(path.to_str().unwrap_or("")) {
                    Ok(config) => {
                        let total_mappings: usize = config
                            .modes
                            .iter()
                            .map(|m| m.mappings.len())
                            .sum::<usize>()
                            + config.global_mappings.len();

                        create_success_response(
                            &id,
                            Some(json!({
                                "valid": true,
                                "modes": config.modes.len(),
                                "mappings": total_mappings,
                                "warnings": []
                            })),
                        )
                    }
                    Err(e) => IpcResponse {
                        id,
                        status: ResponseStatus::Error,
                        data: None,
                        error: Some(ErrorDetails {
                            code: IpcErrorCode::ConfigValidationFailed.as_u16(),
                            message: e.to_string(),
                            details: None,
                        }),
                    },
                }
            }
        }
    }

    /// Reload configuration with atomic swap
    async fn reload_config(&mut self) -> Result<ReloadMetrics> {
        let start = Instant::now();

        // Transition to Reloading state
        let current_state = *self.state.read().await;
        if !current_state.can_transition_to(LifecycleState::Reloading) {
            return Err(DaemonError::InvalidStateTransition {
                from: format!("{}", current_state),
                to: "Reloading".to_string(),
            });
        }
        self.transition_state(LifecycleState::Reloading).await?;

        // Phase 1: Load and validate new config
        let config_load_start = Instant::now();
        let new_config = Config::load(self.config_path.to_str().unwrap_or(""))
            .map_err(|e| DaemonError::Ipc(format!("Config load failed: {}", e)))?;
        let config_load_ms = config_load_start.elapsed().as_millis() as u64;

        // Phase 2: Create new mapping engine
        let mapping_compile_start = Instant::now();
        let mut new_mapping_engine = MappingEngine::new();
        new_mapping_engine.load_from_config(&new_config);
        let mapping_compile_ms = mapping_compile_start.elapsed().as_millis() as u64;

        // Phase 3: Atomic swap
        let swap_start = Instant::now();
        {
            *self.config.write().await = new_config.clone();
            *self.mapping_engine.write().await = new_mapping_engine;
        }
        let swap_ms = swap_start.elapsed().as_millis() as u64;

        // Calculate metrics
        let duration_ms = start.elapsed().as_millis() as u64;
        let total_mappings: usize = new_config
            .modes
            .iter()
            .map(|m| m.mappings.len())
            .sum::<usize>()
            + new_config.global_mappings.len();

        let metrics = ReloadMetrics {
            duration_ms,
            modes_loaded: new_config.modes.len(),
            mappings_loaded: total_mappings,
            config_load_ms,
            mapping_compile_ms,
            swap_ms,
        };

        // Update statistics
        {
            let mut stats = self.statistics.write().await;
            stats.update_reload_metrics(&metrics);
        }

        info!(
            "Config reloaded in {}ms (grade: {}): {} modes, {} mappings [load: {}ms, compile: {}ms, swap: {}ms]",
            duration_ms,
            metrics.performance_grade(),
            metrics.modes_loaded,
            metrics.mappings_loaded,
            config_load_ms,
            mapping_compile_ms,
            swap_ms
        );

        // Transition back to Running
        self.transition_state(LifecycleState::Running).await?;

        Ok(metrics)
    }

    /// Transition to a new lifecycle state
    async fn transition_state(&self, new_state: LifecycleState) -> Result<()> {
        let mut state = self.state.write().await;
        let old_state = *state;

        if !old_state.can_transition_to(new_state) {
            return Err(DaemonError::InvalidStateTransition {
                from: format!("{}", old_state),
                to: format!("{}", new_state),
            });
        }

        *state = new_state;
        info!("State transition: {} → {}", old_state, new_state);

        Ok(())
    }

    /// Update device status
    async fn update_device_status(
        &self,
        connected: bool,
        name: Option<String>,
        port: Option<usize>,
    ) {
        let mut status = self.device_status.write().await;
        status.connected = connected;
        status.name = name;
        status.port = port;

        if connected {
            status.last_event_at = Some(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs(),
            );
        }
    }

    /// Log an error
    async fn log_error(&self, kind: impl Into<String>, message: impl Into<String>) {
        let entry = ErrorEntry::new(kind, message);
        let mut log = self.error_log.write().await;

        log.push(entry);

        // Keep only last 10 errors
        if log.len() > 10 {
            log.remove(0);
        }

        // Update statistics
        let mut stats = self.statistics.write().await;
        stats.errors_since_start += 1;
    }

    /// Get current config info for state persistence
    pub async fn get_config_info(&self) -> Result<ConfigInfo> {
        let checksum = calculate_checksum(&self.config_path).await?;
        let loaded_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Ok(ConfigInfo {
            path: self.config_path.clone(),
            loaded_at,
            checksum,
        })
    }

    /// Get current engine info for state persistence
    pub async fn get_engine_info(&self) -> EngineInfo {
        let config = self.config.read().await;
        let device_status = self.device_status.read().await.clone();

        EngineInfo {
            current_mode: config
                .modes
                .first()
                .map(|m| m.name.clone())
                .unwrap_or_else(|| "None".to_string()),
            current_mode_index: 0,
            device_status,
        }
    }

    /// Get current statistics
    pub async fn get_statistics(&self) -> DaemonStatistics {
        let mut stats = self.statistics.read().await.clone();
        stats.uptime_secs = self.start_time.elapsed().as_secs();
        stats
    }

    /// Get recent errors
    pub async fn get_recent_errors(&self) -> Vec<ErrorEntry> {
        self.error_log.read().await.clone()
    }

    /// Get current lifecycle state
    pub async fn get_state(&self) -> LifecycleState {
        *self.state.read().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use tokio::sync::mpsc;

    fn create_test_config() -> Config {
        Config::default_config()
    }

    #[tokio::test]
    async fn test_engine_manager_creation() {
        let config = create_test_config();
        let (_cmd_tx, cmd_rx) = mpsc::channel(10);
        let (shutdown_tx, _shutdown_rx) = broadcast::channel(1);

        let manager = EngineManager::new(
            config,
            PathBuf::from("/tmp/test.toml"),
            cmd_rx,
            shutdown_tx,
        );

        assert!(manager.is_ok());
    }

    #[tokio::test]
    async fn test_state_transitions() {
        let config = create_test_config();
        let (_cmd_tx, cmd_rx) = mpsc::channel(10);
        let (shutdown_tx, _shutdown_rx) = broadcast::channel(1);

        let manager = EngineManager::new(
            config,
            PathBuf::from("/tmp/test.toml"),
            cmd_rx,
            shutdown_tx,
        )
        .unwrap();

        // Initial state should be Init
        let state = manager.get_state().await;
        assert_eq!(state, LifecycleState::Init);

        // Should be able to transition to Starting
        let result = manager.transition_state(LifecycleState::Starting).await;
        assert!(result.is_ok());

        let state = manager.get_state().await;
        assert_eq!(state, LifecycleState::Starting);
    }

    #[tokio::test]
    async fn test_invalid_state_transition() {
        let config = create_test_config();
        let (_cmd_tx, cmd_rx) = mpsc::channel(10);
        let (shutdown_tx, _shutdown_rx) = broadcast::channel(1);

        let manager = EngineManager::new(
            config,
            PathBuf::from("/tmp/test.toml"),
            cmd_rx,
            shutdown_tx,
        )
        .unwrap();

        // Invalid transition: Init -> Running (must go through Starting)
        let result = manager.transition_state(LifecycleState::Running).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_error_logging() {
        let config = create_test_config();
        let (_cmd_tx, cmd_rx) = mpsc::channel(10);
        let (shutdown_tx, _shutdown_rx) = broadcast::channel(1);

        let manager = EngineManager::new(
            config,
            PathBuf::from("/tmp/test.toml"),
            cmd_rx,
            shutdown_tx,
        )
        .unwrap();

        manager
            .log_error("TestError", "This is a test error")
            .await;

        let errors = manager.get_recent_errors().await;
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].kind, "TestError");
        assert_eq!(errors[0].message, "This is a test error");
    }

    #[tokio::test]
    async fn test_statistics() {
        let config = create_test_config();
        let (_cmd_tx, cmd_rx) = mpsc::channel(10);
        let (shutdown_tx, _shutdown_rx) = broadcast::channel(1);

        let manager = EngineManager::new(
            config,
            PathBuf::from("/tmp/test.toml"),
            cmd_rx,
            shutdown_tx,
        )
        .unwrap();

        // Wait a bit to accumulate uptime
        tokio::time::sleep(Duration::from_millis(100)).await;

        let stats = manager.get_statistics().await;
        assert!(stats.uptime_secs == 0); // Less than 1 second
    }
}
