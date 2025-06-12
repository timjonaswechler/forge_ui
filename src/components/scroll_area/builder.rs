use bevy::prelude::*;

use crate::components::helper::UiBuilder;
use crate::theme::UiTheme;

use super::{ScrollAreaMarker, ScrollAreaViewportMarker};
use super::style::{ScrollAreaStyle, ScrollAreaViewportStyle};

/// Builder for a simple scroll area.
pub struct ScrollAreaBuilder {
    width: Option<Val>,
    height: Option<Val>,
    content: Option<Box<dyn FnOnce(&mut ChildSpawnerCommands, &UiTheme, &Handle<Font>) + Send + Sync>>,
}

impl Default for ScrollAreaBuilder {
    fn default() -> Self {
        Self { width: None, height: None, content: None }
    }
}

impl ScrollAreaBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn width(mut self, width: Val) -> Self {
        self.width = Some(width);
        self
    }

    pub fn height(mut self, height: Val) -> Self {
        self.height = Some(height);
        self
    }

    pub fn content<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut ChildSpawnerCommands, &UiTheme, &Handle<Font>) + Send + Sync + 'static,
    {
        self.content = Some(Box::new(f));
        self
    }
}

impl<'w, 's> UiBuilder<'w, 's> for ScrollAreaBuilder {
    type Output = Entity;

    fn spawn(self, parent: &'s mut ChildSpawnerCommands<'w>, theme: &UiTheme, font: &Handle<Font>) -> Self::Output {
        let mut style = ScrollAreaStyle::new(theme);
        if let Some(w) = self.width { style.node.width = w; }
        if let Some(h) = self.height { style.node.height = h; }
        let mut root = parent.spawn((ScrollAreaMarker, style, Name::new("ScrollArea")));
        root.with_children(|rc| {
            let mut v_style = ScrollAreaViewportStyle::new(theme);
            if let Some(w) = self.width { v_style.node.width = w; }
            if let Some(h) = self.height { v_style.node.min_height = h; }
            let mut viewport = rc.spawn((ScrollAreaViewportMarker, v_style));
            if let Some(f) = self.content {
                viewport.with_children(|vc| {
                    f(vc, theme, font);
                });
            }
        });
        root.id()
    }
}

