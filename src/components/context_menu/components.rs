use bevy::prelude::*;

/// Marker for the context menu container.
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct ContextMenuMarker;

/// Marker for individual context menu items.
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct ContextMenuItemMarker;
