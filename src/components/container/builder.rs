use bevy::prelude::*;

use crate::components::helper::UiBuilder;
use crate::theme::UiTheme;

use super::{ContainerMarker, ContainerStyle};

/// Builder for a responsive container element.
pub struct ContainerBuilder {
    padding: Option<UiRect>,
    width: Option<Val>,
    content: Option<Box<dyn FnOnce(&mut ChildSpawnerCommands, &UiTheme, &Handle<Font>) + Send + Sync>>,
}

impl ContainerBuilder {
    /// Creates a new container with default styling.
    pub fn new() -> Self {
        Self { padding: None, width: None, content: None }
    }

    /// Sets custom padding for the container.
    pub fn padding(mut self, padding: UiRect) -> Self {
        self.padding = Some(padding);
        self
    }

    /// Sets an explicit width for the container.
    pub fn width(mut self, width: Val) -> Self {
        self.width = Some(width);
        self
    }

    /// Provide custom children for the container.
    pub fn content<F>(mut self, func: F) -> Self
    where
        F: FnOnce(&mut ChildSpawnerCommands, &UiTheme, &Handle<Font>) + Send + Sync + 'static,
    {
        self.content = Some(Box::new(func));
        self
    }
}

impl<'w, 's> UiBuilder<'w, 's> for ContainerBuilder {
    type Output = Entity;

    fn spawn(self, parent: &'s mut ChildSpawnerCommands<'w>, theme: &UiTheme, font: &Handle<Font>) -> Self::Output {
        let mut style = ContainerStyle::new(theme);
        if let Some(p) = self.padding { style.node.padding = p; }
        if let Some(w) = self.width { style.node.width = w; }

        let mut cmd = parent.spawn((ContainerMarker, style));
        if let Some(content_fn) = self.content {
            cmd.with_children(|cb| {
                content_fn(cb, theme, font);
            });
        }
        cmd.id()
    }
}

