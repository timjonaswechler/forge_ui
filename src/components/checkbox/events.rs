use bevy::prelude::*;

/// Event, das gesendet wird, wenn sich der Zustand einer Checkbox ändert.
#[derive(Event, Debug, Clone)]
pub struct CheckboxChangedEvent {
    /// Entity der Checkbox, deren Zustand sich geändert hat
    pub checkbox_entity: Entity,
    /// Neuer Zustand: `true` = aktiviert, `false` = deaktiviert
    pub is_checked: bool,
}
