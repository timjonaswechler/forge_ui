use bevy::prelude::*;
use bevy::prelude::ChildOf;

use super::components::{
    SelectContentMarker, SelectMarker, SelectOptionMarker, SelectState,
    SelectTriggerMarker, SelectTriggerTextMarker,
};

/// Toggle select dropdown visibility.
pub fn handle_select_trigger_interaction(
    mut root_q: Query<(&mut SelectState, &Children), With<SelectMarker>>,
    trigger_q: Query<&Interaction, With<SelectTriggerMarker>>,
    mut content_q: Query<&mut Visibility, With<SelectContentMarker>>,
) {
    for (mut state, children) in &mut root_q {
        let mut toggled = false;
        for child in children.iter() {
            if let Ok(interaction) = trigger_q.get(child) {
                if *interaction == Interaction::Pressed {
                    toggled = true;
                }
            }
            if let Ok(mut vis) = content_q.get_mut(child) {
                *vis = if state.open { Visibility::Inherited } else { Visibility::Hidden };
            }
        }
        if toggled {
            state.open = !state.open;
        }
    }
}

/// Handle option selection and update state and trigger text.
pub fn handle_select_option_interaction(
    mut root_q: Query<(&mut SelectState, &Children), With<SelectMarker>>,
    option_q: Query<(&Interaction, &SelectOptionMarker, &ChildOf)>,
    parents: Query<&ChildOf>,
    mut content_q: Query<&mut Visibility, With<SelectContentMarker>>,
) {
    for (interaction, option, parent) in &option_q {
        if *interaction != Interaction::Pressed {
            continue;
        }

        let mut current = parent.parent();
        loop {
            if let Ok((mut state, children)) = root_q.get_mut(current) {
                state.selected = Some(option.value.clone());
                state.open = false;
                for child in children.iter() {
                    if let Ok(mut vis) = content_q.get_mut(child) {
                        *vis = Visibility::Hidden;
                    }
                }
                break;
            } else if let Ok(p) = parents.get(current) {
                current = p.parent();
            } else {
                break;
            }
        }
    }
}
