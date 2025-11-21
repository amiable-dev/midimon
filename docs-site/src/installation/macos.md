# macOS Installation Guide

## Overview

This guide walks through installing and configuring MIDIMon v3.0.0 on macOS. MIDIMon now includes multi-protocol input support (MIDI controllers + game controllers), a background daemon service, and a modern Tauri-based GUI for visual configuration.

**Installation Options**:
- **Option 1 (Recommended)**: Download pre-built GUI app + daemon binaries from [GitHub Releases](https://github.com/amiable-dev/midimon/releases)
- **Option 2**: Build from source (developers/advanced users)

Installation takes approximately 10-15 minutes.

## Option 1: Install Pre-Built Binaries (Recommended)

### 1. Download MIDIMon

Visit the [Releases Page](https://github.com/amiable-dev/midimon/releases/latest) and download:

**For GUI + Daemon (Recommended)**:
- `midimon-gui-macos-universal.tar.gz` - GUI application with daemon
- **OR** download daemon separately: `midimon-aarch64-apple-darwin.tar.gz` (Apple Silicon) or `midimon-x86_64-apple-darwin.tar.gz` (Intel)

### 2. Install the GUI Application

```bash
# Extract the GUI app
tar xzf midimon-gui-macos-universal.tar.gz

# Move to Applications folder
mv "MIDIMon GUI.app" /Applications/

# Open the app
open /Applications/"MIDIMon GUI.app"
```

### 3. Install the Daemon Binary (Optional - GUI includes daemon)

If you want to use the daemon independently:

```bash
# Extract daemon binary
tar xzf midimon-*.tar.gz

# Make it executable
chmod +x midimon

# Move to PATH
sudo mv midimon /usr/local/bin/

# Verify installation
midimon --version
```

**Skip to [Configuring macOS Permissions](#configuring-macos-permissions)**

---

## Option 2: Build from Source

### Prerequisites

#### 1. Hardware Requirements

MIDIMon v3.0 supports two types of input devices:

**MIDI Controllers**:
- Native Instruments Maschine Mikro MK3 (recommended, full RGB LED support)
- Generic MIDI controllers (keyboard controllers, pad controllers, etc.)
- USB-MIDI or MIDI over Bluetooth

**Game Controllers (HID)** (v3.0+):
- Gamepads: Xbox (360, One, Series X|S), PlayStation (DualShock 4, DualSense), Switch Pro Controller
- Joysticks: Flight sticks, arcade sticks
- Racing Wheels: Logitech, Thrustmaster, or any SDL2-compatible wheel
- HOTAS: Hands On Throttle And Stick systems
- Custom Controllers: Any SDL2-compatible HID device

You need at least one MIDI controller OR one game controller to use MIDIMon. Both can be used simultaneously.

#### 2. Software Requirements

**Rust Toolchain** (for building from source):

MIDIMon is written in Rust and requires the Rust compiler and Cargo build system.

**Check if Rust is already installed**:
```bash
rustc --version
cargo --version
```

If you see version numbers (e.g., `rustc 1.75.0`), skip to the next section.

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

**Node.js and npm** (for GUI only):

Required if building the Tauri GUI:

```bash
# Install Node.js via Homebrew
brew install node@20

# Verify installation
node --version  # Should show: v20.x.x
npm --version   # Should show: 10.x.x
```

**SDL2 Library** (for game controllers):

SDL2 is included via the `gilrs` v0.10 Rust crate. No additional installation required - it's built into MIDIMon automatically.

#### 3. Platform-Specific Requirements

**Xcode Command Line Tools** (Required):

Required for compiling native dependencies:

```bash
xcode-select --install
```

If already installed, you'll see: "command line tools are already installed".

#### 4. Device-Specific Drivers (Optional)

**Native Instruments Drivers** (for Maschine Mikro MK3 only):

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

**Game Controller Drivers**:

Most modern game controllers work natively on macOS without additional drivers:
- **Xbox Controllers**: Native support (360, One, Series X|S)
- **PlayStation Controllers**: Native support via Bluetooth or USB
- **Switch Pro Controller**: Native support via Bluetooth or USB
- **Generic SDL2 Controllers**: Usually work without drivers

No additional drivers are required for gamepad support.

### Building from Source

#### 1. Clone the Repository

```bash
# Choose a location for the project
cd ~/projects  # or wherever you keep code

# Clone the repository
git clone https://github.com/amiable-dev/midimon.git
cd midimon
```

#### 2. Build the Daemon

**Release build** (recommended for regular use):
```bash
# Build the entire workspace (daemon + core)
cargo build --release --workspace

# Or build just the daemon binary
cargo build --release --package midimon-daemon
```

The release build takes 2-5 minutes on modern hardware and produces an optimized binary (~3-5MB) in `target/release/midimon`.

**Build output**:
```
   Compiling midimon-core v2.0.0 (/Users/you/projects/midimon/midimon-core)
   Compiling midimon-daemon v2.0.0 (/Users/you/projects/midimon/midimon-daemon)
    Finished release [optimized] target(s) in 2m 14s
```

#### 3. Build the GUI (Optional)

```bash
# Install frontend dependencies
cd midimon-gui/ui
npm ci

# Build the frontend
npm run build

# Build the Tauri backend
cd ../src-tauri
cargo build --release

# The GUI app bundle will be at:
# midimon-gui/src-tauri/target/release/bundle/macos/MIDIMon GUI.app
```

#### 4. Verify the Build

```bash
# Test daemon binary
./target/release/midimon --version

# Or run it
./target/release/midimon

# Test GUI (if built)
open midimon-gui/src-tauri/target/release/bundle/macos/"MIDIMon GUI.app"
```

## Setting Up Configuration

### Using the GUI (Recommended)

v2.0.0 includes a visual configuration editor:

1. **Open MIDIMon GUI**:
   ```bash
   open /Applications/"MIDIMon GUI.app"
   ```

2. **Connect your MIDI device** in the device panel

3. **Use MIDI Learn mode**:
   - Click "Learn" next to any trigger
   - Press a pad/button on your controller
   - The trigger config auto-fills
   - Assign an action (keystroke, launch app, etc.)

4. **Save configuration** - automatically writes to `~/.config/midimon/config.toml`

See [GUI Quick Start](../getting-started/gui-quick-start.md) for detailed tutorial.

### Manual Configuration (Advanced)

If you prefer to edit `config.toml` manually:

**Config location**: `~/.config/midimon/config.toml`

**Create a minimal config**:
```bash
mkdir -p ~/.config/midimon

cat > ~/.config/midimon/config.toml << 'EOF'
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

**Hot-reload**: The daemon automatically reloads config within 0-10ms when you save changes.

## Verifying Device Connection

### Verifying MIDI Controller Connection

#### Check USB Device Enumeration

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

#### Check MIDI Connectivity

```bash
# Open Audio MIDI Setup
open -a "Audio MIDI Setup"
```

In the **MIDI Studio** window (Window → Show MIDI Studio):
- You should see "Maschine Mikro MK3" listed
- It should be connected (not grayed out)
- Double-click to view its properties

#### Test MIDI Events

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
- Check USB connection
- Verify correct port number (try 0, 1, 2, etc.)
- Restart the device
- Check Audio MIDI Setup

### Verifying Game Controller Connection

#### Check System Recognition

```bash
# Open System Settings
open /System/Library/PreferencePanes/GameController.prefPane
```

Or manually navigate:
- Open **System Settings** → **Game Controllers**
- Your gamepad should appear in the list
- Click to test buttons and analog sticks

**Supported indicators**:
- Green icon: Controller is connected and working
- Controller name displayed (e.g., "Xbox Wireless Controller")
- Button test interface available

#### Check via MIDIMon Status

```bash
# Start MIDIMon and check status
midimonctl status

# Look for gamepad in device list
# Example output:
# Connected Devices:
#   - Xbox Wireless Controller (Gamepad)
```

#### Test Gamepad Events

Use MIDIMon's event console to verify gamepad inputs:

```bash
# Start MIDIMon with debug logging
DEBUG=1 midimon --foreground
```

Press buttons on your gamepad. You should see:
```
[GamepadButton] button:128 (A/Cross/B)
[GamepadButton] button:129 (B/Circle/A)
[GamepadAnalogStick] axis:128 value:255 (Left stick right)
```

If nothing appears:
- Check USB or Bluetooth connection
- Verify controller appears in System Settings → Game Controllers
- Try reconnecting the controller
- Restart MIDIMon
- Check battery level (wireless controllers)

#### Platform-Specific Troubleshooting

**Bluetooth Connection Issues**:
1. Forget the device in Bluetooth settings
2. Put controller in pairing mode
3. Re-pair the controller
4. Test in Game Controllers settings

**USB Connection Issues**:
1. Try a different USB port
2. Try a different USB cable
3. Restart the controller (unplug/replug or hold power button)

## Configuring macOS Permissions

### Input Monitoring Permission (Required for HID/LED Control)

macOS requires explicit permission for applications to access HID devices like the Maschine Mikro MK3 and game controllers.

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

**Why this permission is required**:
- **MIDI Controllers**: HID-based RGB LED control (Maschine Mikro MK3)
- **Game Controllers**: Reading gamepad button and analog stick inputs
- **Input Simulation**: Simulating keyboard/mouse actions

**Verify HID access**:
```bash
DEBUG=1 cargo run --release 2
```

Look for:
```
[DEBUG] HID device opened successfully
[DEBUG] LED controller initialized
[DEBUG] Gamepad connected: Xbox Wireless Controller
```

If you see "HID device open failed" or gamepad not detected:
- Input Monitoring permission is enabled
- USB cable is connected (or Bluetooth paired)
- Native Instruments drivers are installed (for Mikro MK3)
- Controller appears in System Settings → Game Controllers

### Accessibility Permission (Optional, for Advanced Actions)

Some actions (e.g., controlling other apps programmatically) may require Accessibility permission:

1. Go to **System Settings** → **Privacy & Security** → **Accessibility**
2. Click the **+** button
3. Navigate to `target/release/midimon` (or add `Terminal`)
4. Click **Open**

This is optional and only needed for specific advanced features.

## Running MIDIMon

### Using the GUI (Recommended)

The simplest way to run MIDIMon v2.0.0:

1. **Launch the GUI**:
   ```bash
   open /Applications/"MIDIMon GUI.app"
   ```

2. **The daemon starts automatically** in the background

3. **Control via GUI**:
   - View real-time MIDI events in the Event Console
   - Edit configuration visually
   - Monitor daemon status in the status bar
   - Pause/resume/reload from the GUI

4. **Control via menu bar** (when daemon is running):
   - Click the MIDIMon icon in menu bar
   - Quick actions: Pause, Reload Config, Open GUI, Quit

### Using the Daemon CLI

For headless operation or scripting:

**Start the daemon**:
```bash
# Start daemon in foreground
midimon

# Start daemon in background
midimon &

# Or use launchd (see Auto-Start section below)
```

**Control the daemon** with `midimonctl`:
```bash
# Check status
midimonctl status

# Reload configuration
midimonctl reload

# Stop daemon
midimonctl stop

# Validate config without reloading
midimonctl validate

# Ping daemon (latency check)
midimonctl ping
```

**Output formats**:
```bash
# Human-readable output (default)
midimonctl status

# JSON output (for scripting)
midimonctl status --json
```

### Legacy CLI Options (Daemon)

The daemon binary still supports v1.0.0 CLI arguments:

```bash
# With LED lighting scheme
midimon --led reactive

# With device profile
midimon --profile ~/Downloads/my-profile.ncmm3

# With debug logging
DEBUG=1 midimon
```

## Auto-Start on Login

### Option 1: GUI Auto-Start (Recommended)

The MIDIMon GUI includes built-in auto-start functionality:

1. Open **MIDIMon GUI** → **Settings**
2. Enable **"Start MIDIMon on login"**
3. Click **Save**

This creates a LaunchAgent automatically and handles daemon startup.

### Option 2: Manual LaunchAgent Setup

For daemon-only auto-start (no GUI):

#### 1. Create LaunchAgent plist

```bash
mkdir -p ~/Library/LaunchAgents

cat > ~/Library/LaunchAgents/com.midimon.daemon.plist << 'EOF'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.midimon.daemon</string>

    <key>ProgramArguments</key>
    <array>
        <string>/usr/local/bin/midimon</string>
    </array>

    <key>RunAtLoad</key>
    <true/>

    <key>KeepAlive</key>
    <dict>
        <key>Crashed</key>
        <true/>
    </dict>

    <key>StandardOutPath</key>
    <string>/tmp/midimon.log</string>

    <key>StandardErrorPath</key>
    <string>/tmp/midimon.err</string>

    <key>EnvironmentVariables</key>
    <dict>
        <key>PATH</key>
        <string>/usr/local/bin:/usr/bin:/bin</string>
    </dict>
</dict>
</plist>
EOF
```

#### 2. Load the LaunchAgent

```bash
launchctl load ~/Library/LaunchAgents/com.midimon.daemon.plist
```

#### 3. Verify It's Running

```bash
# Check launchd
launchctl list | grep midimon

# Check daemon status
midimonctl status
```

You should see:
```
MIDIMon Daemon Status:
  State: Running
  Uptime: 2m 15s
  Config: /Users/you/.config/midimon/config.toml
  IPC Socket: /tmp/midimon.sock
```

#### 4. Control the LaunchAgent

```bash
# Stop
launchctl unload ~/Library/LaunchAgents/com.midimon.daemon.plist

# Start
launchctl load ~/Library/LaunchAgents/com.midimon.daemon.plist

# Restart
launchctl unload ~/Library/LaunchAgents/com.midimon.daemon.plist
launchctl load ~/Library/LaunchAgents/com.midimon.daemon.plist
```

#### 5. Check Logs

```bash
# Standard output
tail -f /tmp/midimon.log

# Errors
tail -f /tmp/midimon.err

# Or use daemon status
midimonctl status --json | jq
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

### Runtime Errors - MIDI

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

### Runtime Errors - Game Controllers

**Error**: `Gamepad not detected`

**Solution**:
1. Check connection (USB or Bluetooth)
2. Verify controller appears in System Settings → Game Controllers
3. Grant Input Monitoring permission
4. Try reconnecting the controller
5. Check debug output: `DEBUG=1 midimon --foreground`

---

**Error**: `Gamepad buttons not responding`

**Solution**:
1. Use MIDI Learn to discover correct button IDs
2. Verify button IDs are in range 128-255 (not 0-127)
3. Check that Input Monitoring permission is granted
4. Test in System Settings → Game Controllers
5. Try a different USB cable or re-pair Bluetooth

---

**Error**: `Analog stick not working`

**Solution**:
1. Check axis IDs (128-131 for sticks, 132-133 for triggers)
2. Verify direction is correct (Clockwise/CounterClockwise)
3. Adjust dead zone if too sensitive
4. Use button triggers instead of analog for precise control

---

### LED Issues (MIDI Controllers Only)

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

---

### Gamepad-Specific Issues

**Controller works in games but not MIDIMon**:
1. Ensure MIDIMon has Input Monitoring permission
2. Check that controller is SDL2-compatible
3. Try USB connection instead of Bluetooth
4. Restart MIDIMon after connecting controller

**Bluetooth pairing issues**:
1. Forget device in Bluetooth settings
2. Put controller in pairing mode (varies by controller):
   - **Xbox**: Hold pair button until LED flashes
   - **PlayStation**: Hold Share + PS button
   - **Switch Pro**: Hold sync button on top
3. Re-pair and test in System Settings
4. Use USB cable as fallback

**Battery/Power issues (wireless)**:
1. Charge or replace batteries
2. Use USB cable for wired mode
3. Check battery indicator in System Settings

For more troubleshooting help, see [Gamepad Support Guide](../guides/gamepad-support.md) and [Common Issues](../troubleshooting/common-issues.md).

## Next Steps

Now that MIDIMon v3.0.0 is installed and running:

### For GUI Users
1. **Learn the GUI**: Read [GUI Quick Start Guide](../getting-started/gui-quick-start.md)
2. **MIDI Learn Tutorial**: See [MIDI Learn Mode](../getting-started/midi-learn.md)
3. **Device Templates**: Check [Using Device Templates](../guides/device-templates.md)
4. **Per-App Profiles**: Set up [Application-Specific Profiles](../guides/per-app-profiles.md)
5. **Gamepad Setup**: Read [Gamepad Support Guide](../guides/gamepad-support.md) (v3.0+)

### For CLI Users
1. **Daemon Control**: Read [Daemon & Hot-Reload Guide](../guides/daemon.md)
2. **CLI Reference**: See [midimonctl Commands](../reference/cli.md)
3. **Manual Configuration**: Check [Configuration Overview](../configuration/overview.md)
4. **Advanced Actions**: Explore [Actions Reference](../reference/actions.md)

### For All Users
- **Gamepad Support**: [Gamepad Support Guide](../guides/gamepad-support.md) (v3.0+)
- **Troubleshooting**: [Common Issues](../troubleshooting/common-issues.md)
- **LED Customization**: [LED System Documentation](../guides/led-system.md)
- **Diagnostic Tools**: [Debugging Guide](../troubleshooting/diagnostics.md)

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

**Last Updated**: November 21, 2025 (v3.0.0)
**macOS Support**: 11.0+ (Big Sur and later)
**Architecture**: Universal Binary (Intel + Apple Silicon)
**Input Support**: MIDI Controllers + Game Controllers (HID)
