# MIDI Learn Mode

**MIDI Learn** is the fastest way to create mappings in MIDIMon v2.0.0. Instead of manually entering note numbers and MIDI parameters, simply press a pad/button on your controller and let MIDIMon auto-detect everything.

## What is MIDI Learn?

MIDI Learn is a one-click workflow:

1. **Click "Learn"** next to a trigger field
2. **Press a control** on your MIDI device (pad, encoder, button, etc.)
3. **MIDIMon auto-fills** the trigger configuration
4. **Assign an action** and save

That's it! No need to know note numbers, CC values, or MIDI channels.

## How to Use MIDI Learn

### Basic Workflow

1. **Open MIDIMon GUI** and ensure your device is connected

2. **Navigate to Mappings** panel

3. **Click "Add Mapping"** or edit an existing one

4. **Click the "Learn" button** next to the Trigger field

5. **MIDI Learn window opens** with a 10-second countdown:
   ```
   MIDI Learn Mode

   Waiting for MIDI input...
   Press any control on your device

   Time remaining: 8 seconds
   [Cancel]
   ```

6. **Press any control** on your MIDI device:
   - Pad
   - Button
   - Encoder/knob
   - Fader
   - Touch strip

7. **Trigger auto-fills**:
   ```
   Trigger Type: Note
   Note: 36
   Channel: 0
   ```

8. **Assign an action** (Keystroke, Launch, Text, etc.)

9. **Click "Save"**

10. **Test it!** Press the same control - the action should execute

### Supported Trigger Types

MIDI Learn auto-detects and configures these trigger types:

#### 1. Note (Basic Pad/Button Press)

**What it detects**:
- Note number
- MIDI channel
- Velocity range (if applicable)

**Example**:
- Press pad → Auto-fills: `Note: 36, Channel: 0`

**Use cases**:
- Basic pad mappings
- Button presses
- Keyboard keys

#### 2. Velocity Range (Pressure-Sensitive)

**What it detects**:
- Note number
- Soft/medium/hard press patterns
- Velocity thresholds

**Example**:
- Press pad softly → `VelocityRange: Note 36, Min: 0, Max: 40`
- Press pad hard → `VelocityRange: Note 36, Min: 81, Max: 127`

**Use cases**:
- Different actions for soft vs hard hits
- Velocity-sensitive controls

#### 3. Long Press

**What it detects**:
- Note number
- Hold duration (auto-calculated from your press)

**Example**:
- Hold pad for 2 seconds → `LongPress: Note 36, Duration: 2000ms`

**Use cases**:
- Hold pad to open app
- Long press for alternate action

#### 4. Double-Tap

**What it detects**:
- Note number
- Double-tap timing window

**Example**:
- Tap pad twice quickly → `DoubleTap: Note 36, Window: 300ms`

**Use cases**:
- Quick double-tap for special actions
- Distinguishing single vs double taps

#### 5. Chord (Multiple Notes)

**What it detects**:
- All pressed notes
- Chord window (how fast notes must be pressed together)

**Example**:
- Press pads 36, 40, 43 together → `Chord: [36, 40, 43], Window: 100ms`

**Use cases**:
- Shortcuts requiring multiple pads
- Musical chord detection

#### 6. Encoder/Knob Rotation

**What it detects**:
- CC (Control Change) number
- Direction (Clockwise/Counterclockwise)
- Value range

**Example**:
- Turn encoder right → `EncoderTurn: CC 1, Direction: Clockwise`
- Turn encoder left → `EncoderTurn: CC 1, Direction: Counterclockwise`

**Use cases**:
- Volume control
- Scrolling
- Parameter adjustment

#### 7. Control Change (CC)

**What it detects**:
- CC number
- Value range
- Continuous vs momentary

**Example**:
- Move fader → `CC: 7, Range: 0-127`
- Press button → `CC: 64, Value: 127`

**Use cases**:
- Faders
- Knobs with CC messages
- Sustain pedals

#### 8. Aftertouch (Pressure)

**What it detects**:
- Note number (if channel aftertouch)
- Pressure threshold

**Example**:
- Press pad harder after initial hit → `Aftertouch: Note 36, Threshold: 64`

**Use cases**:
- Pressure-sensitive effects
- Dynamic parameter control

#### 9. Pitch Bend

**What it detects**:
- Pitch bend range
- Direction (Up/Down/Center)

**Example**:
- Move pitch wheel up → `PitchBend: Direction: Up, Threshold: 8192`

**Use cases**:
- Pitch wheel mappings
- Touch strip controls

## Advanced MIDI Learn Features

### Countdown Timer

The MIDI Learn window shows a **10-second countdown**. This gives you time to:
- Position your hand
- Find the right control
- Try different velocities

**Countdown reaches zero** → MIDI Learn cancels automatically

### Cancellation

Click **"Cancel"** at any time to abort MIDI Learn without creating a mapping.

**Keyboard shortcut**: Press `Esc` to cancel

### Pattern Detection

MIDI Learn is **smart** - it detects patterns automatically:

**Long Press Detection**:
- If you hold a pad for >1 second during Learn mode, MIDIMon suggests a Long Press trigger

**Double-Tap Detection**:
- If you tap a pad twice quickly, MIDIMon suggests a Double-Tap trigger

**Chord Detection**:
- If you press multiple pads within 100ms, MIDIMon suggests a Chord trigger

**Velocity Variation**:
- If you press the same pad with varying velocities, MIDIMon suggests a Velocity Range trigger

### Velocity Range Suggestions

When MIDI Learn detects a note, it suggests velocity ranges:

```
MIDI Learn Complete!

Detected: Note 36

Suggested velocity ranges:
- Soft (0-40): Gentle tap
- Medium (41-80): Normal press
- Hard (81-127): Strong hit

Create separate mappings for each range?
[Yes] [No, use single Note trigger]
```

This makes it easy to create pressure-sensitive mappings.

## Troubleshooting

### No MIDI Input Detected

**Symptoms**: MIDI Learn countdown reaches zero without detecting anything

**Solutions**:
1. **Check device connection**: Ensure MIDI device is connected and showing in Device Panel
2. **Check Event Console**: Open Event Console and verify MIDI events are being received
3. **Try different control**: Some controls may not send MIDI (e.g., mode buttons on some devices)
4. **Check MIDI channel**: Some devices send on non-zero channels

### Wrong Note Number Detected

**Symptoms**: MIDI Learn detects the wrong note

**Solutions**:
1. **Verify in Event Console**: Check what MIDI events are actually being sent
2. **Check pad page**: Some controllers have multiple pad pages (A-H)
3. **Load device template**: Use a pre-configured template for your controller
4. **Manual override**: Click "Advanced" and manually enter the correct note number

### Multiple Events Detected

**Symptoms**: MIDI Learn shows "Multiple events detected, please try again"

**Solutions**:
1. **Press only one control** at a time
2. **Wait for pad to release** before pressing again
3. **Disable auto-repeat**: Some controllers send rapid-fire messages

### Velocity Not Detected

**Symptoms**: MIDI Learn only creates Note trigger, not Velocity Range

**Solutions**:
1. **Vary velocity**: Try pressing soft, medium, and hard during different Learn attempts
2. **Manual configuration**: Create Velocity Range trigger manually after Learn
3. **Check controller**: Some pads don't send velocity (always velocity 127)

## Examples

### Example 1: Basic Pad Mapping

**Goal**: Map pad to copy text (Cmd+C)

1. Click "Add Mapping"
2. Click "Learn" next to Trigger
3. Press pad → Auto-fills `Note: 36`
4. Select Action: **Keystroke**
5. Use Keystroke Picker: Press `Cmd+C`
6. Save

**Result**: Pressing pad executes Cmd+C

### Example 2: Velocity-Sensitive Volume

**Goal**: Soft press = volume down, hard press = volume up

1. Click "Add Mapping"
2. Click "Learn" next to Trigger
3. Press pad **softly** → Auto-fills `VelocityRange: Note 36, Min: 0, Max: 40`
4. Action: **Volume Control** → **Down**
5. Save

6. Click "Add Mapping" again
7. Click "Learn"
8. Press **same pad hard** → Auto-fills `VelocityRange: Note 36, Min: 81, Max: 127`
9. Action: **Volume Control** → **Up**
10. Save

**Result**: Soft press = volume down, hard press = volume up

### Example 3: Long Press to Launch App

**Goal**: Hold pad for 2 seconds to open Spotify

1. Click "Add Mapping"
2. Click "Learn" next to Trigger
3. **Hold pad for 2+ seconds** → Auto-fills `LongPress: Note 36, Duration: 2000ms`
4. Action: **Launch** → Select `/Applications/Spotify.app`
5. Save

**Result**: Holding pad for 2 seconds opens Spotify

### Example 4: Encoder for Volume

**Goal**: Turn encoder to adjust volume

1. Click "Add Mapping"
2. Click "Learn" next to Trigger
3. **Turn encoder right** → Auto-fills `EncoderTurn: CC 1, Direction: Clockwise`
4. Action: **Volume Control** → **Up**
5. Save

6. Click "Add Mapping" again
7. Click "Learn"
8. **Turn encoder left** → Auto-fills `EncoderTurn: CC 1, Direction: Counterclockwise`
9. Action: **Volume Control** → **Down**
10. Save

**Result**: Turning encoder controls system volume

## Tips & Best Practices

### Tip 1: Use Event Console

Before starting MIDI Learn:
1. Open **Event Console**
2. Press your control
3. Verify the MIDI event appears

This helps debug "Learn not detecting" issues.

### Tip 2: Learn in Context

When creating **per-app profiles**, Learn with that app in focus:
1. Switch to target app (e.g., Logic Pro)
2. Switch back to MIDIMon GUI
3. Use MIDI Learn
4. Assign action relevant to that app

### Tip 3: Batch Learn

Create multiple mappings quickly:
1. Click "Learn"
2. Press control
3. Assign action
4. Save
5. Immediately click "Add Mapping" and repeat

### Tip 4: Device Templates First

Before manual Learn:
1. Check if a device template exists for your controller
2. Load template to get 90% of mappings
3. Use Learn to customize the remaining 10%

### Tip 5: Test Immediately

After creating a mapping:
1. Click "Save"
2. **Immediately test** by pressing the control
3. Verify action executes correctly
4. Adjust if needed

## Next Steps

- [Device Templates](../guides/device-templates.md) - Pre-configured controller mappings
- [Trigger Reference](../reference/triggers.md) - All trigger types explained
- [Action Reference](../reference/actions.md) - All action types explained
- [Per-App Profiles](../guides/per-app-profiles.md) - Automatic profile switching

---

**Last Updated**: November 14, 2025 (v2.0.0)
