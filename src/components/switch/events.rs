use bevy::prelude::*;

#[derive(Event, Debug, Clone)]
pub struct SwitchChangedEvent {
    pub switch_entity: Entity,
    pub is_checked: bool,
}
