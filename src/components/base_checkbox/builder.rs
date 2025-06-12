use bevy::prelude::*;

use crate::components::helper::UiBuilder;
use crate::theme::UiTheme;

use super::{BaseCheckboxMarker, BaseCheckboxStyle};

/// Builder for a [`BaseCheckboxMarker`] container.
pub struct BaseCheckboxBuilder {
    content: Option<Box<dyn FnOnce(&mut ChildSpawnerCommands, &UiTheme, &Handle<Font>) + Send + Sync>>,
}

impl BaseCheckboxBuilder {
    /// Creates a new base checkbox builder.
    pub fn new() -> Self {
        Self { content: None }
    }

    /// Provide custom children for the checkbox.
    pub fn content<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut ChildSpawnerCommands, &UiTheme, &Handle<Font>) + Send + Sync + 'static,
    {
        self.content = Some(Box::new(f));
        self
    }
}

impl<'w, 's> UiBuilder<'w, 's> for BaseCheckboxBuilder {
    type Output = Entity;

    fn spawn(self, parent: &'s mut ChildSpawnerCommands<'w>, theme: &UiTheme, font: &Handle<Font>) -> Self::Output {
        let style = BaseCheckboxStyle::default();
        let mut cmd = parent.spawn((BaseCheckboxMarker, style));
        if let Some(content_fn) = self.content {
            cmd.with_children(|cb| {
                content_fn(cb, theme, font);
            });
        }
        cmd.id()
    }
}

