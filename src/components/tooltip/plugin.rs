use bevy::prelude::*;

use crate::plugin::UiState;
use super::systems::handle_tooltip_interaction;

/// Plugin for tooltip interactions.
pub struct TooltipPlugin;

impl Plugin for TooltipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_tooltip_interaction.run_if(in_state(UiState::Ready)));
    }
}
