# CLI Commands Reference

## Overview

MIDIMon provides a main binary and several diagnostic tools, all accessible via the command line. This reference covers all available commands, their options, and usage examples.

## Main Binary: midimon

The primary MIDIMon application.

### Basic Syntax

```bash
# Via cargo
cargo run --release [PORT] [OPTIONS]

# Or directly
./target/release/midimon [PORT] [OPTIONS]
```

### Arguments

#### PORT (Required in some cases)

The MIDI input port number to connect to.

**Finding available ports**:
```bash
# List all MIDI ports
cargo run --release

# Or
./target/release/midimon
```

Output:
```
Available MIDI input ports:
0: USB MIDI Device
1: IAC Driver Bus 1
2: Maschine Mikro MK3 - Input
3: Digital Keyboard
```

**Usage**:
```bash
# Connect to port 2 (Maschine Mikro MK3)
cargo run --release 2

# Connect to port 0
cargo run --release 0
```

**Note**: If `auto_connect = true` in `config.toml`, the port argument is optional and MIDIMon will connect to the first available port.

### Options

#### --led, --lighting <SCHEME>

Select LED lighting scheme (for devices with LED support).

**Syntax**:
```bash
cargo run --release 2 --led <SCHEME>
# or
cargo run --release 2 --lighting <SCHEME>
```

**Available schemes**:
- `reactive` (default) - Velocity-based colors, fade out after release
- `rainbow` - Static rainbow gradient
- `breathing` - Breathing effect (all pads)
- `pulse` - Pulsing effect
- `wave` - Wave pattern with brightness gradient
- `sparkle` - Random twinkling LEDs
- `vumeter` - VU meter style gradient (green → yellow → red)
- `spiral` - Spiral/diagonal pattern
- `static` - Static single color
- `off` - LEDs disabled

**Examples**:
```bash
# Reactive mode (velocity-sensitive)
cargo run --release 2 --led reactive

# Rainbow gradient
cargo run --release 2 --led rainbow

# Breathing effect
cargo run --release 2 --lighting breathing

# Turn off LEDs
cargo run --release 2 --led off
```

**LED Behavior by Scheme**:

**reactive**:
- Soft press (velocity < 50): Green Dim
- Medium press (50 ≤ velocity < 100): Yellow Normal
- Hard press (velocity ≥ 100): Red Bright
- Fades out 1 second after release

**rainbow**:
- Static rainbow gradient across all pads
- No animation (constant colors)

**sparkle**:
- Random white LEDs
- 20% probability per pad per frame
- Updates every 100ms

**vumeter**:
- Green (bottom rows)
- Yellow/Orange (middle)
- Red (top)

**wave**:
- Blue with varying brightness
- Creates wave effect

#### --profile, -p <PATH>

Load a Native Instruments Controller Editor profile (.ncmm3 file).

**Syntax**:
```bash
cargo run --release 2 --profile <PATH>
# or
cargo run --release 2 -p <PATH>
```

**Examples**:
```bash
# Relative path
cargo run --release 2 --profile my-profile.ncmm3

# Absolute path
cargo run --release 2 --profile ~/Downloads/base-template-ni-mikro-mk3.ncmm3

# macOS default location
cargo run --release 2 --profile "$HOME/Documents/Native Instruments/Controller Editor/Profiles/my-profile.ncmm3"
```

**What profiles do**:
- Map physical pad positions to MIDI note numbers
- Support 8 pad pages (A-H) per profile
- Enable correct LED feedback for custom layouts
- Allow seamless integration with NI Controller Editor

**Creating profiles**:
1. Open Native Instruments Controller Editor
2. Select "Maschine Mikro MK3"
3. Edit pad pages (A-H)
4. Assign MIDI notes to each pad
5. Save as `.ncmm3` file
6. Use with `--profile` flag

See [Device Profiles Documentation](../DEVICE_PROFILES.md) for complete guide.

#### --pad-page <PAGE>

Force a specific pad page when using a profile (instead of auto-detection).

**Syntax**:
```bash
cargo run --release 2 --profile <PATH> --pad-page <PAGE>
```

**Valid pages**: A, B, C, D, E, F, G, H (case-insensitive)

**Examples**:
```bash
# Force page A
cargo run --release 2 --profile my-profile.ncmm3 --pad-page A

# Force page H
cargo run --release 2 --profile my-profile.ncmm3 --pad-page h
```

**When to use**:
- Auto-detection not working correctly
- Want to lock to a specific page
- Testing specific page mappings
- Profile has identical notes across multiple pages

**Default behavior** (without `--pad-page`):
- Auto-detect active page from incoming MIDI notes
- Switch pages automatically when notes from different page detected

#### --config, -c <PATH>

Specify custom configuration file location.

**Syntax**:
```bash
cargo run --release 2 --config <PATH>
# or
cargo run --release 2 -c <PATH>
```

**Examples**:
```bash
# Use alternative config
cargo run --release 2 --config config-dev.toml

# Full path
cargo run --release 2 --config /etc/midimon/config.toml
```

**Default**: `./config.toml` (current directory)

#### --help, -h

Display help message with all available options.

**Syntax**:
```bash
cargo run --release -- --help
# or
./target/release/midimon --help
```

Note the `--` separator when using `cargo run`.

#### --version, -v

Display MIDIMon version.

**Syntax**:
```bash
cargo run --release -- --version
# or
./target/release/midimon --version
```

### Environment Variables

#### DEBUG=1

Enable verbose debug logging.

**Syntax**:
```bash
DEBUG=1 cargo run --release 2
```

**Output includes**:
- MIDI event details (note on/off, velocity, channel)
- HID connection status
- LED updates (buffer contents)
- Event processing (velocity detection, long press, chords)
- Mapping matches and action execution
- Mode changes
- Error details

**Example debug output**:
```
[DEBUG] Connected to MIDI port 2: Maschine Mikro MK3 - Input
[DEBUG] HID device opened successfully
[DEBUG] LED controller initialized
[DEBUG] Loaded config with 3 modes, 24 mappings
[DEBUG] Starting in mode 0: Default

[MIDI] NoteOn ch:0 note:12 vel:87
[DEBUG] Processed: Note(12) with velocity Medium
[DEBUG] Matched mapping: "Copy text" (mode: Default)
[DEBUG] Executing action: Keystroke(keys: "c", modifiers: ["cmd"])
[DEBUG] LED update: pad 0 -> color 7 (Green) brightness 2
[MIDI] NoteOff ch:0 note:12 vel:0
[DEBUG] LED fade: pad 0 cleared after 1000ms
```

**When to use**:
- Troubleshooting mapping issues
- Debugging note number mismatches
- Verifying LED control
- Understanding event processing
- Investigating performance issues

#### RUST_LOG

Control Rust logging levels (for development).

**Syntax**:
```bash
RUST_LOG=debug cargo run --release 2
RUST_LOG=trace cargo run --release 2
RUST_LOG=info cargo run --release 2
```

**Levels**:
- `error` - Only errors
- `warn` - Warnings and errors
- `info` - General information (default)
- `debug` - Debug information
- `trace` - Very verbose

**Filter by module**:
```bash
# Only log MIDI events
RUST_LOG=midimon::midi=debug cargo run --release 2

# Multiple modules
RUST_LOG=midimon::event_processor=debug,midimon::mappings=trace cargo run --release 2
```

### Complete Usage Examples

#### Example 1: Basic Usage

```bash
# List ports
cargo run --release

# Connect to port 2 with default settings
cargo run --release 2
```

#### Example 2: With LED Lighting

```bash
# Reactive mode (default)
cargo run --release 2 --led reactive

# Rainbow gradient
cargo run --release 2 --led rainbow

# Sparkle effect
cargo run --release 2 --led sparkle
```

#### Example 3: With Profile

```bash
# Auto-detect page
cargo run --release 2 --profile my-profile.ncmm3

# Force specific page
cargo run --release 2 --profile my-profile.ncmm3 --pad-page H

# With LED scheme
cargo run --release 2 --profile my-profile.ncmm3 --led reactive
```

#### Example 4: Debug Mode

```bash
# Enable debug output
DEBUG=1 cargo run --release 2

# With all options
DEBUG=1 cargo run --release 2 --profile my-profile.ncmm3 --led reactive
```

#### Example 5: Custom Config

```bash
# Use development config
cargo run --release 2 --config config-dev.toml

# Use config from different directory
cargo run --release 2 --config ~/midimon-configs/work.toml
```

#### Example 6: Production Binary

```bash
# Build release
cargo build --release

# Run with all options
./target/release/midimon 2 \
    --profile ~/Documents/NI/my-profile.ncmm3 \
    --led reactive \
    --config ~/midimon-configs/production.toml
```

## Diagnostic Tools

MIDIMon includes several diagnostic utilities for debugging and configuration.

### midi_diagnostic

Visualize all incoming MIDI events in real-time.

**Purpose**: Debug MIDI connectivity, view raw MIDI data, verify device is sending events.

**Syntax**:
```bash
cargo run --bin midi_diagnostic [PORT]
```

**Example**:
```bash
# Connect to port 2
cargo run --bin midi_diagnostic 2
```

**Output**:
```
Connected to MIDI port 2: Maschine Mikro MK3 - Input
Listening for MIDI events... (Ctrl+C to exit)

[NoteOn]  ch:0 note:12 vel:87
[NoteOff] ch:0 note:12 vel:0
[NoteOn]  ch:0 note:13 vel:45
[NoteOff] ch:0 note:13 vel:0
[CC]      ch:0 cc:1 value:64
[PitchBend] ch:0 value:8192
```

**Event types shown**:
- `NoteOn` - Pad/key pressed
- `NoteOff` - Pad/key released
- `CC` - Control Change (knobs, sliders)
- `PitchBend` - Touch strip, pitch wheel
- `Aftertouch` - Pressure sensitivity
- `ProgramChange` - Program/patch change

**When to use**:
- Verify MIDI device is connected
- Find note numbers for pads/keys
- Debug why mappings aren't triggering
- Check velocity ranges
- Verify CC numbers for encoders/knobs

**Press Ctrl+C to exit.**

### led_diagnostic

Test LED functionality and HID connection.

**Purpose**: Verify HID access, test LED control, debug LED issues.

**Syntax**:
```bash
cargo run --bin led_diagnostic
```

**What it does**:
1. Attempts to open HID device
2. Displays connection status
3. Tests individual LED control
4. Cycles through all pads with different colors

**Output**:
```
LED Diagnostic Tool
==================

Searching for Maschine Mikro MK3...
✓ Device found: Maschine Mikro MK3 (17cc:1600)
✓ HID device opened successfully

Testing LED control...
- Lighting pad 0 (Red Bright)
- Lighting pad 1 (Green Bright)
- Lighting pad 2 (Blue Bright)
...
- Clearing all pads

✓ LED diagnostic complete
```

**Error output** (if HID not accessible):
```
✗ Failed to open HID device
  Possible causes:
  - Device not connected
  - Input Monitoring permission not granted
  - Native Instruments drivers not installed

  Solutions:
  1. Check USB connection
  2. Grant Input Monitoring permission (System Settings → Privacy & Security)
  3. Install NI drivers via Native Access
```

**When to use**:
- LEDs not working in main application
- Verify HID access before running midimon
- Test after granting permissions
- Debug LED coordinate mapping issues

### led_tester

Interactive LED testing tool.

**Purpose**: Manual control of individual LEDs for testing and debugging.

**Syntax**:
```bash
cargo run --bin led_tester
```

**Interactive mode**:
```
LED Tester - Interactive Mode
==============================

Commands:
  on <pad> <color> <brightness>  - Turn on LED
  off <pad>                      - Turn off LED
  all <color> <brightness>       - Set all LEDs
  clear                          - Clear all LEDs
  rainbow                        - Show rainbow pattern
  test                           - Cycle through all pads
  quit                           - Exit

> on 0 red 3
✓ Pad 0: Red Bright

> all blue 2
✓ All pads: Blue Normal

> rainbow
✓ Rainbow pattern displayed

> quit
```

**Colors**: red, orange, yellow, green, blue, purple, magenta, white

**Brightness**: 0 (off), 1 (dim), 2 (normal), 3 (bright)

**When to use**:
- Test specific pad LEDs
- Verify coordinate mapping
- Experiment with colors and brightness
- Debug LED patterns

### pad_mapper

Find MIDI note numbers for physical pads.

**Purpose**: Discover note numbers to use in `config.toml`.

**Syntax**:
```bash
cargo run --bin pad_mapper [PORT]
```

**Example**:
```bash
cargo run --bin pad_mapper 2
```

**Usage**:
1. Run the tool
2. Press each pad on your controller
3. Write down the note number displayed
4. Use those numbers in your config

**Output**:
```
Pad Mapper - Press pads to see note numbers
============================================
Connected to port 2: Maschine Mikro MK3 - Input

Press pads... (Ctrl+C to exit)

Pad pressed: Note 12 (velocity: 87)
Pad pressed: Note 13 (velocity: 64)
Pad pressed: Note 14 (velocity: 103)
Pad pressed: Note 15 (velocity: 52)
```

**Tips**:
- Press pads in order (bottom-left to top-right)
- Draw a grid on paper and write down note numbers
- Use this data to update `config.toml`

**When to use**:
- Setting up a new device
- Creating initial config
- Mapping custom devices
- Verifying profile note numbers

### test_midi

Test MIDI port connectivity and enumerate devices.

**Purpose**: Quick MIDI connectivity test, list all available ports.

**Syntax**:
```bash
cargo run --bin test_midi
```

**Output**:
```
MIDI Port Test
==============

Available input ports:
0: USB MIDI Device
1: IAC Driver Bus 1
2: Maschine Mikro MK3 - Input
3: Digital Keyboard

Available output ports:
0: USB MIDI Device
1: IAC Driver Bus 1
2: Maschine Mikro MK3 - Output

Testing port 2 (input)...
✓ Successfully opened port: Maschine Mikro MK3 - Input

Press a pad to test... (5 second timeout)
✓ Received MIDI event: NoteOn ch:0 note:12 vel:87

Connection test: PASSED
```

**When to use**:
- Verify MIDI device detected
- Check port numbers before running midimon
- Debug connectivity issues
- Confirm MIDI cable is working

## Command Quick Reference

| Command | Purpose | Example |
|---------|---------|---------|
| `midimon [PORT]` | Main application | `cargo run --release 2` |
| `--led <SCHEME>` | Set LED scheme | `cargo run --release 2 --led rainbow` |
| `--profile <PATH>` | Load profile | `cargo run --release 2 --profile my.ncmm3` |
| `--pad-page <PAGE>` | Force pad page | `cargo run --release 2 --pad-page H` |
| `--config <PATH>` | Custom config | `cargo run --release 2 --config dev.toml` |
| `DEBUG=1` | Enable debug log | `DEBUG=1 cargo run --release 2` |
| `midi_diagnostic` | View MIDI events | `cargo run --bin midi_diagnostic 2` |
| `led_diagnostic` | Test LEDs | `cargo run --bin led_diagnostic` |
| `led_tester` | Interactive LED test | `cargo run --bin led_tester` |
| `pad_mapper` | Find note numbers | `cargo run --bin pad_mapper 2` |
| `test_midi` | Test MIDI ports | `cargo run --bin test_midi` |

## Common Workflows

### First-Time Setup

```bash
# 1. List available ports
cargo run --release

# 2. Test connectivity
cargo run --bin test_midi

# 3. Map pads to note numbers
cargo run --bin pad_mapper 2

# 4. Run with basic settings
cargo run --release 2

# 5. Test LEDs
cargo run --release 2 --led rainbow
```

### Troubleshooting

```bash
# 1. Check MIDI events are received
cargo run --bin midi_diagnostic 2

# 2. Enable debug logging
DEBUG=1 cargo run --release 2

# 3. Test LED control
cargo run --bin led_diagnostic

# 4. Verify port numbers
cargo run --bin test_midi
```

### Production Use

```bash
# Build optimized binary
cargo build --release

# Run with all options
./target/release/midimon 2 \
    --profile ~/profiles/work.ncmm3 \
    --led reactive \
    --config ~/configs/production.toml

# Or use shell script
#!/bin/bash
DEBUG=0 ./target/release/midimon 2 \
    --profile "$HOME/Documents/NI/my-profile.ncmm3" \
    --led reactive \
    > /tmp/midimon.log 2>&1 &
```

## See Also

- [macOS Installation](../installation/macos.md) - Platform-specific setup
- [Building Guide](../installation/building.md) - Build from source
- [Diagnostics Guide](../troubleshooting/diagnostics.md) - Detailed troubleshooting
- [Configuration Overview](../configuration/overview.md) - Config file structure

---

**Last Updated**: November 11, 2025
**Binary Version**: 0.1.0
