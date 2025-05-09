use super::SectionElementBuilderFn;
use crate::components::button::{ButtonBuilder, NoAction as ButtonNoAction}; // Beispiel für Button
use crate::theme::UiTheme;
use bevy::prelude::*;

#[derive(Default)]
pub struct DialogFooterBuilder {
    // Man könnte ButtonBuilder Instanzen direkt speichern,
    // oder generische Content-Fn für maximale Flexibilität.
    // Wir wählen eine Mischung.
    pub(crate) elements: Vec<SectionElementBuilderFn>,
    // Spezifische Optionen, z.B. Ausrichtung der Elemente (links, rechts, zentriert)
    pub(crate) justify_content: JustifyContent,
}

impl DialogFooterBuilder {
    pub fn new() -> Self {
        Self {
            justify_content: JustifyContent::FlexEnd, // Standardmäßig Buttons rechts
            ..Default::default()
        }
    }

    /// Ändert die Ausrichtung der Elemente im Footer.
    pub fn justify_content(mut self, justify: JustifyContent) -> Self {
        self.justify_content = justify;
        self
    }

    /// Fügt einen Button zum Footer hinzu.
    /// Um Flexibilität zu wahren und nicht den kompletten ButtonBuilder hier neu zu implementieren,
    /// könnte man entweder den `ButtonBuilder` direkt akzeptieren oder eine Closure.
    /// Einfacher ist oft, die volle Konfiguration über eine Closure zu ermöglichen.
    pub fn add_button<A: Component + Clone + Send + Sync + 'static>(
        self,
        config_fn: impl FnOnce(ButtonBuilder<A>, &UiTheme, &Handle<Font>) -> Entity, // Zum Anpassen
    ) -> Self {
        // Dieser Ansatz ist etwas komplizierter, weil wir theme & font hier noch nicht haben.
        // Einfacher ist es, eine Closure zu speichern, die später aufgerufen wird.

        // Wir verwenden `add_custom_content` für Buttons, um es einfach zu halten.
        // Oder wir speichern ButtonBuilder-Konfigurationen und spawnen sie in spawn_into.
        // Für maximale Flexibilität:
        self
    }

    pub fn add_custom_content(
        mut self,
        element_builder: impl FnOnce(&mut ChildSpawnerCommands, &UiTheme, &Handle<Font>)
            + Send
            + Sync
            + 'static,
    ) -> Self {
        self.elements.push(Box::new(element_builder));
        self
    }

    pub(crate) fn spawn_into(
        self,
        parent: &mut ChildSpawnerCommands,
        theme: &UiTheme,
        font_handle: &Handle<Font>,
    ) {
        // Hier werden die Elemente gespawnt.
        // Der Container für den Footer (der `parent`) muss `justify_content` entsprechend setzen.
        // Das passiert im Haupt-DialogBuilder beim Erstellen des Footer-Containers.
        for element_fn in self.elements {
            (element_fn)(parent, theme, font_handle);
        }
    }
}
