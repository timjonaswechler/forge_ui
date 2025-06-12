use bevy::prelude::*;

use super::{TooltipContentMarker, TooltipMarker, TooltipState, TooltipTriggerMarker};

/// Shows or hides tooltip content when the trigger is hovered.
pub fn handle_tooltip_hover(
    mut root_q: Query<(&mut TooltipState, &Children), With<TooltipMarker>>,
    trigger_q: Query<&Interaction, With<TooltipTriggerMarker>>,
    mut content_q: Query<&mut Visibility, With<TooltipContentMarker>>,
) {
    for (mut state, children) in root_q.iter_mut() {
        let mut open = state.open;
        for child in children.iter() {
            if let Ok(interaction) = trigger_q.get(child) {
                open = *interaction == Interaction::Hovered;
            }
            if let Ok(mut vis) = content_q.get_mut(child) {
                *vis = if open {
                    Visibility::Inherited
                } else {
                    Visibility::Hidden
                };
            }
        }
        state.open = open;
    }
}
