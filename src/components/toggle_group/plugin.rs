use super::events::*;
use super::systems::*;
use crate::plugin::UiState;
use bevy::prelude::*;
use std::marker::PhantomData;

/// Registers events & systems for ToggleGroup with a specific action type
pub struct ToggleGroupPlugin<A: Component + Clone + Send + Sync + 'static>(PhantomData<A>);

impl<A: Component + Clone + Send + Sync + 'static> Default for ToggleGroupPlugin<A> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<A: Component + Clone + Send + Sync + 'static> Plugin for ToggleGroupPlugin<A> {
    fn build(&self, app: &mut App) {
        app.add_event::<ToggleGroupChangedEvent<A>>();
        app.add_event::<FocusEvent>();

        // Register systems individually rather than as a tuple
        app.add_systems(
            Update,
            handle_toggle_group_item_changes::<A>.run_if(in_state(UiState::Ready)),
        )
        .add_systems(
            Update,
            update_toggle_group_visuals.run_if(in_state(UiState::Ready)),
        )
        .add_systems(
            Update,
            handle_toggle_group_keyboard_navigation.run_if(in_state(UiState::Ready)),
        )
        .add_systems(
            Startup,
            sync_toggle_group_on_startup.run_if(in_state(UiState::Ready)),
        );
    }
}
