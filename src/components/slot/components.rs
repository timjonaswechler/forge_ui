use bevy::prelude::*;

/// Marker component for a slot root.
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct SlotRootMarker;

/// Marker component for slottable content.
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct SlottableMarker;
