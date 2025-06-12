use bevy::prelude::*;

use crate::components::helper::UiBuilder;
use crate::theme::UiTheme;

use super::{BaseCardMarker, BaseCardStyle};

/// Builder for a [`BaseCardMarker`] container.
pub struct BaseCardBuilder {
    content: Option<Box<dyn FnOnce(&mut ChildSpawnerCommands, &UiTheme, &Handle<Font>) + Send + Sync>>,
}

impl BaseCardBuilder {
    /// Creates a new base card builder.
    pub fn new() -> Self {
        Self { content: None }
    }

    /// Provide custom children for the card.
    pub fn content<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut ChildSpawnerCommands, &UiTheme, &Handle<Font>) + Send + Sync + 'static,
    {
        self.content = Some(Box::new(f));
        self
    }
}

impl<'w, 's> UiBuilder<'w, 's> for BaseCardBuilder {
    type Output = Entity;

    fn spawn(self, parent: &'s mut ChildSpawnerCommands<'w>, theme: &UiTheme, font: &Handle<Font>) -> Self::Output {
        let style = BaseCardStyle::default();
        let mut cmd = parent.spawn((BaseCardMarker, style));
        if let Some(content_fn) = self.content {
            cmd.with_children(|cb| {
                content_fn(cb, theme, font);
            });
        }
        cmd.id()
    }
}

