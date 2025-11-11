# Common Issues and Solutions

## Overview

This guide covers the most frequently encountered issues when using MIDIMon, along with step-by-step solutions. For detailed diagnostic procedures, see [Diagnostics Guide](diagnostics.md).

## MIDI Device Not Found

### Symptoms

```
Error: No MIDI input ports available
```

Or:

```
Available MIDI input ports:
0: IAC Driver Bus 1
# Your device is missing from the list
```

### Causes

- USB cable not connected
- Device not powered on
- MIDI driver not installed (macOS/Windows)
- Device in wrong mode (some controllers have multiple modes)
- MIDI port disabled in system settings

### Solutions

#### 1. Check Physical Connection

```bash
# macOS: Check USB device enumeration
system_profiler SPUSBDataType | grep -i mikro
# or
system_profiler SPUSBDataType | grep -i midi

# Expected output:
# Maschine Mikro MK3:
#   Product ID: 0x1600
#   Vendor ID: 0x17cc (Native Instruments)
```

**If device not found**:
- Try a different USB port
- Try a different USB cable
- Power cycle the device (unplug, wait 10 seconds, replug)
- Check device is powered on (some controllers have power switches)

#### 2. Verify MIDI Setup (macOS)

```bash
# Open Audio MIDI Setup
open -a "Audio MIDI Setup"
```

In the **MIDI Studio** window (Window → Show MIDI Studio):
- Verify your device appears
- Check it's not grayed out (indicates active connection)
- If grayed out, right-click and select "Enable"

**Reset MIDI configuration** (if corrupted):
```bash
# Quit Audio MIDI Setup first
rm ~/Library/Preferences/com.apple.audio.AudioMIDISetup.plist
rm ~/Library/Preferences/ByHost/com.apple.audio.AudioMIDISetup.*
# Reopen Audio MIDI Setup
```

#### 3. Install/Reinstall Drivers

**macOS/Windows**:
1. Download [Native Access](https://www.native-instruments.com/en/support/downloads/)
2. Sign in (free account)
3. Install drivers for your device
4. Restart computer after installation
5. Reconnect device

**Linux**:
- Most MIDI controllers work with built-in ALSA drivers
- Install `alsa-utils`: `sudo apt install alsa-utils`
- List devices: `aconnect -l`

#### 4. Test with System Tools

**macOS**:
```bash
# List MIDI devices using system_profiler
system_profiler SPUSBDataType | grep -B 10 -A 10 MIDI

# Test with midimon diagnostic
cargo run --bin test_midi
```

**Linux**:
```bash
# List ALSA MIDI devices
aconnect -l

# Test with amidi
amidi -l
```

#### 5. Check Port Numbers

```bash
# List all available ports
cargo run --release

# Try each port number
cargo run --bin midi_diagnostic 0
cargo run --bin midi_diagnostic 1
cargo run --bin midi_diagnostic 2
# ... press a pad/key on your controller to verify connection
```

### Still Not Working?

1. Try a different computer (to rule out hardware failure)
2. Test with manufacturer's software (e.g., NI Controller Editor)
3. Check for firmware updates
4. Contact manufacturer support

## LEDs Not Working

### Symptoms

- Pads press correctly but LEDs don't light up
- LEDs flash once then go dark
- Wrong pads lighting up
- LEDs stuck on or flickering

### Causes

- Input Monitoring permission not granted (macOS)
- HID driver not installed
- Profile/coordinate mapping issues
- HID device already in use by another application
- Incorrect LED scheme selected

### Solutions

#### 1. Grant Input Monitoring Permission (macOS)

**Step-by-step**:

1. Run MIDIMon:
   ```bash
   cargo run --release 2
   ```

2. macOS will show a permission dialog

3. Click **Open System Settings**

4. In **Privacy & Security** → **Input Monitoring**:
   - Find `midimon` or `Terminal` (if running via cargo)
   - Toggle switch to **ON**
   - If switch is already ON, toggle OFF then ON again

5. Restart MIDIMon:
   ```bash
   cargo run --release 2
   ```

**Verify permission granted**:
```bash
DEBUG=1 cargo run --release 2
```

Look for:
```
[DEBUG] HID device opened successfully
[DEBUG] LED controller initialized
```

If you see "Failed to open HID device", permission is not granted.

#### 2. Test LED Hardware

```bash
# Run LED diagnostic tool
cargo run --bin led_diagnostic
```

**Expected output**:
```
✓ Device found: Maschine Mikro MK3
✓ HID device opened successfully
✓ Testing LED control...
✓ LED diagnostic complete
```

**Error output**:
```
✗ Failed to open HID device
```

If this fails, see [Diagnostics Guide](diagnostics.md) for HID troubleshooting.

#### 3. Install/Verify Native Instruments Drivers

**macOS/Windows**:
1. Open Native Access
2. Verify "Maschine" or controller-specific software is installed
3. Check for updates
4. Reinstall if necessary
5. Restart computer

**Test after driver installation**:
```bash
cargo run --bin led_diagnostic
```

#### 4. Try Different LED Schemes

Some schemes might not work due to profile issues:

```bash
# Try rainbow (doesn't require note mapping)
cargo run --release 2 --led rainbow

# Try static
cargo run --release 2 --led static

# Try reactive (default)
cargo run --release 2 --led reactive
```

If rainbow/static work but reactive doesn't, it's a profile/mapping issue.

#### 5. Fix Coordinate Mapping Issues

**Wrong pads lighting up** indicates coordinate mapping problems.

**Solution**: Use a device profile:

```bash
# Auto-detect page
cargo run --release 2 --profile path/to/profile.ncmm3

# Force specific page
cargo run --release 2 --profile profile.ncmm3 --pad-page A
```

Create a profile using Native Instruments Controller Editor if you don't have one.

See [Device Profiles Documentation](../DEVICE_PROFILES.md) for complete guide.

#### 6. Check Shared Device Access

If Controller Editor is running simultaneously:

**macOS**: MIDIMon uses shared device access (should work)

**Verify**:
```bash
# Check Cargo.toml includes:
hidapi = { version = "2.4", features = ["macos-shared-device"] }
```

If LEDs work when Controller Editor is closed but not when it's running:
- Update to latest MIDIMon version
- Rebuild from source: `cargo build --release`

### Advanced Debugging

Enable debug logging and watch for LED updates:

```bash
DEBUG=1 cargo run --release 2 --led reactive
```

Press pads and look for:
```
[DEBUG] LED update: pad 0 -> color 7 (Green) brightness 2
[DEBUG] HID write: 81 bytes
```

If you see LED updates logged but LEDs don't light:
- Hardware issue (try different USB port)
- Firmware issue (update device firmware)
- Driver issue (reinstall NI drivers)

## Events Not Triggering

### Symptoms

- Pads press but no actions execute
- Some pads work, others don't
- LEDs work but actions don't fire
- Actions trigger randomly or inconsistently

### Causes

- Note numbers don't match config.toml
- Wrong mode active
- Trigger conditions not met (velocity, timing)
- Config syntax errors
- Event processing disabled

### Solutions

#### 1. Verify Note Numbers

```bash
# Find actual note numbers
cargo run --bin pad_mapper 2
```

Press each pad and write down the note numbers.

**Compare with config.toml**:
```toml
[[modes.mappings]]
[modes.mappings.trigger]
type = "Note"
note = 12  # Must match the actual note number from pad_mapper
```

If they don't match, update config.toml with correct note numbers.

#### 2. Check Active Mode

MIDIMon prints the current mode to console:

```
Mode changed: Default -> Developer
Currently in mode: Developer
```

**Verify you're in the expected mode**:
- Check encoder hasn't switched modes accidentally
- Verify mode-specific mappings are in the correct mode section
- Try using global mappings for testing

**Force to Default mode**:
```toml
[[global_mappings]]
description = "Reset to default mode"
[global_mappings.trigger]
type = "Note"
note = 0
[global_mappings.action]
type = "ModeChange"
mode = "Default"
```

#### 3. Test with Simple Mapping

Add a simple test mapping to verify basic functionality:

```toml
[[global_mappings]]
description = "Test mapping - Should print to console"
[global_mappings.trigger]
type = "Note"
note = 0  # Bottom-left pad
[global_mappings.action]
type = "Shell"
command = "echo 'Mapping works!' && say 'Mapping works'"
```

If this works, the issue is with specific trigger/action configuration.

#### 4. Enable Debug Logging

```bash
DEBUG=1 cargo run --release 2
```

Press pads and watch for:

**Good output** (mapping working):
```
[MIDI] NoteOn ch:0 note:12 vel:87
[DEBUG] Processed: Note(12) with velocity Medium
[DEBUG] Matched mapping: "Copy text" (mode: Default)
[DEBUG] Executing action: Keystroke(keys: "c", modifiers: ["cmd"])
✓ Action executed successfully
```

**Bad output** (no match):
```
[MIDI] NoteOn ch:0 note:12 vel:87
[DEBUG] Processed: Note(12) with velocity Medium
[DEBUG] No mapping matched for event
```

If no mapping matched:
- Note number mismatch
- Wrong mode
- Trigger conditions not met

#### 5. Check Velocity/Timing Requirements

**VelocityRange** triggers require specific velocity:

```toml
[trigger]
type = "VelocityRange"
note = 12
min_velocity = 81  # Only triggers on HARD press (81-127)
max_velocity = 127
```

**Solution**: Press harder or adjust velocity range:
```toml
min_velocity = 0   # Accept any velocity
max_velocity = 127
```

**LongPress** triggers require holding:

```toml
[trigger]
type = "LongPress"
note = 12
hold_duration_ms = 2000  # Must hold for 2 seconds
```

**Solution**: Hold longer or reduce threshold:
```toml
hold_duration_ms = 500  # Only 0.5 seconds
```

**DoubleTap** requires two quick taps:

```toml
[trigger]
type = "DoubleTap"
note = 12
double_tap_timeout_ms = 300  # Must tap twice within 300ms
```

**Solution**: Tap faster or increase timeout:
```toml
double_tap_timeout_ms = 500  # 0.5 seconds between taps
```

#### 6. Validate Config Syntax

```bash
# Check for TOML syntax errors
cargo check

# Or use online validator
# https://www.toml-lint.com/
```

Common syntax errors:
- Missing quotes around strings
- Wrong bracket type (`[]` vs `[[]]`)
- Typos in field names
- Missing required fields

#### 7. Test MIDI Events

Verify your device is sending events:

```bash
cargo run --bin midi_diagnostic 2
```

Press pads - you should see:
```
[NoteOn]  ch:0 note:12 vel:87
[NoteOff] ch:0 note:12 vel:0
```

**If no events appear**:
- MIDI connection issue (see "MIDI Device Not Found" above)
- Device in wrong mode
- Device needs reset (power cycle)

### Still Not Working?

Create a minimal test config:

```bash
cat > test-config.toml << 'EOF'
[[modes]]
name = "Test"

[[modes.mappings]]
description = "Test"
[modes.mappings.trigger]
type = "Note"
note = 0
[modes.mappings.action]
type = "Shell"
command = "say 'test works'"
EOF

cargo run --release 2 --config test-config.toml
```

Press pad 0. If you hear "test works", the system is functioning.

## Profile Detection Issues

### Symptoms

- "Failed to load profile" error
- Wrong pads lighting up
- LEDs work without profile but not with profile
- Auto-page detection not working

### Causes

- Profile file not found
- Invalid XML format
- Wrong pad page active
- Note numbers not in profile
- Profile path has spaces or special characters

### Solutions

#### 1. Verify Profile File Exists

```bash
# Check file exists
ls -la path/to/profile.ncmm3

# Try absolute path
cargo run --release 2 --profile "$HOME/Downloads/profile.ncmm3"

# Escape spaces in path
cargo run --release 2 --profile "My\ Profile.ncmm3"
```

#### 2. Validate Profile XML

Open the .ncmm3 file in a text editor and verify it's valid XML:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<DeviceProfile>
  <DeviceProperties>
    <Name>My Profile</Name>
    <Type>MASCHINE_MIKRO_MK3</Type>
  </DeviceProperties>

  <Mapping>
    <PageList>
      <Page name="Pad Page A">
        <!-- ... -->
      </Page>
    </PageList>
  </Mapping>
</DeviceProfile>
```

**Common issues**:
- Missing `<?xml` declaration
- Unclosed tags
- Invalid characters
- Corrupted file

**Fix**: Re-export from Controller Editor or use a backup.

#### 3. Force Specific Pad Page

Auto-detection might fail if notes overlap between pages:

```bash
# Instead of auto-detect
cargo run --release 2 --profile profile.ncmm3

# Force page A
cargo run --release 2 --profile profile.ncmm3 --pad-page A

# Try each page
for page in A B C D E F G H; do
    echo "Testing page $page"
    cargo run --release 2 --profile profile.ncmm3 --pad-page $page
    sleep 5
    killall midimon
done
```

#### 4. Create New Profile

If profile is corrupted:

1. Open **Native Instruments Controller Editor**
2. Select **Maschine Mikro MK3**
3. Create a simple profile:
   - Page A: Notes 12-27 (chromatic)
   - Page B: Notes 36-51 (drums)
4. Save as `test-profile.ncmm3`
5. Test: `cargo run --release 2 --profile test-profile.ncmm3`

#### 5. Debug Profile Loading

```bash
DEBUG=1 cargo run --release 2 --profile profile.ncmm3
```

Look for:
```
[DEBUG] Loading profile: profile.ncmm3
[DEBUG] Profile loaded: My Profile (MASCHINE_MIKRO_MK3)
[DEBUG] Found 8 pad pages
[DEBUG] Page A: 16 pads mapped (notes 12-27)
```

**If you see errors**:
```
[ERROR] Failed to parse profile: XML error at line 42
[ERROR] Profile validation failed: Missing required element
```

Fix the profile XML or create a new one.

#### 6. Verify Note Numbers Match Profile

```bash
# Run pad mapper
cargo run --bin pad_mapper 2
```

Press pads and verify the notes are in your profile.

If note 50 is pressed but your profile only has notes 12-27, no LED will light.

**Solution**: Either:
- Update profile to include note 50
- Change hardware to send notes 12-27 (in Controller Editor)

## Platform-Specific Issues

### macOS: Permission Dialogs Keep Appearing

**Cause**: Binary changes (recompiling) invalidates permissions

**Solution**: Grant permission to Terminal.app instead:
1. System Settings → Privacy & Security → Input Monitoring
2. Add `Terminal` (or your terminal emulator)
3. Run via `cargo run` - permission persists across rebuilds

**Alternative**: Code-sign the binary:
```bash
codesign --force --deep --sign - target/release/midimon
```

### macOS: "Cannot be opened because the developer cannot be verified"

**Solution**:
```bash
# Remove quarantine attribute
xattr -d com.apple.quarantine target/release/midimon

# Or allow in System Settings
# Right-click binary → Open → Allow
```

### Linux: Permission Denied (USB/HID Access)

**Cause**: User not in `plugdev` group or missing udev rules

**Solution**:

1. Add udev rules:
```bash
sudo tee /etc/udev/rules.d/50-midimon.rules << 'EOF'
# Native Instruments Maschine Mikro MK3
SUBSYSTEM=="usb", ATTRS{idVendor}=="17cc", ATTRS{idProduct}=="1600", MODE="0666", GROUP="plugdev"
SUBSYSTEM=="usb", ATTRS{bInterfaceClass}=="01", ATTRS{bInterfaceSubClass}=="03", MODE="0666", GROUP="plugdev"
EOF

sudo udevadm control --reload-rules
sudo udevadm trigger
```

2. Add user to plugdev:
```bash
sudo usermod -a -G plugdev $USER
```

3. Log out and back in

4. Test:
```bash
cargo run --release 2
```

### Windows: MIDI Device Not Recognized

**Cause**: Driver not installed or generic USB driver used

**Solution**:
1. Open Device Manager
2. Look for "MIDI Device" or your controller under "Sound, video and game controllers"
3. Right-click → Update Driver
4. Choose manufacturer driver (not generic USB)
5. Or install via Native Access

## Performance Issues

### High CPU Usage

**Symptom**: MIDIMon using >10% CPU

**Causes**:
- Debug logging enabled
- Animated LED scheme with high update rate
- Shell actions running slowly
- Event processing loop not optimizing

**Solutions**:

1. Disable debug logging:
   ```bash
   # Don't use DEBUG=1 in production
   cargo run --release 2
   ```

2. Use simpler LED scheme:
   ```bash
   # Instead of animated schemes
   cargo run --release 2 --led reactive  # or off
   ```

3. Optimize shell actions:
   ```bash
   # Avoid long-running commands in mappings
   # Bad:
   command = "sleep 10 && echo done"

   # Good:
   command = "echo done &"  # Background process
   ```

4. Build release mode:
   ```bash
   # Debug builds are 20-30% slower
   cargo build --release
   ./target/release/midimon 2
   ```

### High Latency (Delayed Response)

**Symptom**: Actions trigger 50-100ms+ after pad press

**Solutions**:

1. Use release build (not debug)
2. Check system load (Activity Monitor/Task Manager)
3. Close unnecessary applications
4. Check MIDI buffer settings in Audio MIDI Setup (macOS)

**Verify latency**:
```bash
DEBUG=1 cargo run --release 2
```

Watch timestamps:
```
[16:32:45.123] NoteOn received
[16:32:45.124] Action executed  # Should be <2ms
```

If >10ms, investigate system performance.

## Getting Additional Help

If your issue isn't covered here:

1. Check [Diagnostics Guide](diagnostics.md) for detailed troubleshooting
2. Enable debug logging: `DEBUG=1 cargo run --release 2`
3. Run diagnostic tools:
   ```bash
   cargo run --bin midi_diagnostic 2
   cargo run --bin led_diagnostic
   cargo run --bin test_midi
   ```
4. Collect information:
   - macOS/Linux/Windows version
   - MIDIMon version: `cargo --version`
   - Device model
   - Error messages (full output)
   - Debug log output
5. File an issue on GitHub with collected information

---

**Last Updated**: November 11, 2025
**Status**: Actively maintained
