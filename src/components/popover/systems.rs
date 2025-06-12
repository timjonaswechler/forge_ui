use bevy::prelude::*;

use super::{PopoverContentMarker, PopoverMarker, PopoverState, PopoverTriggerMarker};

/// Toggles popover visibility when the trigger is pressed.
pub fn handle_popover_toggle(
    mut trigger_q: Query<(&Interaction, &bevy::prelude::ChildOf), (Changed<Interaction>, With<PopoverTriggerMarker>)>,
    mut root_q: Query<(&mut PopoverState, &Children), With<PopoverMarker>>,
    mut content_q: Query<&mut Visibility, With<PopoverContentMarker>>,
) {
    for (interaction, parent) in trigger_q.iter_mut() {
        if *interaction == Interaction::Pressed {
            if let Ok((mut state, children)) = root_q.get_mut(parent.parent()) {
                state.open = !state.open;
                for child in children.iter() {
                    if let Ok(mut vis) = content_q.get_mut(child) {
                        *vis = if state.open { Visibility::Inherited } else { Visibility::Hidden };
                    }
                }
            }
        }
    }
}

