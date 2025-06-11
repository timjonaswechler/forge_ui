use bevy::prelude::*;

use crate::components::helper::UiBuilder;
use crate::theme::UiTheme;
use super::components::BlockquoteMarker;
use super::style::BlockquoteStyle;

/// Builder for a blockquote element.
pub struct BlockquoteBuilder {
    text: String,
    citation: Option<String>,
}

impl BlockquoteBuilder {
    /// Creates a new blockquote with the given text.
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into(), citation: None }
    }

    /// Sets an optional citation text displayed below the quote.
    pub fn cite(mut self, cite: impl Into<String>) -> Self {
        self.citation = Some(cite.into());
        self
    }
}

impl<'w, 's> UiBuilder<'w, 's> for BlockquoteBuilder {
    type Output = Entity;

    fn spawn(self, parent: &'s mut ChildSpawnerCommands<'w>, theme: &UiTheme, font: &Handle<Font>) -> Self::Output {
        let style = BlockquoteStyle::new(theme);
        let mut cmd = parent.spawn((BlockquoteMarker, style));
        cmd.with_children(|p| {
            p.spawn((
                Text::new(self.text.clone()),
                TextFont { font: font.clone(), font_size: theme.font.size.base, ..default() },
                TextLayout::default(),
                TextColor(theme.color.gray.step12),
            ));
            if let Some(cite) = self.citation {
                p.spawn((
                    Text::new(cite),
                    TextFont { font: font.clone(), font_size: theme.font.size.sm, ..default() },
                    TextLayout::default(),
                    TextColor(theme.color.gray.step11),
                    Name::new("cite"),
                ));
            }
        });
        cmd.id()
    }
}
