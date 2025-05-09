use crate::theme::UiTheme;
use bevy::{ecs::system::EntityCommands, prelude::*, ui::FocusPolicy};

/// Wird gesendet, wenn sich der Zustand der Checkbox ändert.
#[derive(Event, Debug, Clone)]
pub struct CheckboxChangedEvent {
    pub checkbox_entity: Entity,
    pub is_checked: bool,
}
