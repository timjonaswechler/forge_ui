// src/components/badge/mod.rs
mod builder;
mod components;
mod enums;
mod utils;

/// Haupt-Typ für Buttons, nur Fassade für den Builder.
pub struct Badge;

impl Badge {
    /// Starte die Builder-Kette.
    pub fn build(text: impl Into<String>) -> builder::BadgeBuilder {
        builder::BadgeBuilder::new(text)
    }
}

// Falls du trotzdem den Typ selbst brauchst:
pub use builder::BadgeBuilder;
pub use components::BadgeMarker;
pub use enums::BadgeVariant;
