# MIDI Parsing Architecture Analysis

**Date**: 2025-01-18
**Analysis**: How MIDIMon processes MIDI messages across different layers

## Current Architecture

MIDIMon has a **3-layer architecture** for MIDI processing:

```
┌─────────────────────────────────────────────────────────────┐
│  Layer 1: Raw MIDI Bytes (from midir)                      │
│  - Receives raw bytes from MIDI hardware                    │
│  - Example: [0x90, 60, 100] (Note On, Middle C, vel 100)   │
└─────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────┐
│  Layer 2: MidiEvent (midimon-core)                         │
│  - Protocol-specific parsed events                          │
│  - Example: MidiEvent::NoteOn { note: 60, velocity: 100 }  │
└─────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────┐
│  Layer 3: ProcessedEvent (midimon-core)                    │
│  - High-level semantic events with timing                   │
│  - Example: ProcessedEvent::PadPressed { velocity_level }  │
└─────────────────────────────────────────────────────────────┘
```

## Where MIDI Parsing Happens

### 1. **GUI - MIDI Learn** (`midimon-gui/src-tauri/src/midi_learn.rs`)

**Purpose**: Parse MIDI to create configuration mappings

**Current Implementation**: ✅ **Manual byte parsing**
```rust
let message_type = status & 0xF0;
match message_type {
    0x90 => { /* Note On */ }
    0x80 => { /* Note Off */ }
    0xB0 => { /* Control Change */ }
    0xE0 => { /* Pitch Bend */ }
    0xA0 => { /* Polyphonic Aftertouch */ }
    0xD0 => { /* Channel Aftertouch */ }
    // ...
}
```

**Usage**:
- User presses button in MIDI Learn mode
- Captures raw MIDI message
- Converts to `MidiEvent` for config generation
- Creates trigger mappings

**Should refactor?** ⚠️ **YES - High Priority**
- Manual parsing duplicates logic
- Error-prone (easy to miss message types)
- Would benefit from `midi-msg` for correctness

---

### 2. **GUI - Event Handling** (`midimon-gui/src-tauri/src/events.rs`)

**Purpose**: Handle MIDI events for live preview/monitoring

**Current Implementation**: Uses `MidiEvent::from_bytes()` helper

**Should refactor?** ✅ **ALREADY CLEAN**
- Uses higher-level abstraction
- No manual byte parsing
- Could consider `midi-msg` for consistency

---

### 3. **Diagnostic Tool** (`midimon-daemon/src/bin/midi_diagnostic.rs`)

**Purpose**: Display MIDI messages for hardware testing

**Current Implementation**: ✅ **Now uses `midi-msg` library** (just refactored!)
```rust
match MidiMsg::from_midi(msg) {
    Ok((MidiMsg::ChannelVoice { channel, msg: voice_msg }, _)) => {
        match voice_msg {
            ChannelVoiceMsg::NoteOn { note, velocity } => { /* ... */ }
            ChannelVoiceMsg::PolyPressure { note, pressure } => { /* ... */ }
            // All message types supported
        }
    }
}
```

**Status**: ✅ **DONE** - Fully refactored with note names and MIDI timestamps

---

### 4. **Daemon - Main Event Loop** (NOT FOUND YET)

**Purpose**: Receive MIDI from hardware → Process → Execute actions

**Expected Location**: `midimon-daemon/src/main.rs` or similar

**Question**: How does the daemon parse raw MIDI bytes into `MidiEvent`?

Let me investigate...

---

## Current MIDI Parsing Locations

| Component | File | Parsing Method | Status |
|-----------|------|---------------|---------|
| Diagnostic Tool | `midimon-daemon/bin/midi_diagnostic.rs` | `midi-msg` library | ✅ Refactored |
| MIDI Learn | `midimon-gui/src-tauri/midi_learn.rs` | Manual `msg[0] & 0xF0` | ❌ Needs refactor |
| GUI Events | `midimon-gui/src-tauri/events.rs` | `MidiEvent::from_bytes()` | ⚠️ Consider refactor |
| Daemon Main | ❓ TBD | ❓ TBD | ❓ Needs investigation |

---

## MidiEvent Type (midimon-core)

The daemon uses `midimon-core`'s `MidiEvent` enum:

```rust
pub enum MidiEvent {
    NoteOn { note: u8, velocity: u8, time: Instant },
    NoteOff { note: u8, time: Instant },
    ControlChange { cc: u8, value: u8, time: Instant },
    Aftertouch { pressure: u8, time: Instant },
    PitchBend { value: u16, time: Instant },
    ProgramChange { program: u8, time: Instant },
}
```

**Key Points**:
1. ✅ Protocol-specific (MIDI-aware)
2. ✅ Includes timestamp
3. ⚠️ Does NOT differentiate:
   - Polyphonic vs Channel Aftertouch
   - Note On vs Note Off (velocity 0)
4. ❓ Where is `MidiEvent::from_bytes()` defined?

---

## ProcessedEvent Type (midimon-core)

The event processor transforms `MidiEvent` → `ProcessedEvent`:

```rust
pub enum ProcessedEvent {
    ShortPress { note: u8 },
    LongPress { note: u8, duration_ms: u128 },
    PadPressed { note: u8, velocity: u8, velocity_level: VelocityLevel },
    EncoderTurned { cc: u8, value: u8, direction: EncoderDirection, delta: u8 },
    DoubleTap { note: u8 },
    ChordDetected { notes: Vec<u8> },
    AftertouchChanged { pressure: u8 },
    PitchBendMoved { value: u16 },
}
```

**Key Points**:
1. ✅ High-level semantic events
2. ✅ Timing detection (short/long/double-tap)
3. ✅ Velocity levels (soft/medium/hard)
4. ✅ Chord detection
5. ✅ Encoder direction/delta

---

## MIDI Information Flow

```
MIDI Hardware (Mikro MK3)
         ↓
    midir library
         ↓ raw bytes: [0x90, 60, 100]
         ↓
[PARSING LAYER - WHERE?]
         ↓
MidiEvent::NoteOn { note: 60, velocity: 100, time }
         ↓
EventProcessor::process()
         ↓
ProcessedEvent::PadPressed { note: 60, velocity: 100, velocity_level: Hard }
         ↓
MappingEngine::match_event()
         ↓
Action (Keystroke, Shell, etc.)
         ↓
ActionExecutor::execute()
         ↓
System (keyboard input, shell command, etc.)
```

---

## Questions to Answer

1. **Where does daemon parse raw MIDI bytes?**
   - Need to find: `midimon-daemon/src/main.rs` or equivalent
   - Look for: `midir` callback that receives `msg: &[u8]`

2. **Is there a `MidiEvent::from_bytes()` helper?**
   - If so, where is it defined?
   - Does it do manual parsing or use a library?

3. **Should we centralize parsing?**
   - Option 1: Create `midimon-core` helper using `midi-msg`
   - Option 2: Each component uses `midi-msg` directly
   - Option 3: Keep manual parsing (current state)

4. **Performance considerations?**
   - `midi-msg` parsing overhead in hot path?
   - Should we benchmark before/after?

---

## Recommendation

### Short Term (Now)
✅ **DONE**: Refactored `midi_diagnostic` to use `midi-msg`
- Validates that `midi-msg` works correctly
- Provides example for other refactors

### Medium Term (Next)
1. **Find daemon's MIDI parsing code**
2. **Add `MidiEvent::from_midi_msg()` helper** to `midimon-core`
   ```rust
   impl MidiEvent {
       pub fn from_midi_msg(msg: &midi_msg::MidiMsg, time: Instant) -> Option<Self> {
           // Convert midi-msg types to MidiEvent
       }
   }
   ```
3. **Refactor MIDI Learn** to use `midi-msg`
4. **Benchmark** to ensure no performance regression

### Long Term (Future)
- Consider **fully adopting `midi-msg`** throughout codebase
- Add **Polyphonic Aftertouch** support to `MidiEvent` enum
- Distinguish **Note On (velocity 0)** from **Note Off** if needed

---

## Next Steps

1. **Locate daemon's MIDI parsing code** ← IMMEDIATE
2. Evaluate performance impact of `midi-msg`
3. Create shared helper function in `midimon-core`
4. Refactor MIDI Learn to use `midi-msg`
5. Update documentation

