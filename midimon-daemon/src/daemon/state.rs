// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! State persistence across daemon restarts

use crate::daemon::error::{DaemonError, Result};
use crate::daemon::types::{DaemonStatistics, DeviceStatus, ErrorEntry, LifecycleState};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// Persisted daemon state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistedState {
    pub version: u8,
    pub saved_at: u64, // Unix timestamp in seconds
    pub daemon: DaemonInfo,
    pub config: ConfigInfo,
    pub engine: EngineInfo,
    pub statistics: DaemonStatistics,
    pub last_errors: Vec<ErrorEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonInfo {
    pub lifecycle_state: LifecycleState,
    pub started_at: u64,
    pub pid: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigInfo {
    pub path: PathBuf,
    pub loaded_at: u64,
    pub checksum: String, // SHA256 hash of config content
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineInfo {
    pub current_mode: String,
    pub current_mode_index: usize,
    pub device_status: DeviceStatus,
}

impl PersistedState {
    /// Create a new persisted state
    pub fn new(
        daemon_info: DaemonInfo,
        config_info: ConfigInfo,
        engine_info: EngineInfo,
        statistics: DaemonStatistics,
        last_errors: Vec<ErrorEntry>,
    ) -> Self {
        Self {
            version: 1,
            saved_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            daemon: daemon_info,
            config: config_info,
            engine: engine_info,
            statistics,
            last_errors,
        }
    }
}

/// State manager for atomic persistence
#[derive(Clone)]
pub struct StateManager {
    state_file: PathBuf,
    state: Arc<RwLock<Option<PersistedState>>>,
}

impl StateManager {
    /// Create a new state manager
    pub fn new() -> Result<Self> {
        let state_dir = get_state_dir()?;
        let state_file = state_dir.join("state.json");

        Ok(Self {
            state_file,
            state: Arc::new(RwLock::new(None)),
        })
    }

    /// Load state from disk
    pub async fn load(&self) -> Result<Option<PersistedState>> {
        if !self.state_file.exists() {
            debug!("No state file found at {:?}", self.state_file);
            return Ok(None);
        }

        match tokio::fs::read_to_string(&self.state_file).await {
            Ok(json) => match serde_json::from_str::<PersistedState>(&json) {
                Ok(state) => {
                    info!("Loaded daemon state from {:?}", self.state_file);
                    *self.state.write().await = Some(state.clone());
                    Ok(Some(state))
                }
                Err(e) => {
                    warn!(
                        "Failed to parse state file {:?}, ignoring: {}",
                        self.state_file, e
                    );
                    Ok(None)
                }
            },
            Err(e) => {
                warn!(
                    "Failed to read state file {:?}, ignoring: {}",
                    self.state_file, e
                );
                Ok(None)
            }
        }
    }

    /// Save state to disk with atomic write
    pub async fn save(&self, state: PersistedState) -> Result<()> {
        let state_dir = self
            .state_file
            .parent()
            .ok_or_else(|| DaemonError::StatePersistence("No parent directory".to_string()))?;

        let temp_file = state_dir.join(".state.json.tmp");

        // 1. Serialize to string
        let json = serde_json::to_string_pretty(&state)?;

        // 2. Write to temporary file
        tokio::fs::write(&temp_file, json.as_bytes()).await?;

        // 3. Fsync to ensure data is on disk
        let file = tokio::fs::OpenOptions::new()
            .write(true)
            .open(&temp_file)
            .await?;
        file.sync_all().await?;

        // 4. Atomic rename (POSIX guarantees atomicity)
        tokio::fs::rename(&temp_file, &self.state_file).await?;

        debug!("Saved daemon state to {:?}", self.state_file);

        // Update in-memory state
        *self.state.write().await = Some(state);

        Ok(())
    }

    /// Get current in-memory state
    pub async fn get(&self) -> Option<PersistedState> {
        self.state.read().await.clone()
    }

    /// Emergency save (synchronous, for panic handler)
    pub fn save_emergency(&self, state: &PersistedState) -> Result<()> {
        let state_dir = self
            .state_file
            .parent()
            .ok_or_else(|| DaemonError::StatePersistence("No parent directory".to_string()))?;

        let temp_file = state_dir.join(".state.json.tmp");

        // Serialize
        let json = serde_json::to_string_pretty(state)?;

        // Write to temp file
        std::fs::write(&temp_file, json.as_bytes())?;

        // Atomic rename
        std::fs::rename(&temp_file, &self.state_file)?;

        eprintln!("Emergency state saved to {:?}", self.state_file);

        Ok(())
    }
}

/// Get platform-specific state directory
pub fn get_state_dir() -> Result<PathBuf> {
    let dir = if cfg!(target_os = "macos") || cfg!(target_os = "linux") {
        dirs::home_dir()
            .ok_or_else(|| DaemonError::StatePersistence("No home directory".to_string()))?
            .join(".midimon")
    } else if cfg!(target_os = "windows") {
        dirs::data_dir()
            .ok_or_else(|| DaemonError::StatePersistence("No AppData directory".to_string()))?
            .join("midimon")
    } else {
        return Err(DaemonError::StatePersistence(
            "Unsupported platform".to_string(),
        ));
    };

    // Ensure directory exists
    std::fs::create_dir_all(&dir)?;

    Ok(dir)
}

/// Get platform-specific IPC socket path
pub fn get_socket_path() -> Result<PathBuf> {
    if cfg!(target_os = "windows") {
        Ok(PathBuf::from(r"\\.\pipe\midimon"))
    } else {
        // Unix domain socket on macOS/Linux
        Ok(PathBuf::from("/tmp/midimon.sock"))
    }
}

/// Calculate SHA256 checksum of a file
pub async fn calculate_checksum(path: &Path) -> Result<String> {
    use sha2::{Digest, Sha256};

    let content = tokio::fs::read(path).await?;
    let mut hasher = Sha256::new();
    hasher.update(&content);
    let result = hasher.finalize();

    Ok(format!("sha256:{:x}", result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_state_manager_save_and_load() {
        let state = PersistedState::new(
            DaemonInfo {
                lifecycle_state: LifecycleState::Running,
                started_at: 1000000,
                pid: 12345,
            },
            ConfigInfo {
                path: PathBuf::from("/tmp/config.toml"),
                loaded_at: 1000100,
                checksum: "sha256:abc123".to_string(),
            },
            EngineInfo {
                current_mode: "Default".to_string(),
                current_mode_index: 0,
                device_status: DeviceStatus::default(),
            },
            DaemonStatistics::default(),
            vec![],
        );

        let manager = StateManager::new().unwrap();

        // Save
        manager.save(state.clone()).await.unwrap();

        // Load
        let loaded = manager.load().await.unwrap().unwrap();

        assert_eq!(loaded.daemon.lifecycle_state, LifecycleState::Running);
        assert_eq!(loaded.daemon.pid, 12345);
        assert_eq!(loaded.config.path, PathBuf::from("/tmp/config.toml"));
        assert_eq!(loaded.engine.current_mode, "Default");
    }

    #[tokio::test]
    async fn test_state_manager_nonexistent_file() {
        let manager = StateManager::new().unwrap();

        // Remove state file if it exists
        let _ = tokio::fs::remove_file(&manager.state_file).await;

        let loaded = manager.load().await.unwrap();
        assert!(loaded.is_none());
    }

    #[test]
    fn test_get_state_dir() {
        let dir = get_state_dir().unwrap();

        if cfg!(target_os = "macos") || cfg!(target_os = "linux") {
            assert!(dir.ends_with(".midimon"));
        } else if cfg!(target_os = "windows") {
            assert!(dir.ends_with("midimon"));
        }

        // Directory should be created
        assert!(dir.exists());
    }

    #[test]
    fn test_get_socket_path() {
        let path = get_socket_path().unwrap();

        if cfg!(target_os = "windows") {
            assert_eq!(path, PathBuf::from(r"\\.\pipe\midimon"));
        } else {
            assert_eq!(path, PathBuf::from("/tmp/midimon.sock"));
        }
    }
}
