use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct TooltipMarker;

#[derive(Component, Debug, Clone, Copy)]
pub struct TooltipState {
    pub open: bool,
}

impl Default for TooltipState {
    fn default() -> Self {
        Self { open: false }
    }
}

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct TooltipTriggerMarker;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct TooltipContentMarker;
