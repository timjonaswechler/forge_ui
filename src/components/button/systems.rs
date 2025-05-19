//! Button systems module
//!
//! This module contains systems that handle button interactions, visual updates,
//! and event dispatching. It provides functionality for detecting clicks and
//! updating button visuals based on interaction state.

use super::components::{ButtonMarker, ButtonState}; // NoAction nicht unbedingt hier nötig
use super::events::ButtonClickedEvent; // Generisches Event importieren
use super::style::ButtonStyle;
use bevy::prelude::*;
use std::collections::HashMap;

/// System to update the button's background, border, and children's colors
///
/// Updates the visual appearance of buttons based on their current state and interaction.
/// This includes changing background and border colors to provide visual feedback.
///
/// # Parameters
/// * `query` - Query that selects button entities with their state, interaction, and visual components
pub fn update_button_visuals(
    mut query: Query<
        (
            &ButtonState,
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
        ),
        With<ButtonMarker>,
    >,
) {
    for (state, interaction, mut bg, mut border) in query.iter_mut() {
        *bg = ButtonStyle::background(&state.color_palette, state.variant, *interaction);

        *border = ButtonStyle::border(&state.color_palette, state.variant, *interaction);
    }
}

/// System that detects button clicks and sends a generic `ButtonClickedEvent<A>`.
///
/// Triggers when a button is pressed and emits an event with the associated action component,
/// **only if the button is not disabled.**
///
/// # Type Parameters
/// * `A` - Component type that represents the action associated with the button
///
/// # Parameters
/// * `writer` - Event writer for dispatching button click events
/// * `query` - Query that selects buttons with changed interaction state, their action components, and their `ButtonState`.
pub fn handle_button_press<A: Component + Clone>(
    mut writer: EventWriter<ButtonClickedEvent<A>>,
    query: Query<
        (Entity, &Interaction, &A, &ButtonState),
        (With<ButtonMarker>, Changed<Interaction>),
    >,
) {
    for (entity, interaction, action, state) in query.iter() {
        if state.disabled {
            continue;
        }

        if *interaction == Interaction::Pressed {
            writer.write(ButtonClickedEvent {
                source_entity: entity,
                action_id: Some(action.clone()),
            });
        }
    }
}

/// System that handles button release events and dispatches appropriate actions.
///
/// Detects when a button transitions from pressed to released state and triggers
/// a button click event. Uses a local HashMap to track previous interaction states.
/// **Only if the button is not disabled.**
///
/// # Type Parameters
/// * `A` - Component type that represents the action associated with the button
///
/// # Parameters
/// * `writer` - Event writer for dispatching button click events
/// * `prev` - Local storage for tracking previous interaction states per entity
/// * `query` - Query that selects buttons with their current interaction state, action components, and their `ButtonState`.
pub fn handle_button_release<A: Component + Clone>(
    mut writer: EventWriter<ButtonClickedEvent<A>>,
    mut prev: Local<HashMap<Entity, Interaction>>,
    query: Query<(Entity, &Interaction, &A, &ButtonState), With<ButtonMarker>>,
) {
    for (entity, interaction, action, state) in query.iter() {
        if state.disabled {
            prev.remove(&entity);
            continue;
        }

        let last = *prev.get(&entity).unwrap_or(&Interaction::None);
        if last == Interaction::Pressed
            && (*interaction == Interaction::Hovered || *interaction == Interaction::None)
        {
            writer.write(ButtonClickedEvent {
                source_entity: entity,
                action_id: Some(action.clone()),
            });
        }
        prev.insert(entity, *interaction);
    }
}
