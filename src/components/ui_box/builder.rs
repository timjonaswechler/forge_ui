use bevy::prelude::*;

use crate::components::helper::UiBuilder;
use crate::theme::UiTheme;
use super::{BoxMarker, BoxStyle};

/// Builder for a simple container box element.
pub struct BoxBuilder {
    color: Option<Color>,
    padding: Option<UiRect>,
    margin: Option<UiRect>,
    width: Option<Val>,
    height: Option<Val>,
    content: Option<Box<dyn FnOnce(&mut ChildSpawnerCommands, &UiTheme, &Handle<Font>) + Send + Sync>>,
}

impl BoxBuilder {
    /// Creates a new box builder with default styling.
    pub fn new() -> Self {
        Self {
            color: None,
            padding: None,
            margin: None,
            width: None,
            height: None,
            content: None,
        }
    }

    /// Sets a custom background color.
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Sets custom padding for the box.
    pub fn padding(mut self, padding: UiRect) -> Self {
        self.padding = Some(padding);
        self
    }

    /// Sets custom margin for the box.
    pub fn margin(mut self, margin: UiRect) -> Self {
        self.margin = Some(margin);
        self
    }

    /// Sets explicit width.
    pub fn width(mut self, width: Val) -> Self {
        self.width = Some(width);
        self
    }

    /// Sets explicit height.
    pub fn height(mut self, height: Val) -> Self {
        self.height = Some(height);
        self
    }

    /// Provide custom children for the box.
    pub fn content<F>(mut self, func: F) -> Self
    where
        F: FnOnce(&mut ChildSpawnerCommands, &UiTheme, &Handle<Font>) + Send + Sync + 'static,
    {
        self.content = Some(Box::new(func));
        self
    }
}

impl<'w, 's> UiBuilder<'w, 's> for BoxBuilder {
    type Output = Entity;

    fn spawn(self, parent: &'s mut ChildSpawnerCommands<'w>, theme: &UiTheme, font: &Handle<Font>) -> Self::Output {
        let mut style = BoxStyle::new(theme);
        if let Some(p) = self.padding { style.node.padding = p; }
        if let Some(m) = self.margin { style.node.margin = m; }
        if let Some(w) = self.width { style.node.width = w; }
        if let Some(h) = self.height { style.node.height = h; }
        if let Some(c) = self.color { style.background_color = BackgroundColor(c); }

        let mut cmd = parent.spawn((BoxMarker, style));
        if let Some(content_fn) = self.content {
            cmd.with_children(|cb| {
                content_fn(cb, theme, font);
            });
        }
        cmd.id()
    }
}

