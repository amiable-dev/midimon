# Configuration Examples

Real-world configuration patterns for common use cases. Each example is production-ready and can be adapted to your workflow.

## Table of Contents

1. [Basic Workflows](#basic-workflows)
2. [Developer Productivity](#developer-productivity)
3. [Content Creation](#content-creation)
4. [Repetition & Automation](#repetition--automation)
5. [Advanced Patterns](#advanced-patterns)

## Basic Workflows

### Application Launcher

Quick launch frequently used applications:

```toml
[[modes]]
name = "Launcher"
color = "blue"

[[modes.mappings]]
description = "Open Terminal"
[modes.mappings.trigger]
type = "Note"
note = 60
[modes.mappings.action]
type = "Launch"
app = "Terminal"

[[modes.mappings]]
description = "Open VS Code"
[modes.mappings.trigger]
type = "Note"
note = 61
[modes.mappings.action]
type = "Launch"
app = "Visual Studio Code"

[[modes.mappings]]
description = "Open Browser"
[modes.mappings.trigger]
type = "Note"
note = 62
[modes.mappings.action]
type = "Launch"
app = "Google Chrome"
```

### Text Snippets

Common text expansions for email and documentation:

```toml
[[modes.mappings]]
description = "Email signature"
[modes.mappings.trigger]
type = "Note"
note = 70
[modes.mappings.action]
type = "Text"
text = """
Best regards,
Chris Joseph
Software Engineer
chris@amiable.dev
"""

[[modes.mappings]]
description = "Code review template"
[modes.mappings.trigger]
type = "Note"
note = 71
[modes.mappings.action]
type = "Text"
text = """
## Review Checklist
- [ ] Code follows style guide
- [ ] Tests are passing
- [ ] Documentation updated
- [ ] No obvious security issues
"""
```

## Developer Productivity

### Git Workflow

Common git operations on a single pad:

```toml
[[modes]]
name = "Git"
color = "green"

# Soft press: git status
[[modes.mappings]]
description = "Git status (soft)"
[modes.mappings.trigger]
type = "VelocityRange"
note = 60
ranges = [{ min = 0, max = 40 }]
[modes.mappings.action]
type = "Shell"
command = "git status"

# Medium press: git add & commit
[[modes.mappings]]
description = "Quick commit (medium)"
[modes.mappings.trigger]
type = "VelocityRange"
note = 60
ranges = [{ min = 41, max = 80 }]
[modes.mappings.action]
type = "Sequence"
actions = [
    { type = "Shell", command = "git add -A" },
    { type = "Delay", ms = 100 },
    { type = "Shell", command = "git commit -m 'quick save'" }
]

# Hard press: commit and push
[[modes.mappings]]
description = "Commit and push (hard)"
[modes.mappings.trigger]
type = "VelocityRange"
note = 60
ranges = [{ min = 81, max = 127 }]
[modes.mappings.action]
type = "Sequence"
actions = [
    { type = "Shell", command = "git add -A" },
    { type = "Delay", ms = 100 },
    { type = "Shell", command = "git commit -m 'quick save'" },
    { type = "Delay", ms = 100 },
    { type = "Shell", command = "git push" }
]
```

### Code Navigation

Navigate code with velocity-based jumps:

```toml
# Jump lines based on velocity
[[modes.mappings]]
description = "Jump down (velocity-based)"
[modes.mappings.trigger]
type = "VelocityRange"
note = 65
ranges = [
    { min = 0, max = 40, action = { type = "Keystroke", keys = "down" } },
    { min = 41, max = 80, action = { type = "Repeat", count = 5, action = { type = "Keystroke", keys = "down" } } },
    { min = 81, max = 127, action = { type = "Repeat", count = 10, action = { type = "Keystroke", keys = "down" } } }
]

# Navigate between functions
[[modes.mappings]]
description = "Next function"
[modes.mappings.trigger]
type = "Note"
note = 66
[modes.mappings.action]
type = "Keystroke"
keys = "down"
modifiers = ["cmd", "shift"]
```

## Content Creation

### Video Editing

Timeline navigation and editing:

```toml
[[modes]]
name = "Video"
color = "purple"

# Play/Pause
[[modes.mappings]]
description = "Play/Pause"
[modes.mappings.trigger]
type = "Note"
note = 60
[modes.mappings.action]
type = "Keystroke"
keys = "space"

# Frame-by-frame navigation
[[modes.mappings]]
description = "Previous frame"
[modes.mappings.trigger]
type = "Note"
note = 61
[modes.mappings.action]
type = "Keystroke"
keys = "left"

[[modes.mappings]]
description = "Next frame"
[modes.mappings.trigger]
type = "Note"
note = 62
[modes.mappings.action]
type = "Keystroke"
keys = "right"

# Jump 10 frames
[[modes.mappings]]
description = "Jump forward 10 frames"
[modes.mappings.trigger]
type = "Note"
note = 63
[modes.mappings.action]
type = "Repeat"
count = 10
action = { type = "Keystroke", keys = "right" }
```

### Document Formatting

Batch formatting operations:

```toml
# Apply heading and next line
[[modes.mappings]]
description = "Apply heading 2"
[modes.mappings.trigger]
type = "Note"
note = 70
[modes.mappings.action]
type = "Sequence"
actions = [
    { type = "Keystroke", keys = "home" },  # Start of line
    { type = "Text", text = "## " },
    { type = "Keystroke", keys = "end" },  # End of line
    { type = "Keystroke", keys = "return" },  # New line
]
```

## Repetition & Automation

### Simple Repeats

Scroll through long lists or documents:

```toml
# Scroll down 10 lines
[[modes.mappings]]
description = "Scroll down 10 lines"
[modes.mappings.trigger]
type = "Note"
note = 80
[modes.mappings.action]
type = "Repeat"
count = 10
action = { type = "Keystroke", keys = "down" }

# Slow scroll (with delay)
[[modes.mappings]]
description = "Slow scroll down"
[modes.mappings.trigger]
type = "Note"
note = 81
[modes.mappings.action]
type = "Repeat"
count = 5
delay_between_ms = 200
action = { type = "Keystroke", keys = "down" }

# Page down 5 times
[[modes.mappings]]
description = "Jump 5 pages down"
[modes.mappings.trigger]
type = "Note"
note = 82
[modes.mappings.action]
type = "Repeat"
count = 5
delay_between_ms = 300
action = { type = "Keystroke", keys = "pagedown" }
```

### Batch File Processing

Process multiple files with a repeated sequence:

```toml
[[modes.mappings]]
description = "Batch process 3 files"
[modes.mappings.trigger]
type = "LongPress"
note = 85
hold_duration_ms = 1500
[modes.mappings.action]
type = "Repeat"
count = 3
delay_between_ms = 1000
action = {
    type = "Sequence",
    actions = [
        { type = "Keystroke", keys = "return" },  # Open file
        { type = "Delay", ms = 800 },  # Wait for file to load
        { type = "Keystroke", keys = "e", modifiers = ["cmd"] },  # Export
        { type = "Delay", ms = 500 },
        { type = "Keystroke", keys = "return" },  # Confirm export
        { type = "Delay", ms = 1000 },  # Wait for export
        { type = "Keystroke", keys = "w", modifiers = ["cmd"] },  # Close file
        { type = "Delay", ms = 200 },
        { type = "Keystroke", keys = "down" }  # Next file
    ]
}
```

### Velocity-Based Repeats

Different repeat counts based on pad pressure:

```toml
[[modes.mappings]]
description = "Variable repeat based on velocity"
[modes.mappings.trigger]
type = "VelocityRange"
note = 90
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
        delay_between_ms = 100,
        action = { type = "Keystroke", keys = "down" }
    }},
    # Hard: repeat 10 times
    { min = 81, max = 127, action = {
        type = "Repeat",
        count = 10,
        delay_between_ms = 50,
        action = { type = "Keystroke", keys = "down" }
    }}
]
```

### Nested Repeats

Multiply effect for grid or matrix operations:

```toml
# Navigate a 3x5 grid (15 total moves)
[[modes.mappings]]
description = "Navigate 3x5 grid"
[modes.mappings.trigger]
type = "DoubleTap"
note = 95
max_interval_ms = 300
[modes.mappings.action]
type = "Repeat"
count = 3  # 3 rows
delay_between_ms = 500
action = {
    type = "Sequence",
    actions = [
        # Move right 5 times
        { type = "Repeat", count = 5, delay_between_ms = 100, action = { type = "Keystroke", keys = "right" } },
        # Move down 1
        { type = "Delay", ms = 200 },
        { type = "Keystroke", keys = "down" },
        # Return to start of row
        { type = "Repeat", count = 5, action = { type = "Keystroke", keys = "left" } }
    ]
}
```

### Retry Logic

Attempt operations with error tolerance:

```toml
[[modes.mappings]]
description = "Launch Xcode (retry 3 times)"
[modes.mappings.trigger]
type = "Note"
note = 100
[modes.mappings.action]
type = "Repeat"
count = 3
delay_between_ms = 2000
stop_on_error = false  # Continue even if already running
action = { type = "Launch", app = "Xcode" }

[[modes.mappings]]
description = "Network test (stop on first failure)"
[modes.mappings.trigger]
type = "Note"
note = 101
[modes.mappings.action]
type = "Repeat"
count = 5
delay_between_ms = 1000
stop_on_error = true  # Stop if ping fails
action = { type = "Shell", command = "ping -c 1 8.8.8.8" }
```

## Advanced Patterns

### Spotlight Launcher

Use Spotlight for precise app launching:

```toml
[[modes.mappings]]
description = "Spotlight launcher for Terminal"
[modes.mappings.trigger]
type = "Note"
note = 110
[modes.mappings.action]
type = "Sequence"
actions = [
    { type = "Keystroke", keys = "space", modifiers = ["cmd"] },  # Open Spotlight
    { type = "Delay", ms = 200 },  # Wait for Spotlight
    { type = "Text", text = "Terminal" },
    { type = "Delay", ms = 100 },
    { type = "Keystroke", keys = "return" }
]
```

### Form Automation

Fill complex forms with timing:

```toml
[[modes.mappings]]
description = "Fill registration form"
[modes.mappings.trigger]
type = "LongPress"
note = 115
hold_duration_ms = 2000
[modes.mappings.action]
type = "Sequence"
actions = [
    # First name
    { type = "Text", text = "Chris" },
    { type = "Delay", ms = 100 },
    { type = "Keystroke", keys = "tab" },
    { type = "Delay", ms = 100 },

    # Last name
    { type = "Text", text = "Joseph" },
    { type = "Delay", ms = 100 },
    { type = "Keystroke", keys = "tab" },
    { type = "Delay", ms = 100 },

    # Email
    { type = "Text", text = "chris@amiable.dev" },
    { type = "Delay", ms = 100 },
    { type = "Keystroke", keys = "tab" },
    { type = "Delay", ms = 100 },

    # Phone
    { type = "Text", text = "+1-555-123-4567" },
    { type = "Delay", ms = 200 },

    # Submit
    { type = "Keystroke", keys = "return" }
]
```

### Multi-Click Automation

Repeat mouse clicks at intervals:

```toml
[[modes.mappings]]
description = "Click button 5 times"
[modes.mappings.trigger]
type = "DoubleTap"
note = 120
max_interval_ms = 300
[modes.mappings.action]
type = "Repeat"
count = 5
delay_between_ms = 500
action = { type = "MouseClick", button = "Left" }

# Click at specific location repeatedly
[[modes.mappings]]
description = "Click refresh button"
[modes.mappings.trigger]
type = "Note"
note = 121
[modes.mappings.action]
type = "Repeat"
count = 3
delay_between_ms = 2000
action = { type = "MouseClick", button = "Left", x = 1200, y = 100 }
```

### Macro Pad Layout

Complete 16-pad layout for development:

```toml
[device]
name = "Mikro"
auto_connect = true

[[modes]]
name = "Development"
color = "green"

# Row 1: Git operations
[[modes.mappings]]
description = "Git status"
[modes.mappings.trigger]
type = "Note"
note = 60
[modes.mappings.action]
type = "Shell"
command = "git status"

[[modes.mappings]]
description = "Git diff"
[modes.mappings.trigger]
type = "Note"
note = 61
[modes.mappings.action]
type = "Shell"
command = "git diff"

[[modes.mappings]]
description = "Quick commit"
[modes.mappings.trigger]
type = "Note"
note = 62
[modes.mappings.action]
type = "Sequence"
actions = [
    { type = "Shell", command = "git add -A" },
    { type = "Delay", ms = 100 },
    { type = "Shell", command = "git commit -m 'quick save'" }
]

[[modes.mappings]]
description = "Git push"
[modes.mappings.trigger]
type = "Note"
note = 63
[modes.mappings.action]
type = "Shell"
command = "git push"

# Row 2: Build & test
[[modes.mappings]]
description = "Build project"
[modes.mappings.trigger]
type = "Note"
note = 64
[modes.mappings.action]
type = "Shell"
command = "cargo build"

[[modes.mappings]]
description = "Run tests"
[modes.mappings.trigger]
type = "Note"
note = 65
[modes.mappings.action]
type = "Shell"
command = "cargo test"

[[modes.mappings]]
description = "Run app"
[modes.mappings.trigger]
type = "Note"
note = 66
[modes.mappings.action]
type = "Shell"
command = "cargo run"

[[modes.mappings]]
description = "Format code"
[modes.mappings.trigger]
type = "Note"
note = 67
[modes.mappings.action]
type = "Shell"
command = "cargo fmt"

# Row 3: Navigation
[[modes.mappings]]
description = "Jump down (velocity)"
[modes.mappings.trigger]
type = "VelocityRange"
note = 68
ranges = [
    { min = 0, max = 40, action = { type = "Keystroke", keys = "down" } },
    { min = 41, max = 80, action = { type = "Repeat", count = 5, action = { type = "Keystroke", keys = "down" } } },
    { min = 81, max = 127, action = { type = "Repeat", count = 10, action = { type = "Keystroke", keys = "down" } } }
]

[[modes.mappings]]
description = "Jump up (velocity)"
[modes.mappings.trigger]
type = "VelocityRange"
note = 69
ranges = [
    { min = 0, max = 40, action = { type = "Keystroke", keys = "up" } },
    { min = 41, max = 80, action = { type = "Repeat", count = 5, action = { type = "Keystroke", keys = "up" } } },
    { min = 81, max = 127, action = { type = "Repeat", count = 10, action = { type = "Keystroke", keys = "up" } } }
]

# Row 4: Utilities
[[modes.mappings]]
description = "Open Terminal"
[modes.mappings.trigger]
type = "Note"
note = 72
[modes.mappings.action]
type = "Launch"
app = "Terminal"

[[modes.mappings]]
description = "Open VS Code"
[modes.mappings.trigger]
type = "Note"
note = 73
[modes.mappings.action]
type = "Launch"
app = "Visual Studio Code"

[[modes.mappings]]
description = "Open Browser"
[modes.mappings.trigger]
type = "Note"
note = 74
[modes.mappings.action]
type = "Launch"
app = "Google Chrome"
```

### Context-Aware Actions (Conditional)

Execute different actions based on runtime conditions like active app, time of day, or system state.

#### App-Based Conditional

Different behavior when specific apps are running:

```toml
[[modes.mappings]]
description = "Context-aware play/pause"
[modes.mappings.trigger]
type = "Note"
note = 130
[modes.mappings.action]
type = "Conditional"
conditions = [
    { type = "AppRunning", bundle_id = "com.apple.Logic" }
]
then_action = { type = "Keystroke", keys = "space" }  # Logic: Space to play/pause
else_action = { type = "Keystroke", keys = "space", modifiers = ["cmd"] }  # System: Media play/pause
```

#### Time-Based Automation

Launch different apps based on work hours:

```toml
[[modes.mappings]]
description = "Time-aware launcher"
[modes.mappings.trigger]
type = "Note"
note = 131
[modes.mappings.action]
type = "Conditional"
conditions = [
    { type = "TimeRange", start = "09:00", end = "17:00" },
    { type = "DayOfWeek", days = ["Mon", "Tue", "Wed", "Thu", "Fri"] }
]
operator = "And"
then_action = { type = "Launch", app = "Slack" }  # Work app during work hours
else_action = { type = "Launch", app = "Discord" }  # Personal app otherwise
```

#### Multiple Conditions (OR Logic)

Launch file if any IDE is running:

```toml
[[modes.mappings]]
description = "Open project in active IDE"
[modes.mappings.trigger]
type = "Note"
note = 132
[modes.mappings.action]
type = "Conditional"
conditions = [
    { type = "AppRunning", bundle_id = "com.microsoft.VSCode" },
    { type = "AppRunning", bundle_id = "com.jetbrains.IntelliJ" },
    { type = "AppRunning", bundle_id = "com.sublimetext.4" }
]
operator = "Or"  # Any one IDE is enough
then_action = { type = "Keystroke", keys = "o", modifiers = ["cmd"] }  # Cmd+O to open file
else_action = { type = "Launch", app = "Visual Studio Code" }  # Launch IDE if none running
```

#### Modifier-Based Conditional

Hold Shift for alternative behavior:

```toml
[[modes.mappings]]
description = "Delete or force-delete"
[modes.mappings.trigger]
type = "Note"
note = 133
[modes.mappings.action]
type = "Conditional"
conditions = [
    { type = "ModifierPressed", modifier = "Shift" }
]
then_action = { type = "Keystroke", keys = "delete", modifiers = ["cmd", "option"] }  # Force delete
else_action = { type = "Keystroke", keys = "delete", modifiers = ["cmd"] }  # Normal delete
```

#### Nested Conditionals

Complex decision trees with multiple levels:

```toml
[[modes.mappings]]
description = "Smart launcher with time and app detection"
[modes.mappings.trigger]
type = "LongPress"
note = 135
hold_duration_ms = 1000
[modes.mappings.action]
type = "Conditional"
conditions = [
    { type = "TimeRange", start = "09:00", end = "17:00" }
]
then_action = {
    type = "Conditional",
    conditions = [
        { type = "AppRunning", bundle_id = "com.apple.Logic" }
    ],
    then_action = { type = "Keystroke", keys = "n", modifiers = ["cmd"] },  # New Logic project
    else_action = { type = "Launch", app = "Logic Pro" }  # Launch Logic
}
else_action = {
    type = "Conditional",
    conditions = [
        { type = "DayOfWeek", days = ["Sat", "Sun"] }
    ],
    then_action = { type = "Launch", app = "Spotify" },  # Weekend music
    else_action = { type = "Launch", app = "Safari" }  # Evening browsing
}
```

#### Launch Only if Not Running

Avoid duplicate app launches:

```toml
[[modes.mappings]]
description = "Launch Spotify or play/pause if running"
[modes.mappings.trigger]
type = "Note"
note = 136
[modes.mappings.action]
type = "Conditional"
conditions = [
    { type = "AppNotRunning", bundle_id = "com.spotify.client" }
]
then_action = { type = "Launch", app = "Spotify" }
else_action = { type = "Keystroke", keys = "space" }  # Already running, just play/pause
```

#### Mode-Aware Shortcuts

Different actions per mode using global mappings:

```toml
[[global_mappings]]
description = "Mode-aware F5 key"
[global_mappings.trigger]
type = "Note"
note = 137
[global_mappings.action]
type = "Conditional"
conditions = [
    { type = "ModeActive", mode = 1 }  # Development mode
]
then_action = { type = "Shell", command = "npm test" }  # Run tests in dev mode
else_action = { type = "Keystroke", keys = "f5" }  # Refresh in other modes
```

#### Weekend vs Weekday Behavior

Different workflows based on day of week:

```toml
[[modes.mappings]]
description = "Morning routine launcher"
[modes.mappings.trigger]
type = "Note"
note = 138
[modes.mappings.action]
type = "Conditional"
conditions = [
    { type = "DayOfWeek", days = ["Sat", "Sun"] },
    { type = "TimeRange", start = "08:00", end = "12:00" }
]
operator = "And"
then_action = {
    type = "Sequence",
    actions = [
        { type = "Launch", app = "Apple Music" },
        { type = "Delay", ms = 1000 },
        { type = "Launch", app = "Mail" }
    ]
}
else_action = {
    type = "Sequence",
    actions = [
        { type = "Launch", app = "Slack" },
        { type = "Delay", ms = 1000 },
        { type = "Launch", app = "Calendar" },
        { type = "Delay", ms = 500 },
        { type = "Launch", app = "Visual Studio Code" }
    ]
}
```

## Performance Tips

### Timing Best Practices

1. **Application Launch**: Wait 1000-2000ms after launching apps
2. **UI Navigation**: Use 100-200ms between UI interactions
3. **Form Fills**: 100ms delay between field navigation
4. **File Operations**: 500-1000ms for save/load operations
5. **Network Operations**: 2000ms+ for remote operations

### Repeat Guidelines

1. **Start Small**: Begin with count=1, increase gradually
2. **Add Delays**: Use `delay_between_ms` for counts >5
3. **Test Incrementally**: Verify each step before combining
4. **Consider UX**: Counts >100 may appear as hang
5. **Monitor Performance**: Watch CPU/memory with large counts

### Debugging

Enable debug mode to see action execution:

```bash
DEBUG=1 cargo run --release 2
```

Common issues:
- **Timing too tight**: Increase `delay_between_ms`
- **Actions not executing**: Check application focus
- **Repeats stopping early**: Check `stop_on_error` setting
- **Slow performance**: Reduce repeat counts or add delays

## See Also

- [Actions Reference](actions.md) - Complete action type documentation
- [Action Types Reference](../reference/action-types.md) - Technical specifications
- [Trigger Types](../reference/trigger-types.md) - Trigger configuration
