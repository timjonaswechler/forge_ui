use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct ProgressMarker;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct ProgressTrackMarker;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct ProgressIndicatorMarker;

#[derive(Component, Debug, Clone, Copy)]
pub struct ProgressState {
    pub value: f32,
    pub max: f32,
}
