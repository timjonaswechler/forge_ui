use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct TooltipMarker;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct TooltipTriggerMarker;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct TooltipContentMarker;

#[derive(Component, Debug, Clone, Copy)]
pub struct TooltipState {
    pub open: bool,
}
