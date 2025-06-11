use bevy::prelude::*;

use crate::components::helper::UiBuilder;
use crate::theme::UiTheme;

use super::{CollapsibleMarker, CollapsibleState, CollapsibleStyle};

/// Builder for a collapsible container that can show or hide its children.
pub struct CollapsibleBuilder {
    open: bool,
    content: Option<Box<dyn FnOnce(&mut ChildSpawnerCommands, &UiTheme, &Handle<Font>) + Send + Sync>>,
}

impl CollapsibleBuilder {
    /// Creates a new collapsible container.
    pub fn new() -> Self {
        Self { open: false, content: None }
    }

    /// Sets whether the collapsible starts open.
    pub fn open(mut self, flag: bool) -> Self {
        self.open = flag;
        self
    }

    /// Provide custom children for the collapsible.
    pub fn content<F>(mut self, func: F) -> Self
    where
        F: FnOnce(&mut ChildSpawnerCommands, &UiTheme, &Handle<Font>) + Send + Sync + 'static,
    {
        self.content = Some(Box::new(func));
        self
    }
}

impl<'w, 's> UiBuilder<'w, 's> for CollapsibleBuilder {
    type Output = Entity;

    fn spawn(self, parent: &'s mut ChildSpawnerCommands<'w>, theme: &UiTheme, font: &Handle<Font>) -> Self::Output {
        let mut style = CollapsibleStyle::new(theme);
        if !self.open {
            style.visibility = Visibility::Hidden;
        }
        let mut cmd = parent.spawn((CollapsibleMarker, CollapsibleState { open: self.open }, style));
        if let Some(content_fn) = self.content {
            cmd.with_children(|cb| {
                content_fn(cb, theme, font);
            });
        }
        cmd.id()
    }
}

