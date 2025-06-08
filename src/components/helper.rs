use crate::theme::UiTheme;
use bevy::prelude::*;

/// Der Standard-Aktionstyp die keine spezifische,
/// anwendungsdefinierte Aktion auslösen.
///
/// Wenn `ButtonBuilder` mit diesem Typ verwendet wird (z.B. durch `ButtonBuilder::new()`),
/// und `.action(NoAction)` explizit aufgerufen wird, enthält das gesendete
/// [`ButtonClickedEvent<NoAction>`] im Feld `action_id` ein `Some(NoAction)`.
///
/// Ohne expliziten `.action(NoAction)`-Aufruf wird die `NoAction`-Komponente nicht zur
/// Entität hinzugefügt, und `action_id` im Event wird `None` sein.
#[derive(Component, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct NoAction;

pub trait UiBuilder<'w, 's> {
    type Output;

    fn spawn(
        self,
        parent: &'s mut ChildSpawnerCommands<'w>,
        theme: &UiTheme,
        font: &Handle<Font>,
    ) -> Self::Output;
}
