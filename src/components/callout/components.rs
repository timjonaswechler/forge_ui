use bevy::prelude::*;

/// Marker component for callout boxes.
#[derive(Component, Default, Debug, Clone, Copy)]
pub struct CalloutMarker;

/// Visual variant of a callout.
#[derive(Debug, Clone, Copy, Default)]
pub enum CalloutVariant {
    #[default]
    Info,
    Success,
    Warning,
    Error,
}
