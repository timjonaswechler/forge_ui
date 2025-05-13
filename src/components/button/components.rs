// components/button/components.rs
use super::enums::{ButtonSize, ButtonVariant};
use bevy::prelude::*;
// PhantomData wird nicht mehr für OnClick benötigt

/// Marker-Komponente für alle von `ButtonBuilder` erstellten Buttons.
/// Dient zur einfachen Filterung in Queries.
#[derive(Component, Default, Debug)]
pub struct ButtonMarker;

/// Speichert den konfigurierten Zustand eines Buttons, wie Variante, Größe und Deaktivierungsstatus.
///
/// Wird von `update_button_visuals` verwendet, um das Aussehen des Buttons anzupassen.
#[derive(Component, Debug, Clone)]
pub struct ButtonState {
    pub variant: ButtonVariant,
    pub size: ButtonSize,
    pub disabled: bool,
}
