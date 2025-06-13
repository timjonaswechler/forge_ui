use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct TabsMarker;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct TabsListMarker;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct TabsTriggerMarker;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct TabsContentMarker;

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

#[derive(Component, Debug, Clone)]
pub struct TabsState {
    pub active: Option<String>,
    pub orientation: TabsOrientation,
}

#[derive(Component, Debug, Clone)]
pub struct TabTrigger {
    pub value: String,
}

#[derive(Component, Debug, Clone)]
pub struct TabContent {
    pub value: String,
}
