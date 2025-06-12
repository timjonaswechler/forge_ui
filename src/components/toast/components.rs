use bevy::prelude::*;

#[derive(Component, Default, Debug, Clone, Copy)]
pub struct ToastMarker;

#[derive(Component, Default, Debug, Clone, Copy)]
pub struct ToastCloseMarker;

#[derive(Component, Debug, Clone)]
pub struct ToastLifetime {
    pub timer: Timer,
}
