// src/components/radio/systems.rs
use bevy::prelude::*;

use super::*;
use crate::theme::UiTheme;

/// System to update visuals based on state (variant, size, disabled, checked)
pub fn update_radio_visuals(
    theme: Res<UiTheme>,
    mut query: Query<
        (&RadioState, &mut BackgroundColor, &mut BorderColor),
        (Changed<RadioState>, With<RadioMarker>),
    >,
) {
    for (state, mut bg_color, mut border_color) in query.iter_mut() {
        let style = get_radio_style_def(state.variant, state.size, &theme);

        if state.disabled {
            *bg_color = BackgroundColor(style.disabled);
            *border_color = BorderColor(style.border.with_alpha(0.5));
        } else {
            *bg_color = BackgroundColor(style.background);
            *border_color = BorderColor(style.border);
        }
    }
}

/// System to update the visibility of the inner indicator dot based on RadioState.checked
pub fn update_radio_indicator(
    parent_query: Query<(&RadioState, &Children), (Changed<RadioState>, With<RadioMarker>)>,
    mut indicator_query: Query<&mut Visibility, With<RadioIndicator>>,
) {
    for (state, children) in parent_query.iter() {
        for child in children.iter() {
            if let Ok(mut visibility) = indicator_query.get_mut(child) {
                *visibility = if state.checked {
                    Visibility::Visible
                } else {
                    Visibility::Hidden
                };
            }
        }
    }
}

/// System to handle radio selection via OnSelectId component and registry
pub fn handle_radio_click(
    // Use ParamSet to manage multiple queries with potentially conflicting mutable access
    mut radio_buttons: ParamSet<(
        // Query 0: For the radio button that was interacted with.
        // Gets its Entity, RadioState, RadioGroup (optional), OnSelectId (optional).
        Query<
            (
                Entity,
                &Interaction,
                &mut RadioState,
                Option<&RadioGroup>,
                Option<&OnSelectId>, // OnSelectId is optional on the entity
            ),
            (Changed<Interaction>, With<RadioMarker>),
        >,
        // Query 1: For all radio buttons, to find others in the same group to uncheck them.
        // Needs Entity to ensure we don't uncheck the one that was just clicked.
        Query<
            (Entity, &RadioGroup, &mut RadioState), // Entity, Group, and State needed
            With<RadioMarker>,                      // Ensure these are radio buttons
        >,
    )>,
    // Assuming OnSelectRegistry is always available. If not, Option<ResMut<...>> can be used.
    registry: ResMut<OnSelectRegistry>,
) {
    // Store details of the radio button that was clicked and needs to become checked.
    // (clicked_entity, value_of_selected_radio, callback_id_option, group_name_option)
    let mut clicked_radio_info: Option<(Entity, String, Option<u32>, Option<String>)> = None;

    // Phase 1: Identify the clicked radio and update its state to checked.
    // This block borrows radio_buttons.p0() mutably.
    {
        for (entity, interaction, mut state, group_opt, on_select_id_opt) in
            radio_buttons.p0().iter_mut()
        {
            if *interaction == Interaction::Pressed && !state.disabled {
                if !state.checked {
                    // Only process if it's not already checked
                    state.checked = true; // Check this radio button
                                          // info!("Radio {:?} ({}) became checked.", entity, state.value); // Optional: logging

                    // Store its info for phase 2 (callback and unchecking others)
                    clicked_radio_info = Some((
                        entity,
                        state.value.clone(),
                        on_select_id_opt.map(|id| id.0),
                        group_opt.map(|g| g.0.clone()),
                    ));
                    // Typically, only one radio button is "Pressed" in a single frame.
                    // Break ensures we only process one such event if multiple were to occur.
                    break;
                } else {
                    state.checked = false; // Uncheck this radio button if it was already checked

                    break;
                }
            }
        }
    } // radio_buttons.p0() borrow ends here.

    // Phase 2: If a radio button was clicked and checked, handle its callback and uncheck others in its group.
    if let Some((clicked_entity, value_to_select, cb_id_opt, group_name_opt)) = clicked_radio_info {
        // Call callback if an OnSelectId was present and a callback ID was found
        if let Some(cb_id) = cb_id_opt {
            registry.call(cb_id, value_to_select);
            // info!("Called callback for radio {:?} with ID {}.", clicked_entity, cb_id); // Optional: logging
        }

        // Uncheck other radio buttons in the same group
        if let Some(group_name) = group_name_opt {
            // This block borrows radio_buttons.p1() mutably.
            for (other_entity, other_group, mut other_state) in radio_buttons.p1().iter_mut() {
                // Check if this 'other' radio is in the same group AND is not the one we just clicked
                if other_entity != clicked_entity && other_group.0 == group_name {
                    // If it's currently checked, uncheck it.
                    if other_state.checked {
                        other_state.checked = false;
                        // info!("Radio {:?} ({}) in group '{}' was unchecked.", other_entity, other_state.value, group_name); // Optional: logging
                    }
                }
            }
        }
    }
}
