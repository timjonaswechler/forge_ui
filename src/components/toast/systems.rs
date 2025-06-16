use bevy::prelude::*;

use super::{ToastMarker, ToastTimer};

/// Despawns toasts after their timer has finished.
pub fn update_toast_timers(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut ToastTimer), With<ToastMarker>>,
) {
    for (entity, mut timer) in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            commands.entity(entity).despawn();
        }
    }
}
