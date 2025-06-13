use bevy::prelude::*;

use super::{
    events::TabsChangedEvent,
    systems::{handle_tab_trigger_clicks, update_tab_content_visibility},
};
use crate::plugin::UiState;

pub struct TabsPlugin;

impl Plugin for TabsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TabsChangedEvent>().add_systems(
            Update,
            (
                handle_tab_trigger_clicks.run_if(in_state(UiState::Ready)),
                update_tab_content_visibility.run_if(in_state(UiState::Ready)),
            ),
        );
    }
}
