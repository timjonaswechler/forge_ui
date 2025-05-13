// Interne Module für die Organisation des Dialog-Codes

mod builder;
mod components;
mod events;
mod helper;
mod plugin;
mod systems;

// Öffentliche API
pub use builder::DialogBodyBuilder; // Öffentlich machen
pub use builder::DialogBuilder;
pub use builder::DialogContentBuilder;
pub use builder::DialogFooterBuilder;
pub use builder::DialogHeaderBuilder;
pub use builder::DialogTriggerBuilder;
pub use plugin::*; // Öffentlich machen // Öffentlich machen

pub use components::{DialogAction, DialogId};
pub use events::{CloseDialogEvent, OpenDialogEvent};

// Interne Nutzung für das dialog Modul
pub(crate) use components::*;
pub(crate) use helper::*;
pub(crate) use systems::*;
