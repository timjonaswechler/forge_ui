use bevy::prelude::*;

use super::{
    MenubarMenuContentMarker, MenubarMenuMarker, MenubarMenuState, MenubarMenuTriggerMarker,
};

pub fn handle_menubar_interaction(
    mut menu_q: Query<(&mut MenubarMenuState, &Children), With<MenubarMenuMarker>>,
    trigger_q: Query<&Interaction, With<MenubarMenuTriggerMarker>>,
    mut content_q: Query<&mut Visibility, With<MenubarMenuContentMarker>>,
) {
    for (mut state, children) in menu_q.iter_mut() {
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
