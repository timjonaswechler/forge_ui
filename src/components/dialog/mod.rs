// Interne Module für die Organisation des Dialog-Codes

mod builder;
mod components;
mod events;
mod systems;

// Öffentliche API
pub use builder::DialogBodyBuilder; // Öffentlich machen
pub use builder::DialogBuilder;
pub use builder::DialogFooterBuilder;
pub use builder::DialogHeaderBuilder; // Öffentlich machen // Öffentlich machen

pub use components::{DialogCloseTrigger, DialogId};
pub use events::{CloseDialogEvent, OpenDialogEvent};

// Interne Nutzung für das dialog Modul
pub(crate) use builder::*;
pub(crate) use components::*;
pub(crate) use events::*;
pub(crate) use systems::*;
