# Changelog

All notable changes to MIDIMon will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned
- Documentation site content (user guides, reference docs)
- Comprehensive test coverage (80%+ target)
- Video tutorials and demos
- Hot config reload without restart
- Per-app profile switching (context-aware mappings)

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
