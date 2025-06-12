use bevy::prelude::*;

use super::systems::handle_hover_card_interaction;
use crate::plugin::UiState;

/// Plugin for hover card interactions.
pub struct HoverCardPlugin;

impl Plugin for HoverCardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_hover_card_interaction.run_if(in_state(UiState::Ready)));
    }
}
