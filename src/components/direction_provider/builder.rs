use bevy::prelude::*;

use crate::components::helper::UiBuilder;
use crate::theme::UiTheme;

use super::{DirectionProvider, ReadingDirection};

/// Builder for a [`DirectionProvider`] component.
pub struct DirectionProviderBuilder {
    dir: ReadingDirection,
    content: Option<Box<dyn FnOnce(&mut ChildSpawnerCommands, ReadingDirection, &UiTheme, &Handle<Font>) + Send + Sync>>,
}

impl DirectionProviderBuilder {
    /// Create a new direction provider with the given direction.
    pub fn new(dir: ReadingDirection) -> Self {
        Self { dir, content: None }
    }

    /// Provide child content for this provider.
    pub fn content<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut ChildSpawnerCommands, ReadingDirection, &UiTheme, &Handle<Font>) + Send + Sync + 'static,
    {
        self.content = Some(Box::new(f));
        self
    }
}

impl<'w, 's> UiBuilder<'w, 's> for DirectionProviderBuilder {
    type Output = Entity;

    fn spawn(self, parent: &'s mut ChildSpawnerCommands<'w>, theme: &UiTheme, font: &Handle<Font>) -> Self::Output {
        let dir_copy = self.dir;
        let mut cmd = parent.spawn(DirectionProvider { dir: self.dir });
        if let Some(func) = self.content {
            cmd.with_children(|cb| {
                func(cb, dir_copy, theme, font);
            });
        }
        cmd.id()
    }
}
