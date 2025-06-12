use bevy::prelude::*;

use crate::components::helper::UiBuilder;
use crate::theme::UiTheme;

use super::{ToastCloseMarker, ToastLifetime, ToastMarker, ToastStyle};

/// Builder for a basic toast notification.
pub struct ToastBuilder {
    message: String,
    duration: f32,
}

impl ToastBuilder {
    /// Creates a new toast builder with the given message.
    pub fn new(message: impl Into<String>) -> Self {
        Self { message: message.into(), duration: 5.0 }
    }

    /// Sets how long the toast should stay visible in seconds.
    pub fn duration(mut self, secs: f32) -> Self {
        self.duration = secs;
        self
    }
}

impl<'w, 's> UiBuilder<'w, 's> for ToastBuilder {
    type Output = Entity;

    fn spawn(self, parent: &'s mut ChildSpawnerCommands<'w>, theme: &UiTheme, font: &Handle<Font>) -> Self::Output {
        let mut root = parent.spawn((
            ToastMarker,
            ToastStyle::new(theme),
            ToastLifetime { timer: Timer::from_seconds(self.duration, TimerMode::Once) },
            Name::new("Toast"),
        ));
        root.with_children(|rc| {
            rc.spawn((
                Text::new(self.message.clone()),
                TextFont { font: font.clone(), font_size: theme.font.size.base, ..default() },
                TextColor(theme.color.slate.step12),
            ));
            rc.spawn((
                ToastCloseMarker,
                Button,
                Interaction::default(),
                Text::new("×"),
                TextFont { font: font.clone(), font_size: theme.font.size.base, ..default() },
                TextColor(theme.color.slate.step12),
            ));
        });
        root.id()
    }
}
