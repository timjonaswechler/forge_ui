use bevy::prelude::*;

/// Event emitted when an accordion item is toggled.
#[derive(Event, Debug, Clone)]
pub struct AccordionToggledEvent {
    pub accordion_entity: Entity,
    pub is_open: bool,
}
