use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct SliderMarker;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct SliderTrackMarker;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct SliderRangeMarker;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct SliderThumbMarker;

#[derive(Component, Debug, Clone, Copy)]
pub struct SliderState {
    pub value: f32,
}

impl Default for SliderState {
    fn default() -> Self {
        Self { value: 0.0 }
    }
}

