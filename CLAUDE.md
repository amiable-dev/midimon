# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Status & Migration Plan

⚠️ **IMPORTANT: Current Implementation Preservation**

This repository contains a working, single-binary implementation of MIDIMon. Before making significant architectural changes:

1. **Preserve Current State**: Create a git tag (e.g., `v0.1.0-monolithic`) to preserve the working implementation
2. **Reference Commit**: Document the commit hash where the current implementation is fully functional
3. **Migration Path**: Future work will migrate to a monorepo/workspace structure based on proposals in `.research/`

### Future Architecture Direction

The `.research/` directory contains two implementation proposals for evolving MIDIMon into a larger, more modular product:

- **implementation-viewpoint-1.md**: Monorepo with separate `engine`, `app` (Tauri UI), and config structure
- **implementation-viewpoint-2.md**: Similar approach with different crate organization and UI recommendations (Tauri vs Cacao)

**Migration Goals**:
- Extract core engine logic into a reusable `midimon-core` crate
- Add Tauri-based menu bar UI for configuration
- Support hot-reloading of config files
- Enable launch-at-startup functionality
- Maintain backward compatibility with existing config.toml format

**Before starting migration**:
```bash
# Tag the current working implementation
git add -A
git commit -m "Preserve working monolithic implementation before migration"
git tag -a v0.1.0-monolithic -m "Working single-binary implementation with all features"
git push origin v0.1.0-monolithic
```

## Project Overview

MIDIMon is a Rust-based MIDI controller mapping system that transforms MIDI devices (particularly the Native Instruments Maschine Mikro MK3) into advanced macro pads with velocity sensitivity, long press detection, double-tap, chord detection, and full RGB LED feedback.

**Current Architecture**: Single-binary Rust application with all functionality in one crate
**Target Architecture**: Workspace with `midimon-core` (engine), `midimon-daemon` (background service), `midimon-gui` (Tauri UI)

## Build, Run & Development Commands

### Building
```bash
# Debug build
cargo build

# Release build (optimized, ~3-5MB binary)
cargo build --release
```

### Running the Main Application
```bash
# List available MIDI ports
cargo run --release

# Connect to a specific port (e.g., port 2)
cargo run --release 2

# With LED lighting scheme
cargo run --release 2 --led reactive
cargo run --release 2 --led rainbow

# With device profile support
cargo run --release 2 --profile ~/Downloads/base-template-ni-mikro-mk3.ncmm3
cargo run --release 2 --profile mikro.ncmm3 --pad-page H

# Enable debug logging
DEBUG=1 cargo run --release 2
```

### Diagnostic Tools
```bash
# MIDI diagnostic tool (visualize all MIDI events)
cargo run --bin midi_diagnostic 2

# LED diagnostic tool
cargo run --bin led_diagnostic

# LED tester
cargo run --bin led_tester

# Pad mapper utility
cargo run --bin pad_mapper

# Test MIDI ports
cargo run --bin test_midi
```

### Testing
```bash
# Run all tests
cargo test

# Run with verbose output
cargo test -- --nocapture
```

## Architecture & Key Concepts

### Event Processing Pipeline

The system follows a three-stage event processing architecture:

1. **MIDI Input** (src/main.rs:124-169) - Raw MIDI bytes are converted to `MidiEvent` enum
2. **Event Processing** (src/event_processor.rs) - `MidiEvent` → `ProcessedEvent` (detects velocity levels, long press, double-tap, chords, encoder direction)
3. **Mapping & Execution** (src/mappings.rs, src/actions.rs) - `ProcessedEvent` → `Action` → execution

### Core Components

- **main.rs**: Entry point, MIDI connection, event loop coordination, and device profile handling
- **config.rs**: Configuration structures for TOML parsing (Trigger, Action, Mode definitions)
- **event_processor.rs**: Transforms raw MIDI events into higher-level processed events (velocity detection, timing-based triggers)
- **mappings.rs**: Mapping engine that matches ProcessedEvents to Actions based on current mode
- **actions.rs**: Action execution using `enigo` for keyboard/mouse simulation and shell commands
- **feedback.rs**: Unified LED feedback trait and device factory
- **mikro_leds.rs**: HID-based RGB LED control for Maschine Mikro MK3
- **midi_feedback.rs**: Standard MIDI LED feedback fallback
- **device_profile.rs**: Parser for NI Controller Editor profiles (.ncmm3 XML format)

### Key Design Patterns

**Mode System**: Multiple modes (Default, Development, Media) allow different mapping sets. Mode changes are triggered by encoder rotation or specific pad combinations.

**Global vs Mode Mappings**: Global mappings work across all modes (e.g., emergency exit, encoder volume control), while mode-specific mappings are scoped to their mode.

**Profile-Based Note Mapping**: Supports loading Native Instruments Controller Editor profiles (.ncmm3) to map physical pad positions to MIDI notes. Can auto-detect the active pad page or use a specified page (A-H).

**Velocity Levels**: Three velocity levels (Soft: 0-40, Medium: 41-80, Hard: 81-127) enable velocity-sensitive actions.

**LED Feedback**: Reactive LED system provides visual feedback for pad presses, mode changes, and actions. Supports multiple schemes (reactive, rainbow, pulse, breathing, static, etc.).

**HID Shared Device Access**: Uses `hidapi` with `macos-shared-device` feature to allow simultaneous access with Native Instruments Controller Editor.

### Trigger Types

- **Note**: Basic note on/off with optional velocity range
- **VelocityRange**: Different actions for soft/medium/hard presses on same note
- **LongPress**: Hold detection with configurable duration (default 2000ms)
- **DoubleTap**: Quick double-tap detection (default 300ms window)
- **NoteChord**: Multiple notes pressed simultaneously (default 100ms window)
- **EncoderTurn**: Encoder rotation with direction (Clockwise/CounterClockwise)
- **Aftertouch**: Pressure sensitivity
- **PitchBend**: Touch strip control
- **CC**: Control change messages

### Action Types

- **Keystroke**: Keyboard shortcuts with modifiers
- **Text**: Type text strings
- **Launch**: Open applications
- **Shell**: Execute shell commands
- **VolumeControl**: System volume (Up/Down/Mute/Set)
- **ModeChange**: Switch between mapping modes
- **Sequence**: Chain multiple actions
- **Delay**: Timing control
- **MouseClick**: Mouse simulation
- **Repeat**: Repeat an action N times
- **Conditional**: Conditional execution based on app state, time, etc.

## Configuration (config.toml)

All mappings are defined in TOML format. The config structure:

```toml
[device]
name = "Mikro"
auto_connect = true

[advanced_settings]
chord_timeout_ms = 50
double_tap_timeout_ms = 300
hold_threshold_ms = 2000

[[modes]]
name = "Default"
color = "blue"
[[modes.mappings]]
# Mode-specific mappings...

[[global_mappings]]
# Global mappings (work in all modes)...
```

**Important**: When adding new trigger or action types, you must update:
1. The enum in `config.rs`
2. The processing logic in `event_processor.rs` (for triggers)
3. The matching logic in `mappings.rs`
4. The execution logic in `actions.rs` (for actions)

## LED Feedback System

The LED system uses a trait-based abstraction (`PadFeedback`) to support:
- **HID devices** (Maschine Mikro MK3): Full RGB control via hidapi
- **MIDI devices**: Basic on/off via MIDI Note messages

Available lighting schemes: `off`, `static`, `breathing`, `pulse`, `rainbow`, `wave`, `sparkle`, `reactive`, `vumeter`, `spiral`

**Reactive Mode**: LEDs respond to velocity (green=soft, yellow=medium, red=hard) and fade out 1 second after release.

**Mode Colors**: Each mode has distinct color themes (Mode 0=Blue, Mode 1=Green, Mode 2=Purple).

See LED_FEEDBACK.md for complete documentation.

## Device Profile Support

The system can load Native Instruments Controller Editor profiles (.ncmm3 XML) to:
- Map physical pad positions to MIDI notes (handles different pad pages A-H)
- Auto-detect the active pad page from incoming MIDI events
- Support custom controller configurations

Profile files are parsed in `device_profile.rs` using `quick-xml`.

## Adding New Features

### Adding a New Trigger Type

1. Add variant to `Trigger` enum in `config.rs`
2. Add variant to `ProcessedEvent` enum in `event_processor.rs`
3. Add detection logic in `EventProcessor::process()` (event_processor.rs)
4. Add matching case in `MappingEngine::trigger_matches_processed()` (mappings.rs)

### Adding a New Action Type

1. Add variant to `ActionConfig` enum in `config.rs`
2. Add variant to `Action` enum in `actions.rs`
3. Add execution logic in `ActionExecutor::execute()` (actions.rs)
4. Update `compile_action()` in `mappings.rs` to compile the new config

### Adding a New LED Scheme

1. Add variant to `LightingScheme` enum in `feedback.rs`
2. Implement in `MikroMK3LEDs::run_scheme()` (mikro_leds.rs)
3. Add fallback in `MidiFeedback::run_scheme()` (midi_feedback.rs)
4. Update `LightingScheme::from_str()` and `list_all()`
5. Update help text in `print_usage()` (main.rs)

## Platform-Specific Notes

### macOS
- HID access requires Input Monitoring permissions (System Settings → Privacy & Security)
- Native Instruments Controller Editor can run simultaneously (shared device mode)
- Uses `enigo` for keyboard/mouse simulation
- Volume control uses AppleScript via shell commands

### Linux
- May require udev rules for HID device access
- Consider using `xdotool` for input simulation

### Windows
- USB driver installation may be required
- Check device permissions

## Dependencies

Key external crates:
- **midir**: Cross-platform MIDI I/O
- **enigo**: Keyboard/mouse input simulation
- **hidapi**: HID device access (with `macos-shared-device` feature)
- **serde/toml**: Configuration parsing
- **quick-xml**: XML profile parsing
- **crossbeam-channel**: Lock-free event channels
- **colored**: Terminal output formatting
- **ctrlc**: Graceful shutdown handling

## Troubleshooting

### MIDI Device Not Found
```bash
# Check USB connection
system_profiler SPUSBDataType | grep -i mikro

# List MIDI ports
cargo run --bin test_midi

# Check Audio MIDI Setup
open -a "Audio MIDI Setup"
```

### LEDs Not Working
- Ensure Native Instruments drivers are installed
- Grant Input Monitoring permissions on macOS
- Try `--led reactive` or `--led rainbow` to test
- Check DEBUG=1 output for HID connection errors

### Events Not Triggering
- Run `cargo run --bin midi_diagnostic 2` to verify MIDI events
- Check note numbers match config.toml
- Verify velocity/duration thresholds
- Check mode is correct (encoder to switch modes)

### Profile Detection Issues
- Ensure .ncmm3 file is valid XML from Controller Editor
- Use `--pad-page` to force a specific page instead of auto-detect
- Check DEBUG=1 output for profile parsing errors

## Performance Characteristics

- Response latency: <1ms typical
- Memory usage: 5-10MB
- CPU usage: <1% idle, <5% active
- Binary size: ~3-5MB (release with LTO)

## Release Build Optimizations

The release profile in Cargo.toml uses:
- `opt-level = 3`: Maximum optimization
- `lto = true`: Link-time optimization
- `codegen-units = 1`: Single codegen unit for better optimization
- `strip = true`: Strip debug symbols for smaller binary

---

## Future Workspace Structure (Post-Migration)

When migrating to a monorepo structure, the target layout will be:

```
midimon/
├── Cargo.toml                      # Workspace root
├── midimon-core/                   # Pure Rust engine crate (UI-free)
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── devices.rs              # Device abstraction (MIDI/HID)
│       ├── events.rs               # Event normalization & detection
│       ├── mapping.rs              # Mapping engine
│       ├── state.rs                # State machine for press/hold/chord
│       ├── actions.rs              # Action executor
│       └── config.rs               # Config loading & watching
├── midimon-daemon/                 # Background service (headless)
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs
│       └── menu_bar.rs             # macOS menu bar (status item)
├── midimon-gui/                    # Tauri v2 UI for configuration
│   ├── Cargo.toml
│   ├── src-tauri/
│   │   ├── Cargo.toml
│   │   └── src/main.rs
│   └── ui/                         # Web UI (Svelte/React/Vite)
├── config/
│   ├── default.toml
│   └── device_templates/
│       ├── maschine_mikro_mk3.toml
│       ├── launchpad_mini.toml
│       └── korg_nanokontrol.toml
└── .research/                      # Implementation proposals (reference only)
    ├── implementation-viewpoint-1.md
    └── implementation-viewpoint-2.md
```

### Key Architectural Principles (Post-Migration)

1. **Engine Independence**: `midimon-core` has zero UI dependencies - pure event processing, mapping, and actions
2. **Plugin Architecture**: Device profiles as external TOML templates, easily shareable
3. **Hot Reload**: Config file watching with `notify` crate for zero-downtime updates
4. **Menu Bar UX**: Tauri tray icon with quick actions (Pause, Reload, Open Config)
5. **Auto-Start**: Tauri autostart plugin for macOS LaunchAgent integration
6. **Unified Events**: Normalize MIDI/HID into common `InputEvent` type for consistent handling
7. **State Machine**: Per-element timers for short/long press, double-tap, chord detection
8. **Profile Switching**: Frontmost app detection for context-aware mappings

### Migration Strategy

**Phase 1: Preserve & Document**
- ✅ Tag current working implementation
- ✅ Document in CLAUDE.md and copilot-instructions.md
- Ensure all tests pass and features are documented

**Phase 2: Extract Core Engine**
- Create `midimon-core` crate from existing `src/` modules
- Move device I/O, event processing, mapping engine to core
- Maintain existing config.toml format for compatibility
- Keep all existing trigger/action types working

**Phase 3: Add Daemon & UI**
- Create `midimon-daemon` with menu bar integration
- Add Tauri-based `midimon-gui` for visual configuration
- Implement config hot-reloading
- Add frontmost app detection for per-app profiles

**Phase 4: Enhanced Features**
- MIDI Learn mode (click binding → press device element → auto-fill)
- Virtual MIDI output for DAW integration
- Profile sharing/export
- Live event console for debugging
- Velocity curves and advanced mapping conditions

### References

See `.research/implementation-viewpoint-1.md` and `.research/implementation-viewpoint-2.md` for detailed architectural proposals, crate dependencies, and code examples for the target monorepo structure.
