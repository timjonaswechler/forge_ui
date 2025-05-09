use bevy::prelude::{Component, Entity, Event};
use std::fmt::Debug; // Import für manuelles Debug, falls A es nicht implementiert

#[derive(Event, Clone)] // Debug manuell implementieren, da A nicht unbedingt Debug ist
pub struct ButtonClickedEvent<A: Component + Clone + Send + Sync + 'static> {
    /// The entity ID otf the button that was clicked.
    pub source_entity: Entity,
    /// The optional action associated with this button.
    pub action_id: Option<A>,
}

// Manuelle Debug-Implementierung, die nicht versucht, A zu debuggen,
// falls A nicht Debug implementiert.
impl<A: Component + Clone + Send + Sync + 'static> Debug for ButtonClickedEvent<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ButtonClickedEvent")
            .field("source_entity", &self.source_entity)
            // Um einen Fehler zu vermeiden, wenn A nicht Debug ist:
            .field(
                "action_id",
                &self.action_id.as_ref().map(|_| "Some<A> (Omitted)"),
            )
            // Oder wenn A immer Debug sein soll:
            // .field("action_id", &self.action_id) // Benötigt A: Debug
            .finish_non_exhaustive() // Gut für Bibliotheks-Events, um spätere Feldhinzufügungen zu erlauben
    }
}
