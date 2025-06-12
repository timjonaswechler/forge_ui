use bevy::prelude::*;

use super::{events::RadioGroupChangedEvent, systems::propagate_radio_selection};
use crate::plugin::UiState;

/// Plugin for the [`RadioGroupBuilder`].
pub struct RadioGroupPlugin;

impl Plugin for RadioGroupPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<RadioGroupChangedEvent>()
            .add_systems(
                Update,
                propagate_radio_selection.run_if(in_state(UiState::Ready)),
            );
    }
}
