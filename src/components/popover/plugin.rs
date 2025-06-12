use bevy::prelude::*;

use super::systems::handle_popover_toggle;
use crate::plugin::UiState;

pub struct PopoverPlugin;

impl Plugin for PopoverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_popover_toggle.run_if(in_state(UiState::Ready)));
    }
}

