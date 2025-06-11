use bevy::prelude::*;
use crate::components::checkbox::CheckboxChangedEvent;

use super::{components::{CheckboxGroupItem, CheckboxGroupMarker, CheckboxGroupState}, events::CheckboxGroupChangedEvent};

/// Updates `CheckboxGroupState` when one of its checkboxes toggles and emits a
/// [`CheckboxGroupChangedEvent`]. This must run after [`handle_checkbox_clicks`]
/// from the [`checkbox`] module.
pub fn propagate_checkbox_selection(
    mut changed: EventReader<CheckboxChangedEvent>,
    ancestors: Query<&ChildOf>,
    mut groups: Query<(Entity, &mut CheckboxGroupState), With<CheckboxGroupMarker>>,
    items: Query<&CheckboxGroupItem>,
    mut writer: EventWriter<CheckboxGroupChangedEvent>,
) {
    for ev in changed.read() {
        let mut current = ev.checkbox_entity;
        while let Ok(parent) = ancestors.get(current) {
            current = parent.parent();
            if let Ok((group_entity, mut group_state)) = groups.get_mut(current) {
                if let Ok(item) = items.get(ev.checkbox_entity) {
                    if ev.is_checked {
                        group_state.checked_values.insert(item.value.clone());
                    } else {
                        group_state.checked_values.remove(&item.value);
                    }
                    writer.write(CheckboxGroupChangedEvent {
                        source_entity: group_entity,
                        checked_values: group_state.checked_values.clone(),
                    });
                }
                break;
            }
        }
    }
}
