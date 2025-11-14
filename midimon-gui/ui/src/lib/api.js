// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

/**
 * API wrapper for Tauri commands
 * Provides centralized error handling and type-safe access to backend
 */

import { invoke } from '@tauri-apps/api/core';

/**
 * Custom error class for API errors
 */
export class ApiError extends Error {
  constructor(message, code = null) {
    super(message);
    this.name = 'ApiError';
    this.code = code;
    this.timestamp = new Date();
  }
}

/**
 * Daemon API - Commands for interacting with the daemon
 */
export const daemon = {
  /**
   * Get current daemon status
   * @returns {Promise<DaemonStatus>}
   * @throws {ApiError}
   */
  async getStatus() {
    try {
      return await invoke('get_daemon_status');
    } catch (error) {
      throw new ApiError(`Failed to get daemon status: ${error}`, 'DAEMON_STATUS_ERROR');
    }
  },

  /**
   * Reload daemon configuration
   * @returns {Promise<void>}
   * @throws {ApiError}
   */
  async reload() {
    try {
      return await invoke('reload_config');
    } catch (error) {
      throw new ApiError(`Failed to reload config: ${error}`, 'RELOAD_ERROR');
    }
  },

  /**
   * Stop the daemon
   * @returns {Promise<void>}
   * @throws {ApiError}
   */
  async stop() {
    try {
      return await invoke('stop_daemon');
    } catch (error) {
      throw new ApiError(`Failed to stop daemon: ${error}`, 'STOP_ERROR');
    }
  },

  /**
   * Ping the daemon to measure latency
   * @returns {Promise<number>} Latency in milliseconds
   * @throws {ApiError}
   */
  async ping() {
    try {
      return await invoke('ping_daemon');
    } catch (error) {
      throw new ApiError(`Failed to ping daemon: ${error}`, 'PING_ERROR');
    }
  },
};

/**
 * Config API - Commands for managing configuration
 */
export const config = {
  /**
   * Validate the current configuration
   * @returns {Promise<ConfigValidation>}
   * @throws {ApiError}
   */
  async validate() {
    try {
      return await invoke('validate_config');
    } catch (error) {
      throw new ApiError(`Failed to validate config: ${error}`, 'VALIDATE_ERROR');
    }
  },

  /**
   * Get the configuration file path
   * @returns {Promise<string>}
   * @throws {ApiError}
   */
  async getPath() {
    try {
      return await invoke('get_config_path');
    } catch (error) {
      throw new ApiError(`Failed to get config path: ${error}`, 'CONFIG_PATH_ERROR');
    }
  },

  /**
   * Get current configuration as JSON
   * @returns {Promise<object>}
   * @throws {ApiError}
   */
  async get() {
    try {
      return await invoke('get_config');
    } catch (error) {
      throw new ApiError(`Failed to get config: ${error}`, 'GET_CONFIG_ERROR');
    }
  },

  /**
   * Save configuration
   * @param {object} configData - Configuration object to save
   * @returns {Promise<void>}
   * @throws {ApiError}
   */
  async save(configData) {
    try {
      return await invoke('save_config', { config: configData });
    } catch (error) {
      throw new ApiError(`Failed to save config: ${error}`, 'SAVE_CONFIG_ERROR');
    }
  },
};

/**
 * Devices API - Commands for managing MIDI devices
 */
export const devices = {
  /**
   * List available MIDI devices
   * @returns {Promise<MidiDevice[]>}
   * @throws {ApiError}
   */
  async list() {
    try {
      return await invoke('list_midi_devices');
    } catch (error) {
      throw new ApiError(`Failed to list MIDI devices: ${error}`, 'LIST_DEVICES_ERROR');
    }
  },
};

/**
 * MIDI Learn API - Commands for MIDI Learn mode
 */
export const midiLearn = {
  /**
   * Start a MIDI Learn session
   * @param {number} timeoutSecs - Timeout duration in seconds
   * @returns {Promise<string>} Session ID
   * @throws {ApiError}
   */
  async start(timeoutSecs = 10) {
    try {
      return await invoke('start_midi_learn', { timeoutSecs });
    } catch (error) {
      throw new ApiError(`Failed to start MIDI Learn: ${error}`, 'MIDI_LEARN_START_ERROR');
    }
  },

  /**
   * Get the status of the current MIDI Learn session
   * @returns {Promise<string>} Session state (Idle, Waiting, Captured, TimedOut, Cancelled)
   * @throws {ApiError}
   */
  async getStatus() {
    try {
      return await invoke('get_midi_learn_status');
    } catch (error) {
      throw new ApiError(`Failed to get MIDI Learn status: ${error}`, 'MIDI_LEARN_STATUS_ERROR');
    }
  },

  /**
   * Get remaining time for MIDI Learn session
   * @returns {Promise<number>} Remaining seconds
   * @throws {ApiError}
   */
  async getRemaining() {
    try {
      return await invoke('get_midi_learn_remaining');
    } catch (error) {
      throw new ApiError(`Failed to get remaining time: ${error}`, 'MIDI_LEARN_REMAINING_ERROR');
    }
  },

  /**
   * Cancel the current MIDI Learn session
   * @returns {Promise<void>}
   * @throws {ApiError}
   */
  async cancel() {
    try {
      return await invoke('cancel_midi_learn');
    } catch (error) {
      throw new ApiError(`Failed to cancel MIDI Learn: ${error}`, 'MIDI_LEARN_CANCEL_ERROR');
    }
  },

  /**
   * Get the result of the MIDI Learn session
   * @returns {Promise<MidiLearnResult|null>}
   * @throws {ApiError}
   */
  async getResult() {
    try {
      return await invoke('get_midi_learn_result');
    } catch (error) {
      throw new ApiError(`Failed to get MIDI Learn result: ${error}`, 'MIDI_LEARN_RESULT_ERROR');
    }
  },
};

/**
 * Export all APIs as a single default object
 */
export default {
  daemon,
  config,
  devices,
  midiLearn,
  ApiError,
};
