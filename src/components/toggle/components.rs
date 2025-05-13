use crate::components::toggle::enums::{ToggleSize, ToggleVariant};
use bevy::prelude::*;
/// Marker für die Haupt‑Entität eines Toggles.
#[derive(Component, Debug, Clone, Copy)]
pub struct ToggleMarker;

/// Laufzeit‑Zustand eines Toggles.
#[derive(Component, Debug, Clone)]
pub struct ToggleState {
    pub pressed: bool,
    pub disabled: bool,
    pub variant: ToggleVariant,
    pub size: ToggleSize,
}

impl Default for ToggleState {
    fn default() -> Self {
        Self {
            pressed: false,
            disabled: false,
            variant: ToggleVariant::Default,
            size: ToggleSize::Medium,
        }
    }
}
