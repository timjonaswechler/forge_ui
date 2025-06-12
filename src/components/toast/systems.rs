use bevy::prelude::*;

use super::{ToastCloseMarker, ToastLifetime, ToastMarker};

/// Despawn toasts when their timer finishes.
pub fn handle_toast_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut q: Query<(Entity, &mut ToastLifetime)>,
) {
    for (entity, mut lifetime) in q.iter_mut() {
        lifetime.timer.tick(time.delta());
        if lifetime.timer.finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

/// Close a toast when its close button is pressed.
pub fn handle_toast_close(
    mut commands: Commands,
    mut q: Query<(&Interaction, &bevy::prelude::ChildOf), (Changed<Interaction>, With<ToastCloseMarker>)>,
) {
    for (interaction, parent) in q.iter_mut() {
        if *interaction == Interaction::Pressed {
            commands.entity(parent.parent()).despawn_recursive();
        }
    }
}
