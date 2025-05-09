// components/button/components.rs
use super::enums::{ButtonSize, ButtonVariant};
use bevy::prelude::*;
// PhantomData wird nicht mehr für OnClick benötigt

/// Marker-Komponente für alle von `ButtonBuilder` erstellten Buttons.
/// Dient zur einfachen Filterung in Queries.
#[derive(Component, Default, Debug)]
pub struct ButtonMarker;

/// Speichert den konfigurierten Zustand eines Buttons, wie Variante, Größe und Deaktivierungsstatus.
///
/// Wird von `update_button_visuals` verwendet, um das Aussehen des Buttons anzupassen.
#[derive(Component, Debug, Clone)]
pub struct ButtonState {
    pub variant: ButtonVariant,
    pub size: ButtonSize,
    pub disabled: bool,
}

/// Der Standard-Aktionstyp für Buttons, die keine spezifische,
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
