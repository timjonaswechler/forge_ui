use bevy::prelude::*;

use crate::components::helper::UiBuilder;
use crate::theme::UiTheme;

use super::{VisuallyHiddenMarker, VisuallyHiddenStyle};

/// Builder for visually hidden content that remains accessible to screen readers.
pub struct VisuallyHiddenBuilder {
    text: String,
}

impl VisuallyHiddenBuilder {
    /// Creates a new builder with the given hidden label text.
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}

impl<'w, 's> UiBuilder<'w, 's> for VisuallyHiddenBuilder {
    type Output = Entity;

    fn spawn(self, parent: &'s mut ChildSpawnerCommands<'w>, _theme: &UiTheme, font: &Handle<Font>) -> Self::Output {
        let mut cmd = parent.spawn((VisuallyHiddenMarker, VisuallyHiddenStyle::default()));
        cmd.with_children(|cb| {
            cb.spawn((
                Text::new(self.text.clone()),
                TextFont { font: font.clone(), font_size: 1.0, ..default() },
                TextLayout::default(),
                TextColor(Color::BLACK),
            ));
        });
        cmd.id()
    }
}
