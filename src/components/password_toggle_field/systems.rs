use bevy::prelude::*;

use super::{
    PasswordHiddenTextMarker, PasswordToggleFieldMarker, PasswordToggleFieldState,
    PasswordToggleMarker, PasswordVisibleTextMarker,
};

/// Toggles password visibility when the toggle button is pressed.
pub fn handle_password_toggle(
    mut toggle_q: Query<(&Interaction, &bevy::prelude::ChildOf), (Changed<Interaction>, With<PasswordToggleMarker>)>,
    mut field_q: Query<(&mut PasswordToggleFieldState, &Children), With<PasswordToggleFieldMarker>>,
    mut visible_q: Query<
        &mut Visibility,
        (
            With<PasswordVisibleTextMarker>,
            Without<PasswordHiddenTextMarker>,
        ),
    >,
    mut hidden_q: Query<
        &mut Visibility,
        (
            With<PasswordHiddenTextMarker>,
            Without<PasswordVisibleTextMarker>,
        ),
    >,
) {
    for (interaction, parent) in toggle_q.iter_mut() {
        if *interaction == Interaction::Pressed {
            if let Ok((mut state, children)) = field_q.get_mut(parent.parent()) {
                state.visible = !state.visible;
                for &child in children.into_iter() {
                    if let Ok(mut vis) = visible_q.get_mut(child) {
                        *vis = if state.visible { Visibility::Inherited } else { Visibility::Hidden };
                    }
                    if let Ok(mut vis) = hidden_q.get_mut(child) {
                        *vis = if state.visible { Visibility::Hidden } else { Visibility::Inherited };
                    }
                }
            }
        }
    }
}

