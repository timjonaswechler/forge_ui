use bevy::prelude::*;

/// Marker-Komponente für die Checkbox-Entity.
///
/// Dient zur Identifikation in Queries und für Systemspezifisches Styling.
#[derive(Component, Default, Debug, Clone, Copy)]
pub struct CheckboxMarker;

/// Speichert den aktuellen Zustand der Checkbox.
///
/// - `checked`: Gibt an, ob die Checkbox aktiviert ist.
/// - `disabled`: Gibt an, ob die Checkbox keine Interaktionen erlaubt.
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

/// Enthält die Entity des internal gerenderten Checkmark-Icons.
///
/// Wird genutzt, um Sichtbarkeit und Styling des Icons zu steuern.
#[derive(Component, Debug, Clone, Copy)]
pub struct CheckmarkIconEntity(pub Entity);

/// Marker for the overlay entity shown when a checkbox is disabled.
#[derive(Component, Default, Debug, Clone, Copy)]
pub struct CheckboxDisabledOverlayMarker;
