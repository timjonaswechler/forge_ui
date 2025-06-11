use bevy::prelude::*;

use crate::components::helper::UiBuilder;
use crate::theme::UiTheme;

use super::{CollectionMarker, CollectionStyle};

/// Builder for a generic collection container.
pub struct CollectionBuilder {
    content: Option<Box<dyn FnOnce(&mut ChildSpawnerCommands, &UiTheme, &Handle<Font>) + Send + Sync>>,
}

impl CollectionBuilder {
    /// Creates a new empty collection.
    pub fn new() -> Self {
        Self { content: None }
    }

    /// Provide custom children for the collection container.
    pub fn content<F>(mut self, func: F) -> Self
    where
        F: FnOnce(&mut ChildSpawnerCommands, &UiTheme, &Handle<Font>) + Send + Sync + 'static,
    {
        self.content = Some(Box::new(func));
        self
    }
}

impl<'w, 's> UiBuilder<'w, 's> for CollectionBuilder {
    type Output = Entity;

    fn spawn(self, parent: &'s mut ChildSpawnerCommands<'w>, theme: &UiTheme, font: &Handle<Font>) -> Self::Output {
        let style = CollectionStyle::new(theme);
        let mut cmd = parent.spawn((CollectionMarker, style));
        if let Some(content_fn) = self.content {
            cmd.with_children(|cb| {
                content_fn(cb, theme, font);
            });
        }
        cmd.id()
    }
}

