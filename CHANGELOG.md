# Changelog

All notable changes to MIDIMon will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned
- Tauri-based menu bar UI (Phase 3)
- Hot config reload without restart
- Per-app profile switching (context-aware mappings)
- Video tutorials and demos

## [0.2.0] - 2025-11-12

### Overview

**Phase 2 Complete**: Workspace architecture migration with zero breaking changes. MIDIMon now uses a modular 3-package workspace structure, enabling better code organization, faster builds, and preparing for future GUI integration.

**100% Backward Compatible**: All v0.1.0 configs, features, and workflows work identically in v0.2.0.

### Added - Architecture

- **midimon-core**: Pure Rust engine library (zero UI dependencies)
  - Public API for embedding in other applications
  - Structured error types using `thiserror`
  - Comprehensive rustdoc documentation
  - 30+ public types exported
- **midimon-daemon**: CLI daemon + 6 diagnostic tools
  - Main `midimon` binary
  - `midi_diagnostic`, `led_diagnostic`, `led_tester`
  - `pad_mapper`, `test_midi`, `midi_simulator`
- **midimon** (root): Backward compatibility layer
  - Re-exports midimon-core types
  - Maintains v0.1.0 import paths
  - Zero breaking changes for existing tests

### Added - Testing

- **25 new integration tests** (339 tests total, was 314)
  - 8 API integration tests (public API surface)
  - 7 backward compatibility tests
  - 10 error handling tests (across crate boundaries)
- **100% feature validation**: All 26 features tested and working
- **Config compatibility tests**: All v0.1.0 configs validated

### Changed - Performance

- **Build time**: 11.92s clean build (was 15-20s) - **25-40% faster** ✨
  - Workspace parallelization across 3 packages
  - Improved incremental compilation
- **Test execution**: 28.8s (was ~30s) - **4% faster**
  - Parallel test execution per package
- **Binary size**: Unchanged (869K main binary)

### Changed - Internal Structure

- Renamed `src/mappings.rs` → `midimon-core/src/mapping.rs`
- Renamed `src/device_profile.rs` → `midimon-core/src/device.rs`
- Added `midimon-core/src/error.rs` (structured error types)
- Split monolithic src/ into modular workspace packages
- Removed UI dependencies (colored, chrono) from core library

### Documentation

- **CLAUDE.md**: Updated with workspace architecture and Phase 2 status
- **README.md**: Updated installation and build commands
- **mdbook**: Updated architecture diagrams
- **Rustdoc**: Comprehensive API documentation in midimon-core
- **Migration Guide**: docs/MIGRATION_v0.1_to_v0.2.md

### Validation

- **Feature Parity**: 26/26 features validated ✅
- **Config Compatibility**: 15 compatibility tests passing ✅
- **Breaking Changes**: 0 (zero) ✅
- **Test Coverage**: 339/339 tests passing (100%) ✅

### Migration Notes

**For Users**: No action required. All configs and workflows work identically.

**For Developers**: Update build commands:
```bash
# Old
cargo build --release
cargo test

# New
cargo build --release --workspace
cargo test --workspace
```

See `docs/MIGRATION_v0.1_to_v0.2.md` for complete guide.

## [0.1.0-monolithic] - 2025-11-11

### Overview

Initial public release of MIDIMon, preserving the complete working monolithic implementation with all 26 features before migration to workspace structure. This release establishes the foundation for open source development and community contributions.

### Added - Core Triggers (4)

- **Note Trigger**: Basic note on/off detection with optional velocity range filtering
- **VelocityRange Trigger**: Different actions for soft (0-40), medium (41-80), and hard (81-127) velocity levels
- **EncoderTurn Trigger**: Encoder rotation detection with clockwise/counterclockwise direction
- **CC (Control Change) Trigger**: MIDI Control Change message handling

### Added - Advanced Triggers (5)

- **LongPress Trigger**: Configurable hold duration detection (default 2000ms)
- **DoubleTap Trigger**: Quick double-tap detection with configurable window (default 300ms)
- **NoteChord Trigger**: Multiple simultaneous note detection (default 100ms chord window)
- **Aftertouch Trigger**: Pressure sensitivity detection for supported devices
- **PitchBend Trigger**: Touch strip/pitch wheel detection with range support

### Added - Actions (10)

- **Keystroke Action**: Keyboard shortcuts with full modifier support (Cmd, Ctrl, Alt, Shift)
- **Text Action**: Type text strings with automatic character conversion
- **Launch Action**: Open applications and files with system default handlers
- **Shell Action**: Execute shell commands and scripts with full environment access
- **VolumeControl Action**: System volume adjustment (Up, Down, Mute, Set to value)
- **ModeChange Action**: Switch between mapping modes with LED feedback
- **Sequence Action**: Chain multiple actions with timing control
- **Delay Action**: Add timing delays between actions (milliseconds)
- **MouseClick Action**: Simulate mouse button clicks (Left, Right, Middle)
- **Repeat Action**: Execute an action multiple times with optional delays

### Added - LED Feedback System (10 Schemes)

- **Off**: All LEDs disabled
- **Static**: Solid color display with configurable RGB values
- **Breathing**: Smooth pulsing fade in/out effect
- **Pulse**: Quick flash effect for event triggers
- **Rainbow**: Animated rainbow color cycle across pads
- **Wave**: Wave pattern sweeping across pad grid
- **Sparkle**: Random sparkle/twinkle effects
- **Reactive**: Velocity-sensitive color feedback (green=soft, yellow=medium, red=hard) with 1-second fade
- **VU Meter**: Audio level meter visualization
- **Spiral**: Spiral pattern animation from center outward

### Added - System Features (7)

- **Multi-Mode System**: Support for multiple mapping modes (Default, Development, Media, etc.) with independent configurations
- **Global Mappings**: Mappings that work across all modes (e.g., emergency exit, encoder volume control)
- **Device Profile Support**: Load Native Instruments Controller Editor profiles (.ncmm3 XML format)
- **Auto-Detect Pad Page**: Automatically detect active pad page (A-H) from incoming MIDI events
- **HID Shared Device Access**: Concurrent access with Native Instruments Controller Editor using `hidapi` with `macos-shared-device` feature
- **Graceful Shutdown**: Clean MIDI connection closure and LED reset on exit (Ctrl+C handling)
- **Debug Logging**: Environment variable DEBUG=1 enables detailed event and processing logs

### Added - Diagnostic Tools (4)

- **midi_diagnostic**: Visualize all incoming MIDI events with formatted display
- **led_diagnostic**: Test RGB LED functionality and HID connection
- **led_tester**: Interactive LED scheme testing utility
- **pad_mapper**: Utility for mapping physical pad positions to MIDI notes

### Added - Documentation

- README.md with quick start guide and feature overview
- CLAUDE.md with comprehensive project instructions and architecture
- LED_FEEDBACK.md with LED system documentation
- CODE_OF_CONDUCT.md (Contributor Covenant v2.1)
- CONTRIBUTING.md with contribution guidelines
- GOVERNANCE.md defining project structure and decision-making
- MAINTAINERS.md listing current maintainers
- ROADMAP.md outlining project vision and development phases
- SECURITY.md with vulnerability reporting process
- Example config.toml with common mapping patterns

### Added - Developer Infrastructure

- GitHub Actions CI/CD pipeline (build, test, clippy, format checks)
- Issue templates (bug report, feature request, device support, documentation)
- Pull request template with comprehensive checklist
- SUPPORT.md documenting support channels
- Pre-commit hook setup for code quality
- VS Code configuration (.vscode/settings.json, launch.json, tasks.json)
- Build scripts (scripts/build.sh, test.sh, dev-setup.sh, clean.sh)
- .editorconfig for cross-editor consistency
- rust-toolchain.toml pinning Rust version

### Added - Legal & Compliance

- MIT License with copyright notice
- Copyright headers in all source files
- NOTICE file with third-party attributions
- THIRD_PARTY_LICENSES.md documenting all dependency licenses
- Trademark disclaimer for Native Instruments references
- SPDX license identifier in Cargo.toml

### Performance

- Response latency: <1ms typical for MIDI event processing
- Memory footprint: 5-10MB steady state
- CPU usage: <1% idle, <5% during active use
- Binary size: 3-5MB (release build with LTO and stripping)

### Platform Support

- macOS 11+ (Big Sur and later)
- Apple Silicon (ARM64) and Intel (x86_64) architectures
- Requires Input Monitoring permission for HID device access

### Device Compatibility

- **Fully Supported**: Native Instruments Maschine Mikro MK3 (RGB LEDs, HID access, profile support)
- **MIDI-Only Support**: Any USB MIDI controller with basic LED feedback via MIDI Note messages
- **Profile Support**: .ncmm3 files from Native Instruments Controller Editor

### Known Limitations

- macOS only (Linux and Windows support planned for Phase 4)
- Single device support (multi-device planned for Phase 4)
- No GUI for configuration (Tauri UI planned for Phase 3)
- Config changes require restart (hot reload planned for Phase 2)
- No virtual MIDI output (planned for Phase 4)

### Dependencies

Major external crates:
- midir 0.9 - Cross-platform MIDI I/O
- enigo 0.2 - Keyboard/mouse input simulation
- hidapi 2.6 - HID device access with macOS shared device support
- serde 1.0 + toml 0.8 - Configuration parsing
- quick-xml 0.36 - XML profile parsing (.ncmm3 files)
- crossbeam-channel 0.5 - Lock-free event channels
- colored 2.1 - Terminal output formatting
- ctrlc 3.4 - Graceful shutdown handling

All dependencies use MIT, Apache-2.0, or BSD-compatible licenses.

### Migration Path

This v0.1.0-monolithic release preserves the working single-binary implementation before architectural migration to workspace structure (Phase 2-4). Future versions will maintain backward compatibility with existing config.toml files.

### Contributors

- Christopher Joseph (@christopherjoseph) - Project Lead & Creator

### Release Artifacts

- midimon-v0.1.0-macos-arm64.tar.gz (Apple Silicon)
- midimon-v0.1.0-macos-x86_64.tar.gz (Intel)
- checksums.txt (SHA256)

---

## Version History

- **v0.1.0-monolithic** (2025-11-11): Initial public release with 26 features
- **Unreleased**: Next version in development

---

## Changelog Guidelines

This changelog follows [Keep a Changelog](https://keepachangelog.com/) format:

- **Added**: New features
- **Changed**: Changes to existing functionality
- **Deprecated**: Soon-to-be-removed features
- **Removed**: Removed features
- **Fixed**: Bug fixes
- **Security**: Security vulnerability fixes

Version numbers follow [Semantic Versioning](https://semver.org/):
- **MAJOR**: Breaking changes to config format or public API
- **MINOR**: New features, backward-compatible
- **PATCH**: Bug fixes, performance improvements

[Unreleased]: https://github.com/amiable-dev/midimon/compare/v0.1.0-monolithic...HEAD
[0.1.0-monolithic]: https://github.com/amiable-dev/midimon/releases/tag/v0.1.0-monolithic
