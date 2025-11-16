// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! MIDIMon Library (Compatibility Layer)
//!
//! This module provides backward compatibility for tests written against
//! the old monolithic structure. It re-exports types from midimon_core.
//!
//! New code should use midimon_core directly instead of this module.

// Re-export everything from midimon_core for backward compatibility
pub use midimon_core::*;

// Module aliases for backward compatibility with old test imports
pub mod config {
    pub use midimon_core::config::*;
}

pub mod event_processor {
    pub use midimon_core::event_processor::*;
}

pub mod actions {
    pub use midimon_core::actions::*;
    // Note: ActionExecutor moved to midimon-daemon in Phase 2 refactor
    // Use `midimon_daemon::ActionExecutor` instead
}

pub mod mappings {
    pub use midimon_core::mapping::*;
}

pub mod feedback {
    pub use midimon_core::feedback::*;
}

pub mod device_profile {
    pub use midimon_core::device::*;
}

// Note: mikro_leds and midi_feedback are private implementation details
// in midimon_core and are not re-exported. Tests should use the public
// PadFeedback trait instead.
