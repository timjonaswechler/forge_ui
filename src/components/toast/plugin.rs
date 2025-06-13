use bevy::prelude::*;

use crate::plugin::UiState;
use super::systems::update_toast_timers;

/// Plugin registering toast systems.
pub struct ToastPlugin;

impl Plugin for ToastPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_toast_timers.run_if(in_state(UiState::Ready)));
    }
}
