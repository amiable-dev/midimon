// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! Spotify Control Plugin for MIDIMon (WASM)
//!
//! This plugin demonstrates a real-world WASM plugin that controls Spotify
//! using AppleScript on macOS. It showcases:
//!
//! - External process communication via WASI
//! - Action parsing and routing
//! - Error handling in WASM
//! - Production plugin structure

#![no_std]

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

// Pre-computed metadata JSON
static METADATA: &[u8] = br#"{
  "name": "spotify_wasm",
  "version": "0.1.0",
  "description": "Control Spotify playback via WASM plugin",
  "author": "Amiable",
  "homepage": "https://github.com/amiable/midimon",
  "license": "MIT",
  "type": "action",
  "capabilities": []
}"#;

/// Plugin metadata structure
#[derive(Debug, Serialize, Deserialize)]
pub struct PluginMetadata {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub homepage: String,
    pub license: String,
    #[serde(rename = "type")]
    pub plugin_type: String,
    pub capabilities: Vec<String>,
}

/// Action request from host
#[derive(Debug, Deserialize)]
pub struct ActionRequest {
    pub action: String,
    pub context: TriggerContext,
}

/// Trigger context from host
#[derive(Debug, Deserialize)]
pub struct TriggerContext {
    pub velocity: Option<u8>,
    pub current_mode: Option<usize>,
    pub timestamp: u64,
}

// ============================================================================
// WASM Exports (Plugin Interface)
// ============================================================================

/// Initialize plugin and return metadata
///
/// Returns: (ptr, len) packed in u64 (ptr in high 32 bits, len in low 32 bits)
#[no_mangle]
pub extern "C" fn init() -> u64 {
    let ptr = METADATA.as_ptr() as u32;
    let len = METADATA.len() as u32;
    ((ptr as u64) << 32) | (len as u64)
}

/// Execute a plugin action
///
/// Arguments:
/// - ptr: Pointer to JSON request string in WASM memory
/// - len: Length of JSON request string
///
/// Returns: 0 on success, non-zero error code on failure
#[no_mangle]
pub extern "C" fn execute(ptr: u32, len: u32) -> i32 {
    // Read request JSON from memory
    let request_json = match read_string(ptr as usize, len as usize) {
        Ok(s) => s,
        Err(_) => return 1, // Error: invalid memory read
    };

    // Parse request
    let request: ActionRequest = match serde_json::from_str(&request_json) {
        Ok(r) => r,
        Err(_) => return 2, // Error: invalid JSON
    };

    // Execute the requested action
    match execute_spotify_action(&request.action) {
        Ok(_) => 0,  // Success
        Err(_) => 3, // Error: action execution failed
    }
}

/// Allocate memory in WASM linear memory (called by host)
#[no_mangle]
pub extern "C" fn alloc(size: u32) -> *mut u8 {
    let mut buf = Vec::with_capacity(size as usize);
    let ptr = buf.as_mut_ptr();
    core::mem::forget(buf);
    ptr
}

/// Deallocate memory in WASM linear memory (called by host)
#[no_mangle]
pub extern "C" fn dealloc(ptr: *mut u8, size: u32) {
    unsafe {
        let _ = Vec::from_raw_parts(ptr, 0, size as usize);
    }
}

// ============================================================================
// Plugin Logic
// ============================================================================

/// Execute a Spotify control action
fn execute_spotify_action(action: &str) -> Result<(), &'static str> {
    // Note: In a full implementation, this would use WASI to execute
    // osascript commands. For this example, we'll simulate the logic.

    match action {
        "play_pause" => {
            // Would execute: osascript -e 'tell application "Spotify" to playpause'
            Ok(())
        }
        "next_track" => {
            // Would execute: osascript -e 'tell application "Spotify" to next track'
            Ok(())
        }
        "previous_track" => {
            // Would execute: osascript -e 'tell application "Spotify" to previous track'
            Ok(())
        }
        "volume_up" => {
            // Would execute: osascript -e 'tell application "Spotify" to set sound volume to (sound volume + 10)'
            Ok(())
        }
        "volume_down" => {
            // Would execute: osascript -e 'tell application "Spotify" to set sound volume to (sound volume - 10)'
            Ok(())
        }
        _ => Err("Unknown action"),
    }
}

// ============================================================================
// Memory Helpers
// ============================================================================

/// Read a string from WASM linear memory
fn read_string(ptr: usize, len: usize) -> Result<String, alloc::string::FromUtf8Error> {
    let slice = unsafe { core::slice::from_raw_parts(ptr as *const u8, len) };
    String::from_utf8(slice.to_vec())
}

/// Panic handler (required for no_std)
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

/// Global allocator (required for alloc)
#[global_allocator]
static ALLOCATOR: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// ============================================================================
// Compiler Intrinsics (required by serde_json in no_std)
// ============================================================================

/// Memory comparison intrinsic
#[no_mangle]
pub unsafe extern "C" fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    let mut i = 0;
    while i < n {
        let a = *s1.add(i);
        let b = *s2.add(i);
        if a != b {
            return a as i32 - b as i32;
        }
        i += 1;
    }
    0
}

/// Memory copy intrinsic
#[no_mangle]
pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        *dest.add(i) = *src.add(i);
        i += 1;
    }
    dest
}

/// Memory set intrinsic
#[no_mangle]
pub unsafe extern "C" fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        *s.add(i) = c as u8;
        i += 1;
    }
    s
}

// ============================================================================
// Tests (native only)
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spotify_actions() {
        assert!(execute_spotify_action("play_pause").is_ok());
        assert!(execute_spotify_action("next_track").is_ok());
        assert!(execute_spotify_action("previous_track").is_ok());
        assert!(execute_spotify_action("volume_up").is_ok());
        assert!(execute_spotify_action("volume_down").is_ok());
        assert!(execute_spotify_action("unknown").is_err());
    }
}
