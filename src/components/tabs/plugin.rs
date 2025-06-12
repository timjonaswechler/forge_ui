use bevy::prelude::*;

use super::systems::handle_tab_interaction;
use crate::plugin::UiState;

pub struct TabsPlugin;

impl Plugin for TabsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_tab_interaction.run_if(in_state(UiState::Ready)));
    }
}
