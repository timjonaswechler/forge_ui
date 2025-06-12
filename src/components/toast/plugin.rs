use bevy::prelude::*;

use super::systems::{handle_toast_close, handle_toast_lifetime};
use crate::plugin::UiState;

pub struct ToastPlugin;

impl Plugin for ToastPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (handle_toast_lifetime, handle_toast_close).run_if(in_state(UiState::Ready)),
        );
    }
}
