use bevy::prelude::*;

use super::{events::CheckboxGroupChangedEvent, systems::propagate_checkbox_selection};
use crate::plugin::UiState;

/// Plugin for the [`CheckboxGroup`] component.
pub struct CheckboxGroupPlugin;

impl Plugin for CheckboxGroupPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<CheckboxGroupChangedEvent>()
            .add_systems(
                Update,
                propagate_checkbox_selection.run_if(in_state(UiState::Ready)),
            );
    }
}
