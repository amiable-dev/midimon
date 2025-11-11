# macOS Installation Guide

## Overview

This guide walks through installing and configuring MIDIMon on macOS, from installing prerequisites to running your first mapping. The process takes approximately 15-20 minutes.

## Prerequisites

### 1. Rust Toolchain

MIDIMon is written in Rust and requires the Rust compiler and Cargo build system.

**Check if Rust is already installed**:
```bash
rustc --version
cargo --version
```

If you see version numbers (e.g., `rustc 1.75.0`), skip to step 2.

**Install Rust using rustup**:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Follow the prompts and select the default installation. Then reload your shell:
```bash
source $HOME/.cargo/env
```

**Verify installation**:
```bash
rustc --version  # Should show: rustc 1.75.0 (or later)
cargo --version  # Should show: cargo 1.75.0 (or later)
```

### 2. Native Instruments Drivers (for Maschine Mikro MK3)

If using a Maschine Mikro MK3, install Native Instruments drivers for full RGB LED support.

**Download and install**:
1. Visit [Native Instruments Downloads](https://www.native-instruments.com/en/support/downloads/)
2. Download **Native Access** (the NI installation manager)
3. Install Native Access and sign in (free account)
4. In Native Access, install:
   - **Maschine** software (includes drivers)
   - **Controller Editor** (for creating custom profiles, optional)

**Verify driver installation**:
```bash
system_profiler SPUSBDataType | grep -i maschine
```

You should see output like:
```
Maschine Mikro MK3:
  Product ID: 0x1600
  Vendor ID: 0x17cc  (Native Instruments)
```

### 3. Xcode Command Line Tools

Required for compiling native dependencies:

```bash
xcode-select --install
```

If already installed, you'll see: "command line tools are already installed".

## Building MIDIMon from Source

### 1. Clone the Repository

```bash
# Choose a location for the project
cd ~/projects  # or wherever you keep code

# Clone the repository
git clone https://github.com/yourusername/midimon.git
cd midimon
```

### 2. Build the Project

**Debug build** (faster compilation, larger binary, includes debug symbols):
```bash
cargo build
```

**Release build** (recommended for regular use):
```bash
cargo build --release
```

The release build takes 2-5 minutes on modern hardware and produces an optimized binary (~3-5MB) in `target/release/midimon`.

**Build output**:
```
   Compiling midimon v0.1.0 (/Users/you/projects/midimon)
    Finished release [optimized] target(s) in 2m 34s
```

### 3. Verify the Build

```bash
# List MIDI ports (should show connected devices)
cargo run --release

# Or use the binary directly
./target/release/midimon
```

Expected output:
```
Available MIDI input ports:
0: USB MIDI Device
1: IAC Driver Bus 1
2: Maschine Mikro MK3 - Input
```

## Setting Up Configuration

### 1. Create config.toml

MIDIMon looks for `config.toml` in the project root directory.

**Create a minimal config**:
```bash
cat > config.toml << 'EOF'
[device]
name = "Mikro"
auto_connect = true

[[modes]]
name = "Default"
color = "blue"

[[modes.mappings]]
description = "Test mapping - Copy"
[modes.mappings.trigger]
type = "Note"
note = 12
[modes.mappings.action]
type = "Keystroke"
keys = "c"
modifiers = ["cmd"]

[[global_mappings]]
description = "Emergency exit (hold pad 0 for 3 seconds)"
[global_mappings.trigger]
type = "LongPress"
note = 0
hold_duration_ms = 3000
[global_mappings.action]
type = "Shell"
command = "killall midimon"
EOF
```

This creates a basic configuration with:
- One mode (Default)
- One test mapping (pad 12 = Cmd+C)
- One emergency exit (hold pad 0 to quit)

### 2. Find Your Note Numbers

If you don't know which MIDI note numbers your pads send:

```bash
cargo run --bin pad_mapper
```

Press each pad and write down the note numbers. Update your `config.toml` with the correct values.

## Configuring macOS Permissions

### Input Monitoring Permission (Required for HID/LED Control)

macOS requires explicit permission for applications to access HID devices like the Maschine Mikro MK3.

**Grant permission**:
1. Run MIDIMon once:
   ```bash
   cargo run --release 2
   ```

2. macOS will show a permission dialog: **"midimon would like to receive keystrokes from any application"**

3. Click **Open System Settings** or manually navigate:
   - Open **System Settings** → **Privacy & Security** → **Input Monitoring**
   - Find `midimon` (or `Terminal` if running via `cargo run`)
   - Toggle the switch to **ON**

4. Restart MIDIMon:
   ```bash
   cargo run --release 2
   ```

**Verify HID access**:
```bash
DEBUG=1 cargo run --release 2
```

Look for:
```
[DEBUG] HID device opened successfully
[DEBUG] LED controller initialized
```

If you see "HID device open failed", check:
- Input Monitoring permission is enabled
- USB cable is connected
- Native Instruments drivers are installed

### Accessibility Permission (Optional, for Advanced Actions)

Some actions (e.g., controlling other apps programmatically) may require Accessibility permission:

1. Go to **System Settings** → **Privacy & Security** → **Accessibility**
2. Click the **+** button
3. Navigate to `target/release/midimon` (or add `Terminal`)
4. Click **Open**

This is optional and only needed for specific advanced features.

## Verifying USB Connection

### Check USB Device Enumeration

```bash
system_profiler SPUSBDataType | grep -i mikro
```

Expected output:
```
Maschine Mikro MK3:
  Product ID: 0x1600
  Vendor ID: 0x17cc (Native Instruments)
  Serial Number: XXXXX
  Location ID: 0x14200000 / 5
```

### Check MIDI Connectivity

```bash
# Open Audio MIDI Setup
open -a "Audio MIDI Setup"
```

In the **MIDI Studio** window (Window → Show MIDI Studio):
- You should see "Maschine Mikro MK3" listed
- It should be connected (not grayed out)
- Double-click to view its properties

### Test MIDI Events

```bash
# Run diagnostic tool
cargo run --bin midi_diagnostic 2
```

Press pads on your controller. You should see:
```
[NoteOn] ch:0 note:12 vel:87
[NoteOff] ch:0 note:12 vel:0
```

If nothing appears:
- Check USB cable
- Verify correct port number (try 0, 1, 2, etc.)
- Restart the device
- Check Audio MIDI Setup

## Running MIDIMon

### Basic Usage

```bash
# Auto-detect device (port 0)
cargo run --release 0

# Connect to specific port
cargo run --release 2

# With LED lighting
cargo run --release 2 --led reactive
```

### With Device Profile

If you have a `.ncmm3` profile from NI Controller Editor:

```bash
# Auto-detect pad page
cargo run --release 2 --profile ~/Downloads/my-profile.ncmm3

# Force specific pad page (A-H)
cargo run --release 2 --profile my-profile.ncmm3 --pad-page H
```

### With Debug Logging

```bash
# Enable verbose logging
DEBUG=1 cargo run --release 2
```

### Available LED Schemes

```bash
# Reactive (velocity-based colors, default)
cargo run --release 2 --led reactive

# Rainbow gradient
cargo run --release 2 --led rainbow

# Breathing effect
cargo run --release 2 --led breathing

# Other schemes: pulse, wave, sparkle, vumeter, spiral, static, off
cargo run --release 2 --led wave
```

## Optional: LaunchAgent Setup (Auto-Start on Login)

To run MIDIMon automatically when you log in:

### 1. Create LaunchAgent plist

```bash
mkdir -p ~/Library/LaunchAgents

cat > ~/Library/LaunchAgents/com.midimon.plist << 'EOF'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.midimon</string>

    <key>ProgramArguments</key>
    <array>
        <string>/Users/YOURUSERNAME/projects/midimon/target/release/midimon</string>
        <string>2</string>
        <string>--led</string>
        <string>reactive</string>
    </array>

    <key>RunAtLoad</key>
    <true/>

    <key>KeepAlive</key>
    <false/>

    <key>StandardOutPath</key>
    <string>/tmp/midimon.log</string>

    <key>StandardErrorPath</key>
    <string>/tmp/midimon.err</string>
</dict>
</plist>
EOF
```

**Replace `YOURUSERNAME`** with your actual username, and update the path to your binary.

### 2. Load the LaunchAgent

```bash
launchctl load ~/Library/LaunchAgents/com.midimon.plist
```

### 3. Verify It's Running

```bash
launchctl list | grep midimon
```

You should see:
```
12345   0   com.midimon
```

### 4. Control the LaunchAgent

```bash
# Stop
launchctl unload ~/Library/LaunchAgents/com.midimon.plist

# Start
launchctl load ~/Library/LaunchAgents/com.midimon.plist

# Reload after changes
launchctl unload ~/Library/LaunchAgents/com.midimon.plist
launchctl load ~/Library/LaunchAgents/com.midimon.plist
```

### 5. Check Logs

```bash
# Standard output
tail -f /tmp/midimon.log

# Errors
tail -f /tmp/midimon.err
```

## Post-Installation Steps

### 1. Test Your Mappings

Press pads on your controller and verify actions execute correctly.

**Test checklist**:
- [ ] Pad presses trigger actions
- [ ] LEDs respond to presses (if using Mikro MK3)
- [ ] Mode switching works (if configured)
- [ ] Encoder controls work (if mapped)
- [ ] Long press detection works
- [ ] Double-tap detection works

### 2. Customize config.toml

Edit `config.toml` to add your own mappings. See:
- [First Mapping Tutorial](../getting-started/first-mapping.md)
- [Modes Guide](../getting-started/modes.md)
- [Configuration Overview](../configuration/overview.md)

### 3. Create Device Profile (Optional)

If you want custom pad layouts:

1. Open **Native Instruments Controller Editor**
2. Select **Maschine Mikro MK3**
3. Edit pad pages (A-H)
4. Save as `.ncmm3` file
5. Use with `--profile` flag

See [Device Profiles Documentation](../DEVICE_PROFILES.md) for details.

## Platform-Specific Notes

### macOS Versions

**Tested on**:
- macOS Sonoma (14.x) - Full support
- macOS Ventura (13.x) - Full support
- macOS Monterey (12.x) - Full support

**Known issues**:
- macOS Big Sur (11.x) and earlier: HID shared device access may not work

### Apple Silicon (M1/M2/M3)

MIDIMon works natively on Apple Silicon:

```bash
# Build for current architecture
cargo build --release

# Binary will be ARM64 (aarch64) on M1/M2/M3
file target/release/midimon
# Output: target/release/midimon: Mach-O 64-bit executable arm64
```

No special configuration needed - all dependencies support ARM64.

### Intel Macs

Works identically to Apple Silicon. If you need a universal binary:

```bash
# Build for both architectures
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin

cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin

# Combine into universal binary
lipo -create \
    target/x86_64-apple-darwin/release/midimon \
    target/aarch64-apple-darwin/release/midimon \
    -output midimon-universal
```

### Shared Device Access

MIDIMon uses `macos-shared-device` feature in `hidapi` to allow concurrent access with NI Controller Editor. This means:

- ✅ You can run MIDIMon and Controller Editor simultaneously
- ✅ Both can control LEDs without conflicts
- ✅ Both receive MIDI input

This is enabled by default in `Cargo.toml`:
```toml
[dependencies]
hidapi = { version = "2.4", features = ["macos-shared-device"] }
```

## Troubleshooting

### Build Errors

**Error**: `error: linker 'cc' not found`

**Solution**: Install Xcode Command Line Tools:
```bash
xcode-select --install
```

---

**Error**: `error: could not compile 'hidapi'`

**Solution**: Update Rust and dependencies:
```bash
rustup update
cargo clean
cargo build --release
```

---

### Runtime Errors

**Error**: `No MIDI input ports available`

**Solution**:
1. Check USB connection
2. Open Audio MIDI Setup and verify device appears
3. Try different USB port
4. Restart device

---

**Error**: `Failed to open HID device`

**Solution**:
1. Grant Input Monitoring permission (see above)
2. Install Native Instruments drivers
3. Check USB cable and connection
4. Try running with sudo (not recommended long-term):
   ```bash
   sudo ./target/release/midimon 2
   ```

---

**Error**: `Permission denied (os error 13)`

**Solution**:
1. Check Input Monitoring permission
2. Verify binary has correct permissions:
   ```bash
   ls -l target/release/midimon
   chmod +x target/release/midimon
   ```

---

### LED Issues

**LEDs not lighting up**:
1. Verify Native Instruments drivers installed
2. Check Input Monitoring permission
3. Test with different LED scheme:
   ```bash
   cargo run --release 2 --led rainbow
   ```
4. Check DEBUG output:
   ```bash
   DEBUG=1 cargo run --release 2
   ```

**LEDs lighting wrong pads**:
1. Verify you're using a device profile
2. Check profile has correct note mappings
3. See [Device Profiles](../DEVICE_PROFILES.md)
4. Use pad mapper to verify notes:
   ```bash
   cargo run --bin pad_mapper
   ```

## Next Steps

Now that MIDIMon is installed and running:

1. **Learn the Basics**: Read [First Mapping Tutorial](../getting-started/first-mapping.md)
2. **Configure Modes**: See [Modes Guide](../getting-started/modes.md)
3. **Explore Actions**: Check [Actions Reference](../reference/actions.md)
4. **Customize LEDs**: Read [LED System Documentation](../LED_SYSTEM.md)

## Getting Help

If you encounter issues:

1. Check [Common Issues](../troubleshooting/common-issues.md)
2. Use [Diagnostic Tools](../troubleshooting/diagnostics.md)
3. Enable debug logging: `DEBUG=1 cargo run --release 2`
4. File an issue on GitHub with:
   - macOS version
   - Hardware (Intel/Apple Silicon)
   - Device model
   - Error messages
   - Output of `cargo --version` and `rustc --version`

---

**Last Updated**: November 11, 2025
**macOS Support**: 12.0+ (Monterey and later)
