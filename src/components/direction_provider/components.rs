use bevy::prelude::*;

/// Reading direction for UI components.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReadingDirection {
    Ltr,
    Rtl,
}

impl Default for ReadingDirection {
    fn default() -> Self {
        Self::Ltr
    }
}

/// Component providing a [`ReadingDirection`] to its children.
#[derive(Component, Debug, Clone, Copy)]
pub struct DirectionProvider {
    pub dir: ReadingDirection,
}

impl Default for DirectionProvider {
    fn default() -> Self {
        Self {
            dir: ReadingDirection::Ltr,
        }
    }
}
