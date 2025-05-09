// src/components/badge/components.rs
use bevy::prelude::*;

use crate::{ButtonSize, ButtonVariant};

/// Marker-Komponente für Badges.
#[derive(Component, Default, Debug, Clone, Copy)]
pub struct BadgeMarker;

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
