# Introduction

**Transform any MIDI controller into an advanced, context-aware macro pad with professional-grade feedback and timing-based triggers.**

MIDIMon is a powerful Rust-based MIDI controller mapping system that goes beyond simple note-to-key bindings. It provides velocity sensitivity, long press detection, double-tap, chord recognition, and full RGB LED feedback for modern workflows.

## Key Features

### Core Capabilities

- **4 Core Triggers**: Note, Velocity Range, Encoder, Control Change
- **5 Advanced Triggers**: Long Press, Double-Tap, Chord, Aftertouch, Pitch Bend
- **10 Action Types**: Keystroke, Text, Launch, Shell, Volume Control, and more
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
Manage streaming, recording, and editing with physical MIDI controls.

### Power Users
Replace expensive macro pads with affordable MIDI controllers that do more.

## Why MIDIMon?

Unlike existing MIDI mapping tools, MIDIMon provides:

- **Advanced Timing**: Long press, double-tap, chord detection out of the box
- **Velocity Sensitivity**: Different actions for soft/medium/hard pad hits
- **Full RGB Feedback**: Not just on/off LEDs, but animated schemes and reactive color
- **Modern Architecture**: Fast Rust core, hot-reload config (coming soon), cross-platform
- **Open Source**: Fully customizable, extensible, community-driven

## Quick Example

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

## Platform Support

- **macOS**: Full support (11+ Big Sur, Apple Silicon + Intel)
- **Linux**: Planned for Phase 4 (Q4 2025)
- **Windows**: Planned for Phase 4 (Q4 2025)

## Device Compatibility

- **Fully Supported**: Native Instruments Maschine Mikro MK3 (RGB LEDs, HID access)
- **MIDI-Only Support**: Any USB MIDI controller with basic LED feedback
- **Coming Soon**: Launchpad, APC Mini, Korg nanoKontrol, and more

## Get Started

Ready to dive in? Check out the [Quick Start Guide](getting-started/quick-start.md) or [Installation Instructions](installation/macos.md).

## Project Status

MIDIMon is currently at **v0.1.0-monolithic** (pre-1.0 software). The core features are stable and production-ready, but the API may change before v1.0.0.

See the [Roadmap](resources/roadmap.md) for planned features and timeline.

## Community

- **GitHub**: [https://github.com/amiable-dev/midimon](https://github.com/amiable-dev/midimon)
- **Discussions**: [https://github.com/amiable-dev/midimon/discussions](https://github.com/amiable-dev/midimon/discussions)
- **Issues**: [https://github.com/amiable-dev/midimon/issues](https://github.com/amiable-dev/midimon/issues)

## License

MIDIMon is open source software licensed under the **MIT License**. See [LICENSE](https://github.com/amiable-dev/midimon/blob/main/LICENSE) for details.

---

**Next**: [Install MIDIMon](installation/macos.md) | [Quick Start](getting-started/quick-start.md)
