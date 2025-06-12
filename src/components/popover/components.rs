use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct PopoverMarker;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct PopoverTriggerMarker;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct PopoverContentMarker;

#[derive(Component, Debug, Clone, Copy)]
pub struct PopoverState {
    pub open: bool,
}

