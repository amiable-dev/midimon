// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! MIDI Learn mode implementation for GUI
//!
//! Provides session-based MIDI learning that captures raw MIDI input
//! and converts it to trigger configuration suggestions.

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use uuid::Uuid;

/// Result of a MIDI Learn session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MidiLearnResult {
    /// Whether learning succeeded
    pub success: bool,
    /// Suggested trigger configuration
    pub trigger: Option<TriggerSuggestion>,
    /// Error message if learning failed
    pub error: Option<String>,
    /// Duration of the learning session
    pub duration_ms: u64,
}

/// Suggested trigger configuration from learned MIDI input
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum TriggerSuggestion {
    Note {
        note: u8,
        velocity_range: Option<(u8, u8)>,
    },
    VelocityRange {
        note: u8,
        velocity_min: u8,
        velocity_max: u8,
        level: String, // "soft", "medium", "hard"
    },
    LongPress {
        note: u8,
        duration_ms: u64,
    },
    DoubleTap {
        note: u8,
        timeout_ms: u64,
    },
    Chord {
        notes: Vec<u8>,
        window_ms: u64,
    },
    Encoder {
        cc: u8,
        direction: Option<String>, // "clockwise", "counterclockwise"
    },
    CC {
        cc: u8,
        value_range: Option<(u8, u8)>,
    },
    Aftertouch {
        note: Option<u8>,
        pressure_range: (u8, u8),
    },
    PitchBend {
        bend_range: (i16, i16),
    },
}

/// MIDI Learn session state
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LearnSessionState {
    Idle,
    Waiting,
    Captured,
    TimedOut,
    Cancelled,
}

/// MIDI Learn session manager
#[derive(Clone)]
pub struct MidiLearnSession {
    /// Unique session ID
    pub id: String,
    /// Current state
    state: Arc<RwLock<LearnSessionState>>,
    /// Captured result
    result: Arc<RwLock<Option<MidiLearnResult>>>,
    /// Session start time
    start_time: Arc<RwLock<Instant>>,
    /// Timeout duration
    timeout: Duration,
}

impl MidiLearnSession {
    /// Create a new MIDI Learn session
    pub fn new(timeout_secs: u64) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            state: Arc::new(RwLock::new(LearnSessionState::Idle)),
            result: Arc::new(RwLock::new(None)),
            start_time: Arc::new(RwLock::new(Instant::now())),
            timeout: Duration::from_secs(timeout_secs),
        }
    }

    /// Start the learning session
    pub async fn start(&self) {
        let mut state = self.state.write().await;
        *state = LearnSessionState::Waiting;
    }

    /// Check if session has timed out
    pub async fn is_timed_out(&self) -> bool {
        let state = self.state.read().await;
        if *state == LearnSessionState::Waiting {
            let start_time = self.start_time.read().await;
            start_time.elapsed() > self.timeout
        } else {
            false
        }
    }

    /// Cancel the session
    pub async fn cancel(&self) {
        let mut state = self.state.write().await;
        if *state == LearnSessionState::Waiting {
            *state = LearnSessionState::Cancelled;
        }
    }

    /// Get the current state
    pub async fn get_state(&self) -> LearnSessionState {
        self.state.read().await.clone()
    }

    /// Get the elapsed time in milliseconds
    pub async fn elapsed_ms(&self) -> u64 {
        let start_time = self.start_time.read().await;
        start_time.elapsed().as_millis() as u64
    }

    /// Get remaining time in seconds
    pub async fn remaining_secs(&self) -> u64 {
        let start_time = self.start_time.read().await;
        let elapsed = start_time.elapsed();
        if elapsed >= self.timeout {
            0
        } else {
            (self.timeout - elapsed).as_secs()
        }
    }

    /// Capture a MIDI event and convert to trigger suggestion
    pub async fn capture_event(&self, event: MidiEvent) {
        let mut state = self.state.write().await;
        if *state != LearnSessionState::Waiting {
            return;
        }

        let trigger = self.analyze_event(&event);
        let duration_ms = self.elapsed_ms().await;

        let mut result = self.result.write().await;
        *result = Some(MidiLearnResult {
            success: true,
            trigger: Some(trigger),
            error: None,
            duration_ms,
        });

        *state = LearnSessionState::Captured;
    }

    /// Set session as timed out
    pub async fn set_timed_out(&self) {
        let mut state = self.state.write().await;
        if *state == LearnSessionState::Waiting {
            *state = LearnSessionState::TimedOut;

            let duration_ms = self.elapsed_ms().await;
            let mut result = self.result.write().await;
            *result = Some(MidiLearnResult {
                success: false,
                trigger: None,
                error: Some("Learning session timed out - no MIDI input received".to_string()),
                duration_ms,
            });
        }
    }

    /// Get the result (if available)
    pub async fn get_result(&self) -> Option<MidiLearnResult> {
        self.result.read().await.clone()
    }

    /// Analyze a MIDI event and suggest a trigger
    fn analyze_event(&self, event: &MidiEvent) -> TriggerSuggestion {
        match event {
            MidiEvent::NoteOn { note, velocity } => {
                // Suggest based on velocity
                if *velocity < 40 {
                    TriggerSuggestion::VelocityRange {
                        note: *note,
                        velocity_min: 0,
                        velocity_max: 40,
                        level: "soft".to_string(),
                    }
                } else if *velocity < 80 {
                    TriggerSuggestion::VelocityRange {
                        note: *note,
                        velocity_min: 41,
                        velocity_max: 80,
                        level: "medium".to_string(),
                    }
                } else {
                    TriggerSuggestion::VelocityRange {
                        note: *note,
                        velocity_min: 81,
                        velocity_max: 127,
                        level: "hard".to_string(),
                    }
                }
            }
            MidiEvent::NoteOff { note, .. } => {
                TriggerSuggestion::Note {
                    note: *note,
                    velocity_range: None,
                }
            }
            MidiEvent::ControlChange { controller, value } => {
                TriggerSuggestion::CC {
                    cc: *controller,
                    value_range: Some((*value, *value)),
                }
            }
            MidiEvent::PitchBend { value } => {
                TriggerSuggestion::PitchBend {
                    bend_range: (*value, *value),
                }
            }
            MidiEvent::Aftertouch { note, pressure } => {
                TriggerSuggestion::Aftertouch {
                    note: *note,
                    pressure_range: (*pressure, *pressure),
                }
            }
        }
    }
}

/// Simplified MIDI event type for learning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MidiEvent {
    NoteOn { note: u8, velocity: u8 },
    NoteOff { note: u8, velocity: u8 },
    ControlChange { controller: u8, value: u8 },
    PitchBend { value: i16 },
    Aftertouch { note: Option<u8>, pressure: u8 },
}

impl MidiEvent {
    /// Parse from raw MIDI bytes
    pub fn from_bytes(status: u8, data1: u8, data2: u8) -> Option<Self> {
        let message_type = status & 0xF0;

        match message_type {
            0x90 => { // Note On
                if data2 == 0 {
                    Some(MidiEvent::NoteOff { note: data1, velocity: 0 })
                } else {
                    Some(MidiEvent::NoteOn { note: data1, velocity: data2 })
                }
            }
            0x80 => { // Note Off
                Some(MidiEvent::NoteOff { note: data1, velocity: data2 })
            }
            0xB0 => { // Control Change
                Some(MidiEvent::ControlChange { controller: data1, value: data2 })
            }
            0xE0 => { // Pitch Bend
                let value = (((data2 as i16) << 7) | (data1 as i16)) - 8192;
                Some(MidiEvent::PitchBend { value })
            }
            0xA0 => { // Polyphonic Aftertouch
                Some(MidiEvent::Aftertouch { note: Some(data1), pressure: data2 })
            }
            0xD0 => { // Channel Aftertouch
                Some(MidiEvent::Aftertouch { note: None, pressure: data1 })
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_session_lifecycle() {
        let session = MidiLearnSession::new(10);
        assert_eq!(session.get_state().await, LearnSessionState::Idle);

        session.start().await;
        assert_eq!(session.get_state().await, LearnSessionState::Waiting);

        session.cancel().await;
        assert_eq!(session.get_state().await, LearnSessionState::Cancelled);
    }

    #[tokio::test]
    async fn test_capture_note_event() {
        let session = MidiLearnSession::new(10);
        session.start().await;

        let event = MidiEvent::NoteOn { note: 60, velocity: 100 };
        session.capture_event(event).await;

        assert_eq!(session.get_state().await, LearnSessionState::Captured);

        let result = session.get_result().await.unwrap();
        assert!(result.success);
        assert!(result.trigger.is_some());
    }

    #[test]
    fn test_midi_event_parsing() {
        // Note On (C4, velocity 100)
        let event = MidiEvent::from_bytes(0x90, 60, 100);
        assert!(matches!(event, Some(MidiEvent::NoteOn { note: 60, velocity: 100 })));

        // Control Change (CC 7, value 127)
        let event = MidiEvent::from_bytes(0xB0, 7, 127);
        assert!(matches!(event, Some(MidiEvent::ControlChange { controller: 7, value: 127 })));
    }
}
