// src/components/button/mod.rs
pub mod builder;
pub mod components;
pub mod enums;
pub mod events;
pub mod style;
pub mod systems;

/// Haupt-Typ für Buttons, nur Fassade für den Builder.
pub struct Button;

impl Button {
    /// Starte die Builder-Kette.
    pub fn build() -> builder::ButtonBuilder {
        builder::ButtonBuilder::new()
    }
}

// Falls du trotzdem den Typ selbst brauchst:
pub use builder::ButtonBuilder;
pub use components::ButtonMarker;
pub use enums::{ButtonSize, ButtonVariant};
pub use events::ButtonClickedEvent;
pub use systems::{handle_button_clicks_event, handle_button_clicks_fn, update_button_visuals};
