use bevy::prelude::*;

#[derive(Component)]
pub struct SelectMarker;

#[derive(Component)]
pub struct SelectTriggerMarker;

#[derive(Component)]
pub struct SelectTriggerTextMarker;

#[derive(Component)]
pub struct SelectContentMarker;

#[derive(Component)]
pub struct SelectOptionMarker {
    pub value: String,
    pub label: String,
}

#[derive(Component, Default)]
pub struct SelectState {
    pub open: bool,
    pub selected: Option<String>,
}
