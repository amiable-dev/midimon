# Quick Start

Get MIDIMon up and running in under 5 minutes.

## Prerequisites

- **Rust toolchain** (1.70+) - [Install from rustup.rs](https://rustup.rs/)
- **Native Instruments Maschine Mikro MK3** (or any MIDI controller)
- **macOS** (primary support - Linux/Windows planned for Q4 2025)

## Step 1: Clone and Build

```bash
# Clone the repository
git clone https://github.com/amiable-dev/midimon.git
cd midimon

# Build release binary (optimized, ~3-5MB)
cargo build --release
```

The release build takes 2-3 minutes and produces a highly optimized binary at `target/release/midimon`.

## Step 2: Connect Your Device

1. **Plug in your MIDI controller** via USB
2. **List available MIDI ports**:

```bash
cargo run --release
```

You'll see output like:

```
Available MIDI input ports:
  0: CoreMIDI - Mikro MK3 Public
  1: CoreMIDI - Mikro MK3 Private
  2: CoreMIDI - IAC Driver Bus 1

Select a port number to connect...
```

3. **Note the port number** for your device (usually port `0` or `2` for Mikro MK3)

## Step 3: First Run

Connect to your device:

```bash
cargo run --release 2
```

(Replace `2` with your device's port number)

You should see:

```
Connected to MIDI port 2: CoreMIDI - Mikro MK3 Public
Loaded config from: config.toml
Current mode: Default
Listening for MIDI events... (Press Ctrl+C to exit)
```

## Step 4: Test Your First Mapping

The included `config.toml` has a basic mapping:

```toml
[[modes.mappings]]
trigger = { Note = { note = 60, velocity_min = 0 } }
action = { Keystroke = { keys = ["Space"] } }
```

**Test it**:
1. Press the first pad (Note 60)
2. It should trigger a Space key press
3. LEDs should light up (if your device supports RGB)

## Step 5: Enable LED Feedback (Optional)

If you have a Maschine Mikro MK3, enable reactive LED feedback:

```bash
cargo run --release 2 --led reactive
```

Now pads will:
- Light up **green** for soft press (velocity 0-40)
- Light up **yellow** for medium press (velocity 41-80)
- Light up **red** for hard press (velocity 81-127)
- Fade out over 1 second after release

Try other LED schemes:
```bash
cargo run --release 2 --led rainbow   # Rainbow wave animation
cargo run --release 2 --led pulse     # Pulsing brightness
cargo run --release 2 --led spiral    # Spiral pattern
```

See all schemes: [LED Feedback Configuration](../configuration/led-feedback.md)

## Step 6: Customize Your Mappings

The default config is at `config.toml` in the project root. Edit it to add your own mappings:

```toml
[[modes.mappings]]
trigger = { Note = { note = 61, velocity_min = 0 } }
action = { Keystroke = { keys = ["Cmd", "T"] } }  # Open new tab
```

Reload the config:
- **Stop** MIDIMon (Ctrl+C)
- **Start** it again: `cargo run --release 2`

> **Future**: Hot-reloading (Phase 2+) will let you reload config without restarting

## Step 7: Explore Advanced Features

### Velocity Sensitivity

Different actions based on how hard you hit the pad:

```toml
[[modes.mappings]]
trigger = { VelocityRange = { note = 62, min = 0, max = 40 } }
action = { Text = { text = "soft" } }

[[modes.mappings]]
trigger = { VelocityRange = { note = 62, min = 81, max = 127 } }
action = { Text = { text = "HARD" } }
```

### Long Press Detection

Hold a pad for 2 seconds to trigger a different action:

```toml
[[modes.mappings]]
trigger = { LongPress = { note = 63, velocity_min = 0, hold_ms = 2000 } }
action = { Shell = { command = "open -a Calculator" } }
```

### Chord Detection

Press multiple pads simultaneously (within 100ms):

```toml
[[modes.mappings]]
trigger = { NoteChord = { notes = [60, 64, 67], velocity_min = 0 } }
action = { Text = { text = "C Major Chord!" } }
```

## Troubleshooting

### Device Not Found

```bash
# Check USB connection
system_profiler SPUSBDataType | grep -i mikro

# List MIDI ports again
cargo run --release

# Check Audio MIDI Setup
open -a "Audio MIDI Setup"
```

### LEDs Not Working

1. Ensure Native Instruments drivers are installed
2. Grant **Input Monitoring** permissions:
   - **System Settings** → **Privacy & Security** → **Input Monitoring**
   - Enable for your Terminal app
3. Try different LED schemes: `--led reactive`, `--led rainbow`
4. Check debug output: `DEBUG=1 cargo run --release 2`

### Events Not Triggering

1. **Run diagnostics** to verify MIDI events:
   ```bash
   cargo run --bin midi_diagnostic 2
   ```
2. **Check note numbers** match your config:
   ```bash
   cargo run --bin pad_mapper
   ```
3. **Verify velocity thresholds** aren't too restrictive
4. **Check mode** - is the mapping in the current mode or global?

### Permission Denied (macOS)

If you see "Permission denied" for HID devices:
1. **System Settings** → **Privacy & Security** → **Input Monitoring**
2. Add your Terminal app (Terminal.app, iTerm2, etc.)
3. Restart your terminal
4. Run MIDIMon again

## Next Steps

- [Create Your First Mapping](./first-mapping.md) - Step-by-step mapping tutorial
- [Understanding Modes](./modes.md) - Multi-mode workflow management
- [Configuration Overview](../configuration/overview.md) - Complete config reference
- [Example Configurations](../configuration/examples.md) - Pre-built configs

## Performance

MIDIMon is highly optimized:
- **Response latency**: <1ms typical
- **Memory usage**: 5-10MB
- **CPU usage**: <1% idle, <5% active
- **Binary size**: ~3-5MB (release with LTO)

## Debug Mode

Enable detailed logging for troubleshooting:

```bash
DEBUG=1 cargo run --release 2
```

You'll see:
- MIDI event parsing details
- Trigger matching logic
- Action execution results
- LED feedback commands
- Error stack traces

## Getting Help

- [Troubleshooting Guide](../troubleshooting/common-issues.md)
- [FAQ](../troubleshooting/faq.md)
- [GitHub Discussions](https://github.com/amiable-dev/midimon/discussions)
- [Report Issues](https://github.com/amiable-dev/midimon/issues)
