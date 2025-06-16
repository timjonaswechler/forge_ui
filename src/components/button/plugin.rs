//! Button plugin module
//!
//! This module provides a Bevy plugin for handling button interactions, events,
//! and visual updates. It registers the necessary systems and events for basic
//! button functionality with the default `NoAction` type.
use crate::components::button::{
    handle_button_release, update_button_visuals, ButtonClickedEvent,
};
use crate::components::helper::NoAction;
use crate::plugin::UiState;
use bevy::prelude::*;
use std::marker::PhantomData;

/// Plugin for button functionality
///
/// This plugin registers the necessary events and systems for handling button
/// interactions and visual updates. By default, it registers systems for the
/// `NoAction` button type. For custom action types, users must register
/// additional events and systems themselves.
/// Generisches Plugin zur Registrierung der Button-Systeme für einen Aktionstyp `A`.
pub struct ButtonPlugin<A: Component + Clone + Send + Sync = NoAction>(PhantomData<A>);

impl<A: Component + Clone + Send + Sync> Default for ButtonPlugin<A> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<A: Component + Clone + Send + Sync + std::fmt::Debug> Plugin for ButtonPlugin<A> {
    fn build(&self, app: &mut App) {
        app.add_event::<ButtonClickedEvent<A>>().add_systems(
            Update,
            (
                // handle_button_press::<A>,
                handle_button_release::<A>,
                update_button_visuals,
            )
                .run_if(in_state(UiState::Ready)),
        );
    }
}

/// Bequemer Alias für Buttons ohne Aktionstyp
pub type ButtonNoActionPlugin = ButtonPlugin<NoAction>;
