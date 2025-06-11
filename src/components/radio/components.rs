// src/components/radio/components.rs
use bevy::prelude::*;

use super::{RadioSize, RadioVariant};

/// Marker component for the inner indicator dot.
#[derive(Component)]
pub struct RadioIndicator;

/// Marker component for the radio button entity itself.
#[derive(Component)]
pub struct RadioMarker;

/// Current state for a radio button.
#[derive(Component)]
pub struct RadioState {
    pub checked: bool,
    pub disabled: bool,
    pub value: String,
    pub variant: RadioVariant,
    pub size: RadioSize,
}

/// Wrapper component carrying the callback for a radio selection.
#[derive(Component)]
pub struct OnSelect(pub Box<dyn Fn(String) + Send + Sync + 'static>);

/// Component used to assign radios to a logical group.
#[derive(Component)]
pub struct RadioGroup(pub String);

/// Internal helper component linking to a callback ID.
#[derive(Component)]
pub struct OnSelectId(pub u32);
