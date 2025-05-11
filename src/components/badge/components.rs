// src/components/badge/components.rs
use bevy::prelude::*;

/// Marker-Komponente für Badges.
///
/// Füge diese Komponente einer Entity hinzu, um sie als Badge zu kennzeichnen und
/// entsprechende Systeme / Styles darauf anzuwenden.
#[derive(Component, Default, Debug, Clone, Copy)]
pub struct BadgeMarker;
