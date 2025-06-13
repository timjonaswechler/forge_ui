use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct ToolbarMarker;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolbarOrientation {
    Horizontal,
    Vertical,
}

impl Default for ToolbarOrientation {
    fn default() -> Self { Self::Horizontal }
}

#[derive(Component, Debug, Clone, Copy)]
pub struct ToolbarState {
    pub orientation: ToolbarOrientation,
}
