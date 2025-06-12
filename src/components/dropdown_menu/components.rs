use bevy::prelude::*;

/// Marker for the dropdown menu root container.
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct DropdownMenuMarker;

/// Marker for individual menu items.
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct DropdownMenuItemMarker;
