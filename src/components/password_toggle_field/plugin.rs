use bevy::prelude::*;

use crate::plugin::UiState;

use super::systems::handle_password_toggle;

pub struct PasswordToggleFieldPlugin;

impl Plugin for PasswordToggleFieldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_password_toggle.run_if(in_state(UiState::Ready)));
    }
}

