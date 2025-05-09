use bevy::prelude::*;

/// Marker-Komponente für die Checkbox selbst (der klickbare Bereich).
#[derive(Component, Default, Debug, Clone, Copy)]
pub struct CheckboxMarker;

/// Speichert den aktuellen Zustand der Checkbox.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct CheckboxState {
    pub checked: bool,
    pub disabled: bool,
}

impl Default for CheckboxState {
    fn default() -> Self {
        Self {
            checked: false,
            disabled: false,
        }
    }
}

/// Komponente, die die Entity des internen Checkmark-Icons speichert.
/// Wird verwendet, um dessen Sichtbarkeit zu steuern.
#[derive(Component, Debug, Clone, Copy)]
pub struct CheckmarkIconEntity(pub Entity);
