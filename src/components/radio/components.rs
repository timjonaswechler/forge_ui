// src/components/radio/components.rs

use bevy::prelude::*;

/// Marker component for a radio button entity
#[derive(Component)]
pub struct Radio;

/// Stores the current checked state
#[derive(Component)]
pub struct RadioState {
    pub checked: bool,
    pub disabled: bool,
}

/// Optional callback when selected
pub struct OnSelect(pub Box<dyn Fn(String) + Send + Sync>);
