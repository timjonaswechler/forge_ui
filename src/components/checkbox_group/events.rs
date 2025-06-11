use bevy::prelude::*;
use std::collections::HashSet;

/// Event emitted whenever the selection inside a checkbox group changes.
#[derive(Event, Clone)]
pub struct CheckboxGroupChangedEvent {
    /// Entity id of the checkbox group root.
    pub source_entity: Entity,
    /// Set of all currently checked values.
    pub checked_values: HashSet<String>,
}
