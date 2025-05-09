// src/components/button/mod.rs
pub mod builder;
pub mod components;
pub mod enums;
pub mod events;
pub mod style;
pub mod systems;

// Öffentliche API der Button-Komponente
pub use builder::ButtonBuilder; // Der generische Builder
pub use components::{ButtonMarker, ButtonState, NoAction}; // NoAction ist nützlich für Anwender
pub use enums::{ButtonChild, ButtonSize, ButtonVariant};
pub use events::ButtonClickedEvent; // Das generische Event
pub use systems::handle_button_press;
pub use systems::handle_button_release;
pub use systems::update_button_visuals;

// Fassade für einen einfachen Button ohne spezifische externe Aktion
pub struct Button;
impl Button {
    /// Erstellt einen neuen ButtonBuilder für einen Button ohne spezifische, anwendungsdefinierte Aktion.
    /// Die Aktion ist vom Typ `NoAction`.
    pub fn build() -> builder::ButtonBuilder<components::NoAction> {
        builder::ButtonBuilder::<components::NoAction>::new()
    }
}

// Optional: Ein Plugin, das die Basis-Systeme und -Events (für NoAction) registriert
// Aber da `handle_button_interaction` generisch ist, muss es in der App für jeden Typ A
// separat hinzugefügt werden. Ein Plugin hier könnte nur `update_button_visuals`
// und `ButtonClickedEvent<NoAction>` (als Beispiel) registrieren.
// Das ist oft verwirrender, als dem Anwender die volle Kontrolle über die Registrierung zu geben.
// Vorerst KEIN Plugin hier, der Anwender registriert `handle_button_interaction<A>` und `ButtonClickedEvent<A>` selbst.
