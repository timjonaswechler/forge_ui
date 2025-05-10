// src/components/radio/mod.rs

mod builder;
mod components;
mod enums;
mod events;
mod style;
mod systems;

pub use builder::*;
pub use components::*;
pub use enums::*;
pub use events::*;
pub use style::*;
pub use systems::*;

pub struct Radio;

impl Radio {
    /// Starte die Builder-Kette.
    pub fn new(value: impl Into<String>) -> builder::RadioBuilder {
        builder::RadioBuilder::new(value.into())
    }
}
