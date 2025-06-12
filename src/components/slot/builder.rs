use bevy::prelude::*;

use crate::components::helper::UiBuilder;
use crate::theme::UiTheme;

use super::{SlotRootMarker, SlottableMarker};

/// Builder for a [`SlotRootMarker`] container.
pub struct SlotBuilder {
    content: Option<Box<dyn FnOnce(&mut ChildSpawnerCommands, &UiTheme, &Handle<Font>) + Send + Sync>>,
}

impl SlotBuilder {
    /// Creates an empty slot builder.
    pub fn new() -> Self {
        Self { content: None }
    }

    /// Provide custom children for the slot.
    pub fn content<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut ChildSpawnerCommands, &UiTheme, &Handle<Font>) + Send + Sync + 'static,
    {
        self.content = Some(Box::new(f));
        self
    }
}

impl<'w, 's> UiBuilder<'w, 's> for SlotBuilder {
    type Output = Entity;

    fn spawn(self, parent: &'s mut ChildSpawnerCommands<'w>, theme: &UiTheme, font: &Handle<Font>) -> Self::Output {
        let mut cmd = parent.spawn((SlotRootMarker, Node::default()));
        if let Some(content_fn) = self.content {
            cmd.with_children(|cb| {
                content_fn(cb, theme, font);
            });
        }
        cmd.id()
    }
}

/// Builder for a [`SlottableMarker`] wrapper.
pub struct SlottableBuilder {
    content: Option<Box<dyn FnOnce(&mut ChildSpawnerCommands, &UiTheme, &Handle<Font>) + Send + Sync>>,
}

impl SlottableBuilder {
    /// Creates a new slottable builder.
    pub fn new() -> Self {
        Self { content: None }
    }

    /// Provide custom children for the slottable element.
    pub fn content<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut ChildSpawnerCommands, &UiTheme, &Handle<Font>) + Send + Sync + 'static,
    {
        self.content = Some(Box::new(f));
        self
    }
}

impl<'w, 's> UiBuilder<'w, 's> for SlottableBuilder {
    type Output = Entity;

    fn spawn(self, parent: &'s mut ChildSpawnerCommands<'w>, theme: &UiTheme, font: &Handle<Font>) -> Self::Output {
        let mut cmd = parent.spawn((SlottableMarker, Node::default()));
        if let Some(content_fn) = self.content {
            cmd.with_children(|cb| {
                content_fn(cb, theme, font);
            });
        }
        cmd.id()
    }
}

