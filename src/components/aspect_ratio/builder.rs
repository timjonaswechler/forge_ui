use bevy::prelude::*;

use crate::components::helper::UiBuilder;
use crate::theme::UiTheme;

use super::{AspectRatioMarker, AspectRatioStyle};

/// Builder for an [`AspectRatioMarker`] container maintaining a fixed ratio.
pub struct AspectRatioBuilder {
    ratio: f32,
    content: Option<Box<dyn FnOnce(&mut ChildSpawnerCommands, &UiTheme, &Handle<Font>) + Send + Sync>>,
}

impl AspectRatioBuilder {
    /// Creates a new aspect ratio container with the given `ratio` (width / height).
    pub fn new(ratio: f32) -> Self {
        Self { ratio, content: None }
    }

    /// Provide custom child content for the container.
    pub fn content<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut ChildSpawnerCommands, &UiTheme, &Handle<Font>) + Send + Sync + 'static,
    {
        self.content = Some(Box::new(f));
        self
    }
}

impl<'w, 's> UiBuilder<'w, 's> for AspectRatioBuilder {
    type Output = Entity;

    fn spawn(self, parent: &'s mut ChildSpawnerCommands<'w>, theme: &UiTheme, font: &Handle<Font>) -> Self::Output {
        let mut style = AspectRatioStyle::default();
        style.node.aspect_ratio = Some(self.ratio);
        let mut cmd = parent.spawn((AspectRatioMarker, style));

        if let Some(content_fn) = self.content {
            cmd.with_children(|cb| {
                content_fn(cb, theme, font);
            });
        }

        cmd.id()
    }
}
