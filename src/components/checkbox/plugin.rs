use bevy::prelude::*;

use super::{
    events::CheckboxChangedEvent,
    systems::{
        handle_checkbox_clicks, update_checkbox_visuals,
        update_checkmark_visibility_on_state_change,
    },
};
use crate::plugin::UiState;

/// Plugin registering systems and events for [`CheckboxBuilder`].
pub struct CheckboxPlugin;

impl Plugin for CheckboxPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<CheckboxChangedEvent>()
            .add_systems(
                Update,
                (
                    update_checkbox_visuals,
                    handle_checkbox_clicks,
                    update_checkmark_visibility_on_state_change,
                )
                    .run_if(in_state(UiState::Ready)),
            );
    }
}
