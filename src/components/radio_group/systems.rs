use bevy::prelude::*;

use crate::components::radio::{RadioGroup as RadioChildGroup, RadioMarker, RadioState};

use super::{
    components::{RadioGroupMarker, RadioGroupState},
    events::RadioGroupChangedEvent,
};

/// Keeps the `RadioGroupState` on the root entity in sync with the currently
/// checked radio button **and** emits a `RadioGroupChangedEvent` whenever the
/// selection changes.
///
/// **Must run _after_ `handle_radio_click` from the `radio` module.**
pub fn propagate_radio_selection(
    mut writer: EventWriter<RadioGroupChangedEvent>,
    ancestors: Query<&ChildOf>,
    mut groups: Query<(Entity, &mut RadioGroupState), With<RadioGroupMarker>>,
    radios: Query<
        (Entity, &RadioState, &RadioChildGroup),
        (With<RadioMarker>, Changed<RadioState>),
    >,
) {
    for (radio_entity, radio_state, _group_tag) in &radios {
        if !radio_state.checked {
            continue; // only care for newly *checked* radios
        }

        // Walk up the hierarchy until we find a parent with `RadioGroupMarker`.
        let mut current = radio_entity;
        while let Ok(parent) = ancestors.get(current) {
            current = parent.parent();

            if let Ok((group_entity, mut group_state)) = groups.get_mut(current) {
                // Ignore if the selection did not actually change.
                if group_state.selected_value.as_deref() == Some(&radio_state.value) {
                    break;
                }

                group_state.selected_value = Some(radio_state.value.clone());

                writer.write(RadioGroupChangedEvent {
                    source_entity: group_entity,
                    group_name: group_state.name.clone(),
                    selected_value: radio_state.value.clone(),
                });

                break; // done for this radio
            }
        }
    }
}
