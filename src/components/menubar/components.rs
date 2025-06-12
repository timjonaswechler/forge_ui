use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct MenubarMarker;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct MenubarMenuMarker;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct MenubarMenuTriggerMarker;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct MenubarMenuContentMarker;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct MenubarMenuItemMarker;

#[derive(Component, Debug, Clone)]
pub struct MenubarMenuState {
    pub open: bool,
}
