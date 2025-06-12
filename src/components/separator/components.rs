use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct SeparatorMarker;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeparatorOrientation {
    Horizontal,
    Vertical,
}

impl Default for SeparatorOrientation {
    fn default() -> Self { Self::Horizontal }
}

#[derive(Component, Debug, Clone, Copy)]
pub struct SeparatorState {
    pub orientation: SeparatorOrientation,
    pub decorative: bool,
}

impl Default for SeparatorState {
    fn default() -> Self {
        Self { orientation: SeparatorOrientation::Horizontal, decorative: false }
    }
}
