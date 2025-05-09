use bevy::prelude::*;

#[derive(Component, Clone, Copy)]
pub struct SwitchTrackColor(pub Option<Color>);

#[derive(Component, Default, Debug, Clone, Copy)]
/// Marker für den Switch-Track
pub struct SwitchMarker;

#[derive(Component, Default, Debug, Clone, Copy)]
/// Marker für den Daumen
pub struct SwitchThumbMarker;

#[derive(Component, Default, Debug, Clone, Copy)]
/// Marker für die Overlay-Node bei disabled
pub struct SwitchOverlayMarker;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
/// Zustand des Switches
pub struct SwitchState {
    pub checked: bool,
    pub disabled: bool,
}
