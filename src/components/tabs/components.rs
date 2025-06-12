use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct TabsMarker;

#[derive(Component, Debug, Clone, Copy)]
pub struct TabsState {
    pub selected: usize,
}

impl Default for TabsState {
    fn default() -> Self {
        Self { selected: 0 }
    }
}

#[derive(Component, Debug, Clone, Copy)]
pub struct TabTriggerMarker {
    pub index: usize,
    pub tabs_entity: Entity,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct TabContentMarker {
    pub index: usize,
    pub tabs_entity: Entity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabsOrientation {
    Horizontal,
    Vertical,
}

impl Default for TabsOrientation {
    fn default() -> Self {
        Self::Horizontal
    }
}
