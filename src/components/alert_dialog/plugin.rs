use bevy::prelude::*;

use super::events::{handle_alert_dialog_actions, AlertDialogAction, AlertDialogResultEvent};
use crate::components::button::{handle_button_release, ButtonClickedEvent};
use crate::plugin::UiState;

/// Plugin registering events and systems for [`AlertDialogBuilder`].
pub struct AlertDialogPlugin;

impl Plugin for AlertDialogPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<AlertDialogResultEvent>()
            .add_event::<ButtonClickedEvent<AlertDialogAction>>()
            .add_systems(
                Update,
                (handle_button_release::<AlertDialogAction>, handle_alert_dialog_actions)
                    .run_if(in_state(UiState::Ready)),
            );
    }
}
