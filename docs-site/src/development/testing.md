# Testing Guide

This guide covers testing strategies for MIDIMon, including hardware-independent testing using the MIDI device simulator.

## Table of Contents

- [MIDI Device Simulator](#midi-device-simulator)
- [Running Tests](#running-tests)
- [Code Coverage](#code-coverage)
- [Test Reporting](#test-reporting)
- [Writing Tests](#writing-tests)
- [Interactive CLI Tool](#interactive-cli-tool)
- [Test Scenarios](#test-scenarios)

## MIDI Device Simulator

The MIDI device simulator allows comprehensive testing of MIDIMon without requiring physical hardware. It simulates all MIDI events and complex user interactions with precise timing control.

### Features

The simulator supports:

- **Basic MIDI Events**: Note On/Off, Control Change, Aftertouch, Pitch Bend, Program Change
- **Velocity Levels**: Soft (0-40), Medium (41-80), Hard (81-127)
- **Timing-Based Triggers**: Long press, double-tap detection
- **Complex Gestures**: Chords, encoder rotation, velocity ramps
- **Precise Timing**: Configurable delays and durations
- **Event Capture**: Inspect all generated MIDI messages

### Quick Start

```rust
use midi_simulator::{MidiSimulator, Gesture, EncoderDirection};

// Create a simulator on MIDI channel 0
let sim = MidiSimulator::new(0);

// Simulate a simple note press
sim.note_on(60, 100);
sim.note_off(60);

// Get captured events
let events = sim.get_events();
assert_eq!(events.len(), 2); // Note on + Note off
```

## Running Tests

### Unit Tests

Run the simulator's built-in unit tests:

```bash
cargo test --test midi_simulator
```

Expected output:
```
running 12 tests
test tests::test_note_on_off ... ok
test tests::test_velocity_levels ... ok
test tests::test_control_change ... ok
test tests::test_aftertouch ... ok
test tests::test_pitch_bend ... ok
test tests::test_program_change ... ok
test tests::test_simple_tap_gesture ... ok
test tests::test_chord_gesture ... ok
test tests::test_encoder_simulation ... ok
test tests::test_velocity_ramp_gesture ... ok
test tests::test_scenario_builder ... ok
test tests::test_channel_masking ... ok

test result: ok. 12 passed; 0 failed
```

### Integration Tests

Run integration tests that verify the complete event processing pipeline:

```bash
cargo test --test integration_tests
```

These tests cover:
- Basic note event handling
- Velocity level detection
- Long press simulation with timing validation
- Double-tap detection
- Chord detection with multiple notes
- Encoder direction detection (CW/CCW)
- Aftertouch and pitch bend
- Complex multi-event scenarios

### All Tests

Run all tests including unit and integration:

```bash
cargo test
```

### Using Nextest (Recommended)

For improved test output and parallel execution, use cargo-nextest:

```bash
# Install nextest
cargo install cargo-nextest

# Run tests with nextest
cargo nextest run --all-features

# Or use the convenience script
./scripts/test-nextest.sh
```

Nextest provides:
- Faster test execution through parallelization
- Better output formatting with progress indicators
- More detailed failure reporting
- Per-test timing information

## Code Coverage

MIDIMon uses `cargo-llvm-cov` for code coverage tracking. The project maintains a minimum coverage threshold of 0.35% (baseline) with a Phase 1 target of 85%.

### Installing Coverage Tools

```bash
# Install cargo-llvm-cov
cargo install cargo-llvm-cov
```

### Generating Coverage Reports

#### Terminal Summary (Default)

Generate a coverage summary in the terminal:

```bash
# Using cargo-llvm-cov directly
cargo llvm-cov --all-features --workspace

# Or use the convenience script
./scripts/coverage.sh

# Or use just command
just coverage
```

Output example:
```
Filename                      Regions    Missed Regions     Cover   Functions  Missed Functions  Executed       Lines      Missed Lines     Cover
-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
actions.rs                        212               212     0.00%          12                12     0.00%         134               134     0.00%
config.rs                          52                42    19.23%           3                 2    33.33%          52                47     9.62%
main.rs                           278               278     0.00%          13                13     0.00%         151               151     0.00%
mappings.rs                        96                96     0.00%           8                 8     0.00%          71                71     0.00%
-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
TOTAL                            2263              2253     0.44%          66                65     1.52%        1413              1408     0.35%
```

#### HTML Report

Generate an interactive HTML coverage report:

```bash
# Generate HTML report
./scripts/coverage.sh --html

# Or use just command
just coverage-html

# Report saved to: target/llvm-cov/html/index.html
```

#### HTML Report with Auto-Open

Generate and automatically open the coverage report in your browser:

```bash
# Generate and open HTML report
./scripts/coverage.sh --open

# Or use just command
just coverage-open
```

#### LCOV Format (for CI)

Generate coverage in LCOV format for Codecov or other CI tools:

```bash
# Generate lcov.info
./scripts/coverage.sh --lcov

# Or use just command
just coverage-lcov

# Output: lcov.info
```

### Coverage Configuration

Coverage settings are configured in `.llvm-cov.toml`:

```toml
[report]
# Fail if coverage is below this percentage
fail-under-lines = 0.35

[filter]
# Exclude test files and binaries from coverage
exclude-filename-regex = [
    ".*/tests/.*",
    ".*/bin/.*"
]
```

### Coverage Targets

- **Phase 0 Baseline**: 0.35% (established)
- **Phase 1 Target**: 85% line coverage
- **Minimum Threshold**: 80% (enforced in CI)

### Coverage in CI/CD

Coverage is automatically tracked on every pull request:

1. **GitHub Actions** runs coverage on all PRs
2. **Codecov** receives coverage reports and provides:
   - Coverage percentage badge
   - Line-by-line coverage visualization
   - Coverage diffs on PRs
   - Historical coverage trends
3. **PR Comments** show coverage delta (increase/decrease)
4. **Status Checks** fail if coverage drops below threshold

View current coverage: [![codecov](https://codecov.io/gh/amiable-dev/midimon/branch/main/graph/badge.svg)](https://codecov.io/gh/amiable-dev/midimon)

### Local Coverage Workflow

Recommended workflow for maintaining coverage:

```bash
# 1. Write tests for new code
vim tests/my_feature_test.rs

# 2. Run tests to verify they pass
cargo nextest run

# 3. Generate coverage report
just coverage-open

# 4. Identify uncovered lines (shown in red in HTML report)

# 5. Add tests for uncovered code paths

# 6. Verify coverage improved
just coverage
```

## Test Reporting

### GitHub Actions Integration

All tests run automatically on:
- **Push to main/develop branches**
- **Pull requests**
- **Manual workflow dispatch**

Test results are displayed in the GitHub Actions UI with:
- Test count (passed/failed/skipped)
- Execution time
- Detailed failure logs
- Coverage percentage

### Nextest Reports

Nextest provides enhanced test reporting:

```bash
# Run with detailed output
cargo nextest run --verbose

# Generate JUnit XML report
cargo nextest run --junit junit.xml

# Show only failed tests
cargo nextest run --failure-output immediate
```

### Coverage Reports in PRs

When you create a pull request, Codecov automatically:
1. Analyzes coverage for changed files
2. Posts a comment with coverage diff
3. Updates status checks
4. Shows coverage on changed lines

Example PR comment:
```
Coverage: 85.2% (+2.1%) vs main

Files changed coverage:
  - src/new_feature.rs: 92.3% ✓
  - src/existing.rs: 78.1% ⚠️ (below target)
```

### Local Test Scripts

Use convenience scripts for common testing tasks:

```bash
# Run all tests
./scripts/test.sh

# Run with nextest
./scripts/test-nextest.sh

# Generate coverage
./scripts/coverage.sh [--html|--open|--lcov]
```

### Just Commands

The `justfile` provides convenient shortcuts:

```bash
# View all available commands
just

# Run tests
just test              # Standard cargo test
just test-nextest      # With nextest
just test-watch        # Watch mode (requires cargo-watch)

# Coverage
just coverage          # Terminal summary
just coverage-html     # HTML report
just coverage-open     # HTML report + open in browser
just coverage-lcov     # LCOV format for CI

# Linting and formatting
just lint              # Run clippy
just fmt               # Format code
just fmt-check         # Check formatting

# Complete CI check locally
just ci                # Run all CI checks (fmt, lint, test, coverage)
```

## Writing Tests

### Basic Event Tests

Test simple MIDI event generation:

```rust
#[test]
fn test_my_feature() {
    let sim = MidiSimulator::new(0);

    // Simulate user action
    sim.note_on(60, 100);
    sim.note_off(60);

    // Verify events
    let events = sim.get_events();
    assert_eq!(events.len(), 2);
    assert_eq!(events[0], vec![0x90, 60, 100]); // Note On
    assert_eq!(events[1], vec![0x80, 60, 0x40]); // Note Off
}
```

### Velocity Level Tests

Test velocity-sensitive actions:

```rust
#[test]
fn test_velocity_levels() {
    let sim = MidiSimulator::new(0);

    // Test soft, medium, hard presses
    sim.note_on(60, 30);   // Soft (0-40)
    sim.note_on(60, 70);   // Medium (41-80)
    sim.note_on(60, 110);  // Hard (81-127)

    let events = sim.get_events();
    assert_eq!(events.len(), 3);
}
```

### Timing-Based Tests

Test long press and timing detection:

```rust
#[test]
fn test_long_press() {
    let sim = MidiSimulator::new(0);
    let start = Instant::now();

    sim.perform_gesture(Gesture::LongPress {
        note: 60,
        velocity: 80,
        hold_ms: 2500,
    });

    let duration = start.elapsed();
    assert!(duration >= Duration::from_millis(2500));
}
```

### Double-Tap Tests

Test double-tap detection with gap timing:

```rust
#[test]
fn test_double_tap() {
    let sim = MidiSimulator::new(0);

    sim.perform_gesture(Gesture::DoubleTap {
        note: 60,
        velocity: 80,
        tap_duration_ms: 50,
        gap_ms: 200,
    });

    let events = sim.get_events();
    assert_eq!(events.len(), 4); // 2 note ons + 2 note offs
}
```

### Chord Tests

Test chord detection with multiple simultaneous notes:

```rust
#[test]
fn test_chord() {
    let sim = MidiSimulator::new(0);

    sim.perform_gesture(Gesture::Chord {
        notes: vec![60, 64, 67], // C major chord
        velocity: 80,
        stagger_ms: 10,
        hold_ms: 500,
    });

    let events = sim.get_events();
    assert_eq!(events.len(), 6); // 3 note ons + 3 note offs
}
```

### Encoder Tests

Test encoder rotation with direction detection:

```rust
#[test]
fn test_encoder() {
    let sim = MidiSimulator::new(0);

    sim.perform_gesture(Gesture::EncoderTurn {
        cc: 1,
        direction: EncoderDirection::Clockwise,
        steps: 5,
        step_delay_ms: 0,
    });

    let events = sim.get_events();
    assert_eq!(events.len(), 5);

    // Verify values are increasing
    for i in 1..events.len() {
        assert!(events[i][2] > events[i-1][2]);
    }
}
```

### Scenario Builder

Create complex test scenarios with the builder pattern:

```rust
use midi_simulator::ScenarioBuilder;

#[test]
fn test_complex_scenario() {
    let sim = MidiSimulator::new(0);

    let scenario = ScenarioBuilder::new()
        .note_on(60, 100)
        .wait(100)
        .control_change(1, 64)
        .wait(100)
        .aftertouch(80)
        .wait(100)
        .note_off(60)
        .build();

    sim.execute_sequence(scenario);

    let events = sim.get_events();
    assert_eq!(events.len(), 4);
}
```

## Interactive CLI Tool

The simulator includes an interactive command-line interface for manual testing and experimentation.

### Starting the CLI

```bash
cargo run --bin midi_simulator
```

### Available Commands

```
╭─────────────────────────────────────────────────────────────╮
│ COMMANDS                                                    │
├─────────────────────────────────────────────────────────────┤
│ Basic:                                                      │
│   help, h, ?              Show help message                 │
│   quit, exit, q           Exit the simulator                │
│   clear, c                Clear event queue                 │
│   events, e               Show captured events              │
├─────────────────────────────────────────────────────────────┤
│ MIDI Events:                                                │
│   note <num> <vel>        Send note on/off                  │
│   velocity <note>         Test velocity levels              │
│   long <note> [ms]        Simulate long press               │
│   double <note> [gap_ms]  Simulate double-tap               │
│   chord <n1> <n2> ...     Simulate chord                    │
│   encoder <cc> <cw|ccw>   Simulate encoder rotation         │
│   aftertouch <pressure>   Send aftertouch                   │
│   pitch <value>           Send pitch bend (0-16383)         │
│   cc <num> <val>          Send control change               │
├─────────────────────────────────────────────────────────────┤
│ Scenarios:                                                  │
│   demo                    Run full demonstration            │
│   scenario [name]         Run specific test scenario        │
╰─────────────────────────────────────────────────────────────╯
```

### Example Session

```bash
# Start the CLI
cargo run --bin midi_simulator

# Test velocity levels
> velocity 60
Simulating velocity levels (soft, medium, hard)...
✓ Velocity test complete

# Test long press
> long 60 2500
Simulating long press for 2500ms...
✓ Long press complete

# Test double-tap
> double 60 200
Simulating double-tap with 200ms gap...
✓ Double-tap complete

# Test chord (C major)
> chord 60 64 67
Simulating chord: [60, 64, 67]
✓ Chord complete

# Test encoder rotation
> encoder 1 cw 5
Simulating encoder CC1 Clockwise 5 steps...
✓ Encoder simulation complete

# Show captured events
> events
Captured events:
  1: [90, 60, 64, ...]
  2: [80, 60, 40, ...]
  ...

# Run full demo
> demo
Running demonstration scenarios...
1. Testing velocity levels...
2. Testing long press...
3. Testing double-tap...
4. Testing chord...
5. Testing encoder...
✓ Demo complete

# Exit
> quit
Goodbye!
```

## Test Scenarios

The simulator includes pre-built test scenarios for common testing needs.

### Velocity Scenario

Tests all three velocity levels:

```bash
> scenario velocity
Testing velocity levels: Soft (30), Medium (70), Hard (110)
✓ Velocity scenario complete
```

### Timing Scenario

Tests short, medium, and long press durations:

```bash
> scenario timing
Testing press durations: Short (100ms), Medium (500ms), Long (2500ms)
✓ Timing scenario complete
```

### Double-Tap Scenario

Tests double-tap detection:

```bash
> scenario doubletap
Testing double-tap with 200ms gap
✓ Double-tap scenario complete
```

### Chord Scenario

Tests chord detection with C major:

```bash
> scenario chord
Testing chord detection: C major (60, 64, 67)
✓ Chord scenario complete
```

### Encoder Scenario

Tests encoder rotation in both directions:

```bash
> scenario encoder
Testing encoder: 5 steps CW, then 5 steps CCW
✓ Encoder scenario complete
```

### Complex Scenario

Tests mixed events and complex interactions:

```bash
> scenario complex
Running complex scenario: mixed events...
✓ Complex scenario complete
```

## Advanced Gestures

### Velocity Ramp

Simulate a velocity ramp from soft to hard:

```rust
sim.perform_gesture(Gesture::VelocityRamp {
    note: 60,
    min_velocity: 20,
    max_velocity: 120,
    steps: 5,
});
```

### Simple Tap

Simulate a quick tap with precise duration:

```rust
sim.perform_gesture(Gesture::SimpleTap {
    note: 60,
    velocity: 80,
    duration_ms: 100,
});
```

## Event Inspection

### Getting Events

```rust
// Get all events and clear the queue
let events = sim.get_events();

// Peek at last event without clearing
let last = sim.peek_last_event();

// Clear the queue
sim.clear_events();
```

### Parsing Events

```rust
for event in events {
    let status = event[0];
    let message_type = status & 0xF0;
    let channel = status & 0x0F;

    match message_type {
        0x90 => println!("Note On: {} vel {}", event[1], event[2]),
        0x80 => println!("Note Off: {}", event[1]),
        0xB0 => println!("CC{}: {}", event[1], event[2]),
        0xD0 => println!("Aftertouch: {}", event[1]),
        0xE0 => println!("Pitch Bend: {} {}", event[1], event[2]),
        _ => println!("Unknown message type"),
    }
}
```

## Debug Output

Enable debug output to see all MIDI messages:

```rust
let mut sim = MidiSimulator::new(0);
sim.set_debug(true);

sim.note_on(60, 100);
// Output: [SIM] Sending: [90, 3C, 64]
```

## Best Practices

1. **Clear events between tests**: Always clear the event queue between tests to avoid interference
2. **Use gestures for complex interactions**: Prefer high-level gestures over manual event sequences
3. **Verify timing**: Use `Instant::now()` to verify timing-sensitive operations
4. **Test edge cases**: Test velocity boundaries (0, 40, 41, 80, 81, 127)
5. **Test multiple channels**: Verify channel masking works correctly
6. **Use scenario builder**: Build complex scenarios declaratively with the builder pattern

## Continuous Integration

The simulator is designed to work in CI environments without hardware:

```yaml
# .github/workflows/test.yml
name: Test

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions-rust-lang/setup-rust-toolchain@v1
      - name: Run tests
        run: cargo test --all-features
```

## Troubleshooting

### Tests Timeout

If timing-based tests timeout, increase tolerance:

```rust
assert!(duration >= Duration::from_millis(2500));
assert!(duration < Duration::from_millis(2700)); // 200ms tolerance
```

### Event Count Mismatch

Remember that `get_events()` clears the queue:

```rust
let events1 = sim.get_events(); // Gets and clears
let events2 = sim.get_events(); // Empty, events were already consumed
```

### Velocity Detection

Ensure velocities match expected ranges:

```rust
// Soft: 0-40
// Medium: 41-80
// Hard: 81-127
```

## Related Documentation

- [Event Processing Architecture](../architecture/event-processing.md)
- [MIDI Event Types](../reference/midi-events.md)
- [Action System](../reference/actions.md)
- [Contributing Guide](../contributing.md)

## Examples

See the integration tests in `tests/integration_tests.rs` for complete examples of:
- Velocity detection tests
- Long press simulation
- Double-tap detection
- Chord detection
- Encoder simulation
- Complex multi-event scenarios

## Support

For questions or issues with testing:
- Check existing integration tests for examples
- Use the interactive CLI tool to experiment
- Review the simulator source code in `tests/midi_simulator.rs`
- Open an issue on GitHub with test failure details
