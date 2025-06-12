use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct HoverCardMarker;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct HoverCardTriggerMarker;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct HoverCardContentMarker;

#[derive(Component, Debug, Clone, Copy)]
pub struct HoverCardState {
    pub open: bool,
}
