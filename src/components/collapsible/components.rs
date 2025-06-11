use bevy::prelude::*;

/// Marker for collapsible containers.
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct CollapsibleMarker;

/// Runtime state tracking whether the collapsible is open.
#[derive(Component, Debug, Clone, Copy)]
pub struct CollapsibleState {
    pub open: bool,
}

impl Default for CollapsibleState {
    fn default() -> Self {
        Self { open: false }
    }
}
