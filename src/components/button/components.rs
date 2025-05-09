// components/button/components.rs
use super::enums::{ButtonSize, ButtonVariant};
use bevy::prelude::*;
// PhantomData wird nicht mehr für OnClick benötigt

/// Marker component for query filtering
#[derive(Component, Default, Debug)]
pub struct ButtonMarker;

/// Stores the configured state of the button
#[derive(Component, Debug, Clone)]
pub struct ButtonState {
    pub variant: ButtonVariant,
    pub size: ButtonSize,
    pub disabled: bool,
}

// NEU: Default-Aktionstyp, wenn keine spezifische Aktion benötigt wird.
// Wird auch im Builder als Default-Typ für A verwendet.
#[derive(Component, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct NoAction;
