use bevy::prelude::*;

use super::{
    NavigationMenuContentMarker, NavigationMenuItemMarker, NavigationMenuItemState,
    NavigationMenuTriggerMarker,
};

pub fn handle_navigation_menu_interaction(
    mut item_q: Query<(&mut NavigationMenuItemState, &Children), With<NavigationMenuItemMarker>>,
    trigger_q: Query<&Interaction, With<NavigationMenuTriggerMarker>>,
    mut content_q: Query<&mut Visibility, With<NavigationMenuContentMarker>>,
) {
    for (mut state, children) in item_q.iter_mut() {
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
