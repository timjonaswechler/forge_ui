use bevy::prelude::*;

use crate::components::helper::UiBuilder;
use crate::theme::UiTheme;

use super::{CodeMarker, CodeStyle};

/// Builder for an inline code element.
pub struct CodeBuilder {
    text: String,
}

impl CodeBuilder {
    /// Creates a new code snippet with the given text.
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}

impl<'w, 's> UiBuilder<'w, 's> for CodeBuilder {
    type Output = Entity;

    fn spawn(self, parent: &'s mut ChildSpawnerCommands<'w>, theme: &UiTheme, _font: &Handle<Font>) -> Self::Output {
        let style = CodeStyle::new(theme);
        let mut cmd = parent.spawn((CodeMarker, style));
        cmd.with_children(|cb| {
            cb.spawn((
                Text::new(self.text),
                TextFont {
                    font: theme.font.family.mono.regular.clone(),
                    font_size: theme.font.size.base,
                    ..default()
                },
                TextLayout::default(),
                TextColor(theme.color.slate.step12),
            ));
        });
        cmd.id()
    }
}
