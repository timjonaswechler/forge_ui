// src/components/dialog/events.rs
use super::components::DialogId;
use bevy::prelude::*; // Stelle sicher, dass der Pfad korrekt ist

#[derive(Event, Debug, Clone)]
pub struct OpenDialogEvent(pub DialogId);

#[derive(Event, Debug, Clone)]
pub struct CloseDialogEvent {
    pub id_to_close: Option<DialogId>, // Optional: Gezieltes Schließen
                                       // Wenn None, wird oft der oberste modale Dialog geschlossen
}

impl CloseDialogEvent {
    /// Erstellt ein Event zum Schließen des obersten/aktuellen modalen Dialogs.
    pub fn current_modal() -> Self {
        Self { id_to_close: None }
    }

    /// Erstellt ein Event zum Schließen eines spezifischen Dialogs.
    pub fn specific(id: DialogId) -> Self {
        Self {
            id_to_close: Some(id),
        }
    }
}
