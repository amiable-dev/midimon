# Actions Reference

Actions define what happens when a trigger condition is met. MIDIMon supports a rich set of action types that can be composed in powerful ways.

## Quick Reference

| Action Type | Description | Complexity |
|-------------|-------------|------------|
| [Keystroke](#keystroke) | Send keyboard shortcuts | Simple |
| [Text](#text) | Type text strings | Simple |
| [Launch](#launch) | Open applications | Simple |
| [Shell](#shell) | Execute shell commands | Simple |
| [Sequence](#sequence) | Chain multiple actions | Moderate |
| [Delay](#delay) | Add timing control | Simple |
| [MouseClick](#mouseclickcode>) | Simulate mouse clicks | Simple |
| [Repeat](#repeat) | Repeat actions N times | Moderate |
| [Conditional](#conditional) | Conditional execution | Advanced |

## Simple Actions

### Keystroke

Send keyboard shortcuts with modifiers. The most common action type for productivity workflows.

```toml
# Single key
[modes.mappings.action]
type = "Keystroke"
keys = "space"

# With modifiers
[modes.mappings.action]
type = "Keystroke"
keys = "c"
modifiers = ["cmd"]  # Cmd+C

# Multiple modifiers
[modes.mappings.action]
type = "Keystroke"
keys = "t"
modifiers = ["cmd", "shift"]  # Cmd+Shift+T
```

**Available Modifiers**:
- `cmd` / `command` / `meta` - Command key (macOS) / Windows key
- `ctrl` / `control` - Control key
- `alt` / `option` - Alt/Option key
- `shift` - Shift key

**Special Keys**:
- Navigation: `up`, `down`, `left`, `right`, `home`, `end`, `pageup`, `pagedown`
- Editing: `backspace`, `delete`, `tab`, `return`, `enter`, `escape`, `esc`
- Function: `f1` through `f12`
- Other: `space`

### Text

Type arbitrary text strings with full Unicode support. Uses keyboard simulation to type character-by-character, making it reliable for Unicode and complex text.

```toml
[modes.mappings.action]
type = "Text"
text = "Hello, World!"
```

**Use Cases**:
- Email signatures and contact information
- Code snippets and boilerplate
- Commonly used phrases and templates
- Form auto-fill with saved data
- Multi-language text input
- Text expansion (abbreviation → full text)

**Advanced Examples**:

**Multi-line Code Snippet**:
```toml
[modes.mappings.action]
type = "Text"
text = """
fn main() {
    println!(\"Hello, World!\");
}
"""
```

**Email Signature**:
```toml
[modes.mappings.action]
type = "Text"
text = """
Best regards,
John Doe
Senior Developer
john@example.com
"""
```

**Unicode and Emoji**:
```toml
[modes.mappings.action]
type = "Text"
text = "✅ Task completed! 🎉"
```

**Special Characters**:
```toml
[modes.mappings.action]
type = "Text"
text = "!@#$%^&*()_+-=[]{}|;':\",./<>?"
```

**Slow Typing for Laggy UIs**:
```toml
[modes.mappings.action]
type = "Sequence"
actions = [
    { type = "Text", text = "user" },
    { type = "Delay", ms = 100 },
    { type = "Text", text = "name" },
    { type = "Delay", ms = 100 },
    { type = "Text", text = "123" }
]
```

**TOML String Escaping Tips**:
- Escape quotes: `"He said \"hello\""`
- Multiline strings: `"""Line 1\nLine 2"""`
- Literal strings (no escaping): `'C:\Users\file.txt'`
- Backslashes: Double them `\\` or use literal strings

**Platform Behavior**:
- Does NOT use clipboard (leaves clipboard unchanged)
- Types character-by-character via keyboard simulation
- Respects active keyboard layout
- Requires text input focus in target app

**Troubleshooting**:
- If text doesn't type: Ensure app has input focus
- If Unicode fails: Check app supports UTF-8
- If too fast: Use Sequence with Delay actions
- If wrong characters: Check keyboard layout or escape TOML strings

### Launch

Open applications by name or path. Simple, cross-platform application launcher.

**Basic Usage**:
```toml
[modes.mappings.action]
type = "Launch"
app = "Terminal"  # macOS application name
```

**Full Path**:
```toml
[modes.mappings.action]
type = "Launch"
app = "/Applications/Utilities/Terminal.app"
```

**Script Execution**:
```toml
[modes.mappings.action]
type = "Launch"
app = "/Users/username/scripts/backup.sh"  # Must have execute permissions (chmod +x)
```

**Multiple Apps (Streaming Setup)**:
```toml
[modes.mappings.action]
type = "Sequence"
actions = [
    { type = "Launch", app = "OBS" },
    { type = "Delay", ms = 2000 },  # Wait for OBS to load
    { type = "Launch", app = "Discord" },
    { type = "Delay", ms = 1000 },
    { type = "Launch", app = "Spotify" }
]
```

**Development Environment**:
```toml
[modes.mappings.action]
type = "Sequence"
actions = [
    { type = "Launch", app = "Visual Studio Code" },
    { type = "Delay", ms = 1500 },
    { type = "Launch", app = "Terminal" },
    { type = "Delay", ms = 500 },
    { type = "Launch", app = "Safari" }
]
```

**Platform Behavior**:
- **macOS**: Uses `open -a` command
  - App names: "Safari", "Visual Studio Code", "Logic Pro"
  - Full paths: "/Applications/Safari.app"
  - If running, brings to front (doesn't launch duplicate)
- **Linux**: Direct executable launch
  - Requires full path or executable in $PATH
  - Typically launches new instance even if running
- **Windows**: Uses `cmd /C start` command
  - Accepts app names or paths
  - Uses file associations

**Troubleshooting**:
- Test manually: `open -a "App Name"` (macOS) or `which app-name` (Linux)
- Enable debug logging: `DEBUG=1 cargo run --release 2`
- Use full paths if app names don't work
- Ensure scripts have execute permissions: `chmod +x script.sh`

### Shell

Execute arbitrary shell commands. Provides full system access.

```toml
[modes.mappings.action]
type = "Shell"
command = "git status"
```

**Common Examples**:
```toml
# Git operations
command = "git add . && git commit -m 'quick save'"

# System info
command = "system_profiler SPUSBDataType | grep -i mikro"

# File operations
command = "open ~/Downloads"

# AppleScript (macOS)
command = "osascript -e 'display notification \"MIDI triggered!\"'"
```

## Timing & Flow Control

### Delay

Add pauses between actions in sequences.

```toml
[modes.mappings.action]
type = "Delay"
ms = 500  # 500 milliseconds
```

**Typical Uses**:
- Wait for UI to load
- Slow down rapid automation
- Add deliberate pacing to sequences

### Sequence

Execute multiple actions in order. Automatic 50ms delay between actions.

```toml
[modes.mappings.action]
type = "Sequence"
actions = [
    { type = "Keystroke", keys = "space", modifiers = ["cmd"] },  # Spotlight
    { type = "Delay", ms = 200 },  # Wait for Spotlight
    { type = "Text", text = "Terminal" },
    { type = "Keystroke", keys = "return" }  # Launch
]
```

**Design Patterns**:

**1. Application Launcher**:
```toml
type = "Sequence"
actions = [
    { type = "Keystroke", keys = "space", modifiers = ["cmd"] },
    { type = "Delay", ms = 100 },
    { type = "Text", text = "VS Code" },
    { type = "Keystroke", keys = "return" }
]
```

**2. File Workflow**:
```toml
type = "Sequence"
actions = [
    { type = "Keystroke", keys = "s", modifiers = ["cmd"] },  # Save
    { type = "Delay", ms = 100 },
    { type = "Keystroke", keys = "w", modifiers = ["cmd"] },  # Close
    { type = "Delay", ms = 50 },
    { type = "Keystroke", keys = "down" },  # Next file
    { type = "Keystroke", keys = "return" }  # Open
]
```

**3. Git Workflow**:
```toml
type = "Sequence"
actions = [
    { type = "Shell", command = "git add -A" },
    { type = "Delay", ms = 100 },
    { type = "Shell", command = "git commit -m 'quick save'" },
    { type = "Delay", ms = 100 },
    { type = "Shell", command = "git push" }
]
```

### Repeat

Repeat an action (or sequence) multiple times.

```toml
# Simple repeat: scroll down 10 times
[modes.mappings.action]
type = "Repeat"
count = 10
action = { type = "Keystroke", keys = "down" }
```

**With Delay Between Iterations**:
```toml
[modes.mappings.action]
type = "Repeat"
count = 5
delay_between_ms = 200  # 200ms between each press
action = { type = "Keystroke", keys = "down" }
```

**Repeat a Sequence**:
```toml
[modes.mappings.action]
type = "Repeat"
count = 3
delay_between_ms = 1000
action = {
    type = "Sequence",
    actions = [
        { type = "Keystroke", keys = "down" },  # Select next item
        { type = "Delay", ms = 100 },
        { type = "Keystroke", keys = "return" },  # Open
        { type = "Delay", ms = 500 },
        { type = "Keystroke", keys = "w", modifiers = ["cmd"] }  # Close
    ]
}
```

**Error Handling**:
```toml
[modes.mappings.action]
type = "Repeat"
count = 3
delay_between_ms = 2000
stop_on_error = true  # Stop if any iteration fails
action = { type = "Launch", app = "Xcode" }
```

**Parameters**:
- `count` (required): Number of repetitions (0 = no-op, 1 = run once)
- `action` (required): Action to repeat
- `delay_between_ms` (optional): Delay between iterations (not after last)
- `stop_on_error` (optional): Stop on first failure (default: false)

**Edge Cases**:
- `count = 0` is valid (no-op)
- `count = 1` executes once with no delay
- Nested repeats multiply: 10 outer × 5 inner = 50 total
- Large counts (>1000) may appear as hang
- Blocking operation - cannot interrupt

**Use Cases**:
1. **Pagination**: Scroll through long lists
2. **Batch Processing**: Repeat workflow on multiple items
3. **Velocity Mapping**: Different repeat counts for soft/medium/hard presses
4. **Retry Logic**: Try launching app multiple times

## Mouse Actions

### MouseClick

Simulate mouse clicks with optional positioning.

```toml
# Click at current cursor position
[modes.mappings.action]
type = "MouseClick"
button = "Left"

# Click at specific coordinates
[modes.mappings.action]
type = "MouseClick"
button = "Right"
x = 500
y = 300
```

**Button Options**: `Left`, `Right`, `Middle`

**Use Cases**:
- Click UI elements at fixed positions
- Context menu automation
- Drag-and-drop workflows (with sequences)

## Advanced Actions

### Conditional

Execute actions based on runtime conditions (app state, time, modifiers).

```toml
[modes.mappings.action]
type = "Conditional"
conditions = [
    { type = "AppRunning", bundle_id = "com.apple.Logic" }
]
operator = "And"
then_action = { type = "Keystroke", keys = "space" }
else_action = { type = "Launch", app = "Logic Pro" }
```

**Parameters**:
- `conditions` (array): Array of condition objects
- `operator` (string, optional): "And" or "Or" (default: "And")
- `then_action` (object): Action when conditions are true
- `else_action` (object, optional): Action when conditions are false

**Condition Types**:

**AppRunning / AppNotRunning** - Check if app is running:
```toml
{ type = "AppRunning", bundle_id = "com.spotify.client" }
{ type = "AppNotRunning", bundle_id = "com.microsoft.VSCode" }
```

**TimeRange** - Check current time:
```toml
{ type = "TimeRange", start = "09:00", end = "17:00" }  # 24-hour format
```

**DayOfWeek** - Check day of week:
```toml
{ type = "DayOfWeek", days = ["Mon", "Tue", "Wed", "Thu", "Fri"] }
```

**ModifierPressed** - Check if modifier key is held:
```toml
{ type = "ModifierPressed", modifier = "Shift" }  # Shift, Ctrl, Cmd, Alt
```

**ModeActive** - Check active mode:
```toml
{ type = "ModeActive", mode = 1 }  # Zero-based index
```

**Example - Time-based launcher**:
```toml
[modes.mappings.action]
type = "Conditional"
conditions = [
    { type = "TimeRange", start = "09:00", end = "17:00" },
    { type = "DayOfWeek", days = ["Mon", "Tue", "Wed", "Thu", "Fri"] }
]
operator = "And"
then_action = { type = "Launch", app = "Slack" }  # Work app
else_action = { type = "Launch", app = "Discord" }  # Personal app
```

**Example - Nested conditionals**:
```toml
[modes.mappings.action]
type = "Conditional"
conditions = [{ type = "TimeRange", start = "09:00", end = "17:00" }]
then_action = {
    type = "Conditional",
    conditions = [{ type = "AppRunning", bundle_id = "com.microsoft.VSCode" }],
    then_action = { type = "Keystroke", keys = "b", modifiers = ["cmd", "shift"] },
    else_action = { type = "Launch", app = "Visual Studio Code" }
}
else_action = { type = "Launch", app = "Spotify" }
```

## Action Composition Patterns

### 1. Velocity-Based Repeat Count

Different repeat counts based on how hard you hit the pad:

```toml
[modes.mappings.trigger]
type = "VelocityRange"
note = 10
ranges = [
    # Soft: repeat 3 times
    { min = 0, max = 40, action = {
        type = "Repeat",
        count = 3,
        action = { type = "Keystroke", keys = "down" }
    }},
    # Medium: repeat 5 times
    { min = 41, max = 80, action = {
        type = "Repeat",
        count = 5,
        action = { type = "Keystroke", keys = "down" }
    }},
    # Hard: repeat 10 times
    { min = 81, max = 127, action = {
        type = "Repeat",
        count = 10,
        action = { type = "Keystroke", keys = "down" }
    }}
]
```

### 2. Nested Repeats

Multiply effect (use with caution!):

```toml
[modes.mappings.action]
type = "Repeat"
count = 3  # Outer loop
action = {
    type = "Repeat",
    count = 5,  # Inner loop → 3 × 5 = 15 total
    action = { type = "Keystroke", keys = "down" }
}
```

### 3. Complex Automation Workflow

Combine sequences, repeats, and delays:

```toml
[modes.mappings.action]
type = "Sequence"
actions = [
    # Open file browser
    { type = "Keystroke", keys = "o", modifiers = ["cmd"] },
    { type = "Delay", ms = 500 },

    # Navigate to folder
    { type = "Keystroke", keys = "g", modifiers = ["cmd", "shift"] },
    { type = "Delay", ms = 200 },
    { type = "Text", text = "~/Downloads" },
    { type = "Keystroke", keys = "return" },
    { type = "Delay", ms = 500 },

    # Process first 3 files
    { type = "Repeat", count = 3, delay_between_ms = 1000, action = {
        type = "Sequence",
        actions = [
            { type = "Keystroke", keys = "return" },  # Open file
            { type = "Delay", ms = 800 },
            { type = "Keystroke", keys = "e", modifiers = ["cmd"] },  # Export
            { type = "Delay", ms = 500 },
            { type = "Keystroke", keys = "return" },  # Confirm
            { type = "Delay", ms = 1000 },
            { type = "Keystroke", keys = "w", modifiers = ["cmd"] },  # Close
            { type = "Delay", ms = 200 },
            { type = "Keystroke", keys = "down" }  # Next file
        ]
    }}
]
```

## Performance & Best Practices

### Timing Guidelines

- **Keystroke**: Instant (<1ms)
- **Text**: ~10ms per character
- **Launch**: 100-2000ms (app dependent)
- **Shell**: Variable (command dependent)
- **Sequence**: 50ms auto-delay between actions
- **Delay**: As specified
- **MouseClick**: <5ms
- **Repeat**: count × (action time + delay_between_ms)

### Optimization Tips

1. **Minimize Delays**: Only add delays where needed (UI loading, animations)
2. **Batch Operations**: Use Sequence instead of multiple mappings
3. **Avoid High-Frequency Repeats**: Add `delay_between_ms` for rapid repeats
4. **Test Incrementally**: Start with count=1, then increase
5. **Consider User Experience**: Long-running actions should have feedback

### Common Pitfalls

1. **Blocking Operations**: Repeats and sequences block - cannot interrupt
2. **Timing Fragility**: UI timing varies by system load
3. **Nested Explosions**: 10 × 10 nested repeat = 100 executions
4. **Error Swallowing**: `stop_on_error = false` continues silently

### Debugging Actions

Enable debug mode to see action execution:

```bash
DEBUG=1 cargo run --release 2
```

Watch for:
- Action start/completion logs
- Timing between actions
- Error messages
- Repeat iteration counts

## See Also

- [Action Types Reference](../reference/action-types.md) - Complete technical specifications
- [Configuration Examples](examples.md) - Real-world configuration patterns
- [Trigger Types](../reference/trigger-types.md) - Trigger configuration reference
