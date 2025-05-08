// src/components/radio/events.rs

use bevy::prelude::*;

/// Fired when a radio is selected
pub struct RadioSelectedEvent {
    pub entity: Entity,
    pub value: String,
}
