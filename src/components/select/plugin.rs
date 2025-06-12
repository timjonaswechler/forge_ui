use bevy::prelude::*;

use super::systems::{handle_select_option_interaction, handle_select_trigger_interaction};
use crate::plugin::UiState;

pub struct SelectPlugin;

impl Plugin for SelectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                handle_select_trigger_interaction,
                handle_select_option_interaction,
            )
                .run_if(in_state(UiState::Ready)),
        );
    }
}
