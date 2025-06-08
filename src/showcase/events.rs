use bevy::prelude::*;

/// Tastatur (F12 / Esc) oder Button führt zu einem Toggle-Event
#[derive(Event, Debug, Clone)]
pub struct ToggleShowcaseEvent;
