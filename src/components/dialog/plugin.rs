// /src/components/dialog/plugin.rs
use bevy::prelude::*; // Wichtig für den Trigger

use super::*;
use crate::components::button::*;
use crate::plugin::UiState;

pub struct DialogPlugin;

impl Plugin for DialogPlugin {
    fn build(&self, app: &mut App) {
        app
            // Ressourcen
            .init_resource::<ActiveDialogs>()
            // Events für Dialoge selbst
            .add_event::<OpenDialogEvent>()
            .add_event::<CloseDialogEvent>()
            // Event für den DialogTrigger-Button (basierend auf ButtonClickedEvent)
            .add_event::<ButtonClickedEvent<DialogAction>>()
            // Dialog-Systeme
            .add_systems(
                Update,
                (
                    open_dialog_system,
                    handle_overlay_click_system,
                    handle_button_release::<DialogAction>,
                    handle_dialog_action_buttons,
                    close_dialog_system
                        .run_if(in_state(UiState::Ready))
                        .run_if(|active: Res<ActiveDialogs>| !active.modals.is_empty()),
                )
                    .run_if(in_state(UiState::Ready)), // Annahme: Systeme sollen nur im UiState::Ready laufen
            );
    }
}
