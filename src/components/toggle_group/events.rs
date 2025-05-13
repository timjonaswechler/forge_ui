use bevy::prelude::*;
use std::collections::HashSet;

/// Event triggered when ToggleGroup selection changes
#[derive(Event, Clone)]
pub struct ToggleGroupChangedEvent<A: Component + Clone + Send + Sync + 'static> {
    /// Entity that originated the event
    pub source_entity: Entity,
    /// Current active values
    pub active_values: HashSet<String>,
    /// Optional action component
    pub action_id: Option<A>,
}

impl<A: Component + Clone + Send + Sync + 'static> std::fmt::Debug for ToggleGroupChangedEvent<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ToggleGroupChangedEvent")
            .field("source_entity", &self.source_entity)
            .field("active_values", &self.active_values)
            .finish_non_exhaustive()
    }
}

#[derive(Event, Debug, Clone)]
pub enum FocusEvent {
    Focus { entity: Entity },
    Blur { entity: Entity },
}
