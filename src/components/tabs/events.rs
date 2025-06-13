use bevy::prelude::*;

#[derive(Event, Debug, Clone)]
pub struct TabsChangedEvent {
    pub tabs_entity: Entity,
    pub value: String,
}
