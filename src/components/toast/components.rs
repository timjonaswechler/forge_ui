use bevy::prelude::*;

/// Marker component for toast notifications.
#[derive(Component, Default, Debug, Clone, Copy)]
pub struct ToastMarker;

/// Visual variants for toasts.
#[derive(Debug, Clone, Copy, Default)]
pub enum ToastVariant {
    #[default]
    Info,
    Success,
    Warning,
    Error,
}

/// Timer resource used to despawn a toast after some time.
#[derive(Component, Debug, Clone)]
pub struct ToastTimer(pub Timer);
