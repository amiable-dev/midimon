# Introduction

**Transform any MIDI controller or gamepad into an advanced, context-aware automation platform with professional-grade feedback and timing-based triggers.**

MIDIMon is a powerful Rust-based multi-protocol input mapping system that goes beyond simple note-to-key bindings. Supporting both MIDI controllers and HID gamepads, it provides velocity sensitivity, long press detection, double-tap, chord recognition, and full RGB LED feedback for modern workflows.

## Key Features

### Multi-Protocol Input (v3.0+)

**MIDI Controllers**:
- Full MIDI controller support with RGB LED feedback
- Native Instruments Maschine, Launchpad, APC Mini, and more
- SysEx support for advanced LED control

**HID Gamepads** (NEW!):
- Xbox 360/One/Series controllers
- PlayStation DualShock 4/DualSense controllers
- Nintendo Switch Pro Controller
- Button chords, analog sticks, triggers with full velocity sensitivity
- See the [Gamepad Support Guide](guides/gamepad-support.md) for details

**Coming Soon**:
- OSC (Open Sound Control) for networked devices
- Custom USB HID devices
- Keyboard/mouse intercept for hybrid workflows

### Core Capabilities (v2.0.0)

**Event Detection**:
- **4 Core Triggers**: Note, Velocity Range, Encoder, Control Change
- **5 Advanced Triggers**: Long Press, Double-Tap, Chord, Aftertouch, Pitch Bend
- **10 Action Types**: Keystroke, Text, Launch, Shell, Volume Control, and more

**Visual Interface (NEW!)**:
- **Tauri GUI**: Modern desktop app for visual configuration
- **MIDI Learn**: One-click auto-detection of MIDI inputs
- **Live Preview**: Real-time event monitoring and testing
- **Device Templates**: 6 built-in templates for popular controllers

**Daemon Infrastructure (NEW!)**:
- **Background Service**: Runs as system service with auto-start
- **Hot-Reload**: Config changes applied in 0-10ms without restart
- **IPC Control**: Control daemon via CLI or GUI
- **Per-App Profiles**: Automatic profile switching

**LED & Feedback**:
- **10 LED Schemes**: Reactive, Rainbow, Pulse, Breathing, and custom patterns
- **Multi-Mode System**: Switch between mapping sets on the fly
- **Device Profile Support**: Load Native Instruments Controller Editor configurations

### Performance

- Response latency: **<1ms** typical
- Memory footprint: **5-10MB**
- CPU usage: **<1%** idle, **<5%** active
- Binary size: **3-5MB** (optimized)

## Who is MIDIMon For?

### Music Producers
Control DAWs, plugins, and effects with velocity-sensitive, multi-layer mappings.

### Software Developers
Streamline coding workflows with mode-based hotkey systems and build tool shortcuts.

### Content Creators
Manage streaming, recording, and editing with physical controllers or use your existing gamepad as a powerful macro pad.

### Power Users
Replace expensive macro pads with affordable MIDI controllers or repurpose your $30 Xbox controller for advanced automation.

## Why MIDIMon?

Unlike existing MIDI mapping tools, MIDIMon provides:

- **Multi-Protocol Support**: Use MIDI controllers AND gamepads in the same workflow (v3.0+)
- **Advanced Timing**: Long press, double-tap, chord detection out of the box
- **Velocity Sensitivity**: Different actions for soft/medium/hard pad hits
- **Full RGB Feedback**: Not just on/off LEDs, but animated schemes and reactive color
- **Modern Architecture**: Fast Rust core, hot-reload config, cross-platform
- **Open Source**: Fully customizable, extensible, community-driven

## Quick Examples

### MIDI Controller
```toml
# Press pad lightly for copy, hard for paste
[[modes.mappings]]
trigger = { type = "VelocityRange", note = 36 }
soft = { action = { type = "Keystroke", key = "C", modifiers = ["Cmd"] } }
hard = { action = { type = "Keystroke", key = "V", modifiers = ["Cmd"] } }

# Hold for 2 seconds to open terminal
[[modes.mappings]]
trigger = { type = "LongPress", note = 37, duration_ms = 2000 }
action = { type = "Launch", path = "/Applications/Utilities/Terminal.app" }
```

### Gamepad (v3.0+)
```toml
# Press A button to build your project
[[modes.mappings]]
trigger = { type = "GamepadButton", button = "South" }  # A on Xbox, X on PlayStation
action = { type = "Shell", command = "cargo build" }

# Hold B button for 1 second to run tests
[[modes.mappings]]
trigger = { type = "GamepadButton", button = "East", hold_ms = 1000 }
action = { type = "Shell", command = "cargo test" }
```

## Platform Support

- **macOS**: Full support (11+ Big Sur, Apple Silicon + Intel)
- **Linux**: Planned for Phase 4 (Q4 2025)
- **Windows**: Planned for Phase 4 (Q4 2025)

## Device Compatibility

### MIDI Controllers
- **Fully Supported**: Native Instruments Maschine Mikro MK3 (RGB LEDs, HID access)
- **MIDI-Only Support**: Any USB MIDI controller with basic LED feedback
- **Coming Soon**: Launchpad, APC Mini, Korg nanoKontrol, and more

### HID Gamepads (v3.0+)
- **Xbox**: Xbox 360, Xbox One, Xbox Series X/S controllers
- **PlayStation**: DualShock 4, DualSense (PS5)
- **Nintendo**: Switch Pro Controller
- **Generic**: Any gamepad with standard HID support

## Get Started

Ready to dive in? Check out the [Quick Start Guide](getting-started/quick-start.md) or [Installation Instructions](installation/macos.md).

## Project Status

MIDIMon is currently at **v3.0** with multi-protocol input support, production-ready daemon infrastructure, and visual GUI configuration.

**What's New in v3.0**:
- 🎮 **HID Gamepad Support**: Xbox, PlayStation, Switch Pro controllers
- 🎯 **Unified Input Manager**: MIDI + gamepad in single workflow
- 📦 **Controller Templates**: 3 official gamepad templates (Xbox, PS, Switch)
- 🔍 **MIDI Learn for Gamepads**: Auto-detect gamepad buttons
- ⚡ **Hot-Plug Detection**: Automatic reconnection with exponential backoff

**v2.0.0 Features**:
- 🎛️ **Tauri GUI**: Visual configuration editor with MIDI Learn mode
- 🔄 **Hot-Reload Daemon**: 0-10ms config reloads without restart
- 🎯 **Per-App Profiles**: Automatic profile switching based on active app
- 📊 **Live Event Console**: Real-time event monitoring
- 📦 **Device Templates**: 6 built-in MIDI templates

See the [Roadmap](resources/roadmap.md) for planned features and [Changelog](resources/changelog.md) for full release notes.

## Community

- **GitHub**: [https://github.com/amiable-dev/midimon](https://github.com/amiable-dev/midimon)
- **Discussions**: [https://github.com/amiable-dev/midimon/discussions](https://github.com/amiable-dev/midimon/discussions)
- **Issues**: [https://github.com/amiable-dev/midimon/issues](https://github.com/amiable-dev/midimon/issues)

## License

MIDIMon is open source software licensed under the **MIT License**. See [LICENSE](https://github.com/amiable-dev/midimon/blob/main/LICENSE) for details.

---

**Next**: [Install MIDIMon](installation/macos.md) | [Quick Start](getting-started/quick-start.md)
