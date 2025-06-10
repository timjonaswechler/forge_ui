use bevy::prelude::*;

use super::{
    events::AccordionToggledEvent,
    systems::{handle_accordion_clicks, update_body_visibility},
};
use crate::plugin::UiState;

/// Plugin for accordion functionality.
pub struct AccordionPlugin;

impl Plugin for AccordionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AccordionToggledEvent>().add_systems(
            Update,
            (
                handle_accordion_clicks.run_if(in_state(UiState::Ready)),
                update_body_visibility.run_if(in_state(UiState::Ready)),
            ),
        );
    }
}
