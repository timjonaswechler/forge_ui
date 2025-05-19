//! Button plugin module
//!
//! This module provides a Bevy plugin for handling button interactions, events,
//! and visual updates. It registers the necessary systems and events for basic
//! button functionality with the default `NoAction` type.
use crate::components::button::{
    handle_button_press, handle_button_release, update_button_visuals, ButtonClickedEvent,
};
use crate::components::helper::NoAction;
use crate::plugin::UiState;
use bevy::prelude::*;

/// Plugin for button functionality
///
/// This plugin registers the necessary events and systems for handling button
/// interactions and visual updates. By default, it registers systems for the
/// `NoAction` button type. For custom action types, users must register
/// additional events and systems themselves.
pub struct ButtonPlugin;

impl Plugin for ButtonPlugin {
    /// Builds the button plugin by registering events and systems
    ///
    /// Registers:
    /// - `ButtonClickedEvent<NoAction>` event
    /// - Button press and release handling systems for NoAction buttons
    /// - Visual update system for all button types
    ///
    /// All button systems run conditionally when the application is in the
    /// `UiState::Ready` state.
    ///
    /// # Parameters
    /// * `app` - The Bevy App to configure
    fn build(&self, app: &mut App) {
        app.add_event::<ButtonClickedEvent<NoAction>>().add_systems(
            Update,
            (
                // These systems are only registered for NoAction buttons.
                // For custom action types, users must manually add:
                // handle_button_press::<MyAction>, handle_button_release::<MyAction>,
                // and add_event::<ButtonClickedEvent<MyAction>>().
                handle_button_press::<NoAction>,
                handle_button_release::<NoAction>,
                // update_button_visuals is generic and handles all buttons.
                update_button_visuals,
            )
                .run_if(in_state(UiState::Ready)), // Assumption: Button systems should only run in the Ready state
        );
    }
}
