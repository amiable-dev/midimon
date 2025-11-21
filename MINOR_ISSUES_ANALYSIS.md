# Minor UI Issues - Analysis & Fixes

**Date**: 2025-11-20
**Status**: ⚠️ PARTIALLY RESOLVED
**Context**: Post-gamepad template UI analysis and implementation
**GitHub Issues**: #11 (MIDI hot-plug), #12 (Device selection UX)

## Issue 1: MIDI Device Refresh Not Detecting New Devices

### Symptoms
- User plugs in new MIDI device
- Device appears in `midi_diagnostic` tool
- GUI refresh button doesn't show new device
- Auto-refresh (every 5 seconds) also doesn't detect it

### Root Cause
The issue is NOT in the frontend code. The DeviceList component correctly:
- ✅ Calls `devicesStore.fetch()` on refresh
- ✅ Has auto-refresh every 5 seconds
- ✅ Properly updates the UI when devices change

The problem is in the backend chain:
1. GUI calls `list_midi_devices` Tauri command
2. Backend queries available MIDI ports
3. **Issue**: The backend likely caches the port list or doesn't re-enumerate

### Investigation Needed
Check `midimon-gui/src-tauri/src/commands.rs`:

```rust
#[tauri::command]
pub async fn list_midi_devices(state: State<'_, AppState>) -> Result<Vec<MidiDevice>, String>
```

This command needs to:
- ✅ Create a fresh MIDI input instance each time
- ✅ Call `get_port_count()` to get current port count
- ✅ Enumerate all ports dynamically (not from cache)

### Recommended Fix
Ensure the `list_midi_devices` command creates a new `MidiInput` instance on each call:

```rust
pub async fn list_midi_devices(state: State<'_, AppState>) -> Result<Vec<MidiDevice>, String> {
    // Create fresh MIDI input to get current ports
    let midi_in = MidiInput::new("midimon-gui-query")
        .map_err(|e| format!("Failed to create MIDI input: {}", e))?;

    let port_count = midi_in.port_count();
    let mut devices = Vec::new();

    for i in 0..port_count {
        if let Ok(port_name) = midi_in.port_name(i) {
            devices.push(MidiDevice {
                index: i,
                name: port_name,
                connected: false, // TODO: Check daemon for connected device
            });
        }
    }

    Ok(devices)
}
```

---

## Issue 2: Daemon Status Not Showing Details

### Symptoms
- Daemon Status card only shows: "Running: Yes", "Connected: Yes"
- Missing fields:
  - State (lifecycle_state)
  - Uptime
  - Events Processed

### Root Cause
The frontend is correctly trying to display these fields:

**File**: `midimon-gui/ui/src/lib/views/DevicesView.svelte:166-185`

```svelte
{#if $statusStore.status.lifecycle_state}
  <div class="status-row">
    <span class="label">State:</span>
    <span class="value">{$statusStore.status.lifecycle_state}</span>
  </div>
{/if}
{#if $statusStore.status.uptime_secs !== null}
  <!-- Uptime display -->
{/if}
{#if $statusStore.status.events_processed !== null}
  <!-- Events display -->
{/if}
```

The problem is that the daemon's IPC Status response doesn't include these fields.

### Investigation Needed
Check what the daemon's Status command returns:

**Expected structure**:
```json
{
  "daemon": {
    "lifecycle_state": "Running",
    "uptime_seconds": 123
  },
  "statistics": {
    "events_processed": 456
  },
  "device": {
    "connected": true,
    "name": "Maschine Mikro MK3",
    "port": 2
  }
}
```

**File to check**: `midimon-daemon/src/daemon/ipc_server.rs`

The Status command handler needs to return this complete structure.

### Recommended Fix

Update the IPC Status command handler to include full daemon statistics:

```rust
IpcCommand::Status => {
    let daemon_status = self.engine_manager.get_status().await?;

    json!({
        "daemon": {
            "lifecycle_state": format!("{}", daemon_status.state),
            "uptime_seconds": daemon_status.uptime.as_secs(),
        },
        "statistics": {
            "events_processed": daemon_status.events_processed,
        },
        "device": daemon_status.device.map(|d| json!({
            "connected": d.connected,
            "name": d.name,
            "port": d.port,
        })),
    })
}
```

---

## Priority Assessment

### Issue 1: MIDI Device Refresh
- **Priority**: Medium
- **Impact**: Minor inconvenience - users need to restart GUI to see new devices
- **Workaround**: Restart the GUI application
- **User Impact**: Low (devices are rare to add/remove during session)

### Issue 2: Daemon Status Details
- **Priority**: Low
- **Impact**: Cosmetic - missing helpful information but not blocking functionality
- **Workaround**: None needed - daemon works fine
- **User Impact**: Very Low (status indicators work, just missing details)

---

## Success Criteria for Gamepad Template Workflow

Despite these minor issues, the core gamepad template functionality is **100% working**:

✅ Template discovery and selection
✅ Gamepad category filtering
✅ Confirmation dialogs (native OS dialogs)
✅ Config file creation and saving
✅ Daemon reload
✅ Success notifications

**Recommendation**: These minor issues can be addressed in a future update as they don't block the primary gamepad template workflow.

---

## Testing Steps to Verify Fixes

### Fix 1: MIDI Device Refresh
1. Launch GUI
2. Note current device count
3. Plug in new MIDI device
4. Click "Refresh" button
5. **Expected**: New device appears in list immediately

### Fix 2: Daemon Status Details
1. Launch GUI
2. Navigate to Devices & Profiles
3. Check Daemon Status card
4. **Expected**: Shows State, Uptime, Events Processed fields

---

## Files Involved

### Issue 1 Files:
- `midimon-gui/src-tauri/src/commands.rs` (list_midi_devices function)
- `midimon-gui/ui/src/lib/components/DeviceList.svelte` (already correct)
- `midimon-gui/ui/src/lib/stores.js` (devicesStore - already correct)

### Issue 2 Files:
- `midimon-daemon/src/daemon/ipc_server.rs` (Status command handler)
- `midimon-daemon/src/daemon/engine_manager.rs` (get_status method)
- `midimon-gui/src-tauri/src/commands.rs` (get_daemon_status - already correct)
- `midimon-gui/ui/src/lib/views/DevicesView.svelte` (already correct)

---

## Final Implementation Status

### Issue 1: MIDI Device Refresh ⚠️ NOT FIXED

**Root Cause Confirmed**: midir library limitation (GitHub Issue #78)
- midir doesn't expose CoreMIDI hot-plug notifications
- Creating fresh MidiInput instances doesn't force re-enumeration on macOS
- Only app restart triggers full system device scan

**Attempted Fixes (All Failed During User Testing)**:
1. Documentation + explicit drop() → ❌ No improvement
2. spawn_blocking + warmup cycle + 100ms delay → ❌ Still requires restart
3. Better error handling for invalid ports → ✅ Prevents "Unknown Device" but doesn't fix hot-plug

**Current Workaround**: Users must restart GUI to detect new devices

**Future Solution**: See [GitHub Issue #11](https://github.com/amiable-dev/midimon/issues/11) for implementation options:
1. Use `coremidi` crate directly on macOS (recommended)
2. Contribute hot-plug support to midir upstream
3. Use periodic full restart workaround

**Files Modified**: `midimon-gui/src-tauri/src/commands.rs:357-408` (error handling improvements only)

---

### Issue 2: Daemon Status Details ✅ FIXED

**Root Cause**: Field name mismatch between daemon IPC response and frontend expectations

**File**: `midimon-daemon/src/daemon/engine_manager.rs:246-285`

**Changes Made**: Updated IPC Status handler to return nested structure:
```json
{
  "daemon": {
    "lifecycle_state": "Running",
    "uptime_seconds": 123
  },
  "statistics": {
    "events_processed": 456,
    "errors_since_start": 0,
    "config_reloads": 2
  },
  "device": {
    "connected": true,
    "name": "...",
    "port": 0
  },
  // Legacy flat fields for midimonctl backward compatibility
  "state": "Running",
  "uptime_secs": 123,
  "events_processed": 456
}
```

**Why It Works**:
- Frontend expects: `daemon.lifecycle_state`, `daemon.uptime_seconds`, `statistics.events_processed`
- Daemon now returns both nested (new) and flat (legacy) fields
- Backward compatible with midimonctl CLI tool

**Build Verification**:
- ✅ Daemon package builds successfully
- ✅ GUI package builds successfully
- ✅ All engine_manager unit tests pass (5/5)

**Ready for User Testing**: Yes

---

### Issue 3: Device Selection UX 🔍 DISCOVERED

**Problem**: Users can see MIDI devices but cannot interact with them

**Root Cause**:
- Device cards have no click handlers
- Connect/Disconnect buttons intentionally disabled
- Store has unused `select(device)` and `selectedDevice` properties
- No clear user journey from "see device" to "use device"

**Status**: Requires architectural/UX decision before implementation

**Decision Document**: [GitHub Issue #12](https://github.com/amiable-dev/midimon/issues/12)

**Options**:
1. Information-Only View (monitoring only)
2. Direct Connection Control (point-and-click)
3. Setup Wizard Approach (guided walkthrough)
4. Hybrid - Smart Contextual Actions (recommended)

**Files Involved**:
- `midimon-gui/ui/src/lib/components/DeviceList.svelte`
- `midimon-gui/ui/src/lib/stores.js`

---

## Summary

**What Was Fixed**: ✅ Daemon status details now display correctly

**What Wasn't Fixed**: ⚠️ MIDI hot-plug detection (midir library limitation)

**What Was Discovered**: 🔍 Device selection UX missing (needs decision)

**GitHub Issues Created**:
- Issue #11: MIDI hot-plug detection + solution options
- Issue #12: Device selection UX decision document

**Ready for Commit**: Yes - Daemon status fix can be committed now
