// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

/**
 * Centralized state management for MIDIMon GUI
 * Uses Svelte stores for reactive state
 */

import { writable, derived, get } from 'svelte/store';
import api from './api.js';

/**
 * App-level state store
 */
function createAppStore() {
  const { subscribe, set, update } = writable({
    daemonConnected: false,
    lastError: null,
    loading: false,
  });

  return {
    subscribe,
    setDaemonConnected: (connected) => update(state => ({
      ...state,
      daemonConnected: connected
    })),
    setError: (error) => update(state => ({
      ...state,
      lastError: error
    })),
    clearError: () => update(state => ({
      ...state,
      lastError: null
    })),
    setLoading: (loading) => update(state => ({
      ...state,
      loading
    })),
    reset: () => set({
      daemonConnected: false,
      lastError: null,
      loading: false
    }),
  };
}

/**
 * Daemon status store with auto-refresh
 */
function createStatusStore() {
  const { subscribe, set, update } = writable({
    status: null,
    loading: false,
    error: null,
  });

  let refreshInterval = null;

  return {
    subscribe,

    /**
     * Fetch daemon status
     */
    async fetch() {
      update(state => ({ ...state, loading: true, error: null }));
      try {
        const status = await api.daemon.getStatus();
        set({ status, loading: false, error: null });
        appStore.setDaemonConnected(status.running && status.connected);
        return status;
      } catch (error) {
        const errorMsg = error.message || String(error);
        set({ status: null, loading: false, error: errorMsg });
        appStore.setDaemonConnected(false);
        throw error;
      }
    },

    /**
     * Start auto-refresh (every 2 seconds)
     */
    startAutoRefresh() {
      this.stopAutoRefresh();
      this.fetch(); // Initial fetch
      refreshInterval = setInterval(() => {
        this.fetch().catch(err => {
          console.warn('Auto-refresh failed:', err);
        });
      }, 2000);
    },

    /**
     * Stop auto-refresh
     */
    stopAutoRefresh() {
      if (refreshInterval) {
        clearInterval(refreshInterval);
        refreshInterval = null;
      }
    },

    /**
     * Reset store state
     */
    reset() {
      this.stopAutoRefresh();
      set({ status: null, loading: false, error: null });
    },
  };
}

/**
 * Devices store for MIDI device management
 */
function createDevicesStore() {
  const { subscribe, set, update } = writable({
    devices: [],
    selectedDevice: null,
    loading: false,
    error: null,
  });

  let refreshInterval = null;

  return {
    subscribe,

    /**
     * Fetch device list
     */
    async fetch() {
      update(state => ({ ...state, loading: true, error: null }));
      try {
        const devices = await api.devices.list();
        update(state => ({
          ...state,
          devices,
          loading: false,
          error: null
        }));
        return devices;
      } catch (error) {
        const errorMsg = error.message || String(error);
        set({ devices: [], selectedDevice: null, loading: false, error: errorMsg });
        throw error;
      }
    },

    /**
     * Select a device
     */
    select(device) {
      update(state => ({ ...state, selectedDevice: device }));
    },

    /**
     * Clear selection
     */
    clearSelection() {
      update(state => ({ ...state, selectedDevice: null }));
    },

    /**
     * Start auto-refresh (every 5 seconds)
     */
    startAutoRefresh() {
      this.stopAutoRefresh();
      this.fetch(); // Initial fetch
      refreshInterval = setInterval(() => {
        this.fetch().catch(err => {
          console.warn('Device list auto-refresh failed:', err);
        });
      }, 5000);
    },

    /**
     * Stop auto-refresh
     */
    stopAutoRefresh() {
      if (refreshInterval) {
        clearInterval(refreshInterval);
        refreshInterval = null;
      }
    },

    /**
     * Reset store state
     */
    reset() {
      this.stopAutoRefresh();
      set({ devices: [], selectedDevice: null, loading: false, error: null });
    },
  };
}

/**
 * Config store for configuration management
 */
function createConfigStore() {
  const { subscribe, set, update } = writable({
    config: null,
    validation: null,
    configPath: null,
    loading: false,
    error: null,
  });

  return {
    subscribe,

    /**
     * Fetch configuration
     */
    async fetch() {
      update(state => ({ ...state, loading: true, error: null }));
      try {
        const config = await api.config.get();
        update(state => ({ ...state, config, loading: false, error: null }));
        return config;
      } catch (error) {
        const errorMsg = error.message || String(error);
        update(state => ({ ...state, loading: false, error: errorMsg }));
        throw error;
      }
    },

    /**
     * Save configuration
     */
    async save(configData) {
      update(state => ({ ...state, loading: true, error: null }));
      try {
        await api.config.save(configData);
        update(state => ({ ...state, config: configData, loading: false }));
      } catch (error) {
        const errorMsg = error.message || String(error);
        update(state => ({ ...state, loading: false, error: errorMsg }));
        throw error;
      }
    },

    /**
     * Validate configuration
     */
    async validate() {
      update(state => ({ ...state, loading: true, error: null }));
      try {
        const validation = await api.config.validate();
        update(state => ({ ...state, validation, loading: false }));
        return validation;
      } catch (error) {
        const errorMsg = error.message || String(error);
        update(state => ({ ...state, loading: false, error: errorMsg }));
        throw error;
      }
    },

    /**
     * Get config file path
     */
    async getPath() {
      try {
        const configPath = await api.config.getPath();
        update(state => ({ ...state, configPath }));
        return configPath;
      } catch (error) {
        const errorMsg = error.message || String(error);
        update(state => ({ ...state, error: errorMsg }));
        throw error;
      }
    },

    /**
     * Reset store state
     */
    reset() {
      set({
        config: null,
        validation: null,
        configPath: null,
        loading: false,
        error: null
      });
    },
  };
}

// Export store instances
export const appStore = createAppStore();
export const statusStore = createStatusStore();
export const devicesStore = createDevicesStore();
export const configStore = createConfigStore();

// Derived store: Is daemon connected and ready?
export const isDaemonReady = derived(
  statusStore,
  $status => $status.status?.running && $status.status?.connected
);

// Derived store: Current mode name
export const currentMode = derived(
  statusStore,
  $status => $status.status?.lifecycle_state || 'Unknown'
);

// Derived store: Connected device name
export const connectedDevice = derived(
  statusStore,
  $status => {
    if (!$status.status?.device) return null;
    if (!$status.status.device.connected) return null;
    return $status.status.device.name || `Port ${$status.status.device.port}`;
  }
);
