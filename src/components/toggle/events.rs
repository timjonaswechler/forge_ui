use bevy::prelude::*;

/// Wird gesendet, wenn sich der Toggle‑Zustand ändert.
#[derive(Event, Clone)]
pub struct ToggleChangedEvent<A: Component + Clone + Send + Sync + 'static> {
    pub source_entity: Entity,
    pub pressed: bool,
    pub action_id: Option<A>,
}

impl<A: Component + Clone + Send + Sync + 'static> std::fmt::Debug for ToggleChangedEvent<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ToggleChangedEvent")
            .field("source_entity", &self.source_entity)
            .field("pressed", &self.pressed)
            .finish_non_exhaustive()
    }
}
