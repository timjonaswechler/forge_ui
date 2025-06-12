use bevy::prelude::*;

use crate::plugin::UiState;

use super::systems::handle_menubar_interaction;

pub struct MenubarPlugin;

impl Default for MenubarPlugin {
    fn default() -> Self {
        Self
    }
}

impl Plugin for MenubarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            handle_menubar_interaction.run_if(in_state(UiState::Ready)),
        );
    }
}
