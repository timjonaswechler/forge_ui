use bevy::prelude::*;

use crate::components::helper::UiBuilder;
use crate::theme::UiTheme;

use super::{ToastMarker, ToastStyle, ToastTimer, ToastVariant};

/// Builder for a simple toast notification.
pub struct ToastBuilder {
    text: String,
    variant: ToastVariant,
    duration: f32,
}

impl ToastBuilder {
    /// Create a new toast with default variant and duration.
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into(), variant: ToastVariant::Info, duration: 5.0 }
    }

    /// Set the visual variant used for styling.
    pub fn variant(mut self, variant: ToastVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set how many seconds the toast should stay visible.
    pub fn duration(mut self, secs: f32) -> Self {
        self.duration = secs;
        self
    }
}

impl<'w, 's> UiBuilder<'w, 's> for ToastBuilder {
    type Output = Entity;

    fn spawn(self, parent: &'s mut ChildSpawnerCommands<'w>, theme: &UiTheme, font: &Handle<Font>) -> Self::Output {
        let style = ToastStyle::new(self.variant, theme, font);
        let timer = ToastTimer(Timer::from_seconds(self.duration, TimerMode::Once));
        let mut cmd = parent.spawn((ToastMarker, style, timer, Name::new("Toast")));
        cmd.with_children(|cb| {
            cb.spawn((
                Text::new(self.text),
                TextFont { font: font.clone(), font_size: theme.font.size.base, ..default() },
                TextColor(theme.color.slate.step12),
            ));
        });
        cmd.id()
    }
}
