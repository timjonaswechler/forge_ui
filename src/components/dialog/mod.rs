// Interne Module für die Organisation des Dialog-Codes
mod builder;
mod components;
mod events;
// mod style; // Vorerst nicht benötigt, Styling im Builder
mod systems;

pub(crate) use builder::*;
pub(crate) use components::*;
pub(crate) use events::*;
pub(crate) use systems::*;

// Öffentliche API für die Dialog-Komponente
pub use builder::DialogBuilder;
pub use components::{
    // DialogRootMarker, // Eher internes Detail, nicht unbedingt public nötig
    // DialogContent,  // Eher internes Detail
    // DialogOverlay,  // Eher internes Detail
    DialogCloseTrigger, // Wichtig für Anwender, um Schließen-Buttons zu markieren
    DialogId,           // Damit Anwender Dialoge identifizieren können
};
pub use events::{CloseDialogEvent, OpenDialogEvent};
