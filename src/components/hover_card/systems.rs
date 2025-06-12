use bevy::prelude::*;

use super::{HoverCardContentMarker, HoverCardMarker, HoverCardState, HoverCardTriggerMarker};

/// Shows or hides hover card content when the trigger changes hover state.
pub fn handle_hover_card_interaction(
    mut root_q: Query<(&mut HoverCardState, &Children), With<HoverCardMarker>>,
    trigger_q: Query<&Interaction, With<HoverCardTriggerMarker>>,
    mut content_q: Query<&mut Visibility, With<HoverCardContentMarker>>,
) {
    for (mut state, children) in root_q.iter_mut() {
        let mut open = state.open;
        for child in children.iter() {
            if let Ok(interaction) = trigger_q.get(child) {
                open = *interaction == Interaction::Hovered;
            }
            if let Ok(mut vis) = content_q.get_mut(child) {
                *vis = if open { Visibility::Inherited } else { Visibility::Hidden };
            }
        }
        state.open = open;
    }
}
