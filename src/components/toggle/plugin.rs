use super::{handle_toggle_interaction, update_toggle_visuals, ToggleChangedEvent};
use crate::prelude::*;
use bevy::prelude::*;
use std::marker::PhantomData;

/// Registriert Events & Systeme für einen konkreten Aktionstyp `A`.
pub struct TogglePlugin<A: Component + Clone + Send + Sync + 'static>(PhantomData<A>);

impl<A: Component + Clone + Send + Sync + 'static> Default for TogglePlugin<A> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<A: Component + Clone + Send + Sync + 'static> Plugin for TogglePlugin<A> {
    fn build(&self, app: &mut App) {
        app.add_event::<ToggleChangedEvent<A>>().add_systems(
            Update,
            (handle_toggle_interaction::<A>, update_toggle_visuals)
                .run_if(in_state(UiState::Ready)),
        );
    }
}
