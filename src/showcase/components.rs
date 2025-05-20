use bevy::prelude::*;

#[derive(Component, Clone, Debug, PartialEq, Eq)]
pub enum ShowcaseAction {
    Toggle,
}

/// Marker um den Root‑Knoten des Debug‑Panels später wiederzufinden/zu despawnen.
#[derive(Component)]
pub struct ShowcaseMarker;

/// Marker für die rechte Preview-Fläche
#[derive(Component)]
pub struct ShowcasePreviewRoot;
