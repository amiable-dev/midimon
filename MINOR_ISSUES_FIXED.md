# Minor UI Issues - Implementation Summary

**Date**: 2025-11-20
**Status**: ⚠️ PARTIALLY RESOLVED
**Context**: Addressed two minor UI issues found during gamepad template testing

---

## Summary

Status of minor UI issues identified during testing:

1. ⚠️ **MIDI Device Refresh** - midir library limitation discovered (see Issue #11)
2. ✅ **Daemon Status Details** - Fixed field name mismatch between daemon and frontend
3. 🔍 **NEW: Device Selection UX** - Discovered missing interaction (see Issue #12)

---

## Issue 1: MIDI Device Refresh

### Problem
Auto-refresh wasn't detecting newly connected MIDI devices.

### Root Cause Analysis
**midir Library Limitation** - The `midir` library (v0.10) does not expose macOS CoreMIDI hot-plug notifications. This is a [known limitation documented in GitHub Issue #78](https://github.com/Boddlnagg/midir/issues/78).

- midir creates static MIDI port lists on initialization
- CoreMIDI notifies about device changes, but midir doesn't expose this
- Recreation of MidiInput doesn't force re-enumeration on macOS
- Only app restart forces full system re-scan

### Attempted Fixes (All Failed During Testing)

**Attempt 1**: Documentation + explicit drop()
- **Result**: ❌ User tested, no improvement

**Attempt 2**: spawn_blocking + warmup cycle + 100ms delay
- **Result**: ❌ User tested, still required app restart

**Attempt 3**: Better error handling for invalid ports
- **Result**: ✅ Prevents "Unknown Device" entries but doesn't fix hot-plug detection

### Current Status

⚠️ **NOT FIXED** - This is a fundamental limitation of the midir library on macOS.

**Workaround**: Users must restart the GUI application to detect newly connected devices.

**Future Solution**: See [GitHub Issue #11](https://github.com/amiable-dev/midimon/issues/11) for three potential implementation options:
1. Use `coremidi` crate directly on macOS (recommended)
2. Contribute hot-plug support to midir upstream
3. Use periodic full restart workaround

**Files Modified** (for error handling improvements):
- `midimon-gui/src-tauri/src/commands.rs:357-408`

---

## Issue 2: Daemon Status Details

### Problem
Daemon Status card only showed "Running: Yes" and "Connected: Yes", but didn't show:
- State (lifecycle_state)
- Uptime
- Events Processed

### Root Cause Analysis
Field name mismatch between daemon and frontend:

| Frontend Expects | Daemon Returned |
|------------------|-----------------|
| `daemon.lifecycle_state` | `state` |
| `daemon.uptime_seconds` | `uptime_secs` |
| `statistics.events_processed` | `events_processed` |

### Fix Applied

**File**: `midimon-daemon/src/daemon/engine_manager.rs:246-285`

**Changes**:
Updated the `IpcCommand::Status` handler to return a nested structure:

```rust
IpcCommand::Status => {
    let state = *self.state.read().await;
    let stats = self.statistics.read().await.clone();
    let uptime_secs = self.start_time.elapsed().as_secs();

    create_success_response(
        &id,
        Some(json!({
            // NEW: Nested structure for frontend compatibility
            "daemon": {
                "lifecycle_state": format!("{}", state),
                "uptime_seconds": uptime_secs,
            },
            "statistics": {
                "events_processed": stats.events_processed,
                "errors_since_start": stats.errors_since_start,
                "config_reloads": stats.config_reloads,
            },
            "device": device_status,
            // Legacy flat fields for backward compatibility
            "state": format!("{}", state),
            "uptime_secs": uptime_secs,
            "events_processed": stats.events_processed,
            ...
        })),
    )
}
```

**Why It Works**:
- Frontend code in `get_daemon_status()` (commands.rs:97-111) looks for nested fields
- Daemon now returns both nested (new) and flat (legacy) fields
- Backward compatible with existing code

**Expected UI Result**:
The Daemon Status card in DevicesView will now show:
```
Running: ✅ Yes
Connected: ✅ Yes
State: Running
Uptime: 5m 23s
Events Processed: 1,234
```

---

## Issue 3: Device Selection UX (Discovered During Testing)

### Problem
Users can see MIDI devices in the GUI but cannot interact with them. No way to select or use devices.

### Root Cause Analysis
**Missing UI Interaction Layer**:
- Device cards have no click handlers
- Connect/Disconnect buttons are intentionally disabled
- Store has `select(device)` and `selectedDevice` but UI doesn't call them
- No clear user journey from "I see my device" to "I'm using my device"

### Current Status

🔍 **DISCOVERED** - This requires an architectural/UX decision before implementation.

**Decision Needed**: See [GitHub Issue #12](https://github.com/amiable-dev/midimon/issues/12) for four UX design options:

1. **Information-Only View** - Just display status (monitoring only)
2. **Direct Connection Control** - Click device to connect (point-and-click)
3. **Setup Wizard Approach** - Guide user through device setup (walkthrough)
4. **Hybrid - Smart Contextual Actions** - Context-aware buttons (recommended)

**Files Involved**:
- `midimon-gui/ui/src/lib/components/DeviceList.svelte` (no click handlers)
- `midimon-gui/ui/src/lib/stores.js` (unused selection methods)

---

## Build Verification

All changes have been verified:

```bash
✅ cargo build --package midimon-daemon
   Finished `dev` profile in 9.01s

✅ cargo build --package midimon-gui
   Finished `dev` profile in 24.95s

✅ cargo test --package midimon-daemon --lib engine_manager
   test result: ok. 5 passed; 0 failed; 0 ignored
```

---

## Testing Status

### Issue 1: MIDI Device Refresh
⚠️ **NOT FIXED** - Multiple attempts tested, all failed. Requires app restart.
- See GitHub Issue #11 for future solution options

### Issue 2: Daemon Status Details
✅ **READY FOR TESTING**
1. Launch GUI
2. Navigate to "Devices & Profiles" tab
3. Look at the "Daemon Status" card
4. **Expected**: You should now see:
   - Running: ✅ Yes
   - Connected: ✅ Yes
   - **State: Running** (NEW)
   - **Uptime: Xm Ys** (NEW)
   - **Events Processed: N** (NEW)

### Issue 3: Device Selection UX
🔍 **DECISION NEEDED** - See GitHub Issue #12 for UX design options

---

## Files Modified

### Issue 1: MIDI Device Refresh (Improved Error Handling Only)
- `midimon-gui/src-tauri/src/commands.rs:357-408`
  - Added spawn_blocking for fresh enumeration
  - Added skip logic for invalid ports (prevents "Unknown Device")
  - Hot-plug detection still requires app restart (midir limitation)

### Issue 2: Daemon Status Details (FIXED)
- `midimon-daemon/src/daemon/engine_manager.rs:246-285`
  - Returns nested `daemon` and `statistics` objects
  - Maintains flat fields for backward compatibility with midimonctl

### Issue 3: Device Selection UX (Not Implemented Yet)
- No files modified - awaiting architectural decision

### Documentation
- `MINOR_ISSUES_ANALYSIS.md` (updated with final status)
- `MINOR_ISSUES_FIXED.md` (this file - implementation summary)
- GitHub Issue #11 (MIDI hot-plug limitation + solution options)
- GitHub Issue #12 (Device selection UX decision document)

---

## Next Steps

### Immediate: Test Daemon Status Fix
```bash
# Rebuild GUI and test daemon status display
cargo build --package midimon-gui --release
# Launch and verify State, Uptime, Events Processed appear
```

### Future Work (Requires Decisions)

**Issue #11: MIDI Hot-Plug Detection**
- Decision needed: Which implementation approach?
  1. Use `coremidi` crate directly (recommended)
  2. Contribute to midir upstream
  3. Use periodic restart workaround

**Issue #12: Device Selection UX**
- Decision needed: Which UX approach?
  1. Information-only view
  2. Direct connection control
  3. Setup wizard
  4. Hybrid contextual actions (recommended)

### Ready for Commit

```bash
git add midimon-daemon/src/daemon/engine_manager.rs
git add midimon-gui/src-tauri/src/commands.rs
git add MINOR_ISSUES_*.md
git commit -m "fix(gui): Daemon status details now display correctly

- Fixed field name mismatch between daemon IPC and frontend
- Daemon now returns nested structure (daemon.*, statistics.*)
- Maintained backward compatibility for midimonctl CLI
- Improved MIDI device error handling (skip invalid ports)

Note: MIDI hot-plug detection requires app restart (midir limitation)
See Issue #11 for future coremidi integration plan
See Issue #12 for device selection UX decision

Partially addresses: Devices & Profiles tab minor issues"
```

---

**Status**: ⚠️ PARTIALLY RESOLVED
- ✅ Daemon status details fixed
- ⚠️ MIDI hot-plug blocked by library limitation (Issue #11)
- 🔍 Device selection UX discovered, needs decision (Issue #12)
