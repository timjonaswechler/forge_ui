use bevy::prelude::*;

/// Fired when the selection inside a `RadioGroup` changes.
#[derive(Event, Clone)]
pub struct RadioGroupChangedEvent {
    /// Entity id of the `RadioGroup` root.
    pub source_entity: Entity,
    /// Group identifier (same as `RadioGroupState.name`).
    pub group_name: String,
    /// Newly selected value.
    pub selected_value: String,
}
