# MIDIMon

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![Build Status](https://github.com/amiable-dev/midimon/workflows/CI/badge.svg)](https://github.com/amiable-dev/midimon/actions)
[![codecov](https://codecov.io/gh/amiable-dev/midimon/branch/main/graph/badge.svg)](https://codecov.io/gh/amiable-dev/midimon)
[![Documentation](https://img.shields.io/badge/docs-latest-blue.svg)](https://amiable-dev.github.io/midimon/)

Transform MIDI controllers into advanced macro pads with velocity sensitivity, LED feedback, and modular architecture.

**v0.2.0**: Now with workspace architecture - pure Rust engine library + CLI daemon

![MIDIMon Demo](docs/images/hero-demo.gif)
*Velocity-sensitive RGB LED feedback on Native Instruments Maschine Mikro MK3*

## Features

### Core Features
- **Multi-mode operation** - Switch between different mapping sets
- **Configurable mappings** - Easy TOML-based configuration
- **Low latency** - Sub-millisecond response time
- **Cross-platform** - Works on macOS, Linux, and Windows

### Enhanced Event Detection
- **Velocity Sensitivity** - Different actions for soft/medium/hard presses
- **Long Press Detection** - Hold actions with configurable thresholds
- **Double-Tap Detection** - Quick double-tap triggers
- **Chord Detection** - Multiple pads pressed simultaneously
- **Encoder Direction** - Clockwise/counter-clockwise detection
- **Aftertouch Support** - Pressure-sensitive actions
- **Pitch Bend Support** - Touch strip integration

### LED Feedback System
- **Visual Feedback** - Real-time LED feedback on supported devices
- **Multiple Schemes** - Rainbow, pulse, breathing, reactive, and more
- **Mode Indication** - Color-coded modes for easy identification
- **Velocity Visualization** - LED brightness matches pad velocity
- **HID Support** - Full RGB control for Maschine Mikro MK3
- **MIDI LED** - Basic feedback for standard MIDI devices

> 📖 **See [LED_FEEDBACK.md](LED_FEEDBACK.md) for complete LED system documentation**

## Hardware Compatibility

| Device | Status | LED Feedback | Notes |
|--------|--------|--------------|-------|
| Native Instruments Maschine Mikro MK3 | ✅ Full Support | RGB (HID) | Recommended |
| Generic MIDI Controllers | ✅ Supported | Basic (MIDI) | Most features work |
| Akai APC Mini | ⚠️ Untested | Basic (MIDI) | Should work |
| Novation Launchpad | ⚠️ Untested | Basic (MIDI) | Should work |

**Want to add support for your device?** See [CONTRIBUTING.md](CONTRIBUTING.md#device-support)

## Installation

### From Binary (Recommended)

Download the latest release for your platform:
- [macOS (Intel)](https://github.com/amiable-dev/midimon/releases/latest/download/midimon-macos-intel)
- [macOS (Apple Silicon)](https://github.com/amiable-dev/midimon/releases/latest/download/midimon-macos-arm)

```bash
# Make it executable
chmod +x midimon-*

# Run
./midimon-* 2  # Connect to MIDI port 2
```

### From Source

```bash
# Clone the repository
git clone https://github.com/amiable-dev/midimon.git
cd midimon

# Build the workspace (all 3 packages)
cargo build --release --workspace

# Run the main daemon
cargo run --release --bin midimon 2  # Connect to port 2

# Or run directly
./target/release/midimon 2
```

**Workspace Structure** (v0.2.0):
- `midimon-core`: Pure Rust engine library (zero UI dependencies)
- `midimon-daemon`: CLI daemon + 6 diagnostic tools
- `midimon`: Backward compatibility layer

**Requirements:**
- Rust 1.70+ ([Install via rustup](https://rustup.rs/))
- macOS 10.15+ (Linux/Windows support planned)

## Quick Start

1. **Connect your MIDI controller** (e.g., Native Instruments Maschine Mikro MK3)
2. **Install necessary drivers** (Native Instruments Controller Editor for NI devices)
3. **Run the application**:
   ```bash
   cargo run --release [port_number]
   ```
4. **Press pads** to trigger macros!

## Configuration

Edit `config.toml` to customize your mappings. The enhanced configuration supports:

### Basic Note Trigger
```toml
[[modes.mappings]]
description = "Spotlight Search"
[modes.mappings.trigger]
type = "Note"
note = 12
velocity_min = 1  # Optional
velocity_max = 127  # Optional
[modes.mappings.action]
type = "Keystroke"
keys = "space"
modifiers = ["cmd"]
```

### Velocity-Sensitive Actions
```toml
# Soft press
[[modes.mappings]]
description = "Volume Down (soft)"
[modes.mappings.trigger]
type = "VelocityRange"
note = 13
min_velocity = 1
max_velocity = 40
[modes.mappings.action]
type = "VolumeControl"
action = "Down"

# Hard press
[[modes.mappings]]
description = "Volume Up (hard)"
[modes.mappings.trigger]
type = "VelocityRange"
note = 13
min_velocity = 80
max_velocity = 127
[modes.mappings.action]
type = "VolumeControl"
action = "Up"
```

### Long Press
```toml
[[modes.mappings]]
description = "Quit App (long press)"
[modes.mappings.trigger]
type = "LongPress"
note = 4
min_duration_ms = 1500
[modes.mappings.action]
type = "Keystroke"
keys = "q"
modifiers = ["cmd"]
```

### Double-Tap
```toml
[[modes.mappings]]
description = "Fullscreen (double tap)"
[modes.mappings.trigger]
type = "DoubleTap"
note = 16
max_interval_ms = 300
[modes.mappings.action]
type = "Keystroke"
keys = "f"
modifiers = ["ctrl", "cmd"]
```

### Chord Detection
```toml
[[modes.mappings]]
description = "Force Quit (chord)"
[modes.mappings.trigger]
type = "NoteChord"
notes = [8, 12]  # Both pads must be pressed
max_interval_ms = 100
[modes.mappings.action]
type = "Keystroke"
keys = "escape"
modifiers = ["cmd", "option"]
```

### Encoder Actions
```toml
[[global_mappings]]
description = "Volume Up"
[global_mappings.trigger]
type = "EncoderTurn"
cc = 2
direction = "Clockwise"
[global_mappings.action]
type = "VolumeControl"
action = "Up"
```

## Diagnostic Tools

### MIDI Diagnostic Tool
Visualize all MIDI events from your controller:
```bash
cargo run --bin midi_diagnostic 2
```

Features:
- Real-time event visualization
- Velocity bars
- Hold duration tracking
- Beautiful colored output

### Test MIDI Ports
List all available MIDI devices:
```bash
cargo run --bin test_midi
```

## Action Types

### Basic Actions
- **Keystroke** - Simulate keyboard shortcuts
- **Text** - Type text strings
- **Launch** - Open applications
- **Shell** - Execute shell commands
- **Delay** - Add delays between actions
- **MouseClick** - Simulate mouse clicks

### Advanced Actions
- **VolumeControl** - System volume control (Up/Down/Mute/Set)
- **ModeChange** - Switch between mapping modes
- **Sequence** - Chain multiple actions
- **Repeat** - Repeat an action multiple times
- **Conditional** - Execute based on conditions

## Modes

The system supports multiple modes, each with its own set of mappings:

1. **Default Mode** - General productivity shortcuts
2. **Development Mode** - Git commands, build tools
3. **Media Mode** - Music and video controls
4. **Custom Modes** - Create your own!

Switch modes using:
- Encoder rotation
- Specific pad combinations
- CC messages

## Troubleshooting

### MIDI Device Not Found
1. Check if device is connected: `system_profiler SPUSBDataType | grep -i mikro`
2. Install necessary drivers (e.g., Native Instruments Controller Editor)
3. Check Audio MIDI Setup: `open -a "Audio MIDI Setup"`

### High Latency
- Build in release mode: `cargo build --release`
- Close unnecessary applications
- Check CPU usage

### Events Not Triggering
- Run diagnostic tool to verify MIDI events
- Check config.toml for correct note numbers
- Verify velocity/duration thresholds

## Advanced Configuration

### Timing Settings
```toml
[advanced_settings]
chord_timeout_ms = 50        # Max time between notes for chord
double_tap_timeout_ms = 300  # Max time between taps
hold_threshold_ms = 2000     # Time before hold is detected
```

### Conditional Actions
```toml
[[modes.mappings]]
description = "Context-aware action"
[modes.mappings.trigger]
type = "Note"
note = 20
[modes.mappings.action]
type = "Conditional"
[modes.mappings.action.condition]
type = "AppRunning"
app_name = "Terminal"
[modes.mappings.action.then_action]
type = "Keystroke"
keys = "c"
modifiers = ["ctrl"]
[modes.mappings.action.else_action]
type = "Launch"
app = "Terminal"
```

## Development

### Project Structure
```
midimon/
├── src/
│   ├── main.rs           # Main application
│   ├── config.rs         # Configuration structures
│   ├── actions.rs        # Action execution
│   ├── mappings.rs       # Trigger mapping engine
│   ├── event_processor.rs # Enhanced event processing
│   └── bin/
│       ├── test_midi.rs  # MIDI port tester
│       └── midi_diagnostic.rs # Diagnostic tool
├── config.toml           # User configuration
└── Cargo.toml           # Dependencies
```

### Adding New Trigger Types
1. Add to `Trigger` enum in `config.rs`
2. Add to `ProcessedEvent` enum in `event_processor.rs`
3. Update `EventProcessor::process()` to detect the trigger
4. Update `MappingEngine::trigger_matches_processed()` to handle matching

### Adding New Action Types
1. Add to `ActionConfig` enum in `config.rs`
2. Add to `Action` enum in `actions.rs`
3. Update `ActionExecutor::execute()` to handle the action

## Performance

- **Response Time**: < 1ms typical latency
- **Memory Usage**: 5-10MB
- **CPU Usage**: < 1% idle, < 5% active
- **Binary Size**: ~3-5MB (release build with LTO)

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for:
- How to report bugs
- How to propose features
- Development setup guide
- Coding standards
- Pull request process

Check out [good first issues](https://github.com/amiable-dev/midimon/labels/good-first-issue) to get started.

## Documentation

- **[Full Documentation](https://amiable-dev.github.io/midimon/)** - Complete user guide
- **[LED Feedback System](LED_FEEDBACK.md)** - LED control documentation
- **[Configuration Reference](docs/configuration.md)** - TOML configuration guide
- **[API Documentation](https://docs.rs/midimon)** - Rust API docs

## Community & Support

- **[Discussions](https://github.com/amiable-dev/midimon/discussions)** - Ask questions, share configs
- **[Issues](https://github.com/amiable-dev/midimon/issues)** - Bug reports, feature requests
- **[Security](SECURITY.md)** - Security vulnerability reporting

## Roadmap

### Current: Phase 0 - v0.1.0-monolithic
- ✅ Single-binary implementation with all core features
- ✅ Velocity sensitivity, long press, double-tap, chords
- ✅ Full RGB LED feedback for Maschine Mikro MK3
- ✅ Multi-mode operation
- ✅ Device profile support (.ncmm3)

### Phase 1 - v0.2.0
- 📋 Complete feature specifications
- 📋 85%+ test coverage
- 📋 Comprehensive documentation

### Phase 2 - v1.0.0
- 🔮 Monorepo migration (midimon-core, midimon-daemon, midimon-gui)
- 🔮 Background daemon with menu bar
- 🔮 Config hot-reload

### Phase 3 - v1.5.0
- 🔮 Tauri-based visual configurator
- 🔮 MIDI Learn mode
- 🔮 Per-app profiles

See [docs/implementation-roadmap.md](docs/implementation-roadmap.md) for full roadmap.

## License

MIDIMon is licensed under the [MIT License](LICENSE).

Copyright (c) 2025 Amiable

## Credits

Built with:
- [midir](https://github.com/Boddlnagg/midir) - MIDI I/O
- [enigo](https://github.com/enigo-rs/enigo) - Input simulation
- [colored](https://github.com/mackwic/colored) - Terminal colors
- [serde](https://serde.rs/) - Serialization
