use bevy::prelude::*;

use super::systems::handle_tooltip_hover;
use crate::plugin::UiState;

pub struct TooltipPlugin;

impl Plugin for TooltipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            handle_tooltip_hover.run_if(in_state(UiState::Ready)),
        );
    }
}
