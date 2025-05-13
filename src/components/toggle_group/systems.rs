use super::*;
use crate::components::toggle::{ToggleChangedEvent, ToggleMarker, ToggleState};

use bevy::prelude::*;

/// Listens for toggle changes and updates the toggle group state
pub fn handle_toggle_group_item_changes<A: Component + Clone + Send + Sync + 'static>(
    mut toggle_events: EventReader<ToggleChangedEvent<A>>,
    mut toggle_groups: Query<(Entity, &mut ToggleGroupState, Option<&A>), With<ToggleGroupMarker>>,
    toggle_items: Query<(&ChildOf, &ToggleGroupItemState), With<ToggleGroupItemMarker>>,
    mut toggle_states: Query<&mut ToggleState, With<ToggleMarker>>,
    children_query: Query<&Children>,
    mut toggle_group_events: EventWriter<ToggleGroupChangedEvent<A>>,
) {
    for event in toggle_events.read() {
        // Find the parent toggle group for this toggle
        if let Ok((parent, item_state)) = toggle_items.get(event.source_entity) {
            let parent_entity = parent.get();

            // Get the toggle group state
            if let Ok((group_entity, mut group_state, action)) =
                toggle_groups.get_mut(parent_entity)
            {
                if group_state.disabled {
                    continue;
                }

                // Handle based on group type
                match group_state.group_type {
                    ToggleGroupType::Single => {
                        // Clear current selection if the new item is being activated
                        if event.pressed {
                            let old_values = group_state.active_values.clone();
                            group_state.active_values.clear();
                            group_state.active_values.insert(item_state.value.clone());

                            // Update visual state of other toggles
                            if let Ok(children) = children_query.get(parent_entity) {
                                for child in children.iter() {
                                    if child != event.source_entity {
                                        if let Ok(child_children) = children_query.get(child) {
                                            for toggle_entity in child_children.iter() {
                                                if let Ok(mut toggle_state) =
                                                    toggle_states.get_mut(toggle_entity)
                                                {
                                                    toggle_state.pressed = false;
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            // Only send event if values changed
                            if old_values != group_state.active_values {
                                toggle_group_events.write(ToggleGroupChangedEvent {
                                    source_entity: group_entity,
                                    active_values: group_state.active_values.clone(),
                                    action_id: action.cloned(),
                                });
                            }
                        } else if group_state.active_values.contains(&item_state.value) {
                            // Don't allow deselecting the last item in single mode
                            // Revert the toggle state
                            if let Ok(mut toggle_state) = toggle_states.get_mut(event.source_entity)
                            {
                                toggle_state.pressed = true;
                            }
                        }
                    }
                    ToggleGroupType::Multiple => {
                        // Simply update the active values
                        let old_values = group_state.active_values.clone();

                        if event.pressed {
                            group_state.active_values.insert(item_state.value.clone());
                        } else {
                            group_state.active_values.remove(&item_state.value);
                        }

                        // Only send event if values changed
                        if old_values != group_state.active_values {
                            toggle_group_events.write(ToggleGroupChangedEvent {
                                source_entity: group_entity,
                                active_values: group_state.active_values.clone(),
                                action_id: action.cloned(),
                            });
                        }
                    }
                }
            }
        }
    }
}

/// Handles keyboard navigation for toggle groups
pub fn handle_toggle_group_keyboard_navigation(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    toggle_groups: Query<(&ToggleGroupState, &Children), With<ToggleGroupMarker>>,
    toggle_items: Query<&ToggleGroupItemState, With<ToggleGroupItemMarker>>,
    focused: Query<Entity, With<FocusState>>,
    mut focus_events: EventWriter<FocusEvent>,
) {
    // Get currently focused entity
    let focused_entity = if let Some(entity) = focused.iter().next() {
        entity
    } else {
        return;
    };

    for (group_state, children) in toggle_groups.iter() {
        if group_state.disabled || !group_state.roving_focus {
            continue;
        }

        // Find if focused entity is part of this group
        let mut found_index = None;
        let mut active_items = Vec::new();

        for (idx, child) in children.iter().enumerate() {
            if let Ok(item_state) = toggle_items.get(child) {
                if !item_state.disabled {
                    active_items.push((idx, child));
                    if child == focused_entity {
                        found_index = Some(active_items.len() - 1);
                    }
                }
            }
        }

        // If focused item is in this group, handle navigation
        if let Some(current_idx) = found_index {
            let nav_keys = match group_state.orientation {
                ToggleGroupOrientation::Horizontal => [
                    KeyCode::ArrowLeft,
                    KeyCode::ArrowRight,
                    KeyCode::Home,
                    KeyCode::End,
                ],
                ToggleGroupOrientation::Vertical => [
                    KeyCode::ArrowUp,
                    KeyCode::ArrowDown,
                    KeyCode::Home,
                    KeyCode::End,
                ],
            };

            let mut new_idx = current_idx;

            if keyboard_input.just_pressed(nav_keys[0]) {
                // Previous
                if current_idx > 0 {
                    new_idx = current_idx - 1;
                } else if group_state.loop_navigation {
                    new_idx = active_items.len() - 1;
                }
            } else if keyboard_input.just_pressed(nav_keys[1]) {
                // Next
                if current_idx < active_items.len() - 1 {
                    new_idx = current_idx + 1;
                } else if group_state.loop_navigation {
                    new_idx = 0;
                }
            } else if keyboard_input.just_pressed(nav_keys[2]) {
                // Home
                new_idx = 0;
            } else if keyboard_input.just_pressed(nav_keys[3]) {
                // End
                new_idx = active_items.len() - 1;
            }

            // If index changed, focus the new item
            if new_idx != current_idx {
                let (_, entity) = active_items[new_idx];
                focus_events.write(FocusEvent::Focus { entity });
            }
        }
    }
}

/// Updates the visual representation of toggle groups
pub fn update_toggle_group_visuals(
    mut toggle_groups: Query<
        (
            &ToggleGroupState,
            &mut Node,
            &mut BorderColor,
            &mut BackgroundColor,
        ),
        With<ToggleGroupMarker>,
    >,
    theme: Res<crate::theme::UiTheme>,
) {
    for (state, mut style, mut border_color, mut bg_color) in toggle_groups.iter_mut() {
        let style_def = get_toggle_group_style_def(&theme, state.variant, state.size);

        // Set direction based on orientation
        style.flex_direction = match state.orientation {
            ToggleGroupOrientation::Horizontal => FlexDirection::Row,
            ToggleGroupOrientation::Vertical => FlexDirection::Column,
        };

        // Set spacing based on orientation
        style.column_gap = match state.orientation {
            ToggleGroupOrientation::Horizontal => Val::Px(style_def.spacing),
            ToggleGroupOrientation::Vertical => Val::Px(0.0),
        };

        style.row_gap = match state.orientation {
            ToggleGroupOrientation::Horizontal => Val::Px(0.0),
            ToggleGroupOrientation::Vertical => Val::Px(style_def.spacing),
        };

        // Update colors
        *border_color = BorderColor(style_def.border_color);
        *bg_color = BackgroundColor(style_def.background_color);
    }
}

/// Synchronizes toggle states with toggle group on startup
pub fn sync_toggle_group_on_startup(
    toggle_groups: Query<
        (&ToggleGroupState, &Children),
        (With<ToggleGroupMarker>, Changed<ToggleGroupState>),
    >,
    toggle_items: Query<(&ToggleGroupItemState, &Children), With<ToggleGroupItemMarker>>,
    mut toggle_states: Query<&mut ToggleState, With<ToggleMarker>>,
) {
    for (group_state, children) in toggle_groups.iter() {
        for child in children.iter() {
            if let Ok((item_state, item_children)) = toggle_items.get(child) {
                let is_active = group_state.active_values.contains(&item_state.value);

                // Find the toggle component and update its state
                for toggle_entity in item_children.iter() {
                    if let Ok(mut toggle_state) = toggle_states.get_mut(toggle_entity) {
                        toggle_state.pressed = is_active;
                        // Also respect item's disabled status
                        toggle_state.disabled = group_state.disabled || item_state.disabled;
                    }
                }
            }
        }
    }
}

/// System to apply focus events
pub fn apply_focus_events(
    mut focus_events: EventReader<FocusEvent>,
    mut commands: Commands,
    focused_entities: Query<Entity, With<FocusState>>,
) {
    for event in focus_events.read() {
        match event {
            FocusEvent::Focus { entity } => {
                // Remove focus from currently focused entities
                for focused in focused_entities.iter() {
                    commands.entity(focused).remove::<FocusState>();
                }

                // Apply focus to new entity
                commands.entity(*entity).insert(FocusState);
            }
            FocusEvent::Blur { entity } => {
                commands.entity(*entity).remove::<FocusState>();
            }
        }
    }
}
