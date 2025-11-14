# Quick Start Guide

Get MIDIMon v2.0.0 up and running in under 5 minutes using the visual GUI.

## Prerequisites

- **macOS 11.0+** (Big Sur or later) - Intel or Apple Silicon
- **MIDI controller** (any USB MIDI device - Maschine Mikro MK3 recommended for full RGB LED support)
- **10 minutes** for installation

## Step 1: Download MIDIMon

Visit the [GitHub Releases page](https://github.com/amiable-dev/midimon/releases/latest) and download:

**For most users** (recommended):
- `midimon-gui-macos-universal.tar.gz` - GUI application (includes daemon)

**For CLI-only users**:
- `midimon-aarch64-apple-darwin.tar.gz` (Apple Silicon)
- `midimon-x86_64-apple-darwin.tar.gz` (Intel)

## Step 2: Install the GUI

```bash
# Extract the downloaded file
tar xzf midimon-gui-macos-universal.tar.gz

# Move to Applications folder
mv "MIDIMon GUI.app" /Applications/

# Open the app
open /Applications/"MIDIMon GUI.app"
```

**First launch**: macOS will ask for permissions. Grant **Input Monitoring** permission when prompted (required for LED control and device access).

## Step 3: Connect Your Device

1. **Plug in your MIDI controller** via USB

2. **In the MIDIMon GUI**, go to the **Device Connection** panel

3. Your device should appear in the list. Click **Connect**

4. The status bar at the bottom should show: **"Connected to [Your Device]"**

If your device doesn't appear:
- Check USB cable
- Try a different USB port
- See [Troubleshooting Device Connection](../troubleshooting/device-connection.md)

## Step 4: Create Your First Mapping with MIDI Learn

The fastest way to create a mapping is using **MIDI Learn mode**:

1. **Click "Add Mapping"** in the Mappings panel

2. **Click "Learn"** next to the Trigger field

3. **Press any pad/button** on your MIDI controller

4. The trigger configuration **auto-fills** with the detected input

5. **Choose an action**:
   - **Keystroke** - Press a key (e.g., Cmd+C for copy)
   - **Launch** - Open an application
   - **Text** - Type text
   - **Shell** - Run a command

6. **Click "Save"**

7. **Test it!** Press the same pad/button - the action should execute

**Example**: Map pad to open Spotify:
- Trigger: Note 36 (auto-detected via MIDI Learn)
- Action: Launch → `/Applications/Spotify.app`

## Step 5: Explore Advanced Features

### Velocity Sensitivity

Map different actions based on how hard you hit a pad:

1. Select trigger type: **Velocity Range**
2. Use MIDI Learn to detect the note
3. Set velocity ranges:
   - **Soft** (0-40): Pause playback
   - **Hard** (81-127): Skip track

### Long Press Detection

Hold a pad for 2+ seconds to trigger a different action:

1. Select trigger type: **Long Press**
2. Use MIDI Learn
3. Set **Hold Duration**: 2000ms
4. Action: Open Calculator

### Chord Detection

Press multiple pads simultaneously:

1. Select trigger type: **Chord**
2. Use MIDI Learn and press all pads within 100ms
3. Action: Launch your favorite app

## Step 6: Use Device Templates

Skip manual configuration with built-in device templates:

1. Go to **Settings** → **Device Templates**

2. **Select your controller**:
   - Maschine Mikro MK3
   - Launchpad Mini
   - APC Mini
   - Korg nanoKONTROL2
   - Novation Launchkey Mini
   - AKAI MPK Mini

3. Click **Load Template**

4. The template loads pre-configured mappings for common workflows

5. **Customize** as needed using MIDI Learn

## Step 7: Enable Auto-Start (Optional)

Run MIDIMon automatically when you log in:

1. Go to **Settings** → **General**

2. Enable **"Start MIDIMon on login"**

3. Click **Save**

The daemon will now start automatically in the background every time you log in.

## Using the Daemon CLI (Optional)

For advanced users who prefer terminal control:

```bash
# Check daemon status
midimonctl status

# Reload configuration (hot-reload in 0-10ms)
midimonctl reload

# Stop daemon
midimonctl stop

# Validate config without reloading
midimonctl validate

# Ping daemon (check latency)
midimonctl ping
```

See [Daemon & CLI Guide](../guides/daemon.md) for full details.

## Per-App Profiles (Automatic Profile Switching)

MIDIMon v2.0.0 can automatically switch configurations based on which application is active:

1. Go to **Per-App Profiles** in the GUI

2. **Add a new profile**:
   - Application: Select an app (e.g., "Visual Studio Code")
   - Profile: Select a config profile

3. When you switch to that application, MIDIMon automatically loads the configured profile

4. **Example use cases**:
   - **Logic Pro**: Pads control DAW functions (play, record, etc.)
   - **VS Code**: Pads trigger common shortcuts (run, debug, search)
   - **Chrome**: Pads control tabs and navigation

## Live Event Console

Debug your mappings in real-time:

1. Go to **Event Console** in the GUI

2. **Watch MIDI events** as they happen:
   - Note On/Off
   - Velocity values
   - Control Change
   - Pitch Bend
   - Aftertouch

3. **Filter events** by type or note number

4. **Export logs** for debugging

This is invaluable for troubleshooting "why isn't my mapping working?"

## Troubleshooting

### Device Not Found

1. **Check USB connection**:
   ```bash
   system_profiler SPUSBDataType | grep -i midi
   ```

2. **Restart the daemon**:
   ```bash
   midimonctl stop
   open /Applications/"MIDIMon GUI.app"
   ```

3. **Check Audio MIDI Setup**:
   ```bash
   open -a "Audio MIDI Setup"
   ```

### LEDs Not Working

1. **Ensure Native Instruments drivers are installed** (for Maschine controllers)

2. **Grant Input Monitoring permission**:
   - System Settings → Privacy & Security → Input Monitoring
   - Enable for "MIDIMon GUI"

3. **Check LED scheme** in GUI Settings

4. **View debug logs** in Event Console

### Mappings Not Triggering

1. **Use Event Console** to verify MIDI events are being received

2. **Use MIDI Learn** to verify the correct note numbers

3. **Check mode** - is the mapping in the current mode or global?

4. **Reload config**:
   ```bash
   midimonctl reload
   ```

### Permission Denied (macOS)

If you see "Permission denied" errors:

1. **System Settings** → **Privacy & Security** → **Input Monitoring**
2. Add **"MIDIMon GUI"** to the list
3. Restart the GUI app

## Next Steps

Now that you're up and running:

### For GUI Users
- [MIDI Learn Tutorial](./midi-learn.md) - Master MIDI Learn mode
- [Device Templates Guide](../guides/device-templates.md) - Use pre-built templates
- [Per-App Profiles Guide](../guides/per-app-profiles.md) - Set up application-specific profiles
- [Live Event Console](../guides/event-console.md) - Debug with real-time monitoring

### For CLI Users
- [Daemon & Hot-Reload](../guides/daemon.md) - Master the daemon CLI
- [Manual Configuration](../configuration/overview.md) - Edit config.toml
- [Advanced Triggers](../reference/triggers.md) - All trigger types
- [Advanced Actions](../reference/actions.md) - All action types

### For All Users
- [Understanding Modes](./modes.md) - Multi-mode workflow management
- [LED System](../guides/led-system.md) - Customize LED feedback
- [Example Configurations](../configuration/examples.md) - Pre-built configs

## Performance

MIDIMon v2.0.0 is highly optimized:
- **Response latency**: <1ms typical
- **Config hot-reload**: 0-10ms
- **IPC round-trip**: <1ms
- **Memory usage**: 5-10MB (daemon), ~60MB (GUI)
- **CPU usage**: <1% idle, <5% active
- **Binary size**: ~3-5MB (release with LTO)

## Getting Help

- [Troubleshooting Guide](../troubleshooting/common-issues.md)
- [FAQ](../troubleshooting/faq.md)
- [GitHub Discussions](https://github.com/amiable-dev/midimon/discussions)
- [Report Issues](https://github.com/amiable-dev/midimon/issues)

---

**Congratulations!** You now have MIDIMon v2.0.0 running with visual configuration, hot-reload, and per-app profiles. 🎉
