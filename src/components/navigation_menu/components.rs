use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct NavigationMenuMarker;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct NavigationMenuItemMarker;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct NavigationMenuTriggerMarker;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct NavigationMenuContentMarker;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct NavigationMenuLinkMarker;

#[derive(Component, Debug, Clone)]
pub struct NavigationMenuItemState {
    pub open: bool,
}
