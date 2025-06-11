use bevy::prelude::*;

use crate::components::helper::UiBuilder;
use crate::theme::UiTheme;

use super::{CalloutMarker, CalloutStyle, CalloutVariant};

/// Builder for an informational callout box.
pub struct CalloutBuilder {
    text: String,
    variant: CalloutVariant,
}

impl CalloutBuilder {
    /// Creates a new callout with default variant.
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into(), variant: CalloutVariant::Info }
    }

    /// Sets the visual variant used for styling.
    pub fn variant(mut self, variant: CalloutVariant) -> Self {
        self.variant = variant;
        self
    }
}

impl<'w, 's> UiBuilder<'w, 's> for CalloutBuilder {
    type Output = Entity;

    fn spawn(self, parent: &'s mut ChildSpawnerCommands<'w>, theme: &UiTheme, font: &Handle<Font>) -> Self::Output {
        let style = CalloutStyle::new(self.variant, theme, font);
        let mut cmd = parent.spawn((CalloutMarker, style));
        cmd.with_children(|cb| {
            cb.spawn((
                Text::new(self.text),
                TextFont { font: font.clone(), font_size: theme.font.size.base, ..default() },
                TextLayout::default(),
                TextColor(theme.color.gray.step12),
            ));
        });
        cmd.id()
    }
}

