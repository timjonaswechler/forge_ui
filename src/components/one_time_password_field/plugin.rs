use bevy::prelude::*;

use crate::plugin::UiState;

use super::systems::handle_otp_input_hover;

pub struct OneTimePasswordFieldPlugin;

impl Plugin for OneTimePasswordFieldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_otp_input_hover.run_if(in_state(UiState::Ready)));
    }
}
