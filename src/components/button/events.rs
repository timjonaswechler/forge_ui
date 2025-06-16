use bevy::prelude::{Component, Entity, Event};
use std::fmt::Debug;

/// Wird gesendet, wenn eine Button-Interaktion stattfindet.
///
/// Standardmäßig wird dieses Event beim **Loslassen** der Maustaste über dem Button ausgelöst,
/// wenn das System [`handle_button_release`] registriert ist.
///
/// Es kann auch beim **Drücken** der Maustaste ausgelöst werden, wenn das System
/// [`handle_button_press`] registriert ist. Die meisten Anwendungen werden
/// [`handle_button_release`] für das typische "Klick"-Verhalten bevorzugen.
///
/// Dieses Event ist generisch über den Aktionstyp `A`, der mit dem Button
/// über [`ButtonBuilder::action()`] verknüpft wurde.
///
/// # Felder
/// - `source_entity`: Die [`Entity`] des Buttons, der die Interaktion ausgelöst hat.
/// - `action_id`: Ein `Option<A>`, das ein Klon der Aktions-Komponente enthält,
///   falls eine auf dem Button gesetzt war. Ist `None`, wenn keine Aktion
///   spezifiziert wurde (oder die Komponente `A` nicht auf der Entität war).
///
/// # Event-Handler und Systemregistrierung
/// Um auf Button-Klicks (typischerweise Release) zu reagieren, müssen Sie:
/// 1. Dieses Event für jeden verwendeten Aktionstyp `A` in Ihrer App registrieren:
///    `.add_event::<ButtonClickedEvent<A>>()`
/// 2. Die zugehörigen Interaktionssysteme registrieren. Wählen Sie basierend auf dem gewünschten Verhalten:
///    - Für Reaktion beim **Loslassen** der Maustaste (übliches Klickverhalten):
///      `.add_systems(Update, handle_button_release::<A>)`
///    - Für Reaktion beim **Drücken** der Maustaste:
///      `.add_systems(Update, handle_button_press::<A>)`
///    - Sie können auch beide registrieren, wenn Sie auf beide Phasen reagieren müssen.
/// 3. Das visuelle Update-System registrieren (einmalig für alle Buttons):
///    `.add_systems(Update, update_button_visuals)`
/// 4. Ein eigenes System schreiben, das `EventReader<ButtonClickedEvent<A>>` verwendet,
///    um auf die Events zu lauschen und die `action_id` zu verarbeiten.
///
/// Siehe [`ButtonBuilder`](crate::components::button::ButtonBuilder) für ein umfassenderes Code-Beispiel zur Builder-Verwendung.
/// Siehe auch [`handle_button_press`](crate::components::button::handle_button_press)
/// und [`handle_button_release`](crate::components::button::handle_button_release).
#[derive(Event, Clone)]
pub struct ButtonClickedEvent<A: Component + Clone + Send + Sync> {
    /// Die Entität des Buttons, der geklickt wurde.
    pub source_entity: Entity,
    /// Die optionale anwendungsspezifische Aktion, die mit diesem Button verknüpft ist.
    /// Ist `Some(action_instance)` wenn der Builder mit `.action(action_instance)` konfiguriert wurde,
    /// ansonsten `None`.
    pub action_id: Option<A>,
}

// Manuelle Debug-Implementierung, die nicht versucht, A zu debuggen,
// falls A nicht Debug implementiert. Dies verhindert Kompilierfehler,
// wenn A kein Debug-Trait hat, und ist nützlich für Bibliotheks-Code.
impl<A: Component + Clone + Send + Sync> Debug for ButtonClickedEvent<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ButtonClickedEvent")
            .field("source_entity", &self.source_entity)
            .field(
                "action_id",
                &self
                    .action_id
                    .as_ref()
                    .map(|_| "Some<A> (Inhalt ausgelassen für Debug)"),
            )
            .finish_non_exhaustive()
    }
}
